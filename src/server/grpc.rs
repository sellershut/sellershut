use activitypub_federation::config::Data;
use sellershut_core::users::QueryUserByNameRequest;
use tonic::IntoRequest;

use crate::{entities::user::HutUser, state::AppHandle};

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
