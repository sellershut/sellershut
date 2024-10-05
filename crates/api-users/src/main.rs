use std::path::PathBuf;

use anyhow::Result;
use api_users::entity::auth::{Configuration, OauthDetails};
use infra::{tracing::Telemetry, Services};

use clap::Parser;
use serde::Deserialize;

/// api-users
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to config file
    #[arg(short, long)]
    config_file: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let config = config::Config::builder()
        .add_source(config::File::new(
            args.config_file
                .to_str()
                .expect("config file path is not valid"),
            config::FileFormat::Toml,
        ))
        .build()?;
    let config = config.try_deserialize::<Configuration>()?;

    let _tracing = Telemetry::builder().build();

    let services = Services::builder()
        .with_postgres(&config.base.database)
        .await?
        .build()?;

    api_users::serve(services, config).await
}
