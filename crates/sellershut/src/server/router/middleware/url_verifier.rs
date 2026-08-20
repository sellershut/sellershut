use activitypub_federation::config::UrlVerifier;
use async_trait::async_trait;
use url::Url;

use crate::server::state::AppState;

/// Use this to store your federation blocklist, or a database connection needed to retrieve it.
#[derive(Clone)]
#[allow(dead_code)]
pub struct MyUrlVerifier(AppState);

impl From<AppState> for MyUrlVerifier {
    fn from(value: AppState) -> Self {
        Self(value)
    }
}

#[async_trait]
impl UrlVerifier for MyUrlVerifier {
    async fn verify(&self, url: &Url) -> Result<(), activitypub_federation::error::Error> {
        if url.domain() == Some("malicious.com") {
            Err(activitypub_federation::error::Error::Other(
                "malicious domain".into(),
            ))
        } else {
            Ok(())
        }
    }
}
