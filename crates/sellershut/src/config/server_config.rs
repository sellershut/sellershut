use std::str::FromStr;

use serde::{Deserialize, Serialize};
use url::Url;

use crate::config::logs::LogConfig;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct ServerConfig {
    pub port: u16,
    pub logging: LogConfig,
    pub vault: VaultConfig,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: 2210,
            logging: Default::default(),
            vault: Default::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct VaultConfig {
    pub address: Url,
    pub token: String,
    pub mount: String,
}

impl Default for VaultConfig {
    fn default() -> Self {
        Self {
            address: Url::from_str("http://127.0.0.1:8200").expect("valid default vault url"),
            token: Default::default(),
            mount: String::from("secret"),
        }
    }
}
