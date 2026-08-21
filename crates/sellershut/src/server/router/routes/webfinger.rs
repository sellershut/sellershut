use activitypub_federation::{
    config::Data,
    fetch::webfinger::{build_webfinger_response, extract_webfinger_name},
};
use axum::{Json, extract::Query, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::server::{AppError, state::AppState};
/// Webfinger
#[utoipa::path(
    get,
    path = "/.well-known/webfinger",
    params(
        WebFingerQuery
    ),
    responses(
        (
            status = 200,
            description = "JSON Resource Descriptor",
            content_type = "application/jrd+json",
            body = WebFingerResponse,
            headers(
                (
                    "x-request-id" = String,
                    description = "Unique identifier for the request"
                )
            )
        ),
        (
            status = 400,
            description = "Webfinger name could not be extracted",
            headers(
                (
                    "x-request-id" = String,
                    description = "Unique identifier for the request"
                )
            )
        ),
        (
            status = 404,
            description = "No information is available for the requested resource",
            headers(
                (
                    "x-request-id" = String,
                    description = "Unique identifier for the request"
                )
            )
        ),
        (status = 500, description = "Internal server error")
    ),
    tag = env!("CARGO_PKG_NAME")
)]
pub async fn webfinger(
    Query(query): Query<WebFingerQuery>,
    state: Data<AppState>,
) -> Result<impl IntoResponse, AppError> {
    if let Ok(name) = extract_webfinger_name(&query.resource, &state) {
        match state.user.get_user(name).await? {
            Some(u) => {
                let mut resp =
                    Json(build_webfinger_response(query.resource, u.ap_id.inner())).into_response();
                resp.headers_mut().insert(
                    axum::http::header::CONTENT_TYPE,
                    "application/jrd+json".parse().unwrap(),
                );
                Ok(resp)
            }
            None => Ok((StatusCode::NOT_FOUND).into_response()),
        }
    } else {
        Ok((StatusCode::BAD_REQUEST).into_response())
    }
}

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct WebFingerQuery {
    /// URI identifying the resource being queried.
    #[param(example = "acct:seller@some.hut")]
    pub resource: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct WebFingerResponse {
    /// The URI identifying the requested resource.
    #[schema(example = "acct:seller@some.hut")]
    pub subject: String,

    /// Links associated with the requested resource.
    pub links: Vec<WebFingerLink>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct WebFingerLink {
    /// The link relation describing the relationship between the resource
    /// and the linked resource.
    /// Example: `self`
    #[schema(example = "self")]
    pub rel: String,

    /// The media type of the linked resource.
    #[schema(example = "application/activity+json")]
    #[serde(rename = "type")]
    pub kind: String,

    /// The URL of the linked resource.
    #[schema(example = "https://some.hut/@user")]
    pub href: String,

    /// An optional URI template for constructing resource links.
    ///
    #[schema(example = "https://some.hut/users/seller")]
    pub template: Option<String>,
}
