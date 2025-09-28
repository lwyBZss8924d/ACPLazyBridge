use std::env;
use std::path::PathBuf;
use std::process::{Command, ExitCode};

use anyhow::{Context, Result};
use hook_support::{output_stderr, output_stdout, run_command};

const ISSUE_JQ: &str = r#"{
        "number": .number,
        "title": .title,
        "url": .url,
        "state": .state,
        "labels": [.labels[].name],
        "body": .body
    }"#;

fn main() -> ExitCode {
    match run() {
        Ok(Some(output)) => {
            println!("{output}");
            ExitCode::SUCCESS
        }
        Ok(None) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("sdd-task-fetch hook failed: {err}");
            ExitCode::from(1)
        }
    }
}

fn run() -> Result<Option<String>> {
    let mut args = env::args().skip(1);
    let Some(issue_arg) = args.next() else {
        println!(r#"{{"error": "No issue number or URL provided"}}"#);
        return Ok(None);
    };

    let project_dir = project_root()?;
    let mut command = Command::new("gh");
    command
        .arg("issue")
        .arg("view")
        .arg(&issue_arg)
        .arg("--json")
        .arg("title,body,number,url,state,labels")
        .arg("--jq")
        .arg(ISSUE_JQ)
        .current_dir(&project_dir);

    let output = run_command(&mut command).with_context(|| "failed to execute gh issue view")?;

    if output.status.success() {
        let mut result = output_stdout(&output);
        if !result.ends_with('\n') {
            result.push_str("\n");
        }
        return Ok(Some(result));
    }

    let err_text = output_stdout(&output);
    if !err_text.is_empty() {
        eprintln!("{err_text}");
    }
    let stderr_text = output_stderr(&output);
    if !stderr_text.is_empty() {
        eprintln!("{stderr_text}");
    }

    println!(r#"{{"error": "Failed to fetch issue details"}}"#);
    Ok(None)
}

fn project_root() -> Result<PathBuf> {
    if let Ok(path) = env::var("CLAUDE_PROJECT_DIR") {
        Ok(PathBuf::from(path))
    } else {
        Ok(env::current_dir()?)
    }
}
