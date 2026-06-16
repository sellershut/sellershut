pub mod cache;
pub mod vault;

use std::{collections::HashMap, sync::Arc};

use auth::BasicClient;
use bon::Builder;
use sqlx::PgPool;
use tower_sessions_sqlx_store::PostgresStore;
use tracing::debug;

use crate::{
    config::server_config::{DatabaseConfig, VaultConfig},
    logs::LogHandle,
    server::OauthProvider,
    state::{
        app_state_builder::{IsUnset, SetDatabase, SetVault, State},
        cache::RedisClient,
        vault::check_vault_startup,
    },
};
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};

#[derive(Clone, Builder)]
pub struct AppState {
    pub log_handle: LogHandle,
    #[builder(setters(vis = "", name = vault_internal))]
    pub vault: Arc<VaultClient>,
    #[builder(setters(vis = "", name = database_internal))]
    pub database: PgPool,
    pub oauth_clients: Arc<HashMap<OauthProvider, BasicClient>>,
    pub cache: RedisClient,
    pub http_client: reqwest::Client,
    #[builder(skip = PostgresStore::new(database.clone()))]
    pub session_store: PostgresStore,
}

impl<S: State> AppStateBuilder<S> {
    pub async fn vault(self, config: &VaultConfig) -> anyhow::Result<AppStateBuilder<SetVault<S>>>
    where
        S::Vault: IsUnset,
    {
        debug!(endpoint = ?&config.address.to_string(), "connecting to vault");
        let vault = VaultClient::new(
            VaultClientSettingsBuilder::default()
                .address(config.address.as_str())
                .token(&config.token)
                .build()?,
        )?;

        check_vault_startup(&vault, &config.mount).await?;
        Ok(self.vault_internal(vault.into()))
    }

    pub async fn database(
        self,
        config: &DatabaseConfig,
    ) -> anyhow::Result<AppStateBuilder<SetDatabase<S>>>
    where
        S::Database: IsUnset,
    {
        debug!(endpoint = ?config.host, "connecting to database");

        let db = sqlx::postgres::PgPoolOptions::new()
            .max_connections(config.pool_size)
            .connect(&config.connection_string())
            .await
            .inspect_err(|e| tracing::error!("{e}"))?;

        debug!("running database migrations");
        sqlx::migrate!("../../migrations").run(&db).await?;

        Ok(self.database_internal(db))
    }
}
