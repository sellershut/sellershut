use async_nats::{HeaderMap, HeaderName, HeaderValue};
use opentelemetry::propagation::{Extractor, Injector};

#[derive(Debug)]
/// Wrapper for [HeaderMap] implementing [Injector]
pub struct NatsMetadataInjector<'a>(pub &'a mut HeaderMap);

impl Injector for NatsMetadataInjector<'_> {
    fn set(&mut self, key: &str, value: String) {
        let value = HeaderValue::from(value.as_str());
        self.0.insert(key, value);
    }
}

#[derive(Debug)]
/// Wrapper for [HeaderMap] implementing [Extractor]
pub struct NatsMetadataExtractor<'a>(pub &'a HeaderMap);

impl Extractor for NatsMetadataExtractor<'_> {
    fn get(&self, key: &str) -> Option<&str> {
        <HeaderName as std::str::FromStr>::from_str(&key.to_lowercase())
            .ok()
            .and_then(|key| self.0.get(key).map(|value| value.as_str()))
    }

    /// Collect all the keys from the HashMap.
    fn keys(&self) -> Vec<&str> {
        self.0.iter().map(|(key, _value)| key.as_ref()).collect()
    }
}
