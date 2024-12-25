use sellershut_core::users::{
    CreateUserRequest, CreateUserResponse, DeleteUserRequest, DeleteUserResponse,
    UpdateUserRequest, UpdateUserResponse, UpsertUserRequest, UpsertUserResponse, User,
    mutate_users_server::MutateUsers,
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

        let user = sqlx::query_as!(
            entity::User,
            "insert into \"user\" (id, username, followers, avatar_url, email, inbox, public_key, private_key, local)
                values ($1, $2, $3, $4, $5, $6, $7, $8, $9) returning *",
            &data.id,
            &data.username,
            &data.followers,
            data.avatar_url,
            &data.email,
            &data.inbox,
            &data.public_key,
            data.private_key.as_deref(),
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
    async fn upsert_user(
        &self,
        request: tonic::Request<UpsertUserRequest>,
    ) -> Result<tonic::Response<UpsertUserResponse>, tonic::Status> {
        let data = request.into_inner().user;

        let user = sqlx::query_as!(
            entity::User,
            "insert into \"user\" (id, username, followers, avatar_url, email, inbox, public_key, private_key, local)
                values ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                on conflict (id)
                do update 
                set username = excluded.username,
                    followers = excluded.followers,
                    avatar_url = excluded.avatar_url,
                    email = excluded.email,
                    inbox = excluded.inbox,
                    public_key = excluded.public_key,
                    private_key = excluded.private_key,
                    local = excluded.local
                returning *",
            &data.id,
            &data.username,
            &data.followers,
            data.avatar_url,
            &data.email,
            &data.inbox,
            &data.public_key,
            data.private_key.as_deref(),
            &data.local,
        )
        .fetch_one(&self.database)
        .instrument(debug_span!("pg.upsert"))
        .await
        .map_err(|e| tonic::Status::internal(e.to_string()))?;

        let user = User::from(user);

        let req = UpsertUserResponse { user };

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
