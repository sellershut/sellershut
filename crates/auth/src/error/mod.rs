mod client;
pub use client::*;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserRetrievalError {
    #[error("No email available for this user")]
    MissingEmail,
    #[error("Email is not verified")]
    EmailNotVerified,
    #[error("oauth token for user")]
    UserToken,
    #[error("could not get user through http")]
    UserRetrieval,
    #[error("remote user mismatch")]
    UserDeserialisation,
}
