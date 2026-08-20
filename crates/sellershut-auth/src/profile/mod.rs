use sellershut_core::auth::OauthProvider;

use crate::error::AuthError;

pub mod discord;

pub(crate) struct OAuthProfile {
    pub(crate) provider: OauthProvider,
    pub(crate) id: String,
    pub(crate) email: String,
}

pub(crate) async fn fetch_profile(
    provider: OauthProvider,
    http: &reqwest::Client,
    access_token: &str,
) -> Result<OAuthProfile, AuthError> {
    match provider {
        OauthProvider::Discord => discord::fetch(provider, http, access_token).await,
        OauthProvider::Google => todo!(),
        _ => todo!(),
    }
}
