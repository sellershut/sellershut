use activitypub_federation::config::Data;
use activitypub_federation::traits::ActivityHandler;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::hut::activities::{accept::Accept, create_listing::CreateListing, follow::Follow};

/// List of all activities which this actor can receive.
#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
#[enum_delegate::implement(ActivityHandler)]
pub enum PersonAcceptedActivities {
    Follow(Follow),
    Accept(Accept),
    CreateListing(CreateListing),
}
