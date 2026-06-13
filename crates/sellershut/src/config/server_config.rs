use serde::{Deserialize, Serialize};

use crate::config::logs::LogConfig;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct ServerConfig {
    pub port: u16,
    pub logging: LogConfig,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: 2210,
            logging: Default::default(),
        }
    }
}
