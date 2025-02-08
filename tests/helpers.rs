use sellershut::state::AppState;
use sellershut_services::tracing::TracingBuilder;

use std::sync::Once;

static TRACING: Once = Once::new();

pub struct TestApp {
    state: AppState,
}

impl TestApp {
    pub async fn new(tx: tokio::sync::oneshot::Sender<u16>) -> Self {
        // Set port to 0 so tests can spawn multiple servers on OS assigned ports.

        // Setup tracing. Once.
        TRACING.call_once(|| {
            TracingBuilder::new().build(Some("warn".into()));
        });

        let state = AppState::new(0).await.unwrap();

        tokio::spawn(sellershut::run(state.clone(), tx));

        Self { state }
    }
}
