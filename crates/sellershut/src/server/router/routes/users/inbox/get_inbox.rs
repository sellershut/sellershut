use activitypub_federation::{
    axum::json::FederationJson, config::Data, protocol::context::WithContext, traits::Object,
};
use axum::{extract::Path, http::StatusCode, response::IntoResponse};
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
/// Return the user's inbox collection.
#[utoipa::path(
    get,
    path = "/{username}/inbox",
    security(
        ("bearer_auth" = [])

    ),
    responses(
        (status = 200, description = "A collcetion", body = Person,
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
    params(
            ("username" = String, Path, description = "username", example = "rando69")
    ),
    tag = USERS_TAG,
)]
pub async fn get(
    Path(_username): Path<String>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    state: Data<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let token = bearer.token();

    match state.user.user_from_session(token).await {
        Ok(result) => {
            let user = User::from_database(result, &*state.user)
                .await
                .map_err(|e| {
                    tracing::error!(e=?e, "onboarding");
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;
            match user.into_json(&state).await {
                Ok(u) => {
                    let context = WithContext::new_default(u);
                    Ok(FederationJson(context).into_response())
                }
                Err(e) => {
                    tracing::error!(error=?e, "user decode failed");
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        Err(e) => {
            debug!(err=?e, "unauthorised session");
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}
