use sellershut_core::{
    categories::{
        Category, DeleteCategoryRequest, UpsertCategoryRequest,
        mutate_categories_server::MutateCategories,
    },
    google::protobuf::Empty,
};
use tracing::instrument;

use super::state::ServiceState;

#[tonic::async_trait]
impl MutateCategories for ServiceState {
    #[doc = " Create a category"]
    #[must_use]
    #[instrument(skip(self), err(Debug))]
    async fn create(
        &self,
        _request: tonic::Request<UpsertCategoryRequest>,
    ) -> Result<tonic::Response<Category>, tonic::Status> {
        todo!()
    }

    #[doc = " Update a category"]
    #[must_use]
    #[instrument(skip(self), err(Debug))]
    async fn update(
        &self,
        _request: tonic::Request<UpsertCategoryRequest>,
    ) -> Result<tonic::Response<Category>, tonic::Status> {
        todo!()
    }

    #[doc = " Delete a category"]
    #[must_use]
    #[instrument(skip(self), err(Debug))]
    async fn delete(
        &self,
        _request: tonic::Request<DeleteCategoryRequest>,
    ) -> Result<tonic::Response<Empty>, tonic::Status> {
        todo!()
    }
}
