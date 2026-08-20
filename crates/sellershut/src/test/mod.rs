use crate::{config::Configuration, logger::LogHandle};

use super::*;

use std::sync::OnceLock;

use sqlx::PgPool;
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt, reload};

static TEST_LOG_DATA: OnceLock<LogHandle> = OnceLock::new();

pub async fn test_app(pool: PgPool) -> axum::Router {
    let _log_handle = TEST_LOG_DATA
        .get_or_init(|| {
            let filter = EnvFilter::new("warn");
            let (layer, handle) = reload::Layer::new(filter);

            let subscriber = Registry::default().with(layer);

            let _ = tracing::subscriber::set_global_default(subscriber);

            handle
        })
        .clone();

    let config = Configuration::default();
    let user_driver = UserService::new(pool.clone());
    let state = State::new(&config, user_driver, pool).await.unwrap();

    server::router::router(state, config)
        .await
        .expect("test router")
}
