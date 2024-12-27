use sellershut_core::listings::{
    CreateListingRequest, CreateListingResponse, mutate_listings_server::MutateListings,
};
use tonic::async_trait;
use tracing::instrument;

use super::state::ServiceState;

#[async_trait]
impl MutateListings for ServiceState {
    #[must_use]
    #[instrument(skip(self), err(Debug))]
    async fn create_listing(
        &self,
        _request: tonic::Request<CreateListingRequest>,
    ) -> Result<tonic::Response<CreateListingResponse>, tonic::Status> {
        todo!()
    }
}
