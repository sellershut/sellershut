use anyhow::Context;
use axum::{
    extract::{Path, Query, State},
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::CookieJar;
use sellershut_auth::{
    AuthenticatedSession, LoginOutcome, ONBOARDING_MAX_AGE_SECONDS, SESSION_MAX_AGE_SECONDS,
};
use sellershut_core::auth::OauthProvider;
use serde::{Deserialize, Serialize};

use crate::server::{
    AppError,
    router::routes::auth::{ONBOARDING_COOKIE, SESSION_COOKIE, login::auth_cookie, removal_cookie},
    state::AppState,
};

#[derive(Serialize, Deserialize)]
pub struct OauthResponse {
    code: String,
    state: String,
}

/// Handles the OAuth authorization callback for the specified authentication provider.
#[utoipa::path(
    get,
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
    )
)]
pub async fn authorised(
    State(state): State<AppState>,
    Query(callback): Query<OauthResponse>,
    Path(provider): Path<OauthProvider>,
    jar: CookieJar,
) -> Result<impl IntoResponse, AppError> {
    let code = callback.code;
    let callback_state = callback.state;
    let browser_state = jar.get(&provider.cookie_name());
    dbg!("cookie", browser_state);

    let browser_state = browser_state
        .map(|cookie| cookie.value().to_owned())
        .context("Invalid oauth state")?;

    let outcome = state
        .auth
        .authorise(provider, &code, &callback_state, &browser_state)
        .await?;

    let jar = jar.remove(removal_cookie(
        provider.cookie_name(),
        format!("/auth/{provider}/authorised"),
        state.cookie_secure,
    ));

    match outcome {
        LoginOutcome::Authenticated(AuthenticatedSession { token, .. }) => {
            let jar = jar
                .remove(removal_cookie(
                    ONBOARDING_COOKIE.to_owned(),
                    "/auth".to_owned(),
                    state.cookie_secure,
                ))
                .add(auth_cookie(
                    SESSION_COOKIE.to_owned(),
                    token,
                    "/".to_owned(),
                    SESSION_MAX_AGE_SECONDS,
                    state.cookie_secure,
                ));

            //Ok((jar, Redirect::to(&state.frontend_authenticated_url)).into_response())
            Ok((jar, Redirect::to("http://localhost:5173")).into_response())
        }
        LoginOutcome::OnboardingRequired { onboarding_token } => {
            let jar = jar.add(auth_cookie(
                ONBOARDING_COOKIE.to_owned(),
                onboarding_token,
                "/auth".to_owned(),
                ONBOARDING_MAX_AGE_SECONDS,
                state.cookie_secure,
            ));
            Ok((jar, Redirect::to("http://localhost:5173")).into_response())

            //            Ok((jar, Redirect::to(&state.frontend_onboarding_url)).into_response())
        }
    }
}
