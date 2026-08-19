use std::time::Duration;

use axum::{
    Router,
    body::Bytes,
    extract::MatchedPath,
    http::{HeaderMap, Request},
    response::Response,
};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{Span, info_span};

use crate::server::router::middleware::request_id::RequestId;

pub fn trace_layer(router: Router) -> Router {
    router.layer(
        TraceLayer::new_for_http()
            .make_span_with(|request: &Request<_>| {
                // Log the matched route's path (with placeholders not filled in).
                // Use request.uri() or OriginalUri if you want the real path.
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                let request_id = request
                    .extensions()
                    .get::<RequestId>()
                    .expect("request_id middleware must run before trace middleware")
                    .0
                    .clone();

                info_span!(
                    "http_request",
                    method = ?request.method(),
                    request_id = %request_id,
                    matched_path,
                    some_other_field = tracing::field::Empty,
                )
            })
            .on_request(|_request: &Request<_>, _span: &Span| {
                // You can use `_span.record("some_other_field", value)` in one of these
                // closures to attach a value to the initially empty field in the info_span
                // created above.
                tracing::trace!("started processing request")
            })
            .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
                tracing::trace!("finished processing request")
            })
            .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {
                tracing::trace!("sending body chunk")
            })
            .on_eos(
                |_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {
                    tracing::trace!("stream closed")
                },
            )
            .on_failure(
                |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                    tracing::error!("something went wrong")
                },
            ),
    )
}
