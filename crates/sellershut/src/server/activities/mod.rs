use activitypub_federation::{config::Data, traits::Activity};
use serde::{Deserialize, Serialize};
use url::Url;
use utoipa::ToSchema;

use crate::server::utilities::create::{CreatableObject, Create};

pub mod categories;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(untagged)]
/// List of all activities which this actor can receive.
#[enum_delegate::implement(Activity)]
pub enum OutboxActivity {
    Create(Create<CreatableObject>),
    // Like(Like),
    // Follow(Follow),
    // Delete(Delete),
    // Undo(Undo),
}
