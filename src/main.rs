use anyhow::{Context, Ok, Result};
use std::env;
use std::fs;
use std::os::unix;
use std::os::unix::prelude::PermissionsExt;
use std::process::{self, Output};
use tempfile;

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let (child_command, child_args) = (&args[3], &args[4..]);

    let temp_dir = tempfile::tempdir()?;
    copy_command(&child_command, &temp_dir)?;
    init_sandbox(&temp_dir)?;

    let child_process = spawn_child_process(child_command, child_args)?;

    let status_code = child_process.status.code();
    process::exit(status_code.unwrap_or(1));
}

fn init_sandbox(temp_dir: &tempfile::TempDir) -> Result<()> {
    create_dev_null(&temp_dir)?;
    unix::fs::chroot(temp_dir.path())?;
    env::set_current_dir("/")?;

    Ok(())
}

fn create_dev_null(temp_dir: &tempfile::TempDir) -> Result<()> {
    let permissions = fs::Permissions::from_mode(0o555);
    let dev_path = temp_dir.path().join("dev");
    let dev_null_path = temp_dir.path().join("dev/null");

    fs::create_dir(&dev_path)?;
    fs::set_permissions(&dev_path, permissions.clone())?;

    fs::File::create(&dev_null_path)?;
    fs::set_permissions(&dev_null_path, permissions)?;

    Ok(())
}

fn copy_command(command: &String, destination_dir: &tempfile::TempDir) -> Result<()> {
    let command_path_relative = command.trim_start_matches("/");
    let target_command = destination_dir.path().join(command_path_relative);
    let target_path = target_command.parent().unwrap();

    fs::create_dir_all(target_path)?;
    fs::copy(command, target_command)?;

    Ok(())
}

fn spawn_child_process(command: &String, args: &[String]) -> Result<Output> {
    process::Command::new(command)
        .stdout(process::Stdio::inherit())
        .stderr(process::Stdio::inherit())
        .args(args)
        .output()
        .with_context(|| format!("Tried to run '{}' with arguments {:?}", command, args))
}
