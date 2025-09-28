use std::env;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::Result;
use hook_support::{output_stderr, output_stdout, run_command, write_json_line};
use serde::Deserialize;
use serde::Serialize;

fn main() -> ExitCode {
    match run() {
        Ok(Some(output)) => {
            if let Err(err) = write_json_line(&output) {
                eprintln!("post-sdd-check hook failed to write output: {err}");
                return ExitCode::from(1);
            }
            ExitCode::SUCCESS
        }
        Ok(None) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("post-sdd-check hook failed: {err}");
            ExitCode::from(1)
        }
    }
}

fn run() -> Result<Option<HookOutput>> {
    let input: HookInput = hook_support::read_json()?;

    if !matches!(input.tool_name.as_str(), "Write" | "Edit" | "MultiEdit") {
        return Ok(None);
    }

    let Some(file_path) = input.file_path() else {
        return Ok(None);
    };

    if !file_path.contains("specs/") {
        return Ok(None);
    }

    let project_dir = project_root()?;
    let mut results = Vec::new();

    if file_path.ends_with("spec.md") {
        if let Some(output) = run_script(&project_dir, "scripts/sdd/validate-sdd-docs.sh")? {
            if !output.status.success() {
                let detail = combine_output(&output);
                let message = if detail.is_empty() {
                    "SDD validation: Failed".to_string()
                } else {
                    format!("SDD validation: Failed - {detail}")
                };
                results.push(message);
            }
        }
    } else if file_path.ends_with("plan.md") {
        if let Some(output) = run_script(&project_dir, "scripts/sdd/validate-sdd-docs.sh")? {
            if !output.status.success() {
                results.push("SDD validation: Failed".to_string());
            }
        }

        if let Some(output) = run_script(&project_dir, "scripts/sdd/run_semantic_checks.sh")? {
            if !output.status.success() {
                results.push("Semantic checks: Failed".to_string());
            }
        }
    } else if file_path.ends_with("tasks.md") {
        if let Some(output) = run_script(&project_dir, "scripts/sdd/check-task-prerequisites.sh")? {
            if !output.status.success() {
                results.push("Task prerequisites: Missing".to_string());
            }
        }
    }

    if results.is_empty() {
        return Ok(None);
    }

    let mut context = String::from("SDD Validation Results:");
    for result in results {
        context.push_str("\n");
        context.push_str(&result);
    }

    Ok(Some(HookOutput {
        hook_specific_output: HookSpecificOutput {
            hook_event_name: "PostToolUse".to_string(),
            additional_context: context,
        },
    }))
}

fn project_root() -> Result<PathBuf> {
    if let Ok(path) = env::var("CLAUDE_PROJECT_DIR") {
        Ok(PathBuf::from(path))
    } else {
        Ok(env::current_dir()?)
    }
}

fn run_script(project_dir: &Path, script: &str) -> Result<Option<std::process::Output>> {
    let path = project_dir.join(script);
    if !path.exists() {
        return Ok(None);
    }

    let mut command = std::process::Command::new(&path);
    command.current_dir(project_dir);
    let output = run_command(&mut command)?;
    Ok(Some(output))
}

fn combine_output(output: &std::process::Output) -> String {
    let mut parts = Vec::new();
    let stdout = output_stdout(output);
    if !stdout.is_empty() {
        parts.push(stdout);
    }
    let stderr = output_stderr(output);
    if !stderr.is_empty() {
        parts.push(stderr);
    }
    parts.join(" | ")
}

#[derive(Deserialize)]
struct HookInput {
    tool_name: String,
    #[serde(default)]
    tool_response: Option<ToolResponse>,
}

impl HookInput {
    fn file_path(&self) -> Option<&str> {
        self.tool_response
            .as_ref()
            .and_then(|response| response.file_path.as_deref())
            .filter(|path| !path.is_empty())
    }
}

#[derive(Deserialize)]
struct ToolResponse {
    #[serde(alias = "filePath", alias = "file_path")]
    file_path: Option<String>,
}

#[derive(Serialize)]
struct HookOutput {
    #[serde(rename = "hookSpecificOutput")]
    hook_specific_output: HookSpecificOutput,
}

#[derive(Serialize)]
struct HookSpecificOutput {
    #[serde(rename = "hookEventName")]
    hook_event_name: String,
    #[serde(rename = "additionalContext")]
    additional_context: String,
}
