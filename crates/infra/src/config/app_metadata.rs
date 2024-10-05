use std::sync::Arc;

use serde::Deserialize;

use super::Environment;

#[derive(Clone, Debug, Deserialize)]
pub struct AppMetadata {
    #[serde(skip)]
    pub name: Arc<str>,
    #[serde(skip)]
    pub version: Arc<str>,
    pub env: Environment,
}
