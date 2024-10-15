pub mod mutation;
pub mod query;

use async_graphql::{EmptySubscription, Schema};
use mutation::Mutation;
use query::Query;
use sellershut_core::users::{mutate_users_server::MutateUsers, query_users_server::QueryUsers};

pub struct ApiSchemaBuilder {}

pub type ApiSchema = Schema<Query, Mutation, EmptySubscription>;

impl ApiSchemaBuilder {
    pub fn build<T>(data: T) -> ApiSchema
    where
        T: MutateUsers + QueryUsers,
    {
        Schema::build(Query::default(), Mutation::default(), EmptySubscription)
            .data(data)
            .finish()
    }
}
