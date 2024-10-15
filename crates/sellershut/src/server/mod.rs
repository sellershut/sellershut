mod user;
mod webfinger;

use std::net::SocketAddr;

use activitypub_federation::config::{FederationConfig, FederationMiddleware};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use tracing::info;

use crate::state::AppState;

pub async fn router(addr: SocketAddr, config: FederationConfig<AppState>) -> anyhow::Result<()> {
    let router = Router::new()
        .route("/.well-known/webfinger", get(webfinger::webfinger))
        .route("/:user", get(user::get).put(user::upsert))
        .layer(FederationMiddleware::new(config));

    let listener = tokio::net::TcpListener::bind(addr).await?;

    let socket_addr = listener.local_addr()?;

    info!(addr = ?socket_addr, "listening");

    axum::serve(listener, router).await?;

    Ok(())
}

struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
