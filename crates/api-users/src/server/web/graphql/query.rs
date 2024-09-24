use async_graphql::{Context, MergedObject, Object};
use tracing::instrument;

use crate::state::AppState;

#[derive(Default, Debug, MergedObject)]
pub struct Query(GraphqlQuery);

#[derive(Default, Debug)]
pub struct GraphqlQuery;

#[Object]
impl GraphqlQuery {
    #[instrument(skip(self, ctx), err(Debug))]
    async fn get_user_by_name(
        &self,
        ctx: &Context<'_>,
        username: String,
    ) -> async_graphql::Result<Option<String>> {
        let data = ctx.data::<AppState>()?;

        Ok(String::default().into())
    }
}
