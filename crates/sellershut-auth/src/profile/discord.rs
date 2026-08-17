use sellershut_core::auth::OauthProvider;
use serde::{Deserialize, Serialize};

use crate::{error::AuthError, profile::OAuthProfile};

// The user data we'll get back from Discord.
// https://discord.com/developers/docs/resources/user#user-object-user-structure
#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: String,
    avatar: Option<String>,
    username: String,
    discriminator: String,
    verified: Option<bool>,
    email: Option<String>,
}

pub async fn fetch(
    provider: OauthProvider,
    http: &reqwest::Client,
    access_token: &str,
) -> Result<OAuthProfile, AuthError> {
    let profile = http
        .get("https://discordapp.com/api/users/@me")
        .bearer_auth(access_token)
        .send()
        .await?
        .error_for_status()?
        .json::<User>()
        .await?;

    let email = match (profile.email, profile.verified) {
        (Some(email), Some(true)) if !email.trim().is_empty() => email,
        _ => return Err(AuthError::MissingVerifiedEmail),
    };

    Ok(OAuthProfile {
        provider,
        id: profile.id,
        email,
    })
}
