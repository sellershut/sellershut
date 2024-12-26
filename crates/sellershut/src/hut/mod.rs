use entities::HutUser;
use opentelemetry::global;
use sellershut_utils::grpc::MetadataMap;
use std::sync::Arc;
use time::OffsetDateTime;
use tracing::{Instrument, Span, info_span};
use tracing_opentelemetry::OpenTelemetrySpanExt;
use url::Url;
pub mod activities;
pub mod entities;

use sellershut_core::users::{
    CreateUserRequest, QueryUserByIdRequest, QueryUserByNameRequest, User,
    mutate_users_client::MutateUsersClient, query_users_client::QueryUsersClient,
};
use tonic::{
    IntoRequest, Status,
    service::{Interceptor, interceptor::InterceptedService},
    transport::{Channel, Endpoint},
};
use tracing::debug;

use crate::{HutConfig, server::error::AppError};

/// Instance
#[derive(Clone)]
pub struct Hut {
    pub query_users_client: QueryUsersClient<InterceptedService<Channel, MyInterceptor>>,
    pub mutate_users_client: MutateUsersClient<InterceptedService<Channel, MyInterceptor>>,
    pub system_user: HutUser,
    pub domain: Arc<str>,
}

impl Hut {
    pub async fn new(hut_config: &HutConfig) -> Result<Self, AppError> {
        let channel = Endpoint::new(hut_config.users_endpoint.to_string())?
            .connect()
            .await?;

        let hostname = hut_config.hostname.as_str();
        let username = hut_config.instance_name.as_str();

        let id = format!("http://{hostname}/{username}");
        let user = QueryUserByIdRequest { id: id.clone() }.into_request();

        let mut query_users_client =
            QueryUsersClient::with_interceptor(channel.clone(), MyInterceptor);
        let mut mutate_users_client = MutateUsersClient::with_interceptor(channel, MyInterceptor);

        let user = query_users_client
            .query_user_by_id(user)
            .instrument(info_span!("grpc.get.user"))
            .await?
            .into_inner();

        let keypair = activitypub_federation::http_signatures::generate_actor_keypair()?;

        let user = if let Some(user) = user.user {
            debug!("query ok: {user:?}");
            user
        } else {
            debug!("system user does not exist, creating...");
            let request = CreateUserRequest {
                user: User {
                    id,
                    inbox: Url::parse(&format!("http://{}/{}/inbox", hostname, &username))?
                        .to_string(),
                    username: username.to_string(),
                    public_key: keypair.public_key,
                    private_key: Some(keypair.private_key),
                    last_refreshed_at: OffsetDateTime::now_utc().into(),
                    followers: vec![],
                    local: true,
                    ..Default::default()
                },
            }
            .into_request();

            mutate_users_client
                .create_user(request)
                .instrument(info_span!("grpc.create.user"))
                .await?
                .into_inner()
                .user
        };

        Ok(Self {
            mutate_users_client,
            query_users_client,
            system_user: HutUser::try_from(user)?,
            domain: hostname.into(),
        })
    }

    pub async fn read_user(&self, name: &str) -> Result<HutUser, AppError> {
        let user_by_name = QueryUserByNameRequest {
            username: name.to_string(),
        }
        .into_request();
        let mut client = self.query_users_client.clone();
        let resp = client
            .query_local_user_by_name(user_by_name)
            .instrument(info_span!("grpc.get.user"))
            .await?
            .into_inner()
            .user
            .ok_or_else(|| anyhow::anyhow!("user does not exist"))?;

        HutUser::try_from(resp)
    }
}

#[derive(Clone, Copy)]
pub struct MyInterceptor;

impl Interceptor for MyInterceptor {
    fn call(&mut self, mut request: tonic::Request<()>) -> Result<tonic::Request<()>, Status> {
        let cx = Span::current().context();

        global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&cx, &mut MetadataMap(request.metadata_mut()))
        });

        Ok(request)
    }
}
