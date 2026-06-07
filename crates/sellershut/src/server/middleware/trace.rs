use axum::{
    extract::{MatchedPath, Request},
    middleware::Next,
    response::Response,
};
use std::time::Instant;
use tracing::Instrument;

use super::request_id::RequestId;

pub async fn trace_request(request: Request, next: Next) -> Response {
    let start = Instant::now();

    let method = request.method().clone();
    let uri = request.uri().clone();

    let matched_path = request
        .extensions()
        .get::<MatchedPath>()
        .map(MatchedPath::as_str)
        .unwrap_or(uri.path())
        .to_owned();

    let request_id = request
        .extensions()
        .get::<RequestId>()
        .expect("request_id middleware must run before trace middleware")
        .0
        .clone();

    let span = tracing::info_span!(
        "http_request",
        request_id = %request_id,
        method = %method,
        matched_path = %matched_path,
        status = tracing::field::Empty,
        elapsed_ms = tracing::field::Empty,
    );

    async move {
        tracing::debug!("started processing request");

        let response = next.run(request).await;

        let status = response.status();
        let elapsed_ms = start.elapsed().as_millis();

        tracing::Span::current().record("status", status.as_u16());
        tracing::Span::current().record("elapsed_ms", elapsed_ms);

        if status.is_server_error() {
            tracing::error!(status = status.as_u16(), elapsed_ms, "request failed");
        } else {
            tracing::debug!(
                status = status.as_u16(),
                elapsed_ms,
                "finished processing request"
            );
        }

        response
    }
    .instrument(span)
    .await
}
