use anyhow::Context;
use async_session::Session;
use auth::{CsrfToken, validate_session};
use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
};
use axum_extra::{TypedHeader, headers};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

use crate::{
    server::{
        cache_key::CacheKey,
        error::AppError,
        routes::auth::{CSRF_TOKEN, OauthParams},
    },
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
    let cookie = cookies
        .get(super::COOKIE_NAME)
        .context("missing cookie name")?;

    let client = state
        .oauth_clients
        .get(&provider)
        .context("oauth provider not configured")?;

    let id = Session::id_from_cookie_value(cookie)?;
    let cache_key = CacheKey::Session(&id);

    let session = state.cache.get::<Vec<u8>>(cache_key).await?;
    let session: Session = serde_json::from_slice(&session)?;

    if let Some(session) = session.validate() {
        if let Some(token) = session.get::<CsrfToken>(CSRF_TOKEN) {
            state.cache.del(cache_key).await?;
            if *token.secret() != params.state {
                Err(anyhow::anyhow!("token mismatch").into())
            } else {
                Ok(())
            }
        } else {
            Err(anyhow::anyhow!("no token").into())
        }
    } else {
        Err(anyhow::anyhow!("invalid session").into())
    }
}
