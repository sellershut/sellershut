use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::server::utilities::create::{CreatableObject, Create};

pub mod categories;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(untagged)]
pub enum OutboxActivity {
    Create(Create<CreatableObject>),
    // Like(Like),
    // Follow(Follow),
    // Delete(Delete),
    // Undo(Undo),
}
