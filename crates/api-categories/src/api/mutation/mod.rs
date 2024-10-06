use async_graphql::{Context, MergedObject, Object, Result};
use sellershut_core::categories::{
    mutate_categories_server::MutateCategories, CategoryEvent, DeleteCategoryRequest,
    UpsertCategoryRequest,
};
use tonic::IntoRequest;
use tracing::instrument;

use crate::{api::entity::Category, state::ApiState};

#[derive(Default, Debug, MergedObject)]
pub struct Mutation(GraphqlMutation);

#[derive(Default, Debug)]
pub struct GraphqlMutation;

#[Object]
impl GraphqlMutation {
    #[instrument(skip(self, ctx), err(Debug))]
    async fn create(&self, ctx: &Context<'_>, input: Category) -> Result<Category> {
        let service = ctx.data::<ApiState>()?;

        let category = Some(sellershut_core::categories::Category::from(input));
        let request = UpsertCategoryRequest {
            category,
            event: CategoryEvent::Create.into(),
        };

        let res = service.create(request.into_request()).await?.into_inner();

        Category::try_from(res)
    }

    #[instrument(skip(self, ctx), err(Debug))]
    async fn update(&self, ctx: &Context<'_>, input: Category) -> Result<Category> {
        let service = ctx.data::<ApiState>()?;

        let category = Some(sellershut_core::categories::Category::from(input));
        let request = UpsertCategoryRequest {
            category,
            event: CategoryEvent::Update.into(),
        };

        let res = service.update(request.into_request()).await?.into_inner();

        Category::try_from(res)
    }

    #[instrument(skip(ctx), err(Debug))]
    async fn delete(&self, ctx: &Context<'_>, id: String) -> Result<Option<Category>> {
        let service = ctx.data::<ApiState>()?;

        let req = DeleteCategoryRequest {
            id,
            event: CategoryEvent::Delete.into(),
        };

        let _res = service.delete(req.into_request()).await?.into_inner();

        Ok(None)
    }
}
