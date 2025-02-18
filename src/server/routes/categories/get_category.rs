use activitypub_federation::{
    axum::json::FederationJson, config::Data, fetch::object_id::ObjectId,
    protocol::context::WithContext, traits::Object, FEDERATION_CONTENT_TYPE,
};
use axum::{extract::Path, http::HeaderMap, response::IntoResponse};

use crate::{entities::category::HutCategory, server::error::AppError, state::AppHandle};

pub async fn http_get_category(
    header_map: HeaderMap,
    Path(name): Path<String>,
    data: Data<AppHandle>,
) -> Result<impl IntoResponse, AppError> {
    let accept = header_map.get("accept").map(|v| v.to_str().unwrap());
    if accept == Some(FEDERATION_CONTENT_TYPE) {
        let category_id =
            ObjectId::<HutCategory>::parse(&format!("{}/categories/{name}", data.domain()))?;
        let category = category_id.dereference(&data).await?;

        let json_category = category.into_json(&data).await?;
        Ok(FederationJson(WithContext::new_default(json_category)).into_response())
    } else {
        todo!("c")
    }
}
