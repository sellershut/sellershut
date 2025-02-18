use activitypub_federation::config::FederationConfig;
use async_graphql::{Context, Object, Result};
use sellershut_core::categories::{
    CreateCategoryRequest, DeleteCategoryRequest, UpsertCategoryRequest,
};
use tonic::IntoRequest;
use tracing::instrument;

use crate::{entities::user::GraphQLCategoryType as Category, state::AppHandle};

#[derive(Default, Debug)]
pub struct CategoryGraphqlMutation;

#[Object]
impl CategoryGraphqlMutation {
    #[instrument(skip(self, ctx), err(Debug))]
    async fn create_category(&self, ctx: &Context<'_>, input: Category) -> Result<Category> {
        let service = ctx.data::<FederationConfig<AppHandle>>()?;

        let mut client = service.mutate_categories_client.clone();

        let category = Some(sellershut_core::categories::Category::from(input));
        let request = CreateCategoryRequest { category };

        let res = client
            .create(request.into_request())
            .await?
            .into_inner()
            .category
            .ok_or_else(|| async_graphql::Error::new("insert did not return any records"))?;

        Ok(Category::try_from(res)?)
    }

    #[instrument(skip(self, ctx), err(Debug))]
    async fn update_category(&self, ctx: &Context<'_>, input: Category) -> Result<Category> {
        let service = ctx.data::<FederationConfig<AppHandle>>()?;

        let mut client = service.mutate_categories_client.clone();

        let category = Some(sellershut_core::categories::Category::from(input));
        let request = UpsertCategoryRequest { category };

        let res = client
            .upsert(request.into_request())
            .await?
            .into_inner()
            .category
            .ok_or_else(|| async_graphql::Error::new("insert did not return any records"))?;

        Ok(Category::try_from(res)?)
    }

    #[instrument(skip(ctx), err(Debug))]
    async fn delete_category(&self, ctx: &Context<'_>, id: String) -> Result<Option<Category>> {
        let service = ctx.data::<FederationConfig<AppHandle>>()?;

        let mut client = service.mutate_categories_client.clone();

        let req = DeleteCategoryRequest { ap_id: id };

        let _res = client.delete(req.into_request()).await?.into_inner();

        Ok(None)
    }
}
