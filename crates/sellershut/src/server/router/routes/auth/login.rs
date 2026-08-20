use activitypub_federation::config::Data;
use axum::{Json, extract::Query, response::IntoResponse};
use sellershut_core::auth::OauthProvider;
use serde::Deserialize;
use utoipa::IntoParams;

use crate::server::{AppError, router::routes::auth::AUTH_TAG, state::AppState};

/// AuthQuery
#[derive(Deserialize, IntoParams)]
pub struct AuthQuery {
    /// Oauth provider
    #[param(inline)]
    provider: OauthProvider,
}

/// Oauth login
#[utoipa::path(
    post,
    responses(
        (
            status = 302,
            description = "Redirects the user to the selected OAuth provider",
            headers(
                (
                    "Location" = String,
                    description = "OAuth provider authorization URL"
                ),
                (
                    "x-request-id" = String,
                    description = "Unique identifier for the request"
                )
            )
        ),
        (
            status = 400,
            description = "Invalid OAuth provider",
            headers(
                (
                    "x-request-id" = String,
                    description = "Unique identifier for the request"
                )
            )
        ),
        (
            status = 500,
            description = "Failed to generate the OAuth authorization URL",
            headers(
                (
                    "x-request-id" = String,
                    description = "Unique identifier for the request"
                )
            )
        )
    ),
    operation_id = "login", // https://github.com/juhaku/utoipa/issues/1170
    path = "/login",
    tag = AUTH_TAG,
    params(
        AuthQuery
    ),
)]
pub async fn login(
    Query(query): Query<AuthQuery>,
    state: Data<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let start = state.auth.start_oauth(query.provider).await?;

    Ok(Json(start).into_response())
}
