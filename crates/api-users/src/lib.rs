pub mod state;

use anyhow::Result;
use infra::{config::Configuration, Services};
use state::AppState;

pub async fn serve(services: Services, config: Configuration) -> Result<()> {
    sqlx::migrate!("./migrations")
        .run(&services.postgres)
        .await?;

    let state = AppState {
        config,
        services,
    };

    Ok(())
}
