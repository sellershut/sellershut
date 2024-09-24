use super::TelemetryBuilder;

impl TelemetryBuilder {
    #[cfg(feature = "telemetry")]
    /// Adds opentelemetry
    pub fn try_with_opentelemetry(
        mut self,
        config: &crate::config::app_metadata::AppMetadata,
        endpoint: &str,
    ) -> Result<Self, crate::ServiceError> {
        use opentelemetry::{global, trace::TracerProvider, KeyValue};
        use opentelemetry_otlp::WithExportConfig;
        use opentelemetry_sdk::{
            runtime,
            trace::{BatchConfig, RandomIdGenerator, Sampler},
            Resource,
        };
        use tracing_subscriber::Layer;
        use opentelemetry_semantic_conventions::{
            resource::{DEPLOYMENT_ENVIRONMENT_NAME, SERVICE_NAME, SERVICE_VERSION},
            SCHEMA_URL,
        };
        use tracing_opentelemetry::OpenTelemetryLayer;

        global::set_text_map_propagator(
            opentelemetry_sdk::propagation::TraceContextPropagator::new(),
        );

        let resource = Resource::from_schema_url(
            [
                KeyValue::new(SERVICE_NAME, config.name.to_owned()),
                KeyValue::new(SERVICE_VERSION, config.version.to_owned()),
                KeyValue::new(DEPLOYMENT_ENVIRONMENT_NAME, config.env.to_string()),
            ],
            SCHEMA_URL,
        );

        let provider = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_trace_config(
                opentelemetry_sdk::trace::Config::default()
                    .with_sampler(Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(
                        1.0,
                    ))))
                    .with_id_generator(RandomIdGenerator::default())
                    .with_resource(resource),
            )
            .with_batch_config(BatchConfig::default())
            .with_exporter(
                opentelemetry_otlp::new_exporter()
                    .tonic()
                    .with_endpoint(endpoint),
            )
            .install_batch(runtime::Tokio)
            .unwrap();

        global::set_tracer_provider(provider.clone());
        let tracer = provider.tracer(config.name.to_string());

        self.layer.push(OpenTelemetryLayer::new(tracer).boxed());

        Ok(self)
    }
}
