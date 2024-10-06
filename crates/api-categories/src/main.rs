use std::path::PathBuf;

use clap::Parser;
use infra::{config::Configuration, tracing::Telemetry, Services};

/// api-categories
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to config file
    #[arg(short, long)]
    config_file: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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
        .with_postgres(&config.database)
        .await?
        .with_cache(&config.cache)
        .await?
        .with_nats_jetstream(&config.nats)
        .await?
        .build()?;

    api_categories::run(services, config).await
}
