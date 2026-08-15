pub mod cli;
pub mod log;
pub mod server;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "kebab-case", default)]
pub struct Configuration {
    pub server: server::Server,
    pub log: log::Log,
}
