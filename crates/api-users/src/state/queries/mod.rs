use crate::entity::User as DbUser;
use axum::async_trait;
use sellershut_core::users::{
    query_users_server::QueryUsers, CreateUserRequest, CreateUserResponse, QueryUserByApIdRequest,
    QueryUserByApIdResponse, QueryUserByIdRequest, QueryUserByIdResponse, QueryUserByNameRequest,
    QueryUserByNameResponse, User,
};
use tonic::{Request, Response, Status};
use tracing::{info_span, warn, Instrument};

use super::AppState;

#[async_trait]
impl QueryUsers for AppState {
    #[must_use]
    async fn query_users(
        &self,
        _request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        todo!()
    }

    #[must_use]
    async fn query_user_by_name(
        &self,
        request: Request<QueryUserByNameRequest>,
    ) -> Result<Response<QueryUserByNameResponse>, Status> {
        let db = &self.services.postgres;

        let params = request.into_inner().username;

        let user = sqlx::query_as!(DbUser, "select * from \"user\" where username = $1", params)
            .fetch_one(db)
            .instrument(info_span!("db.user.by.name"))
            .await
            .map_err(|e| {
                warn!("{e}");
                tonic::Status::not_found("user not found")
            })?;

        let response = QueryUserByNameResponse {
            user: Some(User::from(user)),
        };

        Ok(Response::new(response))
    }

    #[must_use]
    async fn query_user_by_ap_id(
        &self,
        request: Request<QueryUserByApIdRequest>,
    ) -> Result<Response<QueryUserByApIdResponse>, Status> {
        let db = &self.services.postgres;

        let params = request.into_inner().ap_id;

        let user = sqlx::query_as!(DbUser, "select * from \"user\" where ap_id = $1", params)
            .fetch_one(db)
            .instrument(info_span!("db.user.by.ap_id"))
            .await
            .map_err(|e| {
                warn!("{e}");
                tonic::Status::not_found("user not found")
            })?;

        let response = QueryUserByApIdResponse {
            user: Some(User::from(user)),
        };

        Ok(Response::new(response))
    }

    #[must_use]
    async fn query_user_by_id(
        &self,
        request: Request<QueryUserByIdRequest>,
    ) -> Result<Response<QueryUserByIdResponse>, Status> {
        let db = &self.services.postgres;

        let params = request.into_inner().id;

        let user = sqlx::query_as!(DbUser, "select * from \"user\" where id = $1", params)
            .fetch_one(db)
            .instrument(info_span!("db.user.by.id"))
            .await
            .map_err(|e| {
                warn!("{e}");
                tonic::Status::not_found("user not found")
            })?;

        let response = QueryUserByIdResponse {
            user: Some(User::from(user)),
        };

        Ok(Response::new(response))
    }
}
