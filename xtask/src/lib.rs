use std::path::{Path, PathBuf};

pub mod docker;
pub mod migrations;

pub type DynError = Box<dyn std::error::Error>;

pub fn print_help() {
    eprintln!(
        "Tasks:

docker-up                           Start docker stack
docker-down                         Stop docker stack
migrate [DATABASE_URL]              Run database migrations
"
    )
}

pub fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}
