pub mod entity;
pub mod server;
pub mod state;

use std::{
    net::{Ipv6Addr, SocketAddr},
    time::Duration,
};

use anyhow::Result;
use axum::{extract::Request, http::header::CONTENT_TYPE};
use entity::auth::Configuration;
use infra::Services;
use server::{
    apply_middleware, grpc,
    web::{
        self,
        routes::auth::{github::github_oauth_client, session::PostgresSessionStore},
    },
};
use state::AppState;
use tower::{make::Shared, steer::Steer};
use tracing::info;

pub async fn serve(services: Services, config: Configuration) -> Result<()> {
    sqlx::migrate!("./migrations")
        .run(&services.postgres)
        .await?;

    let session_store = PostgresSessionStore::from_client(services.postgres.clone());

    session_store.spawn_cleanup_task(Duration::from_secs(1));

    let client = github_oauth_client(
        &config.oauth.github,
        web::routes::auth::OAuthProvider::GitHub,
    )
    .unwrap();

    let http_client = reqwest::Client::new();

    let port = config.base.port;
    let state = AppState {
        config: config.base,
        services,
        github_client: client,
        session_store,
        http_client,
    };

    let web = apply_middleware(web::router(state.clone()));
    let grpc = apply_middleware(grpc::router(state.clone())?);

    let service = Steer::new(vec![web, grpc], |req: &Request, _services: &[_]| {
        if req
            .headers()
            .get(CONTENT_TYPE)
            .map(|content_type| content_type.as_bytes())
            .filter(|content_type| content_type.starts_with(b"application/grpc"))
            .is_some()
        {
            1
        } else {
            0
        }
    });

    let addr = SocketAddr::from((Ipv6Addr::UNSPECIFIED, port));

    let listener = tokio::net::TcpListener::bind(addr).await?;

    let socket_addr = listener.local_addr()?;

    info!(addr = ?socket_addr, "listening");

    axum::serve(listener, Shared::new(service)).await?;

    Ok(())
}
