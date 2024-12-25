use std::sync::Arc;

use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use url::Url;

use crate::{
    ServicesBuilder,
    services_builder::{IsUnset, SetPostgres, State},
};

#[derive(Debug, Deserialize, Clone)]
pub struct PostgresConfig {
    pool_size: u32,
    port: u16,
    name: Arc<str>,
    host: Arc<str>,
    user: Arc<str>,
    password: SecretString,
}

impl PostgresConfig {
    // Getter for size
    pub fn pool_size(&self) -> u32 {
        self.pool_size
    }

    // Getter for port
    pub fn port(&self) -> u16 {
        self.port
    }

    // Getter for name
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    // Getter for host
    pub fn host(&self) -> &str {
        self.host.as_ref()
    }

    // Getter for username
    pub fn username(&self) -> &str {
        self.user.as_ref()
    }

    // Getter for password (you may want to return a reference or handle it differently)
    pub fn password(&self) -> &SecretString {
        &self.password
    }

    pub(crate) fn connection_string(&self) -> Result<Url, crate::ServiceError> {
        Url::parse(&format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.name
        ))
        .map_err(|e| crate::ServiceError::Configuration(e.to_string()))
    }
}

impl<S: State> ServicesBuilder<S> {
    pub async fn postgres(
        self,
        config: &PostgresConfig,
    ) -> Result<ServicesBuilder<SetPostgres<S>>, crate::ServiceError>
    where
        S::Postgres: IsUnset,
    {
        let pg = sqlx::postgres::PgPoolOptions::new()
            // The default connection limit for a Postgres server is 100 connections, with 3 reserved for superusers.
            //
            // If you're deploying your application with multiple replicas, then the total
            // across all replicas should not exceed the Postgres connection limit.
            .max_connections(config.pool_size)
            .connect(config.connection_string()?.as_ref())
            .await?;
        Ok(self.pg_internal(pg))
    }
}
