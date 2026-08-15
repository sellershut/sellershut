use axum::response::IntoResponse;

/// Health
#[utoipa::path(
    method(get, head),
    path = "/api/health",
    responses(
        (
            status = OK, description = "API is live",
            body = Option<str>, content_type = "text/plain",
        ),
        (
            status = REQUEST_TIMEOUT, description = "Request timed out"
        )
    ),
    tag = "sellershut"
)]
pub async fn health() -> impl IntoResponse {
    format!(
        "{} v{} is live",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    )
}

#[cfg(test)]
mod tests {
    use axum::{
        Router,
        body::Body,
        http::{Request, StatusCode},
    };

    use anyhow::Result;
    use tower::ServiceExt;

    async fn check(app: Router, method: &str, expected_result: StatusCode) -> Result<()> {
        let response = app
            .oneshot(
                Request::builder()
                    .method(method)
                    .uri("/api/health")
                    .body(Body::empty())?,
            )
            .await?;
        let actual_result = response.status();
        assert_eq!(expected_result, actual_result);
        Ok(())
    }

    #[tokio::test]
    async fn health() -> Result<()> {
        let app = crate::test::test_app().await;
        check(app.clone(), "GET", StatusCode::OK).await?;
        check(app.clone(), "HEAD", StatusCode::OK).await?;
        Ok(())
    }
}
