use async_graphql::{Context, Object};

#[derive(Default, Debug)]
pub struct ListingGraphqlMutation;

#[Object]
impl ListingGraphqlMutation {
    async fn create_listing(&self, _ctx: &Context<'_>) -> String {
        String::default()
    }
}
