use anyhow::Result;
use sellershut_core::users::{
    CreateUserRequest, QueryUserByIdRequest, User, mutate_users_client::MutateUsersClient,
    query_users_client::QueryUsersClient,
};
use tonic::{
    IntoRequest, Status,
    service::{Interceptor, interceptor::InterceptedService},
    transport::{Channel, Endpoint},
};
use tracing::{debug, trace};

/// Instance
pub struct Hut {
    pub query_users_client: QueryUsersClient<InterceptedService<Channel, MyInterceptor>>,
    pub mutate_users_client: MutateUsersClient<InterceptedService<Channel, MyInterceptor>>,
    pub system_user: User,
}

impl Hut {
    pub async fn new() -> Result<Self> {
        let channel = Endpoint::from_static("http://[::1]:1304").connect().await?;

        let hostname = "localhost:2210".to_string();
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
                hostname,
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
            println!("query ok: {user:?}");
            user?.into_inner().user
        };

        Ok(Self {
            mutate_users_client,
            query_users_client,
            system_user: user,
        })
    }
}

pub struct MyInterceptor;

impl Interceptor for MyInterceptor {
    fn call(&mut self, request: tonic::Request<()>) -> Result<tonic::Request<()>, Status> {
        Ok(request)
    }
}
