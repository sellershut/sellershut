use async_graphql::MergedObject;

use crate::entities::{
    category::{mutation::CategoryGraphqlMutation, query::CategoryGraphqlQuery},
    user::{mutation::UserGraphqlMutation, query::UserGraphqlQuery},
};

#[derive(Default, Debug, MergedObject)]
pub struct GraphQLQueries(UserGraphqlQuery, CategoryGraphqlQuery);

#[derive(Default, Debug, MergedObject)]
pub struct GraphQLMutations(UserGraphqlMutation, CategoryGraphqlMutation);
