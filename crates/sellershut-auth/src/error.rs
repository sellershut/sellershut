use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("unsupported OAuth provider: {0}")]
    UnsupportedProvider(String),
    #[error("invalid or expired OAuth state")]
    InvalidOAuthState,
    #[error("OAuth provider denied the request: {0}")]
    ProviderDenied(String),
    #[error("OAuth token exchange failed: {0}")]
    TokenExchange(String),
    #[error("OAuth provider did not return a verified email")]
    MissingVerifiedEmail,
    #[error("invalid or expired onboarding token")]
    InvalidOnboardingToken,
    #[error("invalid username: {0}")]
    InvalidUsername(String),
    #[error("username is already taken")]
    UsernameTaken,
    #[error("invalid or expired session")]
    InvalidSession,
    #[error("OAuth identity is already linked to another user")]
    IdentityConflict,
    #[error("invalid auth configuration: {0}")]
    Configuration(String),
    #[error("database error")]
    Database(#[from] sqlx::Error),
    #[error("OAuth provider HTTP request failed")]
    Http(#[from] reqwest::Error),
}
