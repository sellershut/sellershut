use std::{env, io::Write, path::PathBuf, process::Command};

use crate::{DynError, print_help, project_root};

pub fn run_migrations() -> Result<(), DynError> {
    let db_url = env::args().nth(2);
    match db_url.as_deref() {
        Some(db_url) => {
            for proj in ["categories-service", "listings-service", "users-service"].into_iter() {
                match &proj.split("-").collect::<Vec<_>>().first() {
                    Some(db) => {
                        let db_url = format!("{db_url}/{db}");
                        let dir = project_root();
                        let path = dir.join("crates").join(proj);
                        run_command_in_directory(&path, db_url)?;
                    }
                    None => todo!(),
                }
            }
        }
        None => print_help(),
    }

    Ok(())
}

fn run_command_in_directory(dir: &PathBuf, db_url: String) -> Result<(), DynError> {
    env::set_current_dir(dir).map_err(|e| format!("Failed to change directory: {}", e))?;
    let cwd = dir.join(".env");
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(cwd)?;

    writeln!(file, "DATABASE_URL={db_url}")?;

    let status = Command::new("cargo")
        .args(["sqlx", "migrate", "run"])
        .status()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("Command failed with status: {}", status).into())
    }
}
