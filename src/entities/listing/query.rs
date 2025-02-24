use async_graphql::{Context, Object};

#[derive(Default, Debug)]
pub struct ListingGraphqlQuery;

#[Object]
impl ListingGraphqlQuery {
    async fn listings(&self, _ctx: &Context<'_>) -> String {
        String::default()
    }
}
