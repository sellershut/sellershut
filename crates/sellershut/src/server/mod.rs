pub mod cache_key;
pub mod error;
pub mod middleware;
mod router;
mod routes;
use std::fmt::Display;

pub use router::router;

#[non_exhaustive]
/// The oauth provider
#[derive(
    Debug,
    Clone,
    Copy,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Hash,
    utoipa::ToSchema,
    serde::Deserialize,
    serde::Serialize,
)]
#[schema(example = "discord")]
#[serde(rename_all = "camelCase")]
pub enum OauthProvider {
    /// Discord
    Discord,
}

impl OauthProvider {
    pub fn scopes(&self) -> Vec<&str> {
        match self {
            OauthProvider::Discord => vec!["identify", "email"],
        }
    }
}

impl Display for OauthProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OauthProvider::Discord => "discord",
            }
        )
    }
}

#[cfg(test)]
mod boostrap {
    use std::sync::OnceLock;

    use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt, reload};

    use crate::{
        logs::LogHandle,
        server::{self},
        state::AppState,
    };

    static TEST_LOG_DATA: OnceLock<LogHandle> = OnceLock::new();

    pub async fn test_app() -> axum::Router {
        let log_handle = TEST_LOG_DATA
            .get_or_init(|| {
                let filter = EnvFilter::new("warn");
                let (layer, handle) = reload::Layer::new(filter);

                let subscriber = Registry::default().with(layer);

                let _ = tracing::subscriber::set_global_default(subscriber);

                handle
            })
            .clone();
        let state = AppState::builder().log_handle(log_handle).build();

        server::router(state).await
    }
}
