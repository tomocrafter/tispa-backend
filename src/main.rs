use anyhow::Context as _;
use async_redis_session::RedisSessionStore;

use axum::{http::Request, response::Html, routing::get, AddExtensionLayer, Router};
use sqlx::postgres::PgPoolOptions;

use std::net::SocketAddr;
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::{
    request_id::{MakeRequestId, RequestId},
    trace::TraceLayer,
    ServiceBuilderExt,
};
use tracing::{debug, info};
use uuid::Uuid;

mod errors;
mod oauth;
mod routing;
mod twitter;

static COOKIE_NAME: &str = "SESSION_ID";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "tispa_backend=debug,tower_http=debug")
    }
    tracing_subscriber::fmt::init();

    let db = PgPoolOptions::new()
        .max_connections(50)
        .connect(&std::env::var("DATABASE_URL").context("you must set database url")?)
        .await
        .context("could not connect to database_url")?;

    sqlx::migrate!().run(&db).await?;

    let addr = SocketAddr::from(([0, 0, 0, 0], 8989));
    info!("listening on {}", addr);

    let redis_url = std::env::var("REDIS_URL").context("you must set redis url")?;

    let store = RedisSessionStore::new(redis_url)?;

    let oauth_client = oauth::oauth_client()?;

    let app = Router::new()
        .route("/", get(handler))
        .nest(
            "/auth",
            Router::new()
                .route("/twitter", get(routing::oauth::twitter_auth))
                .route("/callback", get(routing::oauth::callback)),
        )
        .route("/protected", get(routing::oauth::protected))
        .route("/logout", get(routing::oauth::logout))
        .layer(
            ServiceBuilder::new()
                .set_x_request_id(MakeRequestUuid)
                .layer(AddExtensionLayer::new(store))
                .layer(AddExtensionLayer::new(oauth_client))
                .layer(TraceLayer::new_for_http()),
        );

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

#[derive(Clone, Copy)]
pub struct MakeRequestUuid;

impl MakeRequestId for MakeRequestUuid {
    fn make_request_id<B>(&mut self, _request: &Request<B>) -> Option<RequestId> {
        let request_id = Uuid::new_v4().to_string().parse().unwrap();
        Some(RequestId::new(request_id))
    }
}

#[tracing::instrument]
async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    debug!("signal received, starting graceful shutdown");
}
