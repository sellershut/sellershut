use std::sync::Arc;

use sellershut_auth::OauthDriver;
use sellershut_users::UserDriver;

use crate::server::entities::user::User;

#[derive(Clone)]
pub struct State {
    pub auth: Arc<dyn OauthDriver>,
    pub user: Arc<dyn UserDriver>,
    pub system_user: Arc<User>,
    pub port: u16,
}

pub type AppState = Arc<State>;
