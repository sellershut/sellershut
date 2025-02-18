use async_graphql::MergedObject;

use crate::entities::user::{mutation::UserGraphqlMutation, query::UserGraphqlQuery};

#[derive(Default, Debug, MergedObject)]
pub struct GraphQLQueries(UserGraphqlQuery);

#[derive(Default, Debug, MergedObject)]
pub struct GraphQLMutations(UserGraphqlMutation);
