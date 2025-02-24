use activitypub_federation::{
    axum::json::FederationJson, config::Data, fetch::object_id::ObjectId,
    protocol::context::WithContext, traits::Object, FEDERATION_CONTENT_TYPE,
};
use axum::{extract::Path, http::HeaderMap, response::IntoResponse};

use crate::{entities::listing::HutListing, server::error::AppError, state::AppHandle};

pub async fn http_get_listing(
    header_map: HeaderMap,
    Path(name): Path<String>,
    data: Data<AppHandle>,
) -> Result<impl IntoResponse, AppError> {
    let accept = header_map.get("accept").map(|v| v.to_str().unwrap());
    if accept == Some(FEDERATION_CONTENT_TYPE) {
        let listing_id =
            ObjectId::<HutListing>::parse(&format!("{}/listings/{name}", data.domain()))?;
        let listing = listing_id.dereference(&data).await?;

        let json_listing = listing.into_json(&data).await?;
        Ok(FederationJson(WithContext::new_default(json_listing)).into_response())
    } else {
        todo!("b")
    }
}
