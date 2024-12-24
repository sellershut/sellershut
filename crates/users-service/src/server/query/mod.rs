use sellershut_core::{
    google::protobuf::Empty,
    users::{
        CreateUserResponse, QueryUserByIdRequest, QueryUserByIdResponse, QueryUserByNameRequest,
        QueryUserByNameResponse, query_users_server::QueryUsers,
    },
};
use tracing::instrument;

use crate::entity;

use super::state::ServiceState;

#[tonic::async_trait]
impl QueryUsers for ServiceState {
    #[must_use]
    async fn query_users(
        &self,
        _request: tonic::Request<Empty>,
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
    #[instrument(skip(self))]
    async fn query_user_by_id(
        &self,
        request: tonic::Request<QueryUserByIdRequest>,
    ) -> Result<tonic::Response<QueryUserByIdResponse>, tonic::Status> {
        let id = request.into_inner().id;

        let user = sqlx::query_as!(entity::User, "select * from \"user\" where id = $1", id)
            .fetch_one(&self.database)
            .await
            .map_err(|e| tonic::Status::unavailable(e.to_string()))?;

        let resp = QueryUserByIdResponse { user: user.into() };

        Ok(tonic::Response::new(resp))
    }
}
