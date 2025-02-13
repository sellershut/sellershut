use activitypub_federation::FEDERATION_CONTENT_TYPE;
use axum::{extract::Path, http::HeaderMap, response::IntoResponse};

pub async fn http_get_user(header_map: HeaderMap, Path(name): Path<String>) -> impl IntoResponse {
    let accept = header_map.get("accept").map(|v| v.to_str().unwrap());
    if accept == Some(FEDERATION_CONTENT_TYPE) {
        "hello"
    } else {
        "user"
    }
}
