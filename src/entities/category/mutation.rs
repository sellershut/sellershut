use async_graphql::{Context, Object};

#[derive(Default, Debug)]
pub struct CategoryGraphqlMutation;

#[Object]
impl CategoryGraphqlMutation {
    async fn categories(&self, ctx: &Context<'_>) -> String {
        String::default()
    }
}
