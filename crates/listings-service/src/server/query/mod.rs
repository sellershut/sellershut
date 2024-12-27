use sellershut_core::listings::{
    QueryListingByIdRequest, QueryListingByIdResponse, query_listings_server::QueryListings,
};
use tonic::async_trait;
use tracing::instrument;

use super::state::ServiceState;

#[async_trait]
impl QueryListings for ServiceState {
    #[must_use]
    #[instrument(skip(self), err(Debug))]
    async fn listings_by_id(
        &self,
        _request: tonic::Request<QueryListingByIdRequest>,
    ) -> Result<tonic::Response<QueryListingByIdResponse>, tonic::Status> {
        todo!()
    }
}
