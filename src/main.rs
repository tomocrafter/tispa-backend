use anyhow::{Context, Result};
use async_redis_session::RedisSessionStore;
use axum::{response::Html, routing::get, AddExtensionLayer, Router};
use std::net::SocketAddr;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8989));
    println!("listening on {}", addr);

    let redis_url = std::env::var("REDIS_URL").context("you must set redis url")?;

    let store = RedisSessionStore::new(redis_url)?;

    let app = Router::new()
        .route("/", get(handler))
        .layer(AddExtensionLayer::new(store));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

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

    println!("signal received, starting graceful shutdown");
}
