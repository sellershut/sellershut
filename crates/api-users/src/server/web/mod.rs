pub mod graphql;
pub mod routes;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQL, GraphQLSubscription};
use axum::{extract::Request, response::Html, routing::get, Router};
use graphql::ApiSchemaBuilder;
use infra::config::Environment;
use routes::health::health_check;
use tower_http::trace::TraceLayer;
use tracing::info_span;

use crate::state::AppState;

pub fn router(state: AppState) -> Router {
    let env = state.config.application.env;
    let schema = ApiSchemaBuilder::build(state);
    let router = Router::new().route("/health", get(health_check));

    let router = match env {
        Environment::Development => router.route(
            "/",
            get(|| async {
                Html(playground_source(
                    GraphQLPlaygroundConfig::new("/").subscription_endpoint("/ws"),
                ))
            })
            .post_service(GraphQL::new(schema.clone())),
        ),
        Environment::Production => router.route(
            "/",
            get(|| async {
                format!(
                    "{} v{} is live",
                    env!("CARGO_CRATE_NAME"),
                    env!("CARGO_PKG_VERSION")
                )
            })
            .post_service(GraphQL::new(schema.clone())),
        ),
    };

    router
        .route_service("/ws", GraphQLSubscription::new(schema))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                info_span!(
                    "http_request",
                    method = ?request.method(),
                    trace_id = tracing::field::Empty,
                )
            }), //            .on_request(on_request),
        )
}
