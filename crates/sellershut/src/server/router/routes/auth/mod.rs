use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
pub mod auth;

pub fn router() -> OpenApiRouter {
    let router = OpenApiRouter::new();

    router.routes(utoipa_axum::routes!(auth::get_user))
}

const AUTH_TAG: &str = "Authentication";

#[derive(OpenApi)]
#[openapi(tags((name = AUTH_TAG, description = "Authentication")))]
pub struct AuthDoc;
