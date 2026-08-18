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
use sellershut_core::types::user::ActorType;
use sellershut_users::{CreateUser, UserDriver, UserService};
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

    let database = config.database.connect().await?;
    let auth = AuthService::new(database.clone(), config.server.oauth.0.clone())?;
    let user = UserService::new(database);

    let system_user = if let Some(user) = user.get_user(&config.server.instance_name).await? {
        user
    } else {
        //create system user
        let keypair = activitypub_federation::http_signatures::generate_actor_keypair()?;
        let inbox = server::utilities::inbox_url(
            config.server.port.into(),
            &config.server.domain,
            env!("CARGO_PKG_NAME"),
        )?;
        let data = CreateUser {
            kind: ActorType::Service,
            username: config.server.instance_name.clone(),
            name: None,
            inbox,
            public_key: keypair.public_key,
            private_key: Some(keypair.private_key),
            is_local: true,
        };
        user.create_user(&data).await?
    };

    let state = AppState {
        auth: Arc::new(auth),
        user: Arc::new(user),
    };

    let app = server::router::router(state, config).await?;

    let listener = TcpListener::bind(addr).await?;
    info!(addr = ?listener.local_addr().expect("local addr"), "starting server");

    axum::serve(listener, app).await?;

    Ok(())
}
