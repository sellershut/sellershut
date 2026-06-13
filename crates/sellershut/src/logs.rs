use anyhow::{Context, Result};
use std::{env, path::PathBuf};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

pub type LogHandle = tracing_subscriber::reload::Handle<EnvFilter, tracing_subscriber::Registry>;

pub fn log(
    level: Option<&str>,
    log_dir: Option<&PathBuf>,
) -> Result<(LogHandle, tracing_appender::non_blocking::WorkerGuard)> {
    let level = level.context("missing log level")?;
    let log_dir = log_dir.context("missing log dir")?;

    let file_appender = RollingFileAppender::new(Rotation::DAILY, log_dir, env!("CARGO_PKG_NAME"));

    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let env_filter =
        tracing_subscriber::EnvFilter::try_from_env("HUT_LOG").unwrap_or_else(|_| level.into());

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
