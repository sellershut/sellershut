pub mod error;
use std::{ops::Deref, str::FromStr};

use serde::{Deserialize, Serialize};
use url::Url;

use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, EndpointNotSet, EndpointSet, RedirectUrl, Scope,
    TokenUrl,
};

use crate::error::AuthClientError;

type C = oauth2::basic::BasicClient<
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
>;

#[derive(Clone, Debug)]
pub struct BasicClient(C);

impl Deref for BasicClient {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&OauthConfig> for BasicClient {
    type Error = AuthClientError;

    fn try_from(value: &OauthConfig) -> Result<Self, Self::Error> {
        let auth_url =
            AuthUrl::new(value.auth_url.to_string()).map_err(AuthClientError::InvalidAuthUrl)?;

        let token_url =
            TokenUrl::new(value.token_url.to_string()).map_err(AuthClientError::InvalidTokenUrl)?;

        let redirect_url = RedirectUrl::new(value.redirect_url.to_string())
            .map_err(AuthClientError::InvalidRedirectUrl)?;

        Ok(Self(
            oauth2::basic::BasicClient::new(ClientId::new(value.client_id.to_string()))
                .set_client_secret(ClientSecret::new(value.client_secret.to_string()))
                .set_auth_uri(auth_url)
                .set_token_uri(token_url)
                .set_redirect_uri(redirect_url),
        ))
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct OauthConfig {
    pub auth_url: Url,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: Url,
    pub token_url: Url,
}

impl Default for OauthConfig {
    fn default() -> Self {
        Self {
            auth_url: Url::from_str("https://localhost/authorise").expect("oauth auth_url"),
            client_id: String::from("my-client"),
            client_secret: String::from("client-secret"),
            redirect_url: Url::from_str("https://localhost/authoriseed")
                .expect("oauth redirect_url"),
            token_url: Url::from_str("https://localhost/token").expect("oauth token_url"),
        }
    }
}

pub fn create_csrf_token(client: &BasicClient, scopes: &[&str]) -> (Url, CsrfToken) {
    let mut builder = client.authorize_url(CsrfToken::new_random);

    for pat in scopes {
        builder = builder.add_scope(Scope::new(pat.to_string()));
    }
    builder.url()
}
