pub mod vault;

use std::sync::Arc;

use bon::Builder;
use sqlx::PgPool;

use crate::{
    config::server_config::{DatabaseConfig, VaultConfig},
    logs::LogHandle,
    state::{
        app_state_builder::{IsUnset, SetDatabase, SetVault, State},
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
}

impl<S: State> AppStateBuilder<S> {
    pub async fn vault(self, config: &VaultConfig) -> anyhow::Result<AppStateBuilder<SetVault<S>>>
    where
        S::Vault: IsUnset,
    {
        tracing::debug!(endpoint = ?config.address, "connecting to vault");
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
        tracing::debug!(endpoint = ?config.host, "connecting to database");

        Ok(self.database_internal(
            sqlx::postgres::PgPoolOptions::new()
                .max_connections(config.pool_size)
                .connect(&config.connection_string())
                .await
                .inspect_err(|e| tracing::error!("{e}"))?,
        ))
    }
}
