use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use sellershut::{state::AppState, HutConfig};
use sellershut_services::{tracing::TracingBuilder, Configuration};

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
    #[cfg(not(debug_assertions))]
    let args = Args::parse();

    let _config_path = "sellershut.toml";

    #[cfg(not(debug_assertions))]
    let config_path = args
        .config_file
        .to_str()
        .expect("config file path is not valid");

    let config = config::Config::builder()
        .add_source(config::File::new(_config_path, config::FileFormat::Toml))
        .build()?;

    let config = config.try_deserialize::<Configuration>()?;
    let hut_config: HutConfig = serde_json::from_value(config.misc.clone())?;

    let _tracing = TracingBuilder::new().build(None);

    let (tx, _rx) = tokio::sync::oneshot::channel();
    let state = AppState::new(config.application.port, hut_config).await?;

    sellershut::run(state, tx).await
}
