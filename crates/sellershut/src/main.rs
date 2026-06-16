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
use tokio::{net::TcpListener, task::AbortHandle};
use tower_sessions::ExpiredDeletion;
use tracing::{error, info};

use crate::{
    config::{Args, Commands, Config},
    server::OauthProvider,
    state::{AppState, cache::RedisClient},
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

    let state = AppState::builder()
        .http_client(reqwest::Client::new())
        .vault(&config.server.vault)
        .await
        .inspect_err(|e| {
            error!(error = %e, "Failed to connect to vault");
        })?
        .log_handle(log_handle)
        .database(&config.server.database)
        .await
        .inspect_err(|e| {
            error!(error = %e, "Database");
        })?
        .oauth_clients(oauth_clients.into())
        .cache(
            RedisClient::new(&config.server.cache)
                .await
                .inspect_err(|e| {
                    error!(error = %e, "Failed to connect to cache");
                })?,
        )
        .build();

    let session_store = state.session_store.clone();

    let app = server::router(state).await?;

    let session_cleanup_task = tokio::task::spawn(
        session_store
            .clone()
            .continuously_delete_expired(tokio::time::Duration::from_secs(60)),
    );

    let addr = SocketAddr::from((Ipv6Addr::UNSPECIFIED, config.server.port));

    let listener = TcpListener::bind(addr).await?;
    info!(addr = ?listener.local_addr().expect("local addr"), "starting server");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(session_cleanup_task.abort_handle()))
        .await?;

    let _ = session_cleanup_task.await;

    Ok(())
}

async fn shutdown_signal(deletion_task_abort_handle: AbortHandle) {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => deletion_task_abort_handle.abort(),
        _ = terminate => deletion_task_abort_handle.abort(),
    }
}
