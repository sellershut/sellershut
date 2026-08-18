use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("database error")]
    Database(#[from] sqlx::Error),
}
