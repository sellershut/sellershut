use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

type DynError = Box<dyn std::error::Error>;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(-1);
    }
}

fn try_main() -> Result<(), DynError> {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("docker-up") => docker_up()?,
        Some("migrate") => run_migrations()?,
        Some("docker-down") => docker_down()?,
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        "Tasks:

docker-up            Start docker stack
docker-down          Stop docker stack
migrate              Run database migrations
"
    )
}

fn run_migrations() -> Result<(), DynError> {
    for proj in ["categories-service", "listings-service", "users-service"].into_iter() {
        let dir = project_root();
        let path = dir.join("crates").join(proj);
        run_command_in_directory(&path)?;
    }

    Ok(())
}

fn docker_up() -> Result<(), DynError> {
    docker(true)
}

fn docker_down() -> Result<(), DynError> {
    docker(false)
}

fn docker(start: bool) -> Result<(), DynError> {
    let mut args = vec!["compose", "-f", "contrib/compose.yaml"];

    match start {
        true => args.extend(["up", "-d"]),
        false => args.extend(["down"])
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

fn run_command_in_directory(dir: &PathBuf) -> Result<(), DynError> {
    env::set_current_dir(dir).map_err(|e| format!("Failed to change directory: {}", e))?;

    let status = Command::new("cargo")
        .args(&["sqlx", "migrate", "run"])
        .status()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("Command failed with status: {}", status).into())
    }
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}
