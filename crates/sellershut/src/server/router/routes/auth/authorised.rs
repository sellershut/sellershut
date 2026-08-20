use activitypub_federation::config::Data;
use axum::{Json, extract::Path, response::IntoResponse};
use sellershut_auth::LoginOutcome;
use sellershut_core::{auth::OauthProvider, user::User};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::server::{AppError, router::routes::auth::AUTH_TAG, state::AppState};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct OauthResponse {
    code: String,
    state: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginAuthenticated {
    kind: AuthorisedKind,
    session_token: String,
    user: User,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnboardingNeeded {
    kind: AuthorisedKind,
    onboarding_token: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AuthorisedKind {
    OnboardingRequired,
    Authenticated,
}

/// OAuth authorisation callback
#[utoipa::path(
    post,
    path = "/{provider}/authorised",
    params(
        ("provider" = OauthProvider, Path, description = "OAuth provider")
    ),
    responses(
        (status = 200, description = "Authorization successful",
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
pub async fn authorised(
    Path(provider): Path<OauthProvider>,
    state: Data<AppState>,
    Json(callback): Json<OauthResponse>,
) -> Result<impl IntoResponse, AppError> {
    let code = callback.code;

    let outcome = state
        .auth
        .authorise(provider, &code, &callback.state)
        .await?;
    let resp = match outcome {
        LoginOutcome::Authenticated(value) => Json(LoginAuthenticated {
            kind: AuthorisedKind::Authenticated,
            session_token: value.token,
            user: value.user,
        })
        .into_response(),
        LoginOutcome::OnboardingRequired { onboarding_token } => Json(OnboardingNeeded {
            onboarding_token,
            kind: AuthorisedKind::OnboardingRequired,
        })
        .into_response(),
    };
    Ok(resp)
}
