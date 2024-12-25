use http::HeaderMap;
use opentelemetry::{global, trace::TraceContextExt};
use opentelemetry_http::HeaderExtractor;
use opentelemetry_otlp::SpanExporter;
use opentelemetry_sdk::trace::{RandomIdGenerator, Sampler};
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::Environment;

use super::TracingBuilder;

impl TracingBuilder {
    /// Adds opentelemetry
    pub fn try_with_opentelemetry(
        mut self,
        app_name: &str,
        app_version: &str,
        app_env: &Environment,
        endpoint: &str,
    ) -> Result<Self, crate::ServiceError> {
        use opentelemetry::{KeyValue, global, trace::TracerProvider};
        use opentelemetry_otlp::WithExportConfig;
        use opentelemetry_sdk::{Resource, runtime};
        use opentelemetry_semantic_conventions::{
            SCHEMA_URL,
            resource::{DEPLOYMENT_ENVIRONMENT_NAME, SERVICE_NAME, SERVICE_VERSION},
        };
        use tracing_opentelemetry::OpenTelemetryLayer;
        use tracing_subscriber::Layer;

        global::set_text_map_propagator(
            opentelemetry_sdk::propagation::TraceContextPropagator::new(),
        );

        let resource = Resource::from_schema_url(
            [
                KeyValue::new(SERVICE_NAME, app_name.to_owned()),
                KeyValue::new(SERVICE_VERSION, app_version.to_owned()),
                KeyValue::new(DEPLOYMENT_ENVIRONMENT_NAME, app_env.to_string()),
            ],
            SCHEMA_URL,
        );

        let exporter = SpanExporter::builder()
            .with_tonic()
            .with_endpoint(endpoint)
            .build()?;

        let provider = opentelemetry_sdk::trace::TracerProvider::builder()
            .with_sampler(Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(
                1.0,
            ))))
            .with_id_generator(RandomIdGenerator::default())
            .with_resource(resource)
            .with_batch_exporter(exporter, runtime::Tokio)
            .build();

        global::set_tracer_provider(provider.clone());
        let tracer = provider.tracer(app_name.to_string());

        self.layer.push(OpenTelemetryLayer::new(tracer).boxed());

        Ok(self)
    }
}

pub fn on_http_request(headers: &HeaderMap, span: &Span) {
    let parent_context =
        global::get_text_map_propagator(|propagator| propagator.extract(&HeaderExtractor(headers)));
    span.set_parent(parent_context);
    let trace_id = span.context().span().span_context().trace_id();
    span.record("trace_id", trace_id.to_string());
}
