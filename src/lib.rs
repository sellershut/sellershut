pub mod server;
pub mod state;

use anyhow::Result;
use state::AppState;

pub async fn run(state: AppState, tx: tokio::sync::oneshot::Sender<u16>) -> Result<()> {
    server::serve(state, tx).await
}
