use axum::async_trait;
use sellershut_core::users::{
    query_users_server::QueryUsers, CreateUserRequest, CreateUserResponse,
};

use super::AppState;

#[async_trait]
impl QueryUsers for AppState {
    #[must_use]
    async fn query_users(
        &self,
        request: tonic::Request<CreateUserRequest>,
    ) -> Result<tonic::Response<CreateUserResponse>, tonic::Status> {
        todo!()
    }
}
