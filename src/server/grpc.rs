use activitypub_federation::{config::Data, kinds::collection::CollectionType};
use sellershut_core::users::{QueryUserByNameRequest, QueryUsersFollowingRequest};
use tonic::IntoRequest;

use crate::{
    entities::user::{Follow, HutUser},
    state::AppHandle,
};

use super::error::AppError;

pub mod interceptor {
    use tonic::{
        service::{interceptor::InterceptedService, Interceptor},
        transport::Channel,
        Status,
    };
    use tracing::trace;

    pub type Intercepted = InterceptedService<Channel, MyInterceptor>;

    #[derive(Clone, Copy)]
    pub struct MyInterceptor;

    impl Interceptor for MyInterceptor {
        fn call(&mut self, mut request: tonic::Request<()>) -> Result<tonic::Request<()>, Status> {
            trace!("intercepting");

            Ok(request)
        }
    }
}

pub async fn get_user_by_name(
    query: impl AsRef<str>,
    data: &Data<AppHandle>,
) -> Result<Option<HutUser>, AppError> {
    let mut client = data.app_data().query_users_client.clone();
    let user = QueryUserByNameRequest {
        username: query.as_ref().to_string(),
        local: Some(true),
    }
    .into_request();

    let user = client.query_user_by_name(user).await?.into_inner();
    let resp = user.user.map(HutUser);
    Ok(resp)
}

pub async fn get_user_following(
    query: impl AsRef<str>,
    data: &Data<AppHandle>,
) -> Result<Follow, AppError> {
    let mut client = data.app_data().query_users_client.clone();
    let user = QueryUsersFollowingRequest {
        id: query.as_ref().to_string(),
    }
    .into_request();

    let followers = client.query_user_following(user).await?.into_inner().users;

    Follow::try_from(followers)
}
