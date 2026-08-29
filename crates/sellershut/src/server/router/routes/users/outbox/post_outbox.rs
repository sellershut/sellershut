use activitypub_federation::{
    activity_queue, activity_sending::SendActivityTask, config::Data,
    protocol::context::WithContext, traits::Activity,
};
use axum::{Json, extract::Path, http::StatusCode, response::IntoResponse};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use serde::Serialize;
use tracing::debug;
use url::Url;

use crate::server::{
    activities::{self, categories::scheme::create::CreateCategoryScheme},
    entities::user::{Person, User},
    router::routes::users::outbox::{OUTBOX_TAG, OutboxSubmission},
    state::AppState,
    utilities::{self, ActivityPubIds},
};
/// Publish an activity on behalf of the user
#[utoipa::path(
    post,
    path = "/{username}/outbox",
    security(
        ("bearer_auth" = [])

    ),
    responses(
        (status = 201, description = "Current user", body = Person,
            headers(
                (
                    "x-request-id" = String,
                    description = "Unique identifier for the request"
                )
            )
         ),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 500, description = "Internal server error")
    ),
    params(
            ("username" = String, Path, description = "username", example = "rando69")
    ),
    tag = OUTBOX_TAG,
)]
pub async fn post(
    Path(username): Path<String>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    state: Data<AppState>,
    Json(payload): Json<OutboxSubmission>,
) -> Result<impl IntoResponse, StatusCode> {
    let token = bearer.token();

    let user = state.user.user_from_session(token).await.map_err(|_e| {
        debug!(username = username, "user not found from session");
        StatusCode::UNAUTHORIZED
    })?;

    let preferred_username = user.preferred_username.to_owned();
    if preferred_username.ne(&username) {
        return Err(StatusCode::FORBIDDEN);
    }

    let user = User::from_database(user, &*state.user).await.unwrap();

    let id = ActivityPubIds::new(state.port, state.domain(), &preferred_username).unwrap();

    match payload {
        OutboxSubmission::Activity(outbox_activity) => match outbox_activity {
            activities::OutboxActivity::Create(create) => match create.object {
                utilities::create::CreatableObject::CategoryScheme(category_scheme) => {
                    let scheme = CreateCategoryScheme::new(category_scheme, id.activity().unwrap());
                    send(&user, scheme, vec![], true, &state).await.unwrap();
                }
            },
        },
        OutboxSubmission::Object(_creatable_object) => todo!(),
    }

    Ok(())
}

pub async fn send<A>(
    actor: &User,
    activity: A,
    recipients: Vec<Url>,
    use_queue: bool,
    data: &Data<AppState>,
) -> anyhow::Result<()>
where
    A: Activity + Serialize + std::fmt::Debug + Send + Sync,
    <A as Activity>::Error: From<anyhow::Error> + From<serde_json::Error>,
{
    let activity = WithContext::new_default(activity);
    // Send through queue in some cases and bypass it in others to test both code paths
    if use_queue {
        activity_queue::queue_activity(&activity, actor, recipients, data).await?;
    } else {
        let sends = SendActivityTask::prepare(&activity, actor, recipients, data).await?;
        for send in sends {
            send.sign_and_send(data).await?;
        }
    }
    Ok(())
}
