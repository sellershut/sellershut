use std::sync::Arc;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Nats {
    hosts: Arc<[String]>,
    #[serde(default)]
    #[cfg(feature = "nats-jetstream")]
    pub jetstream: Arc<[StreamConfig]>,
}

#[cfg(feature = "nats-jetstream")]
#[derive(Debug, Deserialize, Default)]
pub struct StreamConfig {
    pub name: Arc<str>,
    pub subjects: Arc<[String]>,
    #[serde(default)]
    pub max_msgs: i64,
    #[serde(default)]
    pub max_bytes: i64,
    pub consumers: Arc<[ConsumerConfig]>,
}

#[cfg(feature = "nats-jetstream")]
#[derive(Debug, Deserialize, Default)]
pub struct ConsumerConfig {
    pub name: Arc<str>,
    pub durable: Option<Arc<str>>,
    pub deliver_subject: Option<Arc<str>>,
}

impl Nats {
    pub fn hosts(&self) -> &[String] {
        self.hosts.as_ref()
    }
}
