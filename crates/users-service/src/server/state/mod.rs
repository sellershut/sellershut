use svc_infra::{Configuration, Services};

#[derive(Clone)]
pub struct ServiceState {
    pub database: sqlx::PgPool,
    pub config: Configuration,
}

impl ServiceState {
    pub fn new(services: Services, config: Configuration) -> Self {
        Self {
            database: services.postgres,
            config,
        }
    }
}
