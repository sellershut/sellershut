use opentelemetry_semantic_conventions::trace;
use sellershut_core::{
    google::protobuf::Empty,
    users::{
        CreateUserResponse, QueryUserByIdRequest, QueryUserByIdResponse, QueryUserByNameRequest,
        QueryUserByNameResponse, query_users_server::QueryUsers,
    },
};
use tracing::{Instrument, instrument};
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::entity;

use super::state::ServiceState;

#[tonic::async_trait]
impl QueryUsers for ServiceState {
    #[must_use]
    #[instrument(skip(self), err(Debug))]
    async fn query_users(
        &self,
        _request: tonic::Request<Empty>,
    ) -> Result<tonic::Response<CreateUserResponse>, tonic::Status> {
        todo!()
    }

    #[must_use]
    #[instrument(skip(self), err(Debug))]
    async fn query_user_by_name(
        &self,
        request: tonic::Request<QueryUserByNameRequest>,
    ) -> Result<tonic::Response<QueryUserByNameResponse>, tonic::Status> {
        let username = request.into_inner().username;

        let user = sqlx::query_as!(
            entity::User,
            "select * from \"user\" where username = $1",
            username
        )
        .fetch_optional(&self.database)
        .instrument({
            let span = tracing::info_span!("db.query");
            span.set_attribute(trace::DB_OPERATION_NAME, "select");
            span.set_attribute(trace::DB_QUERY_PARAMETER, "username");
            span.set_attribute(trace::DB_SYSTEM, "postgres");
            span
        })
        .await
        .map_err(|e| tonic::Status::unavailable(e.to_string()))?;

        let resp = QueryUserByNameResponse {
            user: user.map(Into::into),
        };

        Ok(tonic::Response::new(resp))
    }

    #[must_use]
    #[instrument(skip(self), err(Debug))]
    async fn query_local_user_by_name(
        &self,
        request: tonic::Request<QueryUserByNameRequest>,
    ) -> Result<tonic::Response<QueryUserByNameResponse>, tonic::Status> {
        let username = request.into_inner().username;

        let user = sqlx::query_as!(
            entity::User,
            "select * from \"user\" where username = $1 and local = $2",
            username,
            true
        )
        .fetch_optional(&self.database)
        .instrument({
            let span = tracing::info_span!("db.query");
            span.set_attribute(trace::DB_OPERATION_NAME, "select");
            span.set_attribute(trace::DB_QUERY_PARAMETER, "username");
            span.set_attribute(trace::DB_SYSTEM, "postgres");
            span
        })
        .await
        .map_err(|e| tonic::Status::unavailable(e.to_string()))?;

        let resp = QueryUserByNameResponse {
            user: user.map(Into::into),
        };

        Ok(tonic::Response::new(resp))
    }

    #[must_use]
    #[instrument(skip(self), err(Debug))]
    async fn query_user_by_id(
        &self,
        request: tonic::Request<QueryUserByIdRequest>,
    ) -> Result<tonic::Response<QueryUserByIdResponse>, tonic::Status> {
        let id = request.into_inner().id;

        let user = sqlx::query_as!(entity::User, "select * from \"user\" where ap_id = $1", id)
            .fetch_optional(&self.database)
            .instrument({
                let span = tracing::info_span!("db.query");
                span.set_attribute(trace::DB_OPERATION_NAME, "select");
                span.set_attribute(trace::DB_QUERY_PARAMETER, "ap_id");
                span.set_attribute(trace::DB_SYSTEM, "postgres");
                span
            })
            .await
            .map_err(|e| tonic::Status::unavailable(e.to_string()))?;

        let resp = QueryUserByIdResponse {
            user: user.map(Into::into),
        };

        Ok(tonic::Response::new(resp))
    }
}
