use anyhow::Result;
use svc_infra::tracing::TracingBuilder;

#[tokio::main]
async fn main() -> Result<()> {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");

    let _tracing = TracingBuilder::new()
        .try_with_opentelemetry(
            name,
            version,
            &svc_infra::Environment::Development,
            "http://localhost:4317",
        )?
        .build();

    sellershut::run().await.unwrap();

    Ok(())
}
