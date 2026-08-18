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

#[derive(OpenApi)]
#[openapi(tags((name = AUTH_TAG, description = "Authentication")))]
pub struct AuthDoc;
