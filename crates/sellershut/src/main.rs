use std::{
    collections::HashMap,
    net::{Ipv6Addr, SocketAddr},
};

use auth::BasicClient;
use clap::Parser;
use figment::{
    Figment,
    providers::{Env, Format as _, Toml},
};
use tokio::net::TcpListener;
use tracing::info;

use crate::{
    config::{Args, Commands, Config},
    server::OauthProvider,
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

    let mut config = Figment::new();
    if let Some(file) = args.config {
        config = config.merge(Toml::file(file));
    }
    let config: Config = config
        .merge(
            Env::prefixed("HUT_")
                .split("__")
                // replace - with _ in envs
                .map(|v| v.to_string().to_ascii_lowercase().replace("-", "_").into()),
        )
        .extract()?;

    let (log_handle, _log_guard) = logs::log(
        &config.server.logging.log_level,
        &config.server.logging.log_directory,
    )?;

    let mut oauth_clients = HashMap::default();
    let discord = BasicClient::try_from(&config.auth.discord)?;
    oauth_clients.insert(OauthProvider::Discord, discord);

    let app = server::router(
        AppState::builder()
            .vault(&config.server.vault)
            .await?
            .log_handle(log_handle)
            .database(&config.server.database)
            .await?
            .oauth_clients(oauth_clients.into())
            .build(),
    )
    .await;

    let addr = SocketAddr::from((Ipv6Addr::UNSPECIFIED, config.server.port));

    let listener = TcpListener::bind(addr).await?;
    info!(addr = ?listener.local_addr().expect("local addr"), "starting server");

    axum::serve(listener, app).await?;

    Ok(())
}
