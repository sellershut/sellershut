use std::{
    net::{Ipv6Addr, SocketAddr},
    sync::Arc,
};

use anyhow::Result;
use sellershut_core::users::{
    mutate_users_client::MutateUsersClient, query_users_client::QueryUsersClient,
    CreateUserRequest, QueryUserByIdRequest, User,
};
use tonic::{transport::Endpoint, IntoRequest};
use tracing::{debug, error, info};
use url::Url;

use crate::{
    entities::user::HutUser,
    server::grpc::interceptor::{Intercepted, MyInterceptor},
    utils, HutConfig,
};

pub type AppHandle = Arc<AppState>;

#[derive(Clone)]
pub struct AppState {
    pub addr: SocketAddr,
    pub query_users_client: QueryUsersClient<Intercepted>,
    pub mutate_users_client: MutateUsersClient<Intercepted>,
    pub system_user: HutUser,
    pub domain: Arc<str>,
}

impl AppState {
    pub async fn new(port: u16, hut_config: HutConfig) -> Result<Self> {
        let listen_address = SocketAddr::from((Ipv6Addr::UNSPECIFIED, port));

        debug!(host = %hut_config.users_service, "connecting to users service");

        let users_channel = Endpoint::new(hut_config.users_service.to_string())?
            .connect()
            .await
            .map_err(|e| {
                error!("could not connect to users service");
                e
            })?;

        let mut query_users_client =
            QueryUsersClient::with_interceptor(users_channel.clone(), MyInterceptor);
        let mut mutate_users_client =
            MutateUsersClient::with_interceptor(users_channel, MyInterceptor);
        info!(host = %hut_config.users_service, "connected to users service");

        let (system_user, domain) = Self::check_user(
            hut_config,
            &mut query_users_client,
            &mut mutate_users_client,
        )
        .await?;

        Ok(Self {
            addr: listen_address,
            query_users_client,
            mutate_users_client,
            system_user,
            domain: domain.as_str().into(),
        })
    }

    async fn check_user(
        hut_config: HutConfig,
        query_users_client: &mut QueryUsersClient<Intercepted>,
        mutate_users_client: &mut MutateUsersClient<Intercepted>,
    ) -> Result<(HutUser, String)> {
        let hostname = hut_config.hostname.as_str();
        let username = hut_config.instance_name.as_str();

        let mut id = Url::parse(hostname)?;
        id.set_path(&format!("users/{username}"));

        debug!(id = ?id, "getting user by id");

        let user = QueryUserByIdRequest { id: id.to_string() }.into_request();

        let user = query_users_client
            .query_user_by_id(user)
            .await?
            .into_inner();

        let user = if let Some(user) = user.user {
            info!(id = ?id, "system user exists");
            user
        } else {
            debug!("system user does not exist, creating...");
            let keypair = activitypub_federation::http_signatures::generate_actor_keypair()?;

            let request = CreateUserRequest {
                user: User {
                    ap_id: id.to_string(),
                    inbox: {
                        let mut url = id.clone();
                        url.set_path(&format!("users/{username}/inbox"));
                        url.to_string()
                    },
                    username: username.to_string(),
                    public_key: keypair.public_key,
                    private_key: Some(keypair.private_key),
                    local: true,
                    ..Default::default()
                },
            }
            .into_request();

            mutate_users_client
                .create_user(request)
                .await?
                .into_inner()
                .user
        };

        let domain = utils::get_domain_with_port(&hostname)?;

        Ok((HutUser(user), domain))
    }
}
