pub mod entities;
pub mod server;
pub mod state;
// pub mod utils;

use std::sync::Arc;

use activitypub_federation::config::{FederationConfig, UrlVerifier};
use anyhow::Result;
use sellershut_services::Configuration;
use serde::Deserialize;
use state::AppState;
use url::Url;

#[derive(Deserialize)]
pub struct HutConfig {
    pub hostname: String,
    #[serde(rename = "users-service")]
    pub users_service: String,
    #[serde(rename = "categories-service")]
    pub categories_service: String,
    #[serde(rename = "instance-name")]
    pub instance_name: String,
}

pub async fn run(
    state: AppState,
    tx: tokio::sync::oneshot::Sender<u16>,
    config: Configuration,
) -> Result<()> {
    let state = Arc::new(state);
    let config = FederationConfig::builder()
        .domain(state.domain.to_string())
        .signed_fetch_actor(&state.system_user)
        .app_data(state)
        .url_verifier(Box::new(MyUrlVerifier()))
        .debug(match config.application.env {
            sellershut_services::Environment::Development => true,
            sellershut_services::Environment::Production => false,
        })
        .build()
        .await?;

    server::serve(tx, config).await
}

/// Use this to store your federation blocklist, or a database connection needed to retrieve it.
#[derive(Clone)]
struct MyUrlVerifier();

#[tonic::async_trait]
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
