use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

pub mod authorised;
pub mod login;
pub mod logout;
pub mod onboard;

pub fn router() -> OpenApiRouter {
    let router = OpenApiRouter::new();

    router
        .routes(utoipa_axum::routes!(login::login))
        .routes(utoipa_axum::routes!(authorised::authorised))
        .routes(utoipa_axum::routes!(onboard::complete_onboarding))
        .routes(utoipa_axum::routes!(logout::logout))
}

const AUTH_TAG: &str = "Authentication";

#[derive(OpenApi)]
#[openapi(tags((name = AUTH_TAG, description = "Authentication")))]
pub struct AuthDoc;
