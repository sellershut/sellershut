use std::sync::Arc;

use sellershut_auth::{AuthService, OauthDriver};
use sellershut_core::{RedactedSecret, user::ActorType};
use sellershut_users::{CreateUser, UserDriver};
use sqlx::PgPool;

use crate::{
    config::Configuration,
    server::{self, entities::user::User},
};

#[derive(Clone)]
pub struct State {
    pub auth: Arc<dyn OauthDriver>,
    pub user: Arc<dyn UserDriver>,
    pub system_user: Arc<User>,
    pub port: u16,
}

pub type AppState = Arc<State>;

impl State {
    pub async fn new<U: UserDriver + 'static>(
        config: &Configuration,
        user_driver: U,
        database: PgPool,
    ) -> Result<AppState, anyhow::Error> {
        let system_user = get_system_user(&user_driver, config).await?;
        let user = Arc::new(user_driver);
        let auth = AuthService::new(database, config.server.oauth.0.clone(), Arc::clone(&user))?;

        Ok(Arc::new(Self {
            auth: Arc::new(auth),
            user,
            port: config.server.port.into(),
            system_user: Arc::new(system_user),
        }))
    }
}

pub async fn get_system_user<U>(user: &U, config: &Configuration) -> anyhow::Result<User>
where
    U: UserDriver,
{
    let system_user = if let Some(user) = user.get_user(&config.server.instance_name).await? {
        user
    } else {
        //create system user
        let keypair = activitypub_federation::http_signatures::generate_actor_keypair()?;
        let id = server::utilities::base_url(config.server.port.into(), &config.server.domain)?;
        let inbox = server::utilities::inbox_url(
            config.server.port.into(),
            &config.server.domain,
            &config.server.instance_name,
        )?;
        let data = CreateUser {
            kind: ActorType::Service,
            ap_id: id,
            username: config.server.instance_name.clone(),
            name: None,
            inbox,
            avatar: None,
            public_key: keypair.public_key,
            private_key: Some(RedactedSecret::from(keypair.private_key)),
            is_local: true,
        };
        user.create_user(&data, None).await?
    }
    .into();

    Ok(system_user)
}
