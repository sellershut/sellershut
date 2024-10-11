mod database;

use infra::Services;

#[derive(Clone)]
pub struct ApiState {
    pub state: Services,
}

impl ApiState {
    pub async fn initialise(services: Services) -> anyhow::Result<Self> {
        sqlx::migrate!("./migrations")
            .run(&services.postgres)
            .await?;

        Ok(Self { state: services })
    }
}
