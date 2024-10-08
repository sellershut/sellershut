#![cfg_attr(docsrs, feature(doc_cfg))]

use thiserror::Error;

#[cfg(feature = "tracing")]
pub mod tracing;

pub mod config;

pub mod services;

pub mod events;

#[derive(Clone, Debug)]
pub struct Services {
    #[cfg(feature = "postgres")]
    pub postgres: sqlx::PgPool,
    #[cfg(feature = "nats-core")]
    pub nats: async_nats::Client,
    #[cfg(feature = "nats-jetstream")]
    pub jetstream: async_nats::jetstream::Context,
    #[cfg(feature = "nats-jetstream")]
    pub jetstream_pull_consumers: std::sync::Arc<
        [async_nats::jetstream::consumer::Consumer<
            async_nats::jetstream::consumer::pull::Config,
        >],
    >,
    #[cfg(feature = "nats-jetstream")]
    pub jetstream_push_consumers: std::sync::Arc<
        [async_nats::jetstream::consumer::Consumer<
            async_nats::jetstream::consumer::push::Config,
        >],
    >,
    #[cfg(feature = "cache")]
    pub cache: services::cache::RedisPool,
}

impl Services {
    pub fn builder() -> ServicesBuilder {
        ServicesBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct ServicesBuilder {
    #[cfg(feature = "postgres")]
    pub(crate) postgres: Option<sqlx::PgPool>,
    #[cfg(feature = "nats-core")]
    pub(crate) nats: Option<async_nats::Client>,
    #[cfg(feature = "nats-jetstream")]
    pub(crate) nats_jetstream: Option<async_nats::jetstream::Context>,
    #[cfg(feature = "nats-jetstream")]
    pub(crate) nats_jetstream_pull_consumers: Vec<
        async_nats::jetstream::consumer::Consumer<async_nats::jetstream::consumer::pull::Config>,
    >,
    #[cfg(feature = "nats-jetstream")]
    pub(crate) nats_jetstream_push_consumers: Vec<
        async_nats::jetstream::consumer::Consumer<async_nats::jetstream::consumer::push::Config>,
    >,
    #[cfg(feature = "cache")]
    pub cache: Option<services::cache::RedisPool>,
}

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("service was not initialised")]
    NotInitialised,
    #[error("unknown data store error")]
    Unknown,
    #[cfg(feature = "postgres")]
    #[error(transparent)]
    /// Postgres error
    Postgres(#[from] sqlx::Error),
    #[cfg(any(feature = "nats-core", feature = "nats-jetstream"))]
    #[error(transparent)]
    /// Postgres error
    Nats(#[from] async_nats::error::Error<async_nats::ConnectErrorKind>),
    #[cfg(feature = "telemetry")]
    #[error(transparent)]
    /// When creating the tracing layer
    Opentelemetry(#[from] opentelemetry::trace::TraceError),
    #[cfg(feature = "cache")]
    #[error(transparent)]
    /// When creating the tracing layer
    Cache(#[from] redis::RedisError),
}

impl ServicesBuilder {
    pub fn build(self) -> Result<Services, ServiceError> {
        log::debug!("building services");
        Ok(Services {
            #[cfg(feature = "postgres")]
            postgres: self.postgres.ok_or(ServiceError::NotInitialised)?,
            #[cfg(feature = "nats-core")]
            nats: self.nats.ok_or(ServiceError::NotInitialised)?,
            #[cfg(feature = "nats-jetstream")]
            jetstream: self.nats_jetstream.ok_or(ServiceError::NotInitialised)?,
            #[cfg(feature = "nats-jetstream")]
            jetstream_pull_consumers: self.nats_jetstream_pull_consumers.into(),
            #[cfg(feature = "nats-jetstream")]
            jetstream_push_consumers: self.nats_jetstream_push_consumers.into(),
            #[cfg(feature = "cache")]
            cache: self.cache.ok_or(ServiceError::NotInitialised)?,
        })
    }
}
