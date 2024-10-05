use std::time::Duration;

use axum::{extract::Request, http::StatusCode, response::{IntoResponse, Response}, Router};
use infra::tracing::opentelemetry::on_http_request;
use tower_http::trace::TraceLayer;
use tracing::{info_span, Span};

pub mod grpc;
pub mod pub_sub;
pub mod web;

pub fn apply_middleware(router: Router<()>) -> Router {
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

#[derive(Debug)]
pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
