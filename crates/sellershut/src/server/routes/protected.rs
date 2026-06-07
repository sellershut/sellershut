use axum::{extract::State, response::IntoResponse};

use crate::state::AppState;

/// Protected
#[utoipa::path(
    method(get),
    path = "/api/protected",
    responses(
        (
            status = OK, description = "API is live",
            body = Option<str>, content_type = "text/plain",
        )
    ),
    security(
        ("api_key" = []),
    ),
    tag = "sellershut"
)]
pub async fn protected(State(state): State<AppState>) -> impl IntoResponse {
    String::from("this is protected")
}
