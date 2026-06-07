use std::net::{Ipv6Addr, SocketAddr};

use tokio::net::TcpListener;
use tracing::info;

use crate::state::AppState;

pub mod logs;
pub mod server;
pub mod state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logs::log();
    let app = server::router(AppState).await;

    let addr = SocketAddr::from((Ipv6Addr::UNSPECIFIED, 2210));

    let listener = TcpListener::bind(addr).await?;
    info!(addr = ?listener.local_addr().unwrap(), "starting server");

    axum::serve(listener, app).await?;

    Ok(())
}
