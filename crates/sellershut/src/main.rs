mod config;
mod logger;
mod server;
#[cfg(test)]
mod test;

use std::{
    net::{Ipv6Addr, SocketAddr},
    sync::Arc,
};

use anyhow::Result;
use clap::Parser;
use sellershut_auth::AuthService;
use tokio::net::TcpListener;
use tracing::info;

use crate::{
    config::cli::{Args, Commands},
    server::state::AppState,
};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    if let Some(Commands::GenerateConfig { output }) = args.command {
        let str = toml::to_string_pretty(&config::Configuration::default())?;
        std::fs::write(&output, &str)?;
        println!("Config written to: {:?}", output);
        return Ok(());
    }
    let config = config::load(args.config.as_ref());

    let (_log_handle, _log_guard) = logger::log(&config.log)?;

    let addr = SocketAddr::from((Ipv6Addr::UNSPECIFIED, config.server.port.into()));

    let database = config.database.auth.connect().await?;
    let auth = AuthService::new(database, config.server.oauth.0.clone())?;

    let state = AppState {
        auth: Arc::new(auth),
        cookie_secure: false,
    };

    let app = server::router::router(state, config).await?;

    let listener = TcpListener::bind(addr).await?;
    info!(addr = ?listener.local_addr().expect("local addr"), "starting server");

    axum::serve(listener, app).await?;

    Ok(())
}
