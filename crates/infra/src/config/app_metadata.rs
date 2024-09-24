use std::sync::Arc;

use serde::Deserialize;

use super::Environment;

#[derive(Clone, Debug, Deserialize)]
pub struct AppMetadata {
    pub name: Arc<str>,
    pub version: Arc<str>,
    pub env: Environment,
}
