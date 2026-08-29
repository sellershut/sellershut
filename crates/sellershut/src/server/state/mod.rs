use std::sync::Arc;

use sellershut_auth::{AuthService, OauthDriver};
use sellershut_categories::CategoryDriver;
use sellershut_core::RedactedSecret;
use sellershut_users::{CreateUser, UserDriver};
use sqlx::PgPool;
use url::Url;

use crate::{
    config::Configuration,
    server::{entities::user::User, utilities::ActivityPubIds},
};

#[derive(Clone)]
pub struct State {
    pub auth: Arc<dyn OauthDriver>,
    pub user: Arc<dyn UserDriver>,
    pub category: Arc<dyn CategoryDriver>,
    pub system_user: Arc<User>,
    pub port: u16,
}

pub type AppState = Arc<State>;

impl State {
    pub async fn new<U: UserDriver + 'static, C: CategoryDriver + 'static>(
        config: &Configuration,
        user_driver: U,
        category_driver: C,
        database: PgPool,
    ) -> Result<AppState, anyhow::Error> {
        let system_user = get_system_user(&user_driver, config).await?;
        let user = Arc::new(user_driver);
        let category = Arc::new(category_driver);
        let auth = AuthService::new(database, config.server.oauth.0.clone(), Arc::clone(&user))?;

        Ok(Arc::new(Self {
            auth: Arc::new(auth),
            user,
            category,
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
        let apid = ActivityPubIds::new(config.server.port.into(), &config.server.domain, "")?;

        let port = config.server.port.into();
        let base_url = |port: u16, domain: &str| -> Result<Url, url::ParseError> {
            if cfg!(debug_assertions) {
                Url::parse(&format!("http://localhost:{port}/"))
            } else {
                Url::parse(&format!("https://{domain}/"))
            }
        };

        let id = base_url(port, &config.server.domain)?;
        let inbox = apid.inbox()?;
        let outbox = apid.outbox()?;
        let followers = apid.followers()?;
        let following = apid.following()?;
        let likes = apid.likes()?;

        let data = CreateUser {
            kind: String::from("Service"),
            ap_id: id,
            preferred_username: config.server.instance_name.clone(),
            name: None,
            inbox,
            icon: None,
            public_key: keypair.public_key,
            private_key: Some(RedactedSecret::from(keypair.private_key)),
            is_local: true,
            summary: None,
            outbox,
            followers: Some(followers),
            following: Some(following),
            likes: Some(likes),
        };
        user.create_user(&data, None).await?
    };

    let system_user = User::from_database(system_user, user).await?;

    Ok(system_user)
}
