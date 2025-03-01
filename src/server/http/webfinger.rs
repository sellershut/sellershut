use activitypub_federation::{
    config::Data,
    fetch::{
        object_id::ObjectId,
        webfinger::{Webfinger, build_webfinger_response, extract_webfinger_name},
    },
};
use axum::{Json, debug_handler, extract::Query};
use serde::Deserialize;
use tracing::instrument;
use url::Url;

use crate::{entities::user::HutUser, server::error::AppError, state::AppHandle};

#[derive(Clone, Debug, Deserialize)]
pub struct WebFingerParams {
    resource: String,
}

#[debug_handler]
#[instrument(skip(data), err(Debug))]
pub async fn web_finger(
    Query(params): Query<WebFingerParams>,
    data: Data<AppHandle>,
) -> Result<Json<Webfinger>, AppError> {
    let name = extract_webfinger_name(&params.resource, &data)?;

    let user_id = ObjectId::<HutUser>::parse(&format!("{}/users/{name}", data.hostname))?;
    let db_user = user_id.dereference(&data).await?;

    let url = Url::parse(&db_user.0.ap_id)?;

    Ok(Json(build_webfinger_response(params.resource, url)))
}
