use tracing_subscriber::{
    EnvFilter, Layer, Registry, layer::SubscriberExt, util::SubscriberInitExt,
};

/// Telemetry handle
#[allow(missing_debug_implementations)]
pub struct Tracing {}

impl Tracing {
    /// Create a new builder
    pub fn builder() -> TracingBuilder {
        TracingBuilder::default()
    }
}

/// A builder for initialising [tracing] layers
#[allow(missing_debug_implementations)]
pub struct TracingBuilder {
    layer: Vec<Box<dyn Layer<Registry> + Sync + Send>>,
}

impl Default for TracingBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TracingBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        let types: Box<dyn Layer<Registry> + Sync + Send> =
            tracing_subscriber::fmt::layer().boxed();
        TracingBuilder { layer: vec![types] }
    }

    /// Initialises tracing
    pub fn build(self) -> Tracing {
        tracing_subscriber::registry()
            .with(self.layer)
            .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
            .init();
        Tracing {}
    }
}
