use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tracing_appender::rolling::RollingFileAppender;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "kebab-case", default)]
pub struct Log {
    pub directory: PathBuf,
    pub log_level: String,
    pub rotation: Rotation,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Rotation {
    Weekly,
    Daily,
    Hourly,
    Minutely,
    #[default]
    Never,
}

impl Default for Log {
    fn default() -> Self {
        Self {
            directory: std::env::temp_dir(),
            rotation: Rotation::Never,
            log_level: format!(
                "{}=debug,tower_http=debug,axum::rejection=trace",
                env!("CARGO_CRATE_NAME")
            ),
        }
    }
}

impl From<Rotation> for tracing_appender::rolling::Rotation {
    fn from(value: Rotation) -> Self {
        match value {
            Rotation::Weekly => Self::WEEKLY,
            Rotation::Daily => Self::DAILY,
            Rotation::Hourly => Self::HOURLY,
            Rotation::Minutely => Self::MINUTELY,
            Rotation::Never => Self::NEVER,
        }
    }
}

impl From<&Log> for RollingFileAppender {
    fn from(value: &Log) -> Self {
        RollingFileAppender::new(
            value.rotation.into(),
            &value.directory,
            env!("CARGO_PKG_NAME"),
        )
    }
}
