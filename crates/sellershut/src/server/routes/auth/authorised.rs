use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
};
use axum_extra::{TypedHeader, headers};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

use crate::{
    server::{error::AppError, routes::auth::OauthParams},
    state::AppState,
};

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[allow(dead_code)]
pub struct AuthRequest {
    pub code: String,
    pub state: String,
}

/// Authorised callback
#[utoipa::path(
    get,
    responses(
        (
            status = 200,
            description = "Application authorised",
            headers(
                ("x-request-id", description = "Request identifier"),
                ("set-cookie", description = "Oauth session cookie")
            )
        ),
    ),
    operation_id = "authorised", // https://github.com/juhaku/utoipa/issues/1170
    path = "/auth/{provider}/authorised",
    tag = super::AUTH,
    params(AuthRequest, OauthParams)
)]
pub async fn authorised(
    Query(params): Query<AuthRequest>,
    State(state): State<AppState>,
    Path(provider): Path<super::OauthProvider>,
    TypedHeader(cookies): TypedHeader<headers::Cookie>,
) -> Result<impl IntoResponse, AppError> {
    Ok(String::default())
}
