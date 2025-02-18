use async_graphql::{Context, Object};

#[derive(Default, Debug)]
pub struct UserGraphqlMutation;

#[Object]
impl UserGraphqlMutation {
    async fn create_user(&self, _ctx: &Context<'_>) -> String {
        String::default()
    }
}
