use activitypub_federation::{
    axum::json::FederationJson, config::Data, protocol::context::WithContext, traits::Object,
};
use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use tracing::debug;

use crate::server::{
    entities::user::{Person, User},
    router::routes::users::USERS_TAG,
    state::AppState,
};
/// Get current user
#[utoipa::path(
    get,
    path = "/me",
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
        (status = 500, description = "Internal server error")
    ),
    tag = USERS_TAG,
)]
pub async fn me(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    state: Data<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let token = bearer.token();

    match state.user.user_from_session(token).await {
        Ok(result) => match User::from(result).into_json(&state).await {
            Ok(u) => {
                let context = WithContext::new_default(u);
                Ok(FederationJson(context).into_response())
            }
            Err(e) => {
                tracing::error!(session =?token,error=?e, "user decode failed");
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        },
        Err(e) => {
            debug!(session =?bearer,err=?e, "unauthorised session");
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}
