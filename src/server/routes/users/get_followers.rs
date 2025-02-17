use activitypub_federation::{
    axum::json::FederationJson, config::Data, protocol::context::WithContext,
    FEDERATION_CONTENT_TYPE,
};
use anyhow::anyhow;
use axum::{extract::Path, http::HeaderMap, response::IntoResponse};

use crate::{
    entities::user::Follow,
    server::{error::AppError, grpc::get_user_by_name},
    state::AppHandle,
};

pub async fn http_get_user_followers(
    header_map: HeaderMap,
    Path(name): Path<String>,
    data: Data<AppHandle>,
) -> Result<impl IntoResponse, AppError> {
    let accept = header_map.get("accept").map(|v| v.to_str());
    match accept {
        Some(Ok(FEDERATION_CONTENT_TYPE)) => {
            let user = get_user_by_name(name, &data)
                .await?
                .ok_or_else(|| anyhow!("user does not exist"))?;

            let followers: Follow = user.0.followers.try_into()?;

            Ok(FederationJson(WithContext::new_default(followers)).into_response())
        }
        _ => todo!(),
    }
}
