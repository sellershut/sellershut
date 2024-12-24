use anyhow::Result;
use svc_infra::tracing::TracingBuilder;

#[tokio::main]
async fn main() -> Result<()> {
    let _tracing = TracingBuilder::new().build();

    sellershut::run().await.unwrap();

    Ok(())
}
