use activitypub_federation::{
    axum::json::FederationJson, config::Data, protocol::context::WithContext,
    FEDERATION_CONTENT_TYPE,
};
use axum::{extract::Path, http::HeaderMap, response::IntoResponse};

use crate::{
    server::{error::AppError, grpc::get_user_following},
    state::AppHandle,
};

pub async fn http_get_user_following(
    header_map: HeaderMap,
    Path(name): Path<String>,
    data: Data<AppHandle>,
) -> Result<impl IntoResponse, AppError> {
    let accept = header_map.get("accept").map(|v| v.to_str());
    match accept {
        Some(Ok(FEDERATION_CONTENT_TYPE)) => {
            let following = get_user_following(name, &data).await?;

            Ok(FederationJson(WithContext::new_default(following)).into_response())
        }
        _ => todo!(),
    }
}
