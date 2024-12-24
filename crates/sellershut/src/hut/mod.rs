use anyhow::Result;
use sellershut_core::users::query_users_client::QueryUsersClient;
use tonic::{
    Status,
    service::{Interceptor, interceptor::InterceptedService},
    transport::{Channel, Endpoint},
};

/// Instance
pub struct Hut {
    pub query_users_client: QueryUsersClient<InterceptedService<Channel, MyInterceptor>>,
}

impl Hut {
    pub async fn new() -> Result<Self> {
        let channel = Endpoint::from_static("http://[::1]:1304")
            .connect()
            .await?;
        let query_users_client = QueryUsersClient::with_interceptor(channel, MyInterceptor);

        Ok(Self { query_users_client })
    }
}

pub struct MyInterceptor;

impl Interceptor for MyInterceptor {
    fn call(&mut self, request: tonic::Request<()>) -> Result<tonic::Request<()>, Status> {
        Ok(request)
    }
}
