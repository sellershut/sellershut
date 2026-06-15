use std::str::FromStr;

use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "mode", rename_all = "kebab-case")]
pub enum RedisConfig {
    Standalone { url: Url },
    Cluster { nodes: Vec<String> },
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self::Standalone {
            url: Url::from_str("redis://localhost:6379").expect("redis url"),
        }
    }
}
