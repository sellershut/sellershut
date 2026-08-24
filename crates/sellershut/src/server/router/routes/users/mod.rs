use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

pub mod followers;
pub mod following;
pub mod inbox;
pub mod likes;
pub mod me;
pub mod outbox;

pub fn router() -> OpenApiRouter {
    let router = OpenApiRouter::new();

    router
        .routes(utoipa_axum::routes!(me::me))
        .routes(utoipa_axum::routes!(followers::followers))
        .routes(utoipa_axum::routes!(following::following))
        .routes(utoipa_axum::routes!(likes::likes))
        .merge(inbox::router())
        .merge(outbox::router())
}

const USERS_TAG: &str = "Users";

#[derive(OpenApi)]
#[openapi(tags((name = USERS_TAG, description = "Users")))]
pub struct UsersDoc;
