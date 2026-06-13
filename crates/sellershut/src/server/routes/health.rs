use axum::{extract::State, response::IntoResponse};

use crate::state::AppState;

/// Health
#[utoipa::path(
    method(get, head),
    path = "/api/health",
    responses(
        (
            status = OK, description = "API is live",
            body = Option<str>, content_type = "text/plain",
         )
    ),
    tag = "sellershut"
)]
pub async fn health(State(state): State<AppState>) -> impl IntoResponse {
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

    use crate::server::{self};

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
        let app = server::boostrap::test_app().await;
        check(app.clone(), "GET", StatusCode::OK).await?;
        check(app.clone(), "HEAD", StatusCode::OK).await?;
        Ok(())
    }
}
