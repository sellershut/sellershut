use infra::tracing::Telemetry;
use tracing::info;

fn main() {
    let _tracing = Telemetry::builder().build();
    info!("Hello, world!");
}
