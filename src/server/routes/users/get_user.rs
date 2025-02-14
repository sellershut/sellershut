use activitypub_federation::{
    axum::json::FederationJson, config::Data, protocol::context::WithContext, traits::Object,
    FEDERATION_CONTENT_TYPE,
};
use axum::{extract::Path, http::HeaderMap, response::IntoResponse};

use crate::{entities::user::HutUser, state::AppHandle};

pub async fn http_get_user(
    header_map: HeaderMap,
    Path(name): Path<String>,
    data: Data<AppHandle>,
) -> impl IntoResponse {
    let accept = header_map.get("accept").map(|v| v.to_str().unwrap());
    if accept == Some(FEDERATION_CONTENT_TYPE) {
        //let db_user = data.read_local_user(&name).await.unwrap();
        //let json_user = db_user.into_json(&data).await.unwrap();
        //FederationJson(WithContext::new_default(json_user)).into_response()
        "hello"
    } else {
        "user"
    }
}
