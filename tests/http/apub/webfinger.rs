use anyhow::Result;
use tokio::sync::oneshot;

use crate::helpers::TestApp;

async fn check(resource: &[(&str, &str)], expected_result: bool) -> Result<()> {
    let (tx, rx) = oneshot::channel();
    let _app = TestApp::new(tx).await;

    let port = rx.await.unwrap();
    let address = format!("http://127.0.0.1:{port}");

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{address}/.well-known/webfinger"))
        .query(resource)
        .send()
        .await
        .expect("Failed to execute request.");

    let actual_result = response.status().is_success();

    assert_eq!(actual_result, expected_result);

    Ok(())
}

#[tokio::test]
async fn webfinger_err() -> Result<()> {
    check(&[("res", "hello")], false).await
}

#[tokio::test]
async fn variable_query_params() -> Result<()> {
    check(&[("resource", "")], true).await?;
    check(&[("resource", "resource")], true).await?;
    check(&[("res", "")], false).await
}
