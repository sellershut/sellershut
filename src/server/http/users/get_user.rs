use activitypub_federation::{
    FEDERATION_CONTENT_TYPE, axum::json::FederationJson, config::Data, fetch::object_id::ObjectId,
    protocol::context::WithContext, traits::Object,
};
use axum::{extract::Path, http::HeaderMap, response::IntoResponse};

use crate::{entities::user::HutUser, server::error::AppError, state::AppHandle};

pub async fn http_get_user(
    header_map: HeaderMap,
    Path(name): Path<String>,
    data: Data<AppHandle>,
) -> Result<impl IntoResponse, AppError> {
    let accept = header_map.get("accept").map(|v| v.to_str().unwrap());
    if accept == Some(FEDERATION_CONTENT_TYPE) {
        let user_id = ObjectId::<HutUser>::parse(&format!("{}/users/{name}", data.hostname))?;
        let user = user_id.dereference(&data).await?;

        let json_user = user.into_json(&data).await?;
        Ok(FederationJson(WithContext::new_default(json_user)).into_response())
    } else {
        todo!("b")
    }
}
