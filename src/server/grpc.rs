use activitypub_federation::config::Data;
use sellershut_core::users::QueryUsersFollowingRequest;
use tonic::IntoRequest;

use crate::{entities::user::Follow, state::AppHandle};

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

pub async fn get_user_following(
    query: impl AsRef<str>,
    data: &Data<AppHandle>,
) -> Result<Follow, AppError> {
    let mut client = data.app_data().query_users_client.clone();
    let user = QueryUsersFollowingRequest {
        ap_id: query.as_ref().to_string(),
    }
    .into_request();

    let followers = client.query_user_following(user).await?.into_inner().users;

    Follow::try_from(followers)
}
