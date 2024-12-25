use activitypub_federation::config::{FederationConfig, UrlVerifier};
use hut::Hut;
use serde::Deserialize;
use server::error::AppError;
use svc_infra::Configuration;
use tonic::async_trait;
use url::Url;

pub mod hut;
pub mod server;

#[derive(Deserialize)]
pub struct HutConfig {
    pub hostname: String,
    #[serde(rename = "otel-endpoint")]
    pub otel_endpoint: String,
}

pub async fn run(hut_config: &HutConfig, config: Configuration) -> Result<(), AppError> {
    let hut = Hut::new(hut_config).await?;
    let config = FederationConfig::builder()
        .domain(hut.domain.as_ref())
        .signed_fetch_actor(&hut.system_user)
        .app_data(hut)
        .url_verifier(Box::new(MyUrlVerifier()))
        .debug(match config.application.env {
            svc_infra::Environment::Development => true,
            svc_infra::Environment::Production => false,
        })
        .build()
        .await?;

    Ok(server::serve(&config).await?)
}

/// Use this to store your federation blocklist, or a database connection needed to retrieve it.
#[derive(Clone)]
struct MyUrlVerifier();

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
