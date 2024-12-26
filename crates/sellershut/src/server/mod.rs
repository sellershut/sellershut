pub mod error;

use anyhow::Result;
use axum::{
    debug_handler,
    extract::MatchedPath,
    http::{HeaderMap, HeaderName, Request},
};
use error::AppError;
use serde::Deserialize;
use std::net::ToSocketAddrs;
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};

use activitypub_federation::{
    FEDERATION_CONTENT_TYPE,
    axum::{
        inbox::{ActivityData, receive_activity},
        json::FederationJson,
    },
    config::{Data, FederationConfig, FederationMiddleware},
    fetch::webfinger::{Webfinger, build_webfinger_response, extract_webfinger_name},
    protocol::context::WithContext,
    traits::Object,
};
use axum::{
    Json, Router,
    extract::{Path, Query},
    response::IntoResponse,
    routing::{get, post},
};
use tracing::{Span, info, info_span};

use crate::hut::{
    Hut,
    entities::{HutUser, Person, PersonAcceptedActivities},
};

pub async fn serve(config: &FederationConfig<Hut>) -> Result<()> {
    let hostname = config.domain();
    info!(hostname = hostname, "server starting");

    let config = config.clone();

    let x_request_id = HeaderName::from_static("x-request-id");

    let app = Router::new()
        .route("/:user/inbox", post(http_post_user_inbox))
        .route("/:user", get(http_get_user))
        .route("/.well-known/webfinger", get(webfinger))
        .layer(FederationMiddleware::new(config))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        matched_path,
                    )
                })
                .on_request(|request: &Request<_>, span: &Span| {
                    svc_infra::tracing::opentelemetry::on_http_request(request.headers(), span);
                }),
        )
        .layer(PropagateRequestIdLayer::new(x_request_id.clone()))
        .layer(SetRequestIdLayer::new(
            x_request_id.clone(),
            MakeRequestUuid,
        ));

    let addr = hostname
        .to_socket_addrs()?
        .next()
        .expect("Failed to lookup domain name");

    let listener = tokio::net::TcpListener::bind(addr).await?;

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[debug_handler]
async fn http_get_user(
    header_map: HeaderMap,
    Path(name): Path<String>,
    data: Data<Hut>,
) -> Result<FederationJson<WithContext<Person>>, AppError> {
    let accept = header_map.get("accept").map(|v| v.to_str());
    if let Some(accept) = accept {
        let accept = accept?;
        if Some(accept) == Some(FEDERATION_CONTENT_TYPE) {
            let db_user = data.read_user(&name).await?;
            let json_user = db_user.into_json(&data).await?;
            Ok(FederationJson(WithContext::new_default(json_user)))
        } else {
            // TODO: generate_user_html
            todo!()
        }
    } else {
        todo!()
    }
}

#[debug_handler]
async fn http_post_user_inbox(data: Data<Hut>, activity_data: ActivityData) -> impl IntoResponse {
    receive_activity::<WithContext<PersonAcceptedActivities>, HutUser, Hut>(activity_data, &data)
        .await
}

#[derive(Deserialize)]
struct WebfingerQuery {
    resource: String,
}

#[debug_handler]
async fn webfinger(
    Query(query): Query<WebfingerQuery>,
    data: Data<Hut>,
) -> Result<Json<Webfinger>, AppError> {
    let name = extract_webfinger_name(&query.resource, &data)?;
    let db_user = data.read_user(name).await?;
    Ok(Json(build_webfinger_response(
        query.resource,
        db_user.id.into_inner(),
    )))
}
