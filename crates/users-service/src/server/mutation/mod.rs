use sellershut_core::users::{
    CreateUserRequest, CreateUserResponse, DeleteUserRequest, DeleteUserResponse,
    FollowUserRequest, FollowUserResponse, UpdateUserRequest, UpdateUserResponse,
    UpsertUserRequest, UpsertUserResponse, User, mutate_users_server::MutateUsers,
};
use tonic::async_trait;
use tracing::{Instrument, debug_span, instrument};

use crate::entity;

use super::state::ServiceState;

#[async_trait]
impl MutateUsers for ServiceState {
    #[must_use]
    #[instrument(skip(self), err(Debug))]
    async fn create_user(
        &self,
        request: tonic::Request<CreateUserRequest>,
    ) -> Result<tonic::Response<CreateUserResponse>, tonic::Status> {
        let data = request.into_inner().user;

        let user = sqlx::query_as!(
            entity::User,
            "insert into \"user\" (id, username, followers, avatar_url, inbox, public_key, private_key, local, email, display_name)
                values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) returning *",
            &data.id,
            &data.username,
            &data.followers,
            data.avatar_url,
            &data.inbox,
            &data.public_key,
            data.private_key.as_deref(),
            &data.local,
            data.email,
            data.display_name,
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
    #[instrument(skip(self), err(Debug))]
    async fn upsert_user(
        &self,
        request: tonic::Request<UpsertUserRequest>,
    ) -> Result<tonic::Response<UpsertUserResponse>, tonic::Status> {
        let data = request.into_inner().user;

        let user = sqlx::query_as!(
            entity::User,
            "insert into \"user\" (id, username, followers, avatar_url, inbox, public_key, private_key, local, email, display_name)
                values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                on conflict (id)
                do update 
                set username = excluded.username,
                    followers = excluded.followers,
                    avatar_url = excluded.avatar_url,
                    inbox = excluded.inbox,
                    public_key = excluded.public_key,
                    private_key = excluded.private_key,
                    local = excluded.local
                returning *",
            &data.id,
            &data.username,
            &data.followers,
            data.avatar_url,
            &data.inbox,
            &data.public_key,
            data.private_key.as_deref(),
            &data.local,
            data.email,
            data.display_name
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
    #[instrument(skip(self), err(Debug))]
    async fn follow_user(
        &self,
        request: tonic::Request<FollowUserRequest>,
    ) -> Result<tonic::Response<FollowUserResponse>, tonic::Status> {
        let data = request.into_inner();

        let user = sqlx::query_as!(
            entity::User,
            "update \"user\" set followers = array_append(followers, $1)
            where id = $2 and not $1 = any(followers)
            returning *",
            &data.follow_url,
            &data.url,
        )
        .fetch_one(&self.database)
        .instrument(debug_span!("pg.upsert"))
        .await
        .map_err(|e| tonic::Status::internal(e.to_string()))?;

        let user = User::from(user);

        let req = FollowUserResponse { user };

        Ok(tonic::Response::new(req))
    }

    #[must_use]
    #[instrument(skip(self), err(Debug))]
    async fn update_user(
        &self,
        _request: tonic::Request<UpdateUserRequest>,
    ) -> Result<tonic::Response<UpdateUserResponse>, tonic::Status> {
        todo!()
    }

    #[must_use]
    #[instrument(skip(self), err(Debug))]
    async fn delete_user(
        &self,
        _request: tonic::Request<DeleteUserRequest>,
    ) -> Result<tonic::Response<DeleteUserResponse>, tonic::Status> {
        todo!()
    }
}
