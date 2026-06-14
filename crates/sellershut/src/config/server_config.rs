use std::str::FromStr;

use serde::{Deserialize, Serialize};
use url::Url;

use crate::config::logs::LogConfig;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct ServerConfig {
    pub port: u16,
    pub logging: LogConfig,
    pub vault: VaultConfig,
    pub database: DatabaseConfig,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: 2210,
            logging: Default::default(),
            vault: Default::default(),
            database: Default::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct VaultConfig {
    pub address: Url,
    pub token: String,
    pub mount: String,
}

impl Default for VaultConfig {
    fn default() -> Self {
        Self {
            address: Url::from_str("http://127.0.0.1:8200").expect("valid default vault url"),
            token: Default::default(),
            mount: String::from("secret"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
    pub pool_size: u32,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            host: String::from("127.0.0.1"),
            port: 5432,
            username: String::from("postgres"),
            password: String::from("password"),
            database: String::from("sellershut"),
            pool_size: 10,
        }
    }
}

impl DatabaseConfig {
    pub fn connection_string(&self) -> String {
        let mut url = Url::parse("postgres://localhost").expect("valid postgres url");

        url.set_host(Some(&self.host)).expect("valid postgres host");

        url.set_port(Some(self.port)).expect("valid postgres port");

        url.set_username(&self.username)
            .expect("valid postgres username");

        url.set_password(Some(&self.password))
            .expect("valid postgres password");

        url.set_path(&self.database);

        url.to_string()
    }
}
