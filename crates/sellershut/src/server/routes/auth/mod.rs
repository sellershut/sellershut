mod authorised;
use anyhow::Context;
use async_session::Session;
use axum::{
    extract::{Query, State},
    http::{HeaderMap, header::SET_COOKIE},
    response::{IntoResponse, Redirect},
};
use serde::Deserialize;
use utoipa::{IntoParams, OpenApi};
use utoipa_axum::router::OpenApiRouter;

use crate::{
    server::{OauthProvider, cache_key::CacheKey, error::AppError},
    state::AppState,
};

const AUTH: &str = "Authentication";

static COOKIE_NAME: &str = "SESSION";

static CSRF_TOKEN: &str = "csrf_token";

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
pub async fn auth(
    Query(params): Query<OauthParams>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let client = state
        .oauth_clients
        .get(&params.provider)
        .context("selected auth provider is not supported")?;
    let (auth_url, csrf_token) = auth::create_csrf_token(client, &params.provider.scopes());

    let mut session = Session::new();
    session.insert(CSRF_TOKEN, &csrf_token)?;

    let cache_key = CacheKey::Session(session.id());

    let cache_client = state.cache;
    let cache_session = serde_json::to_vec(&session)?;

    cache_client.set(cache_key, cache_session).await?;

    let cookie = session.into_cookie_value().context("missing session")?;
    let cookie = format!("{COOKIE_NAME}={cookie}; SameSite=Lax; HttpOnly; Secure; Path=/");

    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        cookie.parse().context("failed to parse cookie")?,
    );

    Ok((headers, Redirect::to(auth_url.as_ref())))
}
