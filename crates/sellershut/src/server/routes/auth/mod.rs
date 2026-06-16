mod authorised;
mod csrf;
mod onboard;
use anyhow::Context;
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use serde::Deserialize;
use utoipa::{IntoParams, OpenApi};
use utoipa_axum::router::OpenApiRouter;

use crate::{
    server::{OauthProvider, error::AppError},
    state::AppState,
};
use tracing::{info, instrument, trace};

const AUTH: &str = "Authentication";

const SESSION_PENDING_OAUTH_ID: &str = "pending_oauth_id";
const SESSION_USER_ID: &str = "user_id";

#[derive(OpenApi)]
#[openapi(tags((name = AUTH, description = "Auth endpoints")),components(schemas(OauthProvider))) ]
pub struct AuthDoc;

/// Oauth provider
#[derive(Deserialize, IntoParams)]
pub struct OauthParams {
    provider: OauthProvider,
}

pub fn router(store: AppState) -> OpenApiRouter<AppState> {
    let router = OpenApiRouter::new();

    let router = router
        .routes(utoipa_axum::routes!(auth))
        .routes(utoipa_axum::routes!(csrf::get))
        .routes(utoipa_axum::routes!(onboard::complete_profile))
        .routes(utoipa_axum::routes!(authorised::authorised));

    router.with_state(store)
}

/// Initiate oauth flow
#[utoipa::path(
    get,
    responses(
        (
            status = 302,
            description = "A redirect to the provider's auth URL",
            headers(
                ("x-request-id", description = "Request identifier"),
                ("set-cookie", description = "Oauth session cookie")
            )
        ),
    ),
    operation_id = "auth", // https://github.com/juhaku/utoipa/issues/1170
    path = "/auth",
    tag = AUTH,
    params(OauthParams)
)]
#[instrument(
    name = "oauth_auth",
    skip_all,
    err(Debug),
    fields(provider = ?params.provider)
)]
#[axum::debug_handler]
pub async fn auth(
    Query(params): Query<OauthParams>,
    State(state): State<AppState>,
    session: tower_sessions::Session,
) -> Result<impl IntoResponse, AppError> {
    trace!("starting oauth auth flow");

    let client = state
        .oauth_clients
        .get(&params.provider)
        .context("selected auth provider is not supported")?;

    trace!("oauth client found");

    let (auth_url, csrf_token) = auth::create_csrf_token(client, &params.provider.scopes());

    trace!("created auth url and csrf token");

    let session_state = session_oauth_state_key(params.provider);
    session
        .insert(&session_state, csrf_token.secret().to_owned())
        .await?;

    info!(
        redirect_url = %auth_url.as_ref(),
        "redirecting to oauth provider"
    );

    Ok(Redirect::to(auth_url.as_ref()))
}

fn session_oauth_state_key(provider: OauthProvider) -> String {
    format!("oauth_state:{provider}")
}
