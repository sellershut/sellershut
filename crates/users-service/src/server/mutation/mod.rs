use sellershut_core::users::{
    CreateUserRequest, CreateUserResponse, DeleteUserRequest, DeleteUserResponse,
    UpdateUserRequest, UpdateUserResponse, User, mutate_users_server::MutateUsers,
};
use tonic::async_trait;
use tracing::{Instrument, debug_span};

use crate::entity;

use super::state::ServiceState;

#[async_trait]
impl MutateUsers for ServiceState {
    #[must_use]
    async fn create_user(
        &self,
        request: tonic::Request<CreateUserRequest>,
    ) -> Result<tonic::Response<CreateUserResponse>, tonic::Status> {
        let data = request.into_inner().user;

        let id = sellershut_utils::id::generate_id();

        let user = sqlx::query_as!(
            entity::User,
            "insert into \"user\" (id, username, followers, avatar_url, email)
                values ($1, $2, $3, $4, $5) returning *",
            &id,
            &data.username,
            &data.followers,
            data.avatar_url,
            &data.email
        )
        .fetch_one(&self.database)
        .instrument(debug_span!("pg.insert"))
        .await
        .map_err(|e| tonic::Status::internal(e.to_string()))?;

        let user = User::from(user);

        let req = CreateUserResponse { user };

        Ok(tonic::Response::new(req))
    }

    #[must_use]
    async fn update_user(
        &self,
        _request: tonic::Request<UpdateUserRequest>,
    ) -> Result<tonic::Response<UpdateUserResponse>, tonic::Status> {
        todo!()
    }

    #[must_use]
    async fn delete_user(
        &self,
        _request: tonic::Request<DeleteUserRequest>,
    ) -> Result<tonic::Response<DeleteUserResponse>, tonic::Status> {
        todo!()
    }
}
