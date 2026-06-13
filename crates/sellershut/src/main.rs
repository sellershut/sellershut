use std::net::{Ipv6Addr, SocketAddr};

use clap::Parser;
use figment::{
    Figment,
    providers::{Env, Format as _, Serialized, Toml},
};
use tokio::net::TcpListener;
use tracing::info;

use crate::{
    config::{Args, Commands, Config},
    state::AppState,
};

pub mod config;
pub mod logs;
pub mod server;
pub mod state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    if let Some(Commands::GenerateConfig { output }) = args.command {
        let str = toml::to_string_pretty(&Config::default())?;
        std::fs::write(&output, &str)?;
        println!("Config written to: {:?}", output);
        return Ok(());
    }

    let mut config = Figment::from(Serialized::defaults(Config::default()));
    if let Some(file) = args.config {
        config = config.merge(Toml::file(file));
    }
    let config: Config = config.merge(Env::prefixed("SH_")).extract()?;

    let (log_handle, _log_guard) = logs::log(
        &config.server.logging.log_level,
        &config.server.logging.log_directory,
    )?;
    let app = server::router(AppState::builder().log_handle(log_handle).build()).await;

    let addr = SocketAddr::from((Ipv6Addr::UNSPECIFIED, config.server.port));

    let listener = TcpListener::bind(addr).await?;
    info!(addr = ?listener.local_addr().expect("local addr"), "starting server");

    axum::serve(listener, app).await?;

    Ok(())
}
