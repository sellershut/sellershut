use serde::Deserialize;

#[cfg(feature = "client")]
pub mod hosts;

#[cfg(feature = "postgres")]
pub mod postgres;

#[cfg(any(feature = "nats-core", feature = "nats-jetstream"))]
pub mod nats;

#[derive(Clone, Debug, Deserialize)]
pub struct Configuration {
    pub environment: Environment,
    #[cfg(feature = "postgres")]
    pub database: postgres::PgConfig,
    #[cfg(feature = "api")]
    pub port: u16,
    #[cfg(feature = "client")]
    pub hosts: hosts::Hosts,
    #[cfg(any(feature = "nats-core", feature = "nats-jetstream"))]
    pub nats: nats::Nats,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Development,
    Production,
}
