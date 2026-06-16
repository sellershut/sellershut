use axum::http::HeaderMap;
use tower_sessions::Session;
use tracing::instrument;

use crate::server::error::AppError;

/// Complete Onboarding
#[utoipa::path(
    get,
    responses(
        (
            status = 200,
            description = "Saves the user",
            headers(
                ("x-request-id", description = "Request identifier")
            )
        ),
    ),
    operation_id = "auth-complete-profile", // https://github.com/juhaku/utoipa/issues/1170
    path = "/auth/complete-profile",
    tag = super::AUTH
)]
#[instrument(name = "get_csrf", skip_all, err(Debug))]
pub async fn complete_profile(session: Session, headers: HeaderMap) -> Result<String, AppError> {
    super::csrf::validate_form_csrf(&session, &headers).await?;

    Ok(Default::default())
}
