use serde::Deserialize;

use std::{fmt::Display, sync::Arc};

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    #[serde(skip)]
    pub name: Arc<str>,
    #[serde(skip)]
    pub version: Arc<str>,
    pub env: Environment,
    #[cfg(feature = "api")]
    pub port: u16,
    #[serde(default)]
    #[serde(rename = "log-level")]
    #[cfg(feature = "tracing")]
    pub log_level: Option<Arc<str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Configuration {
    pub application: AppConfig,
    #[cfg(feature = "postgres")]
    pub database: crate::postgres::PostgresConfig,
    #[serde(default)]
    pub misc: serde_json::Value,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Development,
    Production,
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Environment::Development => "development",
            Environment::Production => "production",
        })
    }
}
