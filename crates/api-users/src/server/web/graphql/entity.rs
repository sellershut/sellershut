use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(SimpleObject, InputObject, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct User {
    inbox: Url,
    username: String,
}
