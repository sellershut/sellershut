pub mod entity;
pub mod mutation;
pub mod query;

use async_graphql::{EmptySubscription, Schema};
use mutation::Mutation;
use query::Query;
use sellershut_core::categories::{
    mutate_categories_server::MutateCategories, query_categories_server::QueryCategories,
};

pub struct ApiSchemaBuilder {}

pub type ApiSchema = Schema<Query, Mutation, EmptySubscription>;

impl ApiSchemaBuilder {
    pub fn build<T>(data: T) -> ApiSchema
    where
        T: QueryCategories + MutateCategories,
    {
        Schema::build(Query::default(), Mutation::default(), EmptySubscription)
            .data(data)
            .finish()
    }
}
