use anyhow::Context;
use async_redis_session::RedisSessionStore;
use async_session::{async_trait, Session, SessionStore};
use axum::{
    extract::{
        rejection::TypedHeaderRejectionReason, Extension, FromRequest, Query, RequestParts,
        TypedHeader,
    },
    http::header,
    response::{IntoResponse, Redirect},
};
use headers::HeaderMap;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthorizationCode, CsrfToken,
    PkceCodeChallenge, Scope, TokenResponse,
};
use serde::Deserialize;
use std::time::Duration;

use crate::{
    errors::{Result, ServerError},
    oauth::TWITTER_CLIENT_ID,
    twitter::{TwitterUser, V2TwitterResponse},
    COOKIE_NAME,
};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct AuthRequest {
    code: String,
    state: String,
}

#[tracing::instrument(skip(store, client))]
pub async fn twitter_auth(
    Extension(store): Extension<RedisSessionStore>,
    Extension(client): Extension<BasicClient>,
) -> Result<impl IntoResponse> {
    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
    let (auth_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scopes([
            Scope::new("users.read".to_string()),
            Scope::new("tweet.read".to_string()),
            Scope::new("offline.access".to_string()),
        ])
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    let mut session = Session::new();
    session.expire_in(Duration::from_secs(60 * 60));
    session
        .insert("csrf_state", csrf_state)
        .context("failed to put csrf_state to session")?;
    session
        .insert("pkce_code_verifier", pkce_code_verifier)
        .context("failed to put pkce_code_verifier to session")?;

    let cookie_value = store
        .store_session(session)
        .await
        .context("failed to store session: {}")?
        .context("cookie value already extracted")?;
    let cookie = format!("{}={}; SameSite=Lax; Path=/", COOKIE_NAME, cookie_value);

    let mut headers = HeaderMap::new();
    headers.insert(header::SET_COOKIE, cookie.parse().unwrap());

    // Redirect to Discord's oauth service
    Ok((headers, Redirect::to(auth_url.to_string().parse().unwrap())))
}

#[tracing::instrument(skip(store, oauth_client, cookies))]
pub async fn callback(
    Query(query): Query<AuthRequest>,
    Extension(store): Extension<RedisSessionStore>,
    Extension(oauth_client): Extension<BasicClient>,
    TypedHeader(cookies): TypedHeader<headers::Cookie>,
) -> Result<impl IntoResponse> {
    let code = AuthorizationCode::new(query.code);
    let state = CsrfToken::new(query.state);

    let session_id = match cookies.get(COOKIE_NAME) {
        Some(id) => id,
        None => return Ok(Redirect::temporary("/".parse().unwrap()).into_response()),
    };
    let session = store
        .load_session(session_id.to_string())
        .await
        .context("failed to store session")?;
    let session = match session {
        Some(session) => session,
        None => return Ok(Redirect::temporary("/".parse().unwrap()).into_response()),
    };

    let csrf_state = match session.get::<CsrfToken>("csrf_state") {
        Some(csrf_state) => csrf_state,
        None => return Ok(Redirect::temporary("/".parse().unwrap()).into_response()),
    };
    if state.secret() != csrf_state.secret() {
        return Err(ServerError::Unauthorized);
    }

    let pkce_code_verifier = match session.get("pkce_code_verifier") {
        Some(pkce_code_verifier) => pkce_code_verifier,
        None => return Ok(Redirect::temporary("/".parse().unwrap()).into_response()),
    };

    // Get an auth token
    let token = oauth_client
        .exchange_code(code)
        .add_extra_param("client_id", &*TWITTER_CLIENT_ID)
        .set_pkce_verifier(pkce_code_verifier)
        .request_async(async_http_client)
        .await
        .context("failed to exchange code with twitter")?;

    let client = reqwest::Client::new();
    let user_data = client
        .get("https://api.twitter.com/2/users/me")
        .bearer_auth(token.access_token().secret())
        .query(&[("user.fields", "profile_image_url")])
        .send()
        .await
        .context("failed to request token to twitter")?
        .json::<V2TwitterResponse<TwitterUser>>()
        .await
        .context("failed to parse response json")?;

    let mut session = Session::new();
    session.insert("user", &user_data.data).unwrap();

    let cookie = store.store_session(session).await.unwrap().unwrap();
    let cookie = format!("{}={}; SameSite=Lax; Path=/", COOKIE_NAME, cookie);

    let mut headers = HeaderMap::new();
    headers.insert(header::SET_COOKIE, cookie.parse().unwrap());

    Ok((headers, Redirect::to("/protected".parse().unwrap())).into_response())
}

pub async fn protected(user: TwitterUser) -> impl IntoResponse {
    format!(
        "Welcome to the protected area :)\nHere's your info:\n{:?}",
        user
    )
}

pub async fn logout(
    Extension(store): Extension<RedisSessionStore>,
    TypedHeader(cookies): TypedHeader<headers::Cookie>,
) -> impl IntoResponse {
    let cookie = cookies.get(COOKIE_NAME).unwrap();
    let session = match store.load_session(cookie.to_string()).await.unwrap() {
        Some(s) => s,
        // No session active, just redirect
        None => return Redirect::to("/".parse().unwrap()),
    };

    store.destroy_session(session).await.unwrap();

    Redirect::to("/".parse().unwrap())
}

pub struct RedirectToIndex;

impl IntoResponse for RedirectToIndex {
    fn into_response(self) -> axum::response::Response {
        Redirect::temporary("/".parse().unwrap()).into_response()
    }
}

#[async_trait]
impl<B> FromRequest<B> for TwitterUser
where
    B: Send,
{
    // If anything goes wrong or no session is found, redirect to the auth page
    type Rejection = RedirectToIndex;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(store) = Extension::<RedisSessionStore>::from_request(req)
            .await
            .expect("`RedisSessionStore` extension is missing");

        let cookies = TypedHeader::<headers::Cookie>::from_request(req)
            .await
            .map_err(|e| match *e.name() {
                header::COOKIE => match e.reason() {
                    TypedHeaderRejectionReason::Missing => RedirectToIndex,
                    _ => panic!("unexpected error getting Cookie header(s): {}", e),
                },
                _ => panic!("unexpected error getting cookies: {}", e),
            })?;
        let session_cookie = cookies.get(COOKIE_NAME).ok_or(RedirectToIndex)?;

        let session = store
            .load_session(session_cookie.to_string())
            .await
            .unwrap()
            .ok_or(RedirectToIndex)?;

        let user = session.get::<TwitterUser>("user").ok_or(RedirectToIndex)?;

        Ok(user)
    }
}
