use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct CacheConfig {
    pub dsn: std::sync::Arc<str>,
    pub is_cluster: bool,
    pub max_connections: u32,
}
