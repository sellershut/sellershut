mod config;
mod logger;

use anyhow::Result;
use clap::Parser;

use crate::config::cli::{Args, Commands};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    if let Some(Commands::GenerateConfig { output }) = args.command {
        let str = toml::to_string_pretty(&config::Configuration::default())?;
        std::fs::write(&output, &str)?;
        println!("Config written to: {:?}", output);
        return Ok(());
    }

    println!("Hello, world!");

    Ok(())
}
