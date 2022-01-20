use async_session::serde_json::{self, json};
use axum::{
    response::{IntoResponse, Redirect, Response},
    Json,
};
use oauth2::{basic::BasicErrorResponseType, RequestTokenError, StandardErrorResponse};
use reqwest::StatusCode;
use thiserror::Error;
use tracing::debug;
use anyhow::anyhow;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("session value serialize error")]
    FailedToSerializeSessionValue(#[source] serde_json::Error),
    #[error("session error")]
    Session(#[source] anyhow::Error),
    #[error("twitter resource error")]
    TwitterResource(#[source] reqwest::Error),
}

impl ServerError {
    pub fn status(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {

        let message = format!("{}", &self);

        let status = self.status();

        let a = anyhow!(self);
        debug!(error = %a, "ServerError occurred");

        let body = Json(json!({
            "cause": message,
        }));

        (status, body).into_response()
    }
}

pub type Result<T, E = ServerError> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("redirect")]
    Redirect,
    #[error("csrf mis match")]
    CSRFMisMatch,
    #[error("server error")]
    ServerError(#[from] ServerError),
    #[error("token request")]
    TokenRequest(
        #[from]
        RequestTokenError<
            oauth2::reqwest::Error<reqwest::Error>,
            StandardErrorResponse<BasicErrorResponseType>,
        >,
    ),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        debug!(error = %self, "AuthError occurred");

        match self {
            AuthError::Redirect => Redirect::temporary("/".parse().unwrap()).into_response(),
            AuthError::CSRFMisMatch => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "cause": "csrf mis match",
                })),
            )
                .into_response(),
            AuthError::ServerError(se) => se.into_response(),
            AuthError::TokenRequest(err) => (
                match err {
                    RequestTokenError::ServerResponse(_) => StatusCode::BAD_REQUEST,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                },
                Json(json!({
                    "cause": "failed to request token",
                })),
            )
                .into_response(),
        }
    }
}
