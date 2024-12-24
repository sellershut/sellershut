use std::sync::Arc;
pub mod activities;
pub mod system_user;

use sellershut_core::users::{
    CreateUserRequest, QueryUserByIdRequest, QueryUserByNameRequest,
    mutate_users_client::MutateUsersClient, query_users_client::QueryUsersClient,
};
use system_user::HutUser;
use tonic::{
    IntoRequest, Status,
    service::{Interceptor, interceptor::InterceptedService},
    transport::{Channel, Endpoint},
};
use tracing::{debug, trace};

use crate::server::error::AppError;

/// Instance
#[derive(Clone)]
pub struct Hut {
    pub query_users_client: QueryUsersClient<InterceptedService<Channel, MyInterceptor>>,
    pub mutate_users_client: MutateUsersClient<InterceptedService<Channel, MyInterceptor>>,
    pub system_user: HutUser,
    pub domain: Arc<str>,
}

impl Hut {
    pub async fn new() -> Result<Self, AppError> {
        let channel = Endpoint::from_static("http://[::1]:1304").connect().await?;

        let hostname = "localhost:2210";
        let username = "system".to_string();

        let user = QueryUserByIdRequest {
            id: format!("http://{hostname}/{username}"),
        };

        let mut query_users_client =
            QueryUsersClient::with_interceptor(channel.clone(), MyInterceptor);
        let mut mutate_users_client = MutateUsersClient::with_interceptor(channel, MyInterceptor);

        let user = query_users_client
            .query_user_by_id(user.into_request())
            .await;
        let user = if let Err(status) = user {
            trace!("{status:?}");
            debug!("system user does not exist, creating...");
            let request = CreateUserRequest {
                hostname: hostname.to_string(),
                local: true,
                ..Default::default()
            }
            .into_request();
            mutate_users_client
                .create_user(request)
                .await?
                .into_inner()
                .user
        } else {
            debug!("query ok: {user:?}");
            user?.into_inner().user
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
            .query_user_by_name(user_by_name)
            .await?
            .into_inner()
            .user;

        HutUser::try_from(resp)
    }
}

#[derive(Clone, Copy)]
pub struct MyInterceptor;

impl Interceptor for MyInterceptor {
    fn call(&mut self, request: tonic::Request<()>) -> Result<tonic::Request<()>, Status> {
        Ok(request)
    }
}
