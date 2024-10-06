use std::fmt::Display;

use app_metadata::AppMetadata;
use serde::Deserialize;

#[cfg(feature = "cache")]
pub mod cache;

#[cfg(feature = "client")]
pub mod hosts;

#[cfg(feature = "postgres")]
pub mod postgres;

#[cfg(any(feature = "nats-core", feature = "nats-jetstream"))]
pub mod nats;

pub mod app_metadata;

#[derive(Clone, Debug, Deserialize)]
pub struct Configuration {
    pub application: AppMetadata,
    #[cfg(feature = "postgres")]
    pub database: postgres::PgConfig,
    #[cfg(feature = "api")]
    pub port: u16,
    #[cfg(feature = "client")]
    pub hosts: hosts::Hosts,
    #[cfg(any(feature = "nats-core", feature = "nats-jetstream"))]
    pub nats: nats::Nats,
    #[cfg(feature = "cache")]
    pub cache: cache::CacheConfig,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Development,
    Production,
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Environment::Development => "development",
                Environment::Production => "production",
            }
        )
    }
}
