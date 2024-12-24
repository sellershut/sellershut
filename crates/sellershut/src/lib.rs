use activitypub_federation::config::{FederationConfig, UrlVerifier};
use hut::Hut;
use server::error::AppError;
use tonic::async_trait;
use url::Url;

pub mod hut;
pub mod server;

pub async fn run() -> Result<(), AppError> {
    let hut = Hut::new().await?;
    let config = FederationConfig::builder()
        .domain(hut.domain.as_ref())
        .signed_fetch_actor(&hut.system_user)
        .app_data(hut)
        .url_verifier(Box::new(MyUrlVerifier()))
        // TODO: use application env
        .debug(cfg!(debug_assertions))
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
