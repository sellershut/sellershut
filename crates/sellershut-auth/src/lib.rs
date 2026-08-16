pub mod error;

use oauth2::{AuthUrl, ClientId, ClientSecret, EndpointNotSet, EndpointSet, RedirectUrl, TokenUrl};
use sellershut_core::types::RedactedSecret;
use serde::{Deserialize, Serialize};
use url::Url;

type BasicClient = oauth2::basic::BasicClient<
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Configuration {
    client_id: String,
    client_secret: RedactedSecret,
    redirect_url: Url,
    auth_url: Url,
    token_url: Url,
}

impl Default for Configuration {
    fn default() -> Self {
        let url = Url::parse("http://example.url").expect("valid url");
        Self {
            client_id: Default::default(),
            client_secret: Default::default(),
            redirect_url: url.clone(),
            auth_url: url.clone(),
            token_url: url,
        }
    }
}

impl From<Configuration> for BasicClient {
    fn from(value: Configuration) -> Self {
        oauth2::basic::BasicClient::new(ClientId::new(value.client_id))
            .set_client_secret(ClientSecret::new(value.client_secret.expose()))
            .set_auth_uri(AuthUrl::from_url(value.auth_url))
            .set_token_uri(TokenUrl::from_url(value.token_url))
            .set_redirect_uri(RedirectUrl::from_url(value.redirect_url))
    }
}

pub trait OauthDriver {}
