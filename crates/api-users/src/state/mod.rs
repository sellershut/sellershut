mod mutations;
mod queries;

use infra::{config::Configuration, Services};
use oauth2::basic::BasicClient;

#[derive(Clone, Debug)]
pub struct AppState {
    pub services: Services,
    pub config: Configuration,
    pub client: BasicClient,
}
