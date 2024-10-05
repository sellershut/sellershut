use std::sync::Arc;

use secrecy::SecretString;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Configuration {
    #[serde(flatten)]
    pub base: infra::config::Configuration,
    pub oauth: Oauth,
}

#[derive(Deserialize)]
pub struct Oauth{
    pub github: OauthDetails
}

#[derive(Deserialize)]
pub struct OauthDetails {
    #[serde(rename = "client-id")]
    pub client_id: Arc<str>,
    #[serde(rename = "client-secret")]
    pub client_secret: SecretString,
    #[serde(rename = "redirect-url")]
    pub redirect_url: Arc<str>,
    #[serde(rename = "auth-url")]
    pub auth_url: Arc<str>
}
