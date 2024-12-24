use sellershut_core::users::{
    CreateUserRequest, CreateUserResponse, QueryUserByIdRequest, QueryUserByIdResponse,
    QueryUserByNameRequest, QueryUserByNameResponse, query_users_server::QueryUsers,
};

use super::state::ServiceState;

#[tonic::async_trait]
impl QueryUsers for ServiceState {
    #[must_use]
    async fn query_users(
        &self,
        _request: tonic::Request<CreateUserRequest>,
    ) -> Result<tonic::Response<CreateUserResponse>, tonic::Status> {
        todo!()
    }

    #[must_use]
    async fn query_user_by_name(
        &self,
        _request: tonic::Request<QueryUserByNameRequest>,
    ) -> Result<tonic::Response<QueryUserByNameResponse>, tonic::Status> {
        todo!()
    }

    #[must_use]
    async fn query_user_by_id(
        &self,
        _request: tonic::Request<QueryUserByIdRequest>,
    ) -> Result<tonic::Response<QueryUserByIdResponse>, tonic::Status> {
        todo!()
    }
}
