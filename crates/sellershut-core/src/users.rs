use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
}
