use axum::async_trait;
use sellershut_core::users::{
    mutate_users_server::MutateUsers, CreateUserRequest, CreateUserResponse, UpdateUserRequest,
    UpdateUserResponse, UpsertUserRequest, UpsertUserResponse,
};

use super::AppState;

#[async_trait]
impl MutateUsers for AppState {
    #[must_use]
    async fn create_user(
        &self,
        request: tonic::Request<CreateUserRequest>,
    ) -> Result<tonic::Response<CreateUserResponse>, tonic::Status> {
        todo!()
    }

    #[must_use]
    async fn update_user(
        &self,
        request: tonic::Request<UpdateUserRequest>,
    ) -> Result<tonic::Response<UpdateUserResponse>, tonic::Status> {
        todo!()
    }

    #[must_use]
    async fn upsert_user(
        &self,
        request: tonic::Request<UpsertUserRequest>,
    ) -> Result<tonic::Response<UpsertUserResponse>, tonic::Status> {
        todo!()
    }
}
