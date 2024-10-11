use std::str::FromStr;

use activitypub_federation::{
    config::Data,
    fetch::webfinger::{build_webfinger_response, extract_webfinger_name, Webfinger},
};
use anyhow::Context;
use axum::{debug_handler, extract::Query, Json};
use serde::Deserialize;
use url::Url;

use crate::state::AppState;

use super::{user::get_user_by_name, AppError};

#[derive(Deserialize)]
pub struct WebfingerQuery {
    resource: String,
}

#[debug_handler]
pub async fn webfinger(
    Query(query): Query<WebfingerQuery>,
    data: Data<AppState>,
) -> Result<Json<Webfinger>, AppError> {
    let name = extract_webfinger_name(&query.resource, &data)?;

    let response = get_user_by_name(name, &data)
        .await?
        .context("no user")
        .map(|value| Url::from_str(&value.ap_id))??;

    Ok(Json(build_webfinger_response(query.resource, response)))
}
