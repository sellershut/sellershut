use activitypub_federation::{
    axum::json::FederationJson, config::Data, fetch::object_id::ObjectId,
    protocol::context::WithContext, FEDERATION_CONTENT_TYPE,
};
use axum::{extract::Path, http::HeaderMap, response::IntoResponse};

use crate::{
    entities::user::{Follow, HutUser},
    server::error::AppError,
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
            let user_id = ObjectId::<HutUser>::parse(&format!("{}/users/{name}", data.domain()))?;
            let user = user_id.dereference(&data).await?;

            let followers: Follow = user.0.followers.try_into()?;

            Ok(FederationJson(WithContext::new_default(followers)).into_response())
        }
        _ => todo!("d"),
    }
}
