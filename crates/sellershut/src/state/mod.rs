pub mod cache;
pub mod database;

use infra::{config::Configuration, Services};

#[derive(Clone, Debug)]
pub struct AppState {
    pub services: Services,
    pub config: Configuration,
}
