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
                eprintln!("user-prompt-submit hook failed to write output: {err}");
                return ExitCode::from(1);
            }
            ExitCode::SUCCESS
        }
        Ok(None) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("user-prompt-submit hook failed: {err}");
            ExitCode::from(1)
        }
    }
}

fn run() -> Result<Option<HookOutput>> {
    let input: HookInput = read_json()?;

    if let Some(reason) = detect_sensitive(&input.prompt) {
        return Ok(Some(HookOutput::Blocked(DecisionBlock {
            decision: Decision::Block,
            reason,
        })));
    }

    // No additional context injection here. Dedicated time injection is
    // handled by the separate `inject-datetime` hook.
    Ok(None)
}

#[derive(Deserialize)]
struct HookInput {
    #[serde(default)]
    prompt: String,
}

fn detect_sensitive(prompt: &str) -> Option<String> {
    if prompt.is_empty() {
        return None;
    }

    let pattern = r"(?i)\b(password|secret|key|token)\s*[:=]";
    let re = match Regex::new(pattern) {
        Ok(re) => re,
        Err(err) => {
            eprintln!("user-prompt-submit: failed to compile sensitive pattern regex: {err}");
            return None;
        }
    };

    if re.is_match(prompt) {
        return Some(
            "Security policy violation: Prompt contains potential secrets. Please rephrase your request without sensitive information.".to_string(),
        );
    }

    None
}

#[derive(Serialize)]
#[serde(untagged)]
enum HookOutput {
    Blocked(DecisionBlock),
}

#[derive(Serialize)]
struct DecisionBlock {
    decision: Decision,
    reason: String,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
enum Decision {
    Block,
}

// No context output for this hook; context injection is handled by
// `inject-datetime`.

#[cfg(test)]
mod tests {
    use super::detect_sensitive;

    #[test]
    fn detects_password_pattern() {
        assert!(detect_sensitive("password: hunter2").is_some());
    }

    #[test]
    fn ignores_normal_prompt() {
        assert!(detect_sensitive("Summarize today's tasks").is_none());
    }
}
