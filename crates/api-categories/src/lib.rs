pub mod api;
pub mod routes;
pub mod state;

use std::net::{Ipv6Addr, SocketAddr};

use api::ApiSchemaBuilder;
use axum::{extract::Request, http::header::CONTENT_TYPE};
use futures_util::TryFutureExt;
use infra::{config::Configuration, tracing::opentelemetry::on_http_request, Services};
use routes::router;
use sellershut_core::categories::{
    mutate_categories_server::MutateCategoriesServer,
    query_categories_server::QueryCategoriesServer, CATEGORY_FILE_DESCRIPTOR_SET,
};
use state::ApiState;
use tonic::service::Routes;
use tower::{make::Shared, steer::Steer};
use tower_http::trace::TraceLayer;
use tracing::{info, info_span};

pub async fn run(services: Services, configuration: Configuration) -> anyhow::Result<()> {
    let state = ApiState::initialise(services).await?;
    let schema = ApiSchemaBuilder::build(state.clone());

    let addr = SocketAddr::from((Ipv6Addr::UNSPECIFIED, configuration.port));

    let web = router(schema, configuration.application.env).layer(
        TraceLayer::new_for_http()
            .make_span_with(|request: &Request<_>| {
                info_span!(
                    "http_request",
                    method = ?request.method(),
                    trace_id = tracing::field::Empty,
                )
            })
    );

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(CATEGORY_FILE_DESCRIPTOR_SET)
        .build_v1()?;

    let grpc = Routes::new(reflection_service)
        .add_service(QueryCategoriesServer::new(state.clone()))
        .add_service(MutateCategoriesServer::new(state.clone()));
    let grpc = grpc.into_axum_router().layer(
        TraceLayer::new_for_grpc()
            .make_span_with(|request: &Request<_>| {
                info_span!(
                    "grpc_request",
                    method = ?request.method(),
                    trace_id = tracing::field::Empty
                )
            })
    );

    let service = Steer::new(vec![web, grpc], |req: &Request, _services: &[_]| {
        if req
            .headers()
            .get(CONTENT_TYPE)
            .map(|content_type| content_type.as_bytes())
            .filter(|content_type| content_type.starts_with(b"application/grpc"))
            .is_some()
        {
            1
        } else {
            0
        }
    });

    let listener = tokio::net::TcpListener::bind(addr)
        .map_err(anyhow::Error::new)
        .await?;

    let socket_addr = listener
        .local_addr()
        .expect("should get socket_addr from listener");

    info!(addr = ?socket_addr, "listening");

    axum::serve(listener, Shared::new(service)).await?;

    Ok(())
}
