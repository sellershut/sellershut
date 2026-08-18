use std::sync::Arc;

use sellershut_auth::OauthDriver;
use sellershut_users::UserDriver;

#[derive(Clone)]
pub struct AppState {
    pub auth: Arc<dyn OauthDriver>,
    pub user: Arc<dyn UserDriver>,
}
