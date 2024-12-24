use sellershut_core::users::{
    CreateUserRequest, CreateUserResponse, DeleteUserRequest, DeleteUserResponse,
    UpdateUserRequest, UpdateUserResponse, User, mutate_users_server::MutateUsers,
};
use tonic::async_trait;
use tracing::{Instrument, debug_span};
use url::Url;

use crate::entity;

use super::state::ServiceState;

#[async_trait]
impl MutateUsers for ServiceState {
    #[must_use]
    async fn create_user(
        &self,
        request: tonic::Request<CreateUserRequest>,
    ) -> Result<tonic::Response<CreateUserResponse>, tonic::Status> {
        let data = request.into_inner();

        let ap_id = Url::parse(&format!("http://{}/{}", data.hostname, &data.username))
            .map_err(|e| tonic::Status::invalid_argument(e.to_string()))?
            .to_string();

        let inbox = Url::parse(&format!(
            "http://{}/{}/inbox",
            data.hostname, &data.username
        ))
        .map_err(|e| tonic::Status::invalid_argument(e.to_string()))?
        .to_string();

        let keypair = activitypub_federation::http_signatures::generate_actor_keypair()
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        let user = sqlx::query_as!(
            entity::User,
            "insert into \"user\" (id, username, followers, avatar_url, email, inbox, public_key, private_key, local)
                values ($1, $2, $3, $4, $5, $6, $7, $8, $9) returning *",
            &ap_id,
            &data.username,
            &data.followers,
            data.avatar_url,
            &data.email,
            &inbox,
            &keypair.public_key,
            &keypair.private_key,
            &data.local,
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
