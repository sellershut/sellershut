use crate::ServicesBuilder;

impl ServicesBuilder {
    #[cfg(feature = "postgres")]
    pub async fn with_postgres(
        mut self,
        config: crate::config::postgres::PgConfig,
    ) -> Result<Self, crate::ServiceError> {
        self.postgres = Some(
            sqlx::postgres::PgPoolOptions::new()
                // The default connection limit for a Postgres server is 100 connections, with 3 reserved for superusers.
                //
                // If you're deploying your application with multiple replicas, then the total
                // across all replicas should not exceed the Postgres connection limit.
                .max_connections(config.pool_size())
                .connect(&config.connection_string())
                .await?,
        );

        Ok(self)
    }
}
