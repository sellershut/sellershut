mod health;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQL, GraphQLSubscription};
use axum::{response::Html, routing::get, Router};
use infra::config::Environment;

use crate::api::ApiSchema;

pub fn router(schema: ApiSchema, env: Environment) -> Router {
    let router = Router::new().route("/health", get(health::health_check));

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

    router.route_service("/ws", GraphQLSubscription::new(schema))
}
