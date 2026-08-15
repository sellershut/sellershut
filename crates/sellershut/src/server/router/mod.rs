mod middleware;
mod routes;

use std::time::Duration;

use axum::{Router, http::StatusCode};
use tower_http::{cors::CorsLayer, timeout::TimeoutLayer};
use utoipa::{
    Modify, OpenApi,
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
};
use utoipa_axum::router::OpenApiRouter;

use crate::config::Configuration;

pub struct SecurityAddon;

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    tags(
        (name = "sellershut", description = env!("CARGO_PKG_DESCRIPTION")),
    ),
)]
pub struct ApiDoc;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
            );
        }
    }
}

pub async fn router(config: Configuration) -> anyhow::Result<Router> {
    let doc = ApiDoc::openapi();

    let stubs = OpenApiRouter::with_openapi(doc).routes(utoipa_axum::routes!(routes::health));

    let (router, api) = stubs.split_for_parts();

    let router = {
        use utoipa_scalar::Servable as _;
        router.merge(utoipa_scalar::Scalar::with_url("/scalar", api))
    };

    let cors = &config.server.cors;
    let methods: Vec<_> = (&cors.allowed_methods).into();
    let origins: Result<Vec<_>, _> = cors
        .allowed_origins
        .iter()
        .map(|v| v.as_str().parse())
        .collect();

    Ok(middleware::trace_layer(router)
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(config.server.request.timeout_duration),
        ))
        .layer(
            CorsLayer::new()
                .allow_origin(origins.expect("valid allow_origin"))
                .allow_methods(methods),
        )
        .layer(axum::middleware::from_fn(middleware::request_id)))
}
