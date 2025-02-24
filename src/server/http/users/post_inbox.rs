use activitypub_federation::{
    axum::inbox::{receive_activity, ActivityData},
    config::Data,
    protocol::context::WithContext,
};
use axum::{debug_handler, response::IntoResponse};

use crate::{
    entities::user::{HutUser, PersonAcceptedActivities},
    state::AppHandle,
};

#[debug_handler]
pub async fn http_post_user_inbox(
    data: Data<AppHandle>,
    activity_data: ActivityData,
) -> impl IntoResponse {
    receive_activity::<WithContext<PersonAcceptedActivities>, HutUser, AppHandle>(
        activity_data,
        &data,
    )
    .await
}
