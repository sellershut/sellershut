use std::sync::Arc;

use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct PgConfig {
    pool_size: u32,
    port: u16,
    name: Arc<str>,
    host: Arc<str>,
    user: Arc<str>,
    password: SecretString,
}

impl PgConfig {
    // Getter for size
    pub fn pool_size(&self) -> u32 {
        self.pool_size
    }

    // Getter for port
    pub fn port(&self) -> u16 {
        self.port
    }

    // Getter for name
    pub fn name(&self) -> &str {
        &self.name.as_ref()
    }

    // Getter for host
    pub fn host(&self) -> &str {
        &self.host.as_ref()
    }

    // Getter for username
    pub fn username(&self) -> &str {
        &self.user.as_ref()
    }

    // Getter for password (you may want to return a reference or handle it differently)
    pub fn password(&self) -> &SecretString {
        &self.password
    }

    #[cfg(feature = "postgres")]
    pub(crate) fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.name
        )
    }
}
