use std::env;

use xtask::{
    DynError,
    docker::{docker_down, docker_up},
    migrations::run_migrations,
    print_help,
};

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
