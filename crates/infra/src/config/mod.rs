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
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Development,
    Production,
}
