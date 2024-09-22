use infra::{config::Configuration, Services};
use sellershut_core::users::mutate_users_client::MutateUsersClient;
use tonic::transport::Channel;

#[derive(Clone, Debug)]
pub struct AppState {
    pub services: Services,
    pub config: Configuration,
    pub mutate_users_client: MutateUsersClient<Channel>,
}
