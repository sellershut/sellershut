use infra::events::{Entity, Event};
use sellershut_core::{
    categories::{
        mutate_categories_server::MutateCategories, Category, CategoryEvent, DeleteCategoryRequest,
        UpsertCategoryRequest,
    },
    google::protobuf::Empty,
};
use sellershut_utils::id::generate_id;
use tracing::{debug, debug_span, Instrument};

use crate::{
    api::entity,
    state::{database::publish_event, ApiState},
};

use super::map_err;

#[tonic::async_trait]
impl MutateCategories for ApiState {
    #[doc = " Create a category"]
    #[must_use]
    #[tracing::instrument(skip(self), err(Debug))]
    async fn create(
        &self,
        request: tonic::Request<UpsertCategoryRequest>,
    ) -> Result<tonic::Response<Category>, tonic::Status> {
        let category = request.into_inner().category.expect("category to exist");
        let id = generate_id();

        // Check if the value fits within the range of i64
        let category = sqlx::query_as!(
            entity::Category,
            "insert into category (id, name, sub_categories, image_url, parent_id)
                values ($1, $2, $3, $4, $5) returning *",
            &id,
            &category.name,
            &category.sub_categories,
            category.image_url,
            category.parent_id
        )
        .fetch_one(&self.state.postgres)
        .instrument(debug_span!("pg.insert"))
        .await
        .map_err(|e| tonic::Status::internal(e.to_string()))?;

        let category = Category::from(category);

        let req = UpsertCategoryRequest {
            category: Some(category.clone()),
            event: CategoryEvent::Create.into(),
        };

        let event = Event::SetSingle(Entity::Categories);

        publish_event(req, event, &self.state.jetstream).await?;

        Ok(tonic::Response::new(category))
    }

    #[doc = " Update a category"]
    #[must_use]
    #[tracing::instrument(skip(self), err(Debug))]
    async fn update(
        &self,
        request: tonic::Request<UpsertCategoryRequest>,
    ) -> Result<tonic::Response<Category>, tonic::Status> {
        let category = request.into_inner().category.expect("category to exist");
        // Check if the value fits within the range of i64
        let category = sqlx::query_as!(
            entity::Category,
            "update category set name = $2, sub_categories = $3, image_url = $4, parent_id = $5
                where id = $1 returning *",
            category.id,
            category.name,
            &category.sub_categories,
            category.image_url,
            category.parent_id,
        )
        .fetch_one(&self.state.postgres)
        .instrument(debug_span!("pg.update"))
        .await
        .map_err(map_err)?;

        let category = Category::from(category);

        let event = Event::UpdateSingle(Entity::Categories);
        publish_event(category.clone(), event, &self.state.jetstream).await?;

        Ok(tonic::Response::new(category))
    }

    #[doc = " Delete a category"]
    #[must_use]
    #[tracing::instrument(skip(self), err(Debug))]
    async fn delete(
        &self,
        request: tonic::Request<DeleteCategoryRequest>,
    ) -> Result<tonic::Response<Empty>, tonic::Status> {
        let id = request.into_inner().id;

        sqlx::query!("delete from category where id = $1", id)
            .execute(&self.state.postgres)
            .instrument(debug_span!("pg.delete"))
            .await
            .map_err(map_err)?;
        debug!("row deleted");

        let event = Event::DeleteSingle(Entity::Categories);

        publish_event(Empty::default(), event, &self.state.jetstream).await?;

        Ok(tonic::Response::new(Empty::default()))
    }
}
