use anyhow::Result;
use tracing_appender::rolling::RollingFileAppender;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::log::Log;

pub type LogHandle = tracing_subscriber::reload::Handle<EnvFilter, tracing_subscriber::Registry>;

pub fn log(config: &Log) -> Result<(LogHandle, tracing_appender::non_blocking::WorkerGuard)> {
    let file_appender = RollingFileAppender::from(config);

    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let env_filter = tracing_subscriber::EnvFilter::try_from_env("HUT_LOG")
        .unwrap_or_else(|_| config.log_level.to_string().into());

    let (filter_layer, reload_handle) = tracing_subscriber::reload::Layer::new(env_filter);

    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_target(true);

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(tracing_subscriber::fmt::layer())
        .with(file_layer)
        .init();

    Ok((reload_handle, guard))
}
