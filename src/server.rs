pub mod error;
pub mod grpc;
pub mod routes;

use std::time::Duration;

use activitypub_federation::config::{FederationConfig, FederationMiddleware};
use axum::{routing::get, Router};
use tokio::signal;
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};
use tracing::info;

use crate::state::AppHandle;

pub async fn serve(
    tx: tokio::sync::oneshot::Sender<u16>,
    data: FederationConfig<AppHandle>,
) -> anyhow::Result<()> {
    let addr = data.addr;
    // Create a regular axum app.
    let app = Router::new()
        .route("/health", get(routes::health_check))
        .route("/.well-known/webfinger", get(routes::web_finger))
        //.route("/users/{user}", get(routes::users::http_get_user)) -- AXUM 0.8
        .route("/users/:name", get(routes::users::http_get_user))
        .layer(FederationMiddleware::new(data))
        .layer((
            TraceLayer::new_for_http(),
            // Graceful shutdown will wait for outstanding requests to complete. Add a timeout so
            // requests don't hang forever.
            TimeoutLayer::new(Duration::from_secs(10)),
        ));

    // Create a `TcpListener` using tokio.

    let listener = tokio::net::TcpListener::bind(addr).await?;

    let socket_addr = listener
        .local_addr()
        .expect("should get socket_addr from listener");

    tx.send(socket_addr.port())
        .expect("port channel to be open");

    info!(addr = ?socket_addr, "listening");

    // Run the server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
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
}
