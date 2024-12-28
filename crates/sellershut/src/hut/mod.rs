use entities::HutUser;
use opentelemetry::global;
use opentelemetry_semantic_conventions::trace;
use sellershut_utils::grpc::MetadataMap;
use std::sync::Arc;
use time::OffsetDateTime;
use tracing::{Instrument, Span, info};
use tracing_opentelemetry::OpenTelemetrySpanExt;
use url::Url;
pub mod activities;
pub mod entities;

use sellershut_core::{
    categories::{
        mutate_categories_client::MutateCategoriesClient,
        query_categories_client::QueryCategoriesClient,
    },
    listings::{
        mutate_listings_client::MutateListingsClient, query_listings_client::QueryListingsClient,
    },
    users::{
        CreateUserRequest, QueryUserByIdRequest, QueryUserByNameRequest, User,
        mutate_users_client::MutateUsersClient, query_users_client::QueryUsersClient,
    },
};
use tonic::{
    IntoRequest, Status,
    service::{Interceptor, interceptor::InterceptedService},
    transport::{Channel, Endpoint},
};
use tracing::debug;

use crate::{HutConfig, get_domain_with_port, server::error::AppError};

/// Instance
#[derive(Clone)]
pub struct Hut {
    pub query_users_client: QueryUsersClient<InterceptedService<Channel, MyInterceptor>>,
    pub mutate_users_client: MutateUsersClient<InterceptedService<Channel, MyInterceptor>>,
    pub query_categories_client: QueryCategoriesClient<InterceptedService<Channel, MyInterceptor>>,
    pub mutate_categories_client:
        MutateCategoriesClient<InterceptedService<Channel, MyInterceptor>>,
    pub query_listings_client: QueryListingsClient<InterceptedService<Channel, MyInterceptor>>,
    pub mutate_listings_client: MutateListingsClient<InterceptedService<Channel, MyInterceptor>>,
    pub system_user: HutUser,
    pub domain: Arc<str>,
}

impl Hut {
    pub async fn new(hut_config: &HutConfig) -> Result<Self, AppError> {
        let users_channel = Endpoint::new(hut_config.users_endpoint.to_string())?
            .connect()
            .await?;

        let categories_channel = Endpoint::new(hut_config.categories_endpoint.to_string())?
            .connect()
            .await?;

        let listings_channel = Endpoint::new(hut_config.listings_endpoint.to_string())?
            .connect()
            .await?;

        let hostname = hut_config.hostname.as_str();
        let username = hut_config.instance_name.as_str();

        let mut id = Url::parse(hostname)?;
        id.set_path(&format!("users/{username}"));

        let user = QueryUserByIdRequest { id: id.to_string() }.into_request();

        let mut query_users_client =
            QueryUsersClient::with_interceptor(users_channel.clone(), MyInterceptor);
        let mut mutate_users_client =
            MutateUsersClient::with_interceptor(users_channel, MyInterceptor);

        let query_listings_client =
            QueryListingsClient::with_interceptor(listings_channel.clone(), MyInterceptor);
        let mutate_listings_client =
            MutateListingsClient::with_interceptor(listings_channel, MyInterceptor);

        let query_categories_client =
            QueryCategoriesClient::with_interceptor(categories_channel.clone(), MyInterceptor);
        let mutate_categories_client =
            MutateCategoriesClient::with_interceptor(categories_channel, MyInterceptor);

        debug!(id = ?id, "getting user by id");

        let user = query_users_client
            .query_user_by_id(user)
            .instrument({
                let span = tracing::info_span!("grpc.call");
                span.set_attribute(trace::RPC_SERVICE, "QueryUsersClient");
                span.set_attribute(trace::RPC_METHOD, "QueryUserById");
                span
            })
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
                    id: sellershut_utils::id::generate_id(),
                    ap_id: id.to_string(),
                    inbox: {
                        let mut url = id.clone();
                        url.set_path(&format!("users/{username}/inbox"));
                        url.to_string()
                    },
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
                .instrument({
                    let span = tracing::info_span!("grpc.call");
                    span.set_attribute(trace::RPC_SERVICE, "MutateUsersClient");
                    span.set_attribute(trace::RPC_METHOD, "CreateUser");
                    span
                })
                .await?
                .into_inner()
                .user
        };

        let domain = get_domain_with_port(&hostname)?;

        Ok(Self {
            mutate_users_client,
            query_users_client,
            query_categories_client,
            mutate_categories_client,
            query_listings_client,
            mutate_listings_client,
            system_user: HutUser(user),
            domain: domain.as_str().into(),
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
            .instrument({
                let span = tracing::info_span!("grpc.call");
                span.set_attribute(trace::RPC_SERVICE, "QueryUsersClient");
                span.set_attribute(trace::RPC_METHOD, "QueryLocalUserByName");
                span
            })
            .await?
            .into_inner()
            .user
            .ok_or_else(|| anyhow::anyhow!("user does not exist"))?;

        Ok(HutUser(resp))
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
