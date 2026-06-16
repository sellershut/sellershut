use sellershut_core::users::User;
use serde::{Deserialize, Serialize};

use crate::error::UserRetrievalError;

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordUser {
    id: String,
    avatar: Option<String>,
    username: String,
    discriminator: String,
    email: Option<String>,
    verified: bool,
}

impl TryFrom<DiscordUser> for User {
    type Error = UserRetrievalError;

    fn try_from(user_data: DiscordUser) -> Result<Self, Self::Error> {
        match (&user_data.email, user_data.verified) {
            (None, _) => Err(Self::Error::MissingEmail),
            (_, false) => Err(Self::Error::EmailNotVerified),
            (Some(_), true) => Ok(Self {
                id: user_data.id,
                username: user_data.username,
                email: user_data.email.ok_or(Self::Error::MissingEmail)?,
            }),
        }
    }
}
