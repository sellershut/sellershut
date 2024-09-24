pub mod entity;
pub mod server;
pub mod state;

use std::net::{Ipv6Addr, SocketAddr};

use anyhow::Result;
use axum::{extract::Request, http::header::CONTENT_TYPE};
use infra::{config::Configuration, Services};
use server::{grpc, web};
use state::AppState;
use tower::{make::Shared, steer::Steer};
use tracing::info;

pub async fn serve(services: Services, config: Configuration) -> Result<()> {
    sqlx::migrate!("./migrations")
        .run(&services.postgres)
        .await?;

    let port = config.port;
    let state = AppState { config, services };

    let web = web::router(state.clone());
    let grpc = grpc::router(state)?;

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

    let addr = SocketAddr::from((Ipv6Addr::UNSPECIFIED, port));

    let listener = tokio::net::TcpListener::bind(addr).await?;

    let socket_addr = listener.local_addr()?;

    info!(addr = ?socket_addr, "listening");

    axum::serve(listener, Shared::new(service)).await?;

    Ok(())
}

// fn on_request<B>(request: &Request<B>, span: &Span) {
//     let parent_context = global::get_text_map_propagator(|propagator| {
//         propagator.extract(&HeaderExtractor(request.headers()))
//     });
//     span.set_parent(parent_context);
//     let trace_id = span.context().span().span_context().trace_id();
//     span.record("trace_id", trace_id.to_string());
// }
