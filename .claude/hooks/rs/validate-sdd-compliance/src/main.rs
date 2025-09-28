use std::process::ExitCode;

use anyhow::Result;
use hook_support::{read_json, write_json_line};
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;

fn main() -> ExitCode {
    match run() {
        Ok(Some(output)) => {
            if let Err(err) = write_json_line(&output) {
                eprintln!("validate-sdd-compliance hook failed to write output: {err}");
                return ExitCode::from(1);
            }
            ExitCode::SUCCESS
        }
        Ok(None) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("validate-sdd-compliance hook failed: {err}");
            ExitCode::from(1)
        }
    }
}

fn run() -> Result<Option<HookOutput>> {
    let input: HookInput = read_json()?;

    if !matches!(input.tool_name.as_str(), "Write" | "Edit" | "MultiEdit") {
        return Ok(None);
    }

    let file_path = input.tool_input.file_path();
    if file_path.is_empty() {
        return Ok(None);
    }

    let mut issues = Vec::new();

    if file_path.contains("specs/") {
        let slug_re = match Regex::new(r"specs/[0-9]{3}-[a-z-]+/") {
            Ok(re) => re,
            Err(err) => {
                eprintln!("validate-sdd-compliance: failed to compile slug regex: {err}");
                return Ok(None);
            }
        };
        if !slug_re.is_match(&file_path) {
            issues.push("SDD specs must follow naming convention: specs/NNN-slug/".to_string());
        }

        if let Some(content) = input.tool_input.content() {
            if is_task_document(&file_path) {
                if !content.contains("constitution:") {
                    issues.push(format!(
                        "{file_path} must include constitutional version in YAML metadata"
                    ));
                }

                if file_path.ends_with("spec.md") && !content.contains("issue_uri:") {
                    issues.push("spec.md must include issue_uri in metadata".to_string());
                }
            }
        }
    }

    if is_code_path(&file_path) && !is_test_path(&file_path) {
        issues.push(
            "WARNING: Tests must be written BEFORE implementation (RED → GREEN → REFACTOR)"
                .to_string(),
        );
    }

    if file_path.ends_with(".rs") && !is_test_path(&file_path) {
        if let Some(content) = input.tool_input.content() {
            if content.contains("println!") || content.contains("print!") {
                issues.push(
                    "stdout is reserved for protocol/JSONL only; use stderr for logs (eprintln!)"
                        .to_string(),
                );
            }
        }
    }

    if issues.is_empty() {
        return Ok(None);
    }

    let mut reason = String::from("SDD Compliance Check:");
    for issue in &issues {
        reason.push('\n');
        reason.push_str("• ");
        reason.push_str(issue);
    }

    Ok(Some(HookOutput {
        hook_specific_output: HookSpecificOutput {
            hook_event_name: "PreToolUse".to_string(),
            permission_decision: "ask".to_string(),
            permission_decision_reason: reason,
        },
    }))
}

fn is_task_document(path: &str) -> bool {
    path.ends_with("spec.md") || path.ends_with("plan.md") || path.ends_with("tasks.md")
}

fn is_code_path(path: &str) -> bool {
    path.contains("src/") || path.contains("lib/")
}

fn is_test_path(path: &str) -> bool {
    path.contains("test") || path.contains("tests/")
}

#[derive(Deserialize)]
struct HookInput {
    tool_name: String,
    #[serde(default)]
    tool_input: ToolInput,
}

#[derive(Default, Deserialize)]
struct ToolInput {
    file_path: Option<String>,
    content: Option<String>,
    #[serde(rename = "new_string")]
    new_string: Option<String>,
}

impl ToolInput {
    fn file_path(&self) -> String {
        self.file_path.clone().unwrap_or_default()
    }

    fn content(&self) -> Option<&str> {
        self.content
            .as_deref()
            .filter(|s| !s.is_empty())
            .or_else(|| self.new_string.as_deref().filter(|s| !s.is_empty()))
    }
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
    #[serde(rename = "permissionDecision")]
    permission_decision: String,
    #[serde(rename = "permissionDecisionReason")]
    permission_decision_reason: String,
}
