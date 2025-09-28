use anyhow::{Context, Result};
use hook_support::{read_to_string, write_json_line};
use serde::Serialize;
use serde_json::Value;
use std::process::ExitCode;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("pre-tool-use-approval hook failed: {err}");
            ExitCode::from(1)
        }
    }
}

fn run() -> Result<()> {
    let buffer = read_to_string()?;

    if buffer.trim().is_empty() {
        return Ok(());
    }

    let payload: Value = serde_json::from_str(&buffer).context("invalid JSON input")?;

    let tool_name = payload
        .get("tool_name")
        .and_then(Value::as_str)
        .unwrap_or("");

    if tool_name != "Read" {
        return Ok(());
    }

    let file_path = payload
        .get("tool_input")
        .and_then(Value::as_object)
        .and_then(|tool_input| tool_input.get("file_path"))
        .and_then(Value::as_str)
        .unwrap_or("");

    if should_auto_approve(file_path) {
        let response = HookResponse {
            decision: "approve",
            reason: "Documentation file auto-approved",
            suppress_output: true,
        };

        write_json_line(&response)?;
    }

    Ok(())
}

#[derive(Serialize)]
struct HookResponse<'a> {
    decision: &'a str,
    reason: &'a str,
    #[serde(rename = "suppressOutput")]
    suppress_output: bool,
}

fn should_auto_approve(path: &str) -> bool {
    const DOC_EXTENSIONS: [&str; 4] = [".md", ".mdx", ".txt", ".json"];
    DOC_EXTENSIONS.iter().any(|ext| path.ends_with(ext))
}

#[cfg(test)]
mod tests {
    use super::should_auto_approve;

    #[test]
    fn approves_supported_extensions() {
        for sample in [
            "/docs/readme.md",
            "notes.mdx",
            "guides/intro.txt",
            "schema.json",
        ] {
            assert!(
                should_auto_approve(sample),
                "{} should auto-approve",
                sample
            );
        }
    }

    #[test]
    fn rejects_other_extensions() {
        for sample in ["main.rs", "archive.zip", "README", "diagram.png"] {
            assert!(
                !should_auto_approve(sample),
                "{} should not auto-approve",
                sample
            );
        }
    }
}
