use async_graphql::{Context, MergedObject, Object};
use tracing::instrument;

#[derive(Default, Debug, MergedObject)]
pub struct Mutation(GraphqlMutation);

#[derive(Default, Debug)]
pub struct GraphqlMutation;

#[Object]
impl GraphqlMutation {
    #[instrument(skip(self, ctx), err(Debug))]
    async fn create(&self, ctx: &Context<'_>) -> String {
        String::default()
    }
}
