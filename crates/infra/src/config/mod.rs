use serde::Deserialize;

#[cfg(feature = "postgres")]
pub mod postgres;

#[derive(Clone, Debug, Deserialize)]
pub struct Configuration {
    #[cfg(feature = "postgres")]
    pub database: postgres::PgConfig,
    #[cfg(feature = "api")]
    pub port: u16,
}
