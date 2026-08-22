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
use sellershut_auth::OauthDriver;
use sellershut_svc::cache::Cache;
use sellershut_users::UserService;
use tokio::net::TcpListener;
use tracing::info;

use crate::{
    config::cli::{Args, Commands},
    server::state::State,
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

    let (database, cache) = tokio::join!(config.database.connect(), Cache::connect(&config.cache));
    let database = database?;
    let cache = cache?;

    let user = UserService::new(database.clone(), cache);

    let state = State::new(&config, user, database.clone()).await?;

    let app = server::router::router(Arc::clone(&state), config).await?;

    let maintenance_task = tokio::spawn(auth_housekeeping(Arc::clone(&state.auth)));
    let listener = TcpListener::bind(addr).await?;
    info!(addr = ?listener.local_addr().expect("local addr"), "starting server");

    axum::serve(listener, app).await?;

    maintenance_task.abort();

    Ok(())
}

async fn auth_housekeeping(auth: Arc<dyn OauthDriver>) {
    let mut interval = tokio::time::interval(tokio::time::Duration::from_mins(15));

    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

    loop {
        interval.tick().await;

        match auth.house_keep().await {
            Ok(result) => {
                tracing::debug!(
                    oauth_flows_deleted = result.oauth_flows_deleted,
                    pending_logins_deleted = result.pending_logins_deleted,
                    sessions_deleted = result.sessions_deleted,
                    "auth housekeeping completed"
                );
            }

            Err(error) => {
                tracing::error!(
                    %error,
                    "auth maintenance failed"
                );
            }
        }
    }
}
