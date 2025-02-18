pub mod categories;
mod health;
pub mod users;
mod webfinger;

use activitypub_federation::config::FederationConfig;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_axum::{GraphQL, GraphQLSubscription};
use axum::{response::Html, routing::get, Router};
pub use health::*;
pub use webfinger::*;

use crate::state::AppHandle;

use super::graphql::{GraphQLMutations, GraphQLQueries};

pub fn graphql<T>(router: Router<T>, data: FederationConfig<AppHandle>) -> Router<T>
where
    T: Clone + Send + Sync + 'static,
{
    let schema = Schema::build(
        GraphQLQueries::default(),
        GraphQLMutations::default(),
        EmptySubscription,
    )
    .data(data)
    .finish();
    router
        .route(
            "/api",
            get(|| async {
                Html(playground_source(
                    GraphQLPlaygroundConfig::new("/api").subscription_endpoint("/ws"),
                ))
            })
            .post_service(GraphQL::new(schema.clone())),
        )
        .route_service("/ws", GraphQLSubscription::new(schema))
}
