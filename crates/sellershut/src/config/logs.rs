use std::{env::temp_dir, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct LogConfig {
    pub log_directory: PathBuf,
    pub log_level: String,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            log_directory: temp_dir(),
            log_level: format!(
                "{}=debug,tower_http=debug,axum::rejection=trace",
                env!("CARGO_CRATE_NAME")
            ),
        }
    }
}
