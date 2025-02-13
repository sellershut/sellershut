pub mod entities;
pub mod server;
pub mod state;
pub mod utils;

use anyhow::Result;
use serde::Deserialize;
use state::AppState;

#[derive(Deserialize)]
pub struct HutConfig {
    pub hostname: String,
    #[serde(rename = "users-service")]
    pub users_service: String,
    #[serde(rename = "instance-name")]
    pub instance_name: String,
}

pub async fn run(state: AppState, tx: tokio::sync::oneshot::Sender<u16>) -> Result<()> {
    server::serve(state, tx).await
}
