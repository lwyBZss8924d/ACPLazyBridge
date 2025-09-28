use std::process::ExitCode;

use anyhow::{Context, Result};
use chrono::Utc;
use hook_support::{read_to_string, write_json_line};
use serde::Serialize;

fn main() -> ExitCode {
    match run() {
        Ok(Some(output)) => {
            if let Err(err) = write_json_line(&output) {
                eprintln!("inject-datetime hook failed to write output: {err}");
                return ExitCode::from(1);
            }
            ExitCode::SUCCESS
        }
        Ok(None) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("inject-datetime hook failed: {err}");
            ExitCode::from(1)
        }
    }
}

fn run() -> Result<Option<HookOutput>> {
    let input = read_to_string()?;
    serde_json::from_str::<serde_json::Value>(&input).context("invalid JSON input")?;

    let now = Utc::now();
    let date = now.format("%Y-%m-%d");
    let iso = now.to_rfc3339();
    let day = now.format("%A");

    let context = format!(
        "⚠️ **UTC TIME:** {iso} ({day})

**Reference Mapping:**
• 'today'/'current' → {date}
• 'latest'/'recent' → ≤ {iso}
• past → < {date} | future → > {date}

_Use this timestamp for any temporal references in user's request._"
    );

    Ok(Some(HookOutput {
        hook_specific_output: HookSpecificOutput {
            hook_event_name: "UserPromptSubmit".to_string(),
            additional_context: context,
        },
    }))
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
