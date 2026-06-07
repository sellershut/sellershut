use axum::{extract::State, response::IntoResponse};

use crate::state::AppState;

/// Health
#[utoipa::path(
    method(get, head),
    path = "/api/health",
    responses(
        (
            status = OK, description = "API is live",
            body = Option<str>, content_type = "text/plain",
         )
    ),
    tag = "sellershut"
)]
pub async fn health(State(state): State<AppState>) -> impl IntoResponse {
    String::default()
}
