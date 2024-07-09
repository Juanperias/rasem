use anyhow::{Context, Result};
use std::process::Command;

pub fn exec(command: &str) -> Result<()> {
    let result = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .with_context(|| format!("ERROR RUNNING THE COMMAND: {}", command))?;

    if !result.status.success() {
        let stderr_string = String::from_utf8_lossy(&result.stderr);
        eprintln!("ERROR:\n{}", stderr_string);
        std::process::exit(1); // Termina la ejecución con un código de error
    }

    Ok(())
}
