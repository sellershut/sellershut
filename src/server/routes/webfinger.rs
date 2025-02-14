use activitypub_federation::{
    config::Data,
    fetch::webfinger::{build_webfinger_response, extract_webfinger_name, Webfinger},
};
use anyhow::Context;
use axum::{debug_handler, extract::Query, response::IntoResponse, Json};
use serde::Deserialize;
use url::Url;

use crate::{
    server::{error::AppError, grpc::get_user_by_name},
    state::AppHandle,
};

#[derive(Clone, Debug, Deserialize)]
pub struct WebFingerParams {
    resource: String,
}

#[debug_handler]
pub async fn web_finger(
    Query(params): Query<WebFingerParams>,
    data: Data<AppHandle>,
) -> Result<Json<Webfinger>, AppError> {
    let name = extract_webfinger_name(&params.resource, &data)?;
    let db_user = get_user_by_name(name, data)
        .await?
        .context("no such user exists")?;

    let url = Url::parse(&db_user.0.ap_id)?;

    Ok(Json(build_webfinger_response(params.resource, url)))
}
