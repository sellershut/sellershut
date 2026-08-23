use thiserror::Error;
use vaultrs::{client::VaultClientSettingsBuilderError, error::ClientError};

#[derive(Debug, Error)]
pub enum UserError {
    #[error("database error")]
    Database(#[from] sqlx::Error),
    #[error("username is unavailable")]
    UsernameTaken,
    #[error("database error")]
    VaultClient(#[from] ClientError),
    #[error("database error")]
    VaultClientBuilder(#[from] VaultClientSettingsBuilderError),
}
