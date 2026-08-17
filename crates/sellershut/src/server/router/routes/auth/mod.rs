use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

use crate::server::state::AppState;
pub mod auth;

pub fn router(state: AppState) -> OpenApiRouter {
    let router = OpenApiRouter::new();

    router
        .routes(utoipa_axum::routes!(auth::login))
        .with_state(state)
}

const AUTH_TAG: &str = "Authentication";

#[derive(OpenApi)]
#[openapi(tags((name = AUTH_TAG, description = "Authentication")))]
pub struct AuthDoc;
