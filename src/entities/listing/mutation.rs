use async_graphql::{Context, Object};

#[derive(Default, Debug)]
pub struct ListingGraphqlMutation;

#[Object]
impl ListingGraphqlMutation {
    async fn create_listing(&self, _ctx: &Context<'_>) -> String {
        String::default()
    }

    async fn update_listing(&self, _ctx: &Context<'_>) -> String {
        String::default()
    }

    async fn delete_listing(&self, _ctx: &Context<'_>) -> String {
        String::default()
    }
}
