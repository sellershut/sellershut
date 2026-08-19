use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

pub mod me;

pub fn router() -> OpenApiRouter {
    let router = OpenApiRouter::new();

    router.routes(utoipa_axum::routes!(me::me))
}

const USERS_TAG: &str = "Users";

#[derive(OpenApi)]
#[openapi(tags((name = USERS_TAG, description = "Users")))]
pub struct UsersDoc;
