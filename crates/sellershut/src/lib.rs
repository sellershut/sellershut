use std::net::{Ipv6Addr, SocketAddr};

use activitypub_federation::config::FederationConfig;
use state::AppState;

pub mod entities;
pub mod server;
pub mod state;

pub async fn serve(state: AppState) -> anyhow::Result<()> {
    let addr = SocketAddr::from((Ipv6Addr::UNSPECIFIED, state.config.port));
    let host = addr.to_string();
    let config = FederationConfig::builder()
        .domain(addr.to_string())
        .app_data(state.clone())
        .debug(true)
        .build()
        .await?;

    tokio::join!(
        server::nats::start_consumer(state),
        server::web::router(addr, config),
    );

    Ok(())
}
