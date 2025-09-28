use std::io::{self, Read, Write};
use std::process::{Command, Output};

use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub fn read_to_string() -> Result<String> {
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .context("failed to read hook stdin")?;
    Ok(buffer)
}

pub fn read_json<T: DeserializeOwned>() -> Result<T> {
    let input = read_to_string()?;
    if input.trim().is_empty() {
        anyhow::bail!("empty hook input");
    }
    let value = serde_json::from_str(&input).context("invalid JSON input")?;
    Ok(value)
}

pub fn write_json_line<T: Serialize>(value: &T) -> Result<()> {
    let mut stdout = io::stdout().lock();
    serde_json::to_writer(&mut stdout, value).context("failed to write JSON output")?;
    stdout
        .write_all(b"\n")
        .context("failed to write trailing newline")?;
    stdout.flush().context("failed to flush stdout")?;
    Ok(())
}

pub fn run_command(command: &mut Command) -> Result<Output> {
    let program = format!(
        "{} {:?}",
        command.get_program().to_string_lossy(),
        command.get_args()
    );
    let output = command
        .output()
        .with_context(|| format!("failed to run command: {program}"))?;
    Ok(output)
}

pub fn ensure_success(output: &Output, context: &str) -> Result<()> {
    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    anyhow::bail!("{context}: {stderr}");
}

pub fn output_stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

pub fn output_stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).trim().to_string()
}
