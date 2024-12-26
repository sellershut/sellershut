use std::path::PathBuf;

use anyhow::Result;
use categories_service::AppConfig;
use clap::Parser;
use svc_infra::{Configuration, Services, tracing::TracingBuilder};

/// categories-service
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to config file
    #[arg(short, long)]
    config_file: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(not(debug_assertions))]
    let args = Args::parse();

    let _config_path = "config/categories.toml";

    #[cfg(not(debug_assertions))]
    let config_path = args
        .config_file
        .to_str()
        .expect("config file path is not valid");

    let config = config::Config::builder()
        .add_source(config::File::new(_config_path, config::FileFormat::Toml))
        .build()?;

    let config = config.try_deserialize::<Configuration>()?;
    let app_config: AppConfig = serde_json::from_value(config.misc.clone())?;

    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");

    let _tracing = TracingBuilder::new()
        .try_with_opentelemetry(
            name,
            version,
            &config.application.env,
            &app_config.otel_endpoint,
        )?
        .build(config.application.log_level.clone());

    let services = Services::builder()
        .postgres(&config.database)
        .await?
        .build();

    categories_service::run(services, config).await
}
