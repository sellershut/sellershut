use activitypub_federation::{
    axum::json::FederationJson, config::Data, protocol::context::WithContext, traits::Object,
};
use anyhow::Context;
use axum::extract::Path;
use tracing::instrument;

use crate::{
    entities::user::{LocalUser, Person},
    state::AppState,
};

use super::AppError;

pub async fn get_user(
    Path(name): Path<String>,
    data: Data<AppState>,
) -> Result<FederationJson<WithContext<Person>>, AppError> {
    let user = get_user_by_name(&name, &data)
        .await?
        .context("no user found")?;

    let json_user = LocalUser::from(user).into_json(&data).await?;
    Ok(FederationJson(WithContext::new_default(json_user)))
}

#[instrument(skip(client))]
pub async fn get_user_by_name(
    username: &str,
    client: &AppState,
) -> Result<Option<LocalUser>, AppError> {
    todo!()
}
