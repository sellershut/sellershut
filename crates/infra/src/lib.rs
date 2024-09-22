use thiserror::Error;

#[cfg(feature = "tracing")]
pub mod tracing;

pub mod config;

pub mod services;

#[derive(Clone, Debug)]
pub struct Services {
    #[cfg(feature = "postgres")]
    pub postgres: sqlx::PgPool,
}

impl Services {
    pub fn builder() -> ServicesBuilder {
        ServicesBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct ServicesBuilder {
    #[cfg(feature = "postgres")]
    pub(crate) postgres: Option<sqlx::PgPool>,
}

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("service was not initialised")]
    NotInitialised,
    #[error("unknown data store error")]
    Unknown,
    #[cfg(feature = "postgres")]
    #[error(transparent)]
    /// Postgres error
    Postgres(#[from] sqlx::Error),
}

impl ServicesBuilder {
    pub fn build(self) -> Result<Services, ServiceError> {
        Ok(Services {
            #[cfg(feature = "postgres")]
            postgres: self.postgres.ok_or(ServiceError::NotInitialised)?,
        })
    }
}
