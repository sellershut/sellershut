use anyhow::Result;
use sellershut_services::tracing::TracingBuilder;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    let _tracing = TracingBuilder::new().build(None);
    info!("Hello, world!");

    Ok(())
}
