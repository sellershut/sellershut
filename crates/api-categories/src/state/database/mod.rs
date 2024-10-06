use std::error::Error;

use async_nats::{jetstream::Context, HeaderMap};
use infra::{events::Event, services::utils::nats_otel::NatsMetadataInjector};
use opentelemetry::global;
use prost::Message;
use tracing::{debug_span, instrument, trace, Instrument, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

pub mod mutation;
pub mod query;

pub fn map_err(err: impl Error) -> tonic::Status {
    tonic::Status::new(tonic::Code::Internal, err.to_string())
}

#[instrument(skip(value, event, jetstream), err(Debug))]
async fn publish_event(
    value: impl Message,
    event: Event,
    jetstream: &Context,
) -> Result<(), tonic::Status> {
    let mut headers = HeaderMap::new();
    let buf = Message::encode_to_vec(&value);

    let event = event.to_string();

    global::get_text_map_propagator(|propagator| {
        let context = Span::current().context();
        trace!("injecting opentelemetry context");
        propagator.inject_context(&context, &mut NatsMetadataInjector(&mut headers))
    });

    jetstream
        .publish_with_headers(event, headers, buf.into())
        .instrument(debug_span!("jetstream.publish"))
        .await
        .map_err(map_err)?;

    trace!("message published");

    Ok(())
}
