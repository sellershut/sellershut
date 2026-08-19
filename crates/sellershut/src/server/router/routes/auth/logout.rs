use crate::server::{AppError, router::routes::auth::AUTH_TAG, state::AppState};
use activitypub_federation::config::Data;
use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};

/// Logout of current session
#[utoipa::path(
    post,
    path = "/logout",
    security(
        ("bearer_auth" = [])

    ),
    responses(
        (status = 204, description = "User logged out",
            headers(
                (
                    "x-request-id" = String,
                    description = "Unique identifier for the request"
                )
            )
         ),
        (status = 500, description = "Internal server error")
    ),
    tag = AUTH_TAG,
)]
pub async fn logout(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    state: Data<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let token = bearer.token();

    state.auth.revoke_session(token).await?;

    Ok(StatusCode::NO_CONTENT)
}
