use std::time::Duration;

use axum::{extract::Request, response::Response, Router};
use infra::tracing::opentelemetry::on_http_request;
use tower_http::trace::TraceLayer;
use tracing::{info_span, Span};

pub mod grpc;
pub mod pub_sub;
pub mod web;

pub fn apply_middleware(router: Router<()>) -> Router<()> {
    router.layer(
        TraceLayer::new_for_http()
            .make_span_with(|request: &Request<_>| {
                info_span!(
                    "request",
                    method = ?request.method(),
                    trace_id = tracing::field::Empty,
                )
            })
            .on_request(|req: &Request<_>, span: &Span| on_http_request(req.headers(), span))
            .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
                // ...
            }),
    )
}
