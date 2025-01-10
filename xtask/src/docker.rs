use std::process::Command;

use crate::{DynError, project_root};

pub fn docker_up() -> Result<(), DynError> {
    docker(true)
}

pub fn docker_down() -> Result<(), DynError> {
    docker(false)
}

fn docker(start: bool) -> Result<(), DynError> {
    let mut args = vec!["compose", "-f", "contrib/compose.yaml"];

    match start {
        true => args.extend(["up", "-d"]),
        false => args.extend(["down"]),
    }

    let status = Command::new("docker")
        .current_dir(project_root())
        .args(args)
        .status()?;

    if !status.success() {
        Err("cargo build failed")?;
    }

    Ok(())
}
