use anyhow::{Context, Result};
use std::{
    io::{self, Write},
    process,
};

// Usage: your_docker.sh run <image> <command> <arg1> <arg2> ...
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();

    let piped_command = &args[3];
    let piped_command_args = &args[4..];
    let output = process::Command::new(piped_command)
        .args(piped_command_args)
        .output()
        .with_context(|| {
            format!(
                "Tried to run '{}' with arguments {:?}",
                piped_command, piped_command_args
            )
        })?;

    if !output.status.success() {
        process::exit(1);
    }

    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;

    Ok(())
}
