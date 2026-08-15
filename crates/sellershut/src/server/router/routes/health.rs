use axum::response::IntoResponse;

/// Health
#[utoipa::path(
    method(get, head),
    path = "/api/health",
    responses(
        (
            status = OK, description = "API is live",
            body = Option<str>, content_type = "text/plain",
        ),
        (
            status = REQUEST_TIMEOUT, description = "Request timed out"
        )
    ),
    tag = "sellershut"
)]
pub async fn health() -> impl IntoResponse {
    format!(
        "{} v{} is live",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    )
}
