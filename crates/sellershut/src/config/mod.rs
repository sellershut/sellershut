pub mod auth;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

use crate::config::{auth::AuthConfig, server_config::ServerConfig};

pub mod logs;
pub mod server_config;

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "kebab-case", default)]
pub struct Config {
    pub server: ServerConfig,
    pub auth: AuthConfig,
}

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
