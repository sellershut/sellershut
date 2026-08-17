use cookie::{Cookie, SameSite};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

use crate::server::state::AppState;
pub mod authorised;
pub mod login;

pub fn router(state: AppState) -> OpenApiRouter {
    let router = OpenApiRouter::new();

    router
        .routes(utoipa_axum::routes!(login::login))
        .routes(utoipa_axum::routes!(authorised::authorised))
        .with_state(state)
}

const AUTH_TAG: &str = "Authentication";

const ONBOARDING_COOKIE: &str = "auth_onboarding";
const SESSION_COOKIE: &str = "auth_session";

#[derive(OpenApi)]
#[openapi(tags((name = AUTH_TAG, description = "Authentication")))]
pub struct AuthDoc;

fn removal_cookie(name: String, path: String, secure: bool) -> Cookie<'static> {
    Cookie::build(name)
        .path(path)
        .http_only(true)
        .same_site(SameSite::Lax)
        .secure(secure)
        .build()
}
