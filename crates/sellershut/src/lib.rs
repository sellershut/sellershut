use activitypub_federation::config::{FederationConfig, UrlVerifier};
use hut::Hut;
use serde::Deserialize;
use server::error::AppError;
use svc_infra::Configuration;
use tonic::async_trait;
use url::{ParseError, Url};

pub mod hut;
pub mod server;

#[derive(Deserialize)]
pub struct HutConfig {
    pub hostname: String,
    #[serde(rename = "instance-name")]
    pub instance_name: String,
    #[serde(rename = "otel-endpoint")]
    pub otel_endpoint: String,
    #[serde(rename = "users-endpoint")]
    pub users_endpoint: String,
    #[serde(rename = "categories-endpoint")]
    pub categories_endpoint: String,
    #[serde(rename = "listings-endpoint")]
    pub listings_endpoint: String,
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

pub fn generate_object_id(domain: &str) -> Result<Url, ParseError> {
    let id = sellershut_utils::id::generate_id();
    Url::parse(&format!("http://{domain}/objects/{}", id))
}

pub fn get_domain_with_port(url_str: &str) -> Result<String, AppError> {
    let url = Url::parse(url_str)?;
    let host = url
        .host_str()
        .ok_or_else(|| anyhow::anyhow!("host str unavailable"))?;

    let port = url.port_or_known_default();

    if let Some(port) = port {
        Ok(format!("{}:{}", host, port))
    } else {
        Ok(host.to_string())
    }
}
