use axum::{
    Json,
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use sellershut_auth::OAUTH_STATE_MAX_AGE_SECONDS;
use sellershut_core::auth::OauthProvider;
use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

use crate::server::{AppError, router::routes::auth::AUTH_TAG, state::AppState};

/// AuthQuery
#[derive(Deserialize, IntoParams)]
pub struct AuthQuery {
    /// Oauth provider
    #[param(inline)]
    provider: OauthProvider,
}

#[derive(Serialize)]
struct StartOAuthResponse {
    authorisation_url: String,
}

/// Oauth login
#[utoipa::path(
    get,
    responses(
        (
            status = 302,
            description = "Redirects the user to the selected OAuth provider",
            headers(
                (
                    "Location" = String,
                    description = "OAuth provider authorization URL"
                ),
                (
                    "x-request-id" = String,
                    description = "Unique identifier for the request"
                )
            )
        ),
        (
            status = 400,
            description = "Invalid OAuth provider",
            headers(
                (
                    "x-request-id" = String,
                    description = "Unique identifier for the request"
                )
            )
        ),
        (
            status = 500,
            description = "Failed to generate the OAuth authorization URL",
            headers(
                (
                    "x-request-id" = String,
                    description = "Unique identifier for the request"
                )
            )
        )
    ),
    operation_id = "login", // https://github.com/juhaku/utoipa/issues/1170
    path = "/login",
    tag = AUTH_TAG,
    params(
        AuthQuery
    ),
)]
pub async fn login(
    Query(query): Query<AuthQuery>,
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<impl IntoResponse, AppError> {
    let start = state.auth.start_oauth(query.provider).await.unwrap();

    let callback_url = format!("/auth/{}/authorised", query.provider);

    let jar = jar.add(auth_cookie(
        query.provider.cookie_name(),
        start.browser_state,
        callback_url,
        OAUTH_STATE_MAX_AGE_SECONDS,
        state.cookie_secure,
    ));

    Ok((
        jar,
        Redirect::to(&start.authorisation_url),
        // Json(StartOAuthResponse {
        //     authorisation_url: start.authorisation_url,
        // }),
    )
        .into_response())
}

pub fn auth_cookie(
    name: String,
    value: String,
    path: String,
    max_age_seconds: i64,
    secure: bool,
) -> Cookie<'static> {
    Cookie::build((name, value))
        .path(path)
        .http_only(true)
        .same_site(SameSite::Lax)
        .secure(secure)
        .max_age(cookie::time::Duration::seconds(max_age_seconds))
        .build()
}
