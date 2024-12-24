pub mod entity;
pub mod server;

use anyhow::Result;
use sellershut_core::users::{
    mutate_users_server::MutateUsersServer, query_users_server::QueryUsersServer,
};
use server::state::ServiceState;
use svc_infra::{Configuration, Services};
use tonic::{Request, Status, transport::Server};
use tracing::info;

pub async fn run(services: Services, configuration: Configuration) -> Result<()> {
    sqlx::migrate!("./migrations")
        .run(&services.postgres)
        .await?;

    let app_state = ServiceState::new(services, configuration);
    let addr = format!("[::1]:{}", app_state.config.application.port).parse()?;

    info!(addr = ?addr, "starting server");

    let query_service = QueryUsersServer::with_interceptor(app_state.clone(), intercept);
    let mutate_service = MutateUsersServer::with_interceptor(app_state, intercept);
    Server::builder()
        .add_service(query_service)
        .add_service(mutate_service)
        .serve(addr)
        .await?;

    Ok(())
}

/// This function will get called on each inbound request, if a `Status`
/// is returned, it will cancel the request and return that status to the
/// client.
fn intercept(mut req: Request<()>) -> Result<Request<()>, Status> {
    println!("Intercepting request: {:?}", req);

    // Set an extension that can be retrieved by `say_hello`
    req.extensions_mut().insert(MyExtension {
        some_piece_of_data: "foo".to_string(),
    });

    Ok(req)
}

#[derive(Clone)]
struct MyExtension {
    some_piece_of_data: String,
}
