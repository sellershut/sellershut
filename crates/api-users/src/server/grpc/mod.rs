use sellershut_core::users::{
    mutate_users_server::MutateUsersServer, query_users_server::QueryUsersServer, USERS_DESCRIPTOR,
};
use tonic::service::Routes;

use crate::state::AppState;

pub fn router(state: AppState) -> anyhow::Result<axum::Router> {
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(USERS_DESCRIPTOR)
        .build_v1()?;

    let grpc = Routes::new(reflection_service)
        .add_service(QueryUsersServer::new(state.clone()))
        .add_service(MutateUsersServer::new(state.clone()))
        .into_axum_router();

    Ok(grpc)
}
