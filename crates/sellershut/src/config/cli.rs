use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// A federated marketplace platform
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Configuration file directory
    #[arg(short, long)]
    pub config: Option<PathBuf>,
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate a default [TOML] config file
    GenerateConfig {
        #[arg(short, long)]
        /// The filepath to write the config file to
        output: PathBuf,
    },
}
