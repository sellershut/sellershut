use activitypub_federation::config::Data;
use axum::{Json, extract::Path, response::IntoResponse};
use sellershut_auth::{AuthenticatedSession, LoginOutcome};
use sellershut_core::{auth::OauthProvider, types::user::User};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::server::{AppError, router::routes::auth::AUTH_TAG, state::AppState};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct OauthResponse {
    code: String,
    state: String,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum OAuthCallbackResponse {
    Authenticated { session_token: String, user: User },
    OnboardingRequired { onboarding_token: String },
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
        LoginOutcome::Authenticated(AuthenticatedSession { token, user }) => {
            OAuthCallbackResponse::Authenticated {
                session_token: token,
                user,
            }
        }
        LoginOutcome::OnboardingRequired { onboarding_token } => {
            OAuthCallbackResponse::OnboardingRequired { onboarding_token }
        }
    };
    Ok(Json(resp).into_response())
}
