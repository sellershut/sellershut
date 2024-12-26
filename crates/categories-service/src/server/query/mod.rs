use sellershut_core::{
    categories::{
        Connection, GetAllSubCategoriesRequest, GetAllSubCategoriesResponse,
        GetCategoryByIdResponse, GetCategoryRequest, GetSubCategoriesRequest,
        query_categories_server::QueryCategories,
    },
    common::pagination::Cursor,
};
use tracing::instrument;

use super::state::ServiceState;

#[tonic::async_trait]
impl QueryCategories for ServiceState {
    #[doc = " gets all categories"]
    #[must_use]
    #[instrument(skip(self), err(Debug))]
    async fn categories(
        &self,
        _request: tonic::Request<Cursor>,
    ) -> Result<tonic::Response<Connection>, tonic::Status> {
        todo!()
    }

    #[doc = " get category by id"]
    #[must_use]
    #[instrument(skip(self), err(Debug))]
    async fn category_by_id(
        &self,
        _request: tonic::Request<GetCategoryRequest>,
    ) -> Result<tonic::Response<GetCategoryByIdResponse>, tonic::Status> {
        todo!()
    }

    #[doc = " get subcategories"]
    #[must_use]
    #[instrument(skip(self), err(Debug))]
    async fn sub_categories(
        &self,
        _request: tonic::Request<GetSubCategoriesRequest>,
    ) -> Result<tonic::Response<Connection>, tonic::Status> {
        todo!()
    }

    #[doc = " get subcategories"]
    #[must_use]
    #[instrument(skip(self), err(Debug))]
    async fn all_sub_categories(
        &self,
        _request: tonic::Request<GetAllSubCategoriesRequest>,
    ) -> Result<tonic::Response<GetAllSubCategoriesResponse>, tonic::Status> {
        todo!()
    }
}
