use std::net::{Ipv6Addr, SocketAddr};

use tokio::net::TcpListener;
use tracing::info;

use crate::state::AppState;

pub mod logs;
pub mod server;
pub mod state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (log_handle, _log_guard) = logs::log(None, None)?;
    let app = server::router(AppState::builder().log_handle(log_handle).build()).await;

    let addr = SocketAddr::from((Ipv6Addr::UNSPECIFIED, 2210));

    let listener = TcpListener::bind(addr).await?;
    info!(addr = ?listener.local_addr().expect("local addr"), "starting server");

    axum::serve(listener, app).await?;

    Ok(())
}
