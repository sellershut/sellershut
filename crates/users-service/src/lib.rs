pub mod entity;
pub mod server;

use anyhow::Result;
use opentelemetry::global;
use opentelemetry_semantic_conventions::trace;
use sellershut_core::users::{
    mutate_users_server::MutateUsersServer, query_users_server::QueryUsersServer,
};
use sellershut_utils::grpc::MetadataMap;
use serde::Deserialize;
use server::state::ServiceState;
use svc_infra::{Configuration, Services};
use tonic::{Request, Status, transport::Server};
use tracing::{Span, info, trace};
use tracing_opentelemetry::OpenTelemetrySpanExt;

#[derive(Deserialize)]
pub struct AppConfig {
    #[serde(rename = "otel-endpoint")]
    pub otel_endpoint: String,
}

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
        .trace_fn(|_| {
            let span = tracing::info_span!("users/server");
            span.set_attribute(trace::RPC_SERVICE, env!("CARGO_PKG_NAME"));
            span
        })
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
    trace!("Intercepting request: {:?}", req);

    let parent_cx =
        global::get_text_map_propagator(|prop| prop.extract(&MetadataMap(req.metadata_mut())));

    let span = Span::current();
    span.set_parent(parent_cx);

    Ok(req)
}
