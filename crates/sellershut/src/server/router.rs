use axum::{Router, middleware};
use tower_sessions::{
    Expiry, SessionManagerLayer,
    cookie::{SameSite, time::Duration},
};
use utoipa::{
    Modify, OpenApi,
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
};
use utoipa_axum::router::OpenApiRouter;

use crate::{
    server::{
        self,
        routes::{self, auth::AuthDoc},
    },
    state::AppState,
};

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    tags(
        (name = "sellershut", description = env!("CARGO_PKG_DESCRIPTION")),
    ),
)]
pub struct ApiDoc;

pub struct SecurityAddon;

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

pub async fn router(state: AppState) -> anyhow::Result<Router<()>> {
    state.session_store.migrate().await?;

    let mut doc = ApiDoc::openapi();
    doc.merge(AuthDoc::openapi());

    let stubs = OpenApiRouter::with_openapi(doc)
        .routes(utoipa_axum::routes!(routes::protected::protected))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            server::middleware::auth,
        ))
        .routes(utoipa_axum::routes!(routes::health::health))
        .nest("/api", routes::auth::router(state.clone()))
        .with_state(state.clone());

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

    let session_layer = SessionManagerLayer::new(state.session_store.clone())
        .with_name("hut.sid")
        .with_secure(true)
        .with_http_only(true)
        .with_same_site(SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(Duration::days(14)));

    Ok(router
        .layer(middleware::from_fn(server::middleware::trace_request))
        .layer(middleware::from_fn(server::middleware::request_id))
        .layer(session_layer))
}
