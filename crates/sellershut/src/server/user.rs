use std::str::FromStr;

use activitypub_federation::{
    axum::json::FederationJson, config::Data, protocol::context::WithContext, traits::Object,
};
use anyhow::Context;
use axum::extract::Path;
use url::Url;

use crate::{
    entities::user::{LocalUser, Person},
    state::AppState,
};

use super::AppError;

pub async fn get_user(
    Path(id): Path<String>,
    data: Data<AppState>,
) -> Result<FederationJson<WithContext<Person>>, AppError> {
    let url = Url::from_str(&id)?;
    let results = LocalUser::read_from_id(url, &data).await?.context(":")?;

    let json_user = results.into_json(&data).await?;
    Ok(FederationJson(WithContext::new_default(json_user)))
}
