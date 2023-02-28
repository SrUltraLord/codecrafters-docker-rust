use anyhow::{Context, Result};
use std::process;

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();

    let piped_command = &args[3];
    let piped_command_args = &args[4..];
    let child_process = process::Command::new(piped_command)
        .stdout(process::Stdio::inherit())
        .stderr(process::Stdio::inherit())
        .args(piped_command_args)
        .output()
        .with_context(|| {
            format!(
                "Tried to run '{}' with arguments {:?}",
                piped_command, piped_command_args
            )
        })?;

    let status_code = child_process.status.code();
    handle_exit_code(status_code);

    Ok(())
}

fn handle_exit_code(status_code: Option<i32>) {
    if let None = status_code {
        return;
    }

    process::exit(status_code.unwrap());
}
