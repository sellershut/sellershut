use async_graphql::{Context, MergedObject, Object};
use tracing::instrument;

#[derive(Default, Debug, MergedObject)]
pub struct Query(GraphqlQuery);

#[derive(Default, Debug)]
pub struct GraphqlQuery;

#[Object]
impl GraphqlQuery {
    #[instrument(skip(self, ctx), err(Debug))]
    async fn create(&self, ctx: &Context<'_>) -> String {
        String::default()
    }
}
