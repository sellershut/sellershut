use activitypub_federation::{
    config::Data, http_signatures::generate_actor_keypair, traits::Object,
};
use axum::{Json, response::IntoResponse};
use sellershut_auth::AuthenticatedSession;
use sellershut_core::{RedactedSecret, auth::OauthProvider, user::ActorType};
use sellershut_users::CreateUser;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::server::{
    AppError,
    entities::user::{Person, User},
    router::routes::auth::AUTH_TAG,
    state::AppState,
    utilities,
};

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OnboardingRequest {
    onboarding_token: String,
    username: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
struct SessionResponse {
    session_token: String,
    user: Person,
}

/// Onboard
#[utoipa::path(
    post,
    path = "/onboard",
    params(
        ("provider" = OauthProvider, Path, description = "OAuth provider")
    ),
    responses(
        (status = 200, description = "Authorization successful", body = SessionResponse,
            headers(
                (
                    "x-request-id" = String,
                    description = "Unique identifier for the request"
                )
            )
         ),
        (status = 400, description = "Invalid provider"),
        (status = 401, description = "Unauthorized")
    ),
    tag = AUTH_TAG,
)]
pub async fn complete_onboarding(
    state: Data<AppState>,
    Json(request): Json<OnboardingRequest>,
) -> Result<impl IntoResponse, AppError> {
    let domain = state.domain();
    let port = state.port;

    let ap_id = utilities::users_url(port, domain, &request.username)?;
    let inbox = utilities::inbox_url(port, domain, &request.username)?;
    tracing::debug!(id =%ap_id, inbox=%inbox,"creating user");

    let keypair = generate_actor_keypair()?;

    let user_data = CreateUser {
        kind: ActorType::Person,
        ap_id,
        username: request.username,
        name: None,
        inbox,
        avatar: None,
        public_key: keypair.public_key,
        private_key: Some(RedactedSecret::from(keypair.private_key)),
        is_local: true,
    };

    let AuthenticatedSession { token, user } = state
        .auth
        .complete_onboarding(&request.onboarding_token, &user_data)
        .await?;

    let user: User = user.into();

    Ok(Json(SessionResponse {
        session_token: token,
        user: user.into_json(&state).await?,
    })
    .into_response())
}
