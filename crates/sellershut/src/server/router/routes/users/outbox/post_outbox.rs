use activitypub_federation::
    config::Data 
;
use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use tracing::debug;

use crate::server::{
    entities::user::Person,
    router::routes::users::USERS_TAG,
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
        (status = 200, description = "Current user", body = Person,
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
    tag = USERS_TAG,
)]
pub async fn post(
    Path(username): Path<String>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    state: Data<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let token = bearer.token();

    let user =  state.user.user_from_session(token).await.map_err(|_e| {
        debug!(username=username, "user not found from session");
        StatusCode::UNAUTHORIZED
    })?;

    if user.preferred_username.ne(&username) {
      return  Err(StatusCode::FORBIDDEN);
    }


    Ok(())
}
