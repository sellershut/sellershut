use serde::Deserialize;

#[cfg(feature = "postgres")]
pub mod postgres;

#[derive(Clone, Debug, Deserialize)]
pub struct Configuration {
    pub environment: Environment,
    #[cfg(feature = "postgres")]
    pub database: postgres::PgConfig,
    #[cfg(feature = "api")]
    pub port: u16,
    #[cfg(feature = "client")]
    pub hosts: Hosts,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Development,
    Production,
}

#[cfg(feature = "client")]
#[derive(Debug, Deserialize, Clone)]
pub struct Hosts {
    #[cfg(feature = "users-client")]
    pub users: String,
}
