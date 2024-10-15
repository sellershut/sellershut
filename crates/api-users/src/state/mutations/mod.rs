use std::collections::HashMap;

use axum::async_trait;
use sellershut_core::users::{
    mutate_users_server::MutateUsers, CreateUserRequest, CreateUserResponse, UpdateUserRequest,
    UpdateUserResponse, UpsertUserRequest, UpsertUserResponse, User,
};
use sellershut_utils::id::generate_id;
use tracing::{debug_span, info_span, Instrument};

use crate::entity;

use super::AppState;

#[async_trait]
impl MutateUsers for AppState {
    #[must_use]
    async fn create_user(
        &self,
        request: tonic::Request<CreateUserRequest>,
    ) -> Result<tonic::Response<CreateUserResponse>, tonic::Status> {
        let data = request.into_inner().user.expect("user to exist");

        let id = generate_id();

        let user = sqlx::query_as!(
            entity::User,
            "insert into \"user\" (id, username, followers)
                values ($1, $2, $3) returning *",
            &id,
            &data.username,
            &data.followers,
        )
        .fetch_one(&self.services.postgres)
        .instrument(debug_span!("pg.insert"))
        .await
        .map_err(|e| tonic::Status::internal(e.to_string()))?;

        let user = User::from(user);

        let mut map = HashMap::new();
        map.insert("hostname", "api-categories");
        map.insert("username", &user.username);

        let res = self
            .http_client
            .post("http://httpbin.org/post")
            .json(&map)
            .send()
            .instrument(info_span!("upstream.user.create"))
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        let req = CreateUserResponse { user: Some(user) };

        Ok(tonic::Response::new(req))
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
