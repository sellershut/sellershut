mod mutations;
mod queries;

use infra::{config::Configuration, Services};
use oauth2::basic::BasicClient;
use reqwest::Client;

use crate::server::web::routes::auth::session::PostgresSessionStore;

#[derive(Clone, Debug)]
pub struct AppState {
    pub services: Services,
    pub config: Configuration,
    pub github_client: BasicClient,
    pub session_store: PostgresSessionStore,
    pub http_client: Client,
}
