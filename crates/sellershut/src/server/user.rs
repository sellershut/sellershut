use std::str::FromStr;

use activitypub_federation::{
    axum::json::FederationJson, config::Data, protocol::context::WithContext, traits::Object,
};
use anyhow::Context;
use axum::{extract::Path, Json};
use serde::Deserialize;
use url::Url;

use crate::{
    entities::user::{FederatedUser, Person},
    state::AppState,
};

use super::AppError;

pub async fn get(
    Path(id): Path<String>,
    data: Data<AppState>,
) -> Result<FederationJson<WithContext<Person>>, AppError> {
    let url = Url::from_str(&id)?;
    let results = FederatedUser::read_from_id(url, &data)
        .await?
        .context("no user available")?;

    let json_user = results.into_json(&data).await?;
    Ok(FederationJson(WithContext::new_default(json_user)))
}

#[derive(Debug, Deserialize)]
pub struct UserUpsertData {
    hostname: String,
    username: String,
}

pub async fn upsert(
    data: Data<AppState>,
    Json(user): Json<UserUpsertData>,
) -> Result<FederationJson<WithContext<Person>>, AppError> {
    let results = FederatedUser::new(&user.hostname, &user.username)?;

    let person = results.into_json(&data).await?;
    FederatedUser::from_json(person.clone(), &data).await?;

    Ok(FederationJson(WithContext::new_default(person)))
}
