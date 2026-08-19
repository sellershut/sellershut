use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("database error")]
    Database(#[from] sqlx::Error),
    #[error("username is unavailable")]
    UsernameTaken,
}
