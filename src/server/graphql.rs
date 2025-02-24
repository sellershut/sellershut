use async_graphql::MergedObject;

use crate::entities::{
    category::{mutation::CategoryGraphqlMutation, query::CategoryGraphqlQuery},
    listing::{mutation::ListingGraphqlMutation, query::ListingGraphqlQuery},
    user::{mutation::UserGraphqlMutation, query::UserGraphqlQuery},
};

#[derive(Default, Debug, MergedObject)]
pub struct GraphQLQueries(UserGraphqlQuery, CategoryGraphqlQuery, ListingGraphqlQuery);

#[derive(Default, Debug, MergedObject)]
pub struct GraphQLMutations(
    UserGraphqlMutation,
    CategoryGraphqlMutation,
    ListingGraphqlMutation,
);
