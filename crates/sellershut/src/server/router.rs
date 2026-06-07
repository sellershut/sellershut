use axum::{Router, middleware};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

use crate::{
    server::{self, routes},
    state::AppState,
};

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "sellershut", description = env!("CARGO_PKG_DESCRIPTION")),
    ),
)]
pub struct ApiDoc;

pub async fn router(state: AppState) -> Router<()> {
    let mut doc = ApiDoc::openapi();

    let stubs = OpenApiRouter::with_openapi(doc)
        .routes(utoipa_axum::routes!(routes::health::health))
        .with_state(state);

    let (router, _api) = stubs.split_for_parts();

    #[cfg(feature = "doc-swagger")]
    let router = router.merge(
        utoipa_swagger_ui::SwaggerUi::new("/swagger-ui")
            .url("/api-docs/swaggerdoc.json", _api.clone()),
    );

    #[cfg(feature = "doc-redoc")]
    let router = {
        use utoipa_redoc::Servable as _;
        router.merge(utoipa_redoc::Redoc::with_url("/redoc", _api.clone()))
    };

    #[cfg(feature = "doc-scalar")]
    let router = {
        use utoipa_scalar::Servable as _;
        router.merge(utoipa_scalar::Scalar::with_url("/scalar", _api.clone()))
    };

    #[cfg(feature = "doc-rapidoc")]
    let router = router.merge(
        utoipa_rapidoc::RapiDoc::with_openapi("/api-docs/rapidoc.json", _api).path("/rapidoc"),
    );

    router
        .layer(middleware::from_fn(server::middleware::trace_request))
        .layer(middleware::from_fn(server::middleware::request_id))
}
