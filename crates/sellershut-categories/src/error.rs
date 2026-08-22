use thiserror::Error;

#[derive(Debug, Error)]
pub enum CategoryError {
    #[error("database error")]
    Database(#[from] sqlx::Error),
    #[error("database error")]
    Url(#[from] url::ParseError),
    #[error("username is unavailable")]
    UsernameTaken,
}
