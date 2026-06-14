use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthClientError {
    #[error("missing field: {0}")]
    MissingField(&'static str),
    #[error("invalid auth url: {0}")]
    InvalidAuthUrl(#[from] oauth2::url::ParseError),
    #[error("invalid token url: {0}")]
    InvalidTokenUrl(#[source] oauth2::url::ParseError),
    #[error("invalid redirect url: {0}")]
    InvalidRedirectUrl(#[source] oauth2::url::ParseError),
}
