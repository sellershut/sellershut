use std::path::PathBuf;

use anyhow::Result;
use infra::{config::Configuration, tracing::Telemetry, Services};

use sellershut::state::AppState;

use clap::Parser;

/// sellershut
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

    let _tracing = Telemetry::builder()
        .try_with_opentelemetry(&config.application, "")?
        .build();

    let services = Services::builder()
        .with_postgres(&config.database)
        .await?
        .with_cache(&config.cache)
        .await?
        .build()?;

    let state = AppState {
        services: services,
        config,
    };

    sellershut::serve(state).await
}
