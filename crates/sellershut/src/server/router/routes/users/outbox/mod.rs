pub mod get_outbox;
pub mod post_outbox;

use utoipa_axum::router::OpenApiRouter;

pub fn router() -> OpenApiRouter {
    let router = OpenApiRouter::new();

    router
        .routes(utoipa_axum::routes!(get_outbox::get))
        .routes(utoipa_axum::routes!(post_outbox::post))
}
