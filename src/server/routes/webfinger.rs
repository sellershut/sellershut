use axum::{debug_handler, extract::Query, response::IntoResponse};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct WebFingerParams {
    resource: String,
}

#[debug_handler]
pub async fn web_finger(Query(params): Query<WebFingerParams>) -> impl IntoResponse {
    "hello"
}
