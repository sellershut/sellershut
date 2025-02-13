use activitypub_federation::FEDERATION_CONTENT_TYPE;
use anyhow::Result;
use axum::http::{HeaderMap, HeaderValue};
use reqwest::header::ACCEPT;
use tokio::sync::oneshot;

use crate::helpers::TestApp;

async fn check(user: &str, headers: HeaderMap, expected_result: &str) -> Result<()> {
    let (tx, rx) = oneshot::channel();
    let _app = TestApp::new(tx).await;

    let port = rx.await?;
    let address = format!("http://127.0.0.1:{port}");

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{address}/users/{user}"))
        .headers(headers)
        .send()
        .await
        .expect("Failed to execute request.");

    let actual_result = String::from_utf8(response.bytes().await?.to_vec())?;
    assert_eq!(&actual_result, expected_result);

    Ok(())
}

#[tokio::test]
async fn get_user_no_header() -> Result<()> {
    check("hello", HeaderMap::default(), "user").await
}

#[tokio::test]
async fn get_user_headers() -> Result<()> {
    let mut map = HeaderMap::new();
    map.insert(
        ACCEPT,
        HeaderValue::from_str(FEDERATION_CONTENT_TYPE.to_string().as_str()).unwrap(),
    );
    check("hello", map, "hello").await?;

    check("hello", HeaderMap::default(), "user").await
}
