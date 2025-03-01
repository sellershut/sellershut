use async_graphql::{Context, Object};

#[derive(Default, Debug)]
pub struct UserGraphqlQuery;

#[Object]
impl UserGraphqlQuery {
    async fn users(&self, _ctx: &Context<'_>) -> String {
        String::default()
    }
}
