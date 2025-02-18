use activitypub_federation::{
    config::Data,
    fetch::{
        object_id::ObjectId,
        webfinger::{build_webfinger_response, extract_webfinger_name, Webfinger},
    },
};
use axum::{debug_handler, extract::Query, Json};
use serde::Deserialize;
use url::Url;

use crate::{entities::user::HutUser, server::error::AppError, state::AppHandle};

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

    let user_id = ObjectId::<HutUser>::parse(&format!("{}/users/{name}", data.domain()))?;
    let db_user = user_id.dereference(&data).await?;

    let url = Url::parse(&db_user.0.ap_id)?;

    Ok(Json(build_webfinger_response(params.resource, url)))
}
