use std::env;

use anyhow::{Context as _, Result};
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use once_cell::sync::Lazy;

pub static TWITTER_CLIENT_ID: Lazy<String> = Lazy::new(|| {
    env::var("TWITTER_CLIENT_ID")
        .context("Missing TWITTER_CLIENT_ID!")
        .unwrap()
});

pub fn oauth_client() -> Result<BasicClient> {
    let client_id = env::var("TWITTER_CLIENT_ID").context("Missing TWITTER_CLIENT_ID!")?;
    let client_secret =
        env::var("TWITTER_CLIENT_SECRET").context("Missing TWITTER_CLIENT_SECRET!")?;

    let redirect_url = env::var("REDIRECT_URL")
        .unwrap_or_else(|_| "http://localhost:8989/auth/callback".to_string());

    let auth_url = env::var("AUTH_URL")
        .unwrap_or_else(|_| "https://twitter.com/i/oauth2/authorize".to_string());

    let token_url = env::var("TOKEN_URL")
        .unwrap_or_else(|_| "https://api.twitter.com/2/oauth2/token".to_string());

    Ok(BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new(auth_url).unwrap(),
        Some(TokenUrl::new(token_url).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap()))
}
