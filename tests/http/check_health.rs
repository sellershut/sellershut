use crate::helpers::TestApp;
use tokio::sync::oneshot;

#[tokio::test]
async fn check_health_endpoint() -> anyhow::Result<()> {
    let (tx, rx) = oneshot::channel();
    let _app = TestApp::new(tx).await;

    let port = rx.await.unwrap();
    let address = format!("http://127.0.0.1:{port}");

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{address}/health"))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());

    Ok(())
}
