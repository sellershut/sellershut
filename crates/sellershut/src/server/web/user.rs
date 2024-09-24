use activitypub_federation::{
    axum::json::FederationJson, config::Data, protocol::context::WithContext, traits::Object,
};
use anyhow::Context;
use axum::extract::Path;
use sellershut_core::users::{
    query_users_client::QueryUsersClient, QueryUserByNameRequest, User as DbUser,
};
use tonic::{transport::Channel, IntoRequest};
use tracing::instrument;

use crate::{
    entities::user::{LocalUser, User},
    state::AppState,
};

use super::AppError;

pub async fn get_user(
    Path(name): Path<String>,
    data: Data<AppState>,
) -> Result<FederationJson<WithContext<User>>, AppError> {
    let user = get_user_by_name(&name, data.query_users_client.clone())
        .await?
        .context("no user found")?;

    let json_user = LocalUser::from(user).into_json(&data).await?;
    Ok(FederationJson(WithContext::new_default(json_user)))
}

#[instrument(skip(client))]
pub async fn get_user_by_name(
    username: &str,
    mut client: QueryUsersClient<Channel>,
) -> Result<Option<DbUser>, AppError> {
    let request = QueryUserByNameRequest {
        username: username.to_string(),
    };

    Ok(client
        .query_user_by_name(request.into_request())
        .await?
        .into_inner()
        .user)
}
