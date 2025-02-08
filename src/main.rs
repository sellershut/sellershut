use anyhow::Result;
use sellershut::state::AppState;
use sellershut_services::tracing::TracingBuilder;

#[tokio::main]
async fn main() -> Result<()> {
    let _tracing = TracingBuilder::new().build(None);

    let (tx, _rx) = tokio::sync::oneshot::channel();
    let state = AppState::new(1610).await?;

    sellershut::run(state, tx).await
}
