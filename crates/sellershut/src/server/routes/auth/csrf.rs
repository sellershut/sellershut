use anyhow::Context;
use auth::CsrfToken;
use axum::{Json, http::HeaderMap};
use tower_sessions::Session;
use tracing::instrument;
use utoipa::ToSchema;

use crate::server::error::AppError;

const SESSION_FORM_CSRF: &str = "form_csrf";
const CSRF_HEADER: &str = "x-csrf-token";

#[derive(serde::Serialize, ToSchema)]
pub struct CsrfResponse {
    csrf_token: String,
}

/// Get CSRF Token
#[utoipa::path(
    get,
    responses(
        (
            status = 200,
            description = "A CSRF Token for the session",
            body = CsrfResponse,
            headers(
                ("x-request-id", description = "Request identifier")
            )
        ),
    ),
    operation_id = "auth-csrf", // https://github.com/juhaku/utoipa/issues/1170
    path = "/auth/csrf",
    tag = super::AUTH
)]
#[instrument(name = "get_csrf", skip_all, err(Debug))]
pub async fn get(session: Session) -> Result<Json<CsrfResponse>, AppError> {
    let token = CsrfToken::new_random().secret().to_owned();

    session.insert(SESSION_FORM_CSRF, token.clone()).await?;

    Ok(Json(CsrfResponse { csrf_token: token }))
}

pub async fn validate_form_csrf(session: &Session, headers: &HeaderMap) -> Result<(), AppError> {
    let expected = session
        .get::<String>(SESSION_FORM_CSRF)
        .await?
        .context("CSRF Missing")?;

    let received = headers
        .get(CSRF_HEADER)
        .and_then(|value| value.to_str().ok())
        .context("CSRF Missing")?;

    if received != expected {
        return Err(anyhow::anyhow!("CSRF mismatch").into());
    }

    // one-time token behavior.
    session.remove::<String>(SESSION_FORM_CSRF).await?;

    Ok(())
}
