use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Hosts {
    #[cfg(feature = "users-client")]
    pub users: std::sync::Arc<str>,
}
