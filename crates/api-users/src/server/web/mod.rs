pub mod graphql;
pub mod routes;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQL, GraphQLSubscription};
use axum::{response::Html, routing::get, Router};
use graphql::ApiSchemaBuilder;
use infra::config::Environment;
use routes::{
    auth::github::{github_auth, login_authorised_github},
    health::health_check,
};

use crate::state::AppState;

pub fn router(state: AppState) -> Router {
    let env = state.config.application.env;
    let schema = ApiSchemaBuilder::build(state.clone());
    let router = Router::new()
        .route("/health", get(health_check))
        .route("/auth/github", get(github_auth))
        .route("/auth/authorised", get(login_authorised_github))
        .with_state(state);

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
