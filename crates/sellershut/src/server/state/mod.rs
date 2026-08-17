use std::sync::Arc;

use sellershut_auth::OauthDriver;

#[derive(Clone)]
pub struct AppState {
    pub auth: Arc<dyn OauthDriver>,
    pub cookie_secure: bool,
}
