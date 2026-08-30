use activitypub_federation::{
    axum::inbox::{ActivityData, receive_activity},
    config::Data,
    protocol::context::WithContext,
};
use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use tracing::debug;

use crate::server::{
    AppError,
    activities::OutboxActivity,
    entities::user::{Person, User},
    router::routes::users::outbox::{OUTBOX_TAG, OutboxSubmission},
    state::AppState,
};
/// Publish an activity on behalf of the user
#[utoipa::path(
    post,
    path = "/{username}/outbox",
    security(
        ("bearer_auth" = [])

    ),
    responses(
        (status = 201, description = "Current user", body = Person,
            headers(
                (
                    "x-request-id" = String,
                    description = "Unique identifier for the request"
                )
            )
         ),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 500, description = "Internal server error")
    ),
    params(
            ("username" = String, Path, description = "username", example = "rando69")
    ),
    tag = OUTBOX_TAG,
)]
pub async fn post(
    Path(username): Path<String>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    state: Data<AppState>,
    //Json(payload): Json<OutboxSubmission>,
    data: ActivityData,
) -> Result<impl IntoResponse, AppError> {
    let token = bearer.token();

    let user = state.user.user_from_session(token).await;

    if let Err(_code) = user {
        debug!(username = username, "user not found from session");
        return Ok((StatusCode::UNAUTHORIZED).into_response());
    }
    let user = user?;

    let preferred_username = user.preferred_username.to_owned();
    if preferred_username.ne(&username) {
        return Ok((StatusCode::FORBIDDEN).into_response());
    }

    receive_activity::<WithContext<OutboxSubmission>, User, AppState>(data, &state)
        .await
        .unwrap();

    Ok((StatusCode::CREATED).into_response())
}
