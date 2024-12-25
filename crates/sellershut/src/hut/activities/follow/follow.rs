use activitypub_federation::{
    activity_queue::queue_activity,
    activity_sending::SendActivityTask,
    config::Data,
    fetch::webfinger::webfinger_resolve_actor,
    protocol::context::WithContext,
    traits::{ActivityHandler, Actor},
};
use serde::Serialize;
use std::fmt::Debug;
use url::Url;

use crate::{
    generate_object_id,
    hut::{Hut, HutUser},
    server::error::AppError,
};

use super::Follow;

impl HutUser {
    pub async fn follow(&self, other: &str, data: &Data<Hut>) -> Result<(), AppError> {
        let other: HutUser = webfinger_resolve_actor(other, data).await.unwrap();

        let id = generate_object_id(data.domain())?;
        let follow = Follow::new(self.id.clone(), other.id.clone(), id.clone());

        self.send(follow, vec![other.shared_inbox_or_inbox()], true, data)
            .await?;

        Ok(())
    }

    pub(crate) async fn send<Activity>(
        &self,
        activity: Activity,
        recipients: Vec<Url>,
        use_queue: bool,
        data: &Data<Hut>,
    ) -> Result<(), AppError>
    where
        Activity: ActivityHandler + Serialize + Debug + Send + Sync,
        <Activity as ActivityHandler>::Error: From<anyhow::Error> + From<serde_json::Error>,
    {
        let activity = WithContext::new_default(activity);
        // Send through queue in some cases and bypass it in others to test both code paths
        if use_queue {
            queue_activity(&activity, self, recipients, data).await?;
        } else {
            let sends = SendActivityTask::prepare(&activity, self, recipients, data).await?;
            for send in sends {
                send.sign_and_send(data).await?;
            }
        }
        Ok(())
    }
}
