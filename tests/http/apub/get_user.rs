use activitypub_federation::FEDERATION_CONTENT_TYPE;
use anyhow::Result;
use axum::http::{HeaderMap, HeaderValue};
use reqwest::header::ACCEPT;
use tokio::sync::oneshot;

use crate::helpers::TestApp;

async fn check(headers: HeaderMap, expected_result: bool) -> Result<()> {
    let (tx, rx) = oneshot::channel();
    let app = TestApp::new(tx).await;

    let port = rx.await?;
    let address = format!("http://127.0.0.1:{port}");

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{address}/users/{}", app.instance_name))
        .headers(headers)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status().is_success(), expected_result);

    Ok(())
}

//#[tokio::test]
//async fn get_user_no_header() -> Result<()> {
//    // TODO: unimplemented!()
//    check(HeaderMap::default(), false).await
//}

#[tokio::test]
async fn get_user_headers() -> Result<()> {
    let mut map = HeaderMap::new();
    map.insert(
        ACCEPT,
        HeaderValue::from_str(FEDERATION_CONTENT_TYPE.to_string().as_str()).unwrap(),
    );
    check(map, true).await

    //  check(HeaderMap::default(), false).await
}
