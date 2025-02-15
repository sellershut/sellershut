use activitypub_federation::{
    axum::json::FederationJson, config::Data, protocol::context::WithContext, traits::Object,
    FEDERATION_CONTENT_TYPE,
};
use anyhow::Context;
use axum::{extract::Path, http::HeaderMap, response::IntoResponse};

use crate::{
    server::{error::AppError, grpc::get_user_by_name},
    state::AppHandle,
};

pub async fn http_get_user(
    header_map: HeaderMap,
    Path(name): Path<String>,
    data: Data<AppHandle>,
) -> Result<impl IntoResponse, AppError> {
    let accept = header_map.get("accept").map(|v| v.to_str().unwrap());
    if accept == Some(FEDERATION_CONTENT_TYPE) {
        let user = get_user_by_name(name, &data)
            .await?
            .context("user does not exist")?;

        let json_user = user.into_json(&data).await?;
        Ok(FederationJson(WithContext::new_default(json_user)).into_response())
    } else {
        todo!()
    }
}
