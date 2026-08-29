pub mod get_outbox;
pub mod post_outbox;

use serde::Deserialize;
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;

use crate::server::{activities::OutboxActivity, utilities::create::CreatableObject};

#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(untagged)]
pub enum OutboxSubmission {
    Activity(OutboxActivity),
    Object(CreatableObject),
}

const OUTBOX_TAG: &str = "Outbox";

pub fn router() -> OpenApiRouter {
    let router = OpenApiRouter::new();

    router
        .routes(utoipa_axum::routes!(get_outbox::get))
        .routes(utoipa_axum::routes!(post_outbox::post))
}
