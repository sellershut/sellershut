pub mod cli;
pub mod log;
pub mod oauth;
pub mod server;

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "kebab-case", default)]
pub struct Configuration {
    pub server: server::Server,
    pub log: log::Log,
    pub database: sellershut_svc::database::Config,
}

pub fn load(cli: Option<&PathBuf>) -> Configuration {
    cli.and_then(|value| {
        std::fs::read_to_string(value)
            .inspect_err(|e| eprintln!("file: {value:?}, {e}"))
            .ok()
    })
    .and_then(|s| {
        toml::from_str(&s)
            .inspect_err(|_e| eprintln!("invalid config file"))
            .ok()
    })
    .unwrap_or_default()
}
