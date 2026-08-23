pub mod get_inbox;
pub mod post_inbox;

use utoipa_axum::router::OpenApiRouter;

pub fn router() -> OpenApiRouter {
    let router = OpenApiRouter::new();

    router
        .routes(utoipa_axum::routes!(get_inbox::get))
        .routes(utoipa_axum::routes!(post_inbox::post))
}
