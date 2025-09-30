use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use anyhow::{Context, Result};
use hook_support::{read_json, run_command};
use regex::Regex;
use serde::Deserialize;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("markdown-formatter hook failed: {err}");
            ExitCode::from(1)
        }
    }
}

fn run() -> Result<()> {
    let input: HookInput = read_json()?;
    let Some(file_path_str) = input.tool_input.file_path() else {
        return Ok(());
    };

    if !file_path_str.ends_with(".md") && !file_path_str.ends_with(".mdx") {
        return Ok(());
    }

    let project_dir = project_root()?;
    let file_path = PathBuf::from(file_path_str);
    let file_abs = if file_path.is_absolute() {
        file_path
    } else {
        project_dir.join(&file_path)
    };

    if !file_abs.exists() {
        return Ok(());
    }

    run_markdownlint(&project_dir, &file_abs)?;

    let content = fs::read_to_string(&file_abs)
        .with_context(|| format!("failed to read markdown file: {}", file_abs.display()))?;
    let formatted = format_markdown(&content);

    let mut changed = false;
    if formatted != content {
        fs::write(&file_abs, formatted).with_context(|| {
            format!("failed to write formatted markdown: {}", file_abs.display())
        })?;
        changed = true;
    }

    run_markdownlint(&project_dir, &file_abs)?;

    if changed {
        println!("✓ Fixed markdown formatting in {}", file_abs.display());
    }

    Ok(())
}

fn format_markdown(content: &str) -> String {
    let mut result = String::new();
    let mut in_code_block = false;
    let lines: Vec<&str> = content.lines().collect();
    let had_trailing_newline = content.ends_with('\n');
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];
        if !in_code_block {
            if let Some((indent, info)) = parse_fence(line) {
                let mut info_owned = info.to_string();
                if info_owned.trim().is_empty() {
                    let block_content = gather_block(&lines, i + 1);
                    let lang = detect_language(&block_content);
                    info_owned = lang.to_string();
                }
                result.push_str(indent);
                result.push_str("```");
                result.push_str(&info_owned);
                result.push('\n');
                in_code_block = true;
                i += 1;
                continue;
            }

            // Removed: Multiple blank line removal logic
            // Let markdownlint-cli2 handle blank line formatting according to MD012 config
        } else {
            if is_fence(line) {
                in_code_block = false;
            }
        }

        result.push_str(line);
        result.push('\n');
        i += 1;
    }

    if !had_trailing_newline {
        result.pop();
    }

    if !result.ends_with('\n') {
        result.push('\n');
    }

    result
}

fn parse_fence(line: &str) -> Option<(&str, &str)> {
    let indent_len = line.chars().take_while(|c| matches!(c, ' ' | '\t')).count();
    let (indent, rest) = line.split_at(indent_len);
    if rest.starts_with("```") {
        let info = &rest[3..];
        Some((indent, info.trim_end()))
    } else {
        None
    }
}

fn is_fence(line: &str) -> bool {
    let indent_len = line.chars().take_while(|c| matches!(c, ' ' | '\t')).count();
    let (_, rest) = line.split_at(indent_len);
    rest.starts_with("```")
}

fn gather_block(lines: &[&str], start: usize) -> String {
    let mut block = String::new();
    let mut idx = start;
    while idx < lines.len() {
        if is_fence(lines[idx]) {
            break;
        }
        block.push_str(lines[idx]);
        block.push('\n');
        idx += 1;
    }
    block
}

fn detect_language(code: &str) -> &'static str {
    let trimmed = code.trim();
    if trimmed.is_empty() {
        return "text";
    }

    if trimmed.starts_with('{') || trimmed.starts_with('[') {
        if serde_json::from_str::<serde_json::Value>(trimmed).is_ok() {
            return "json";
        }
    }

    if regex_is_match(r"(?m)^\s*def\s+\w+\s*\(", trimmed)
        || regex_is_match(r"(?m)^\s*(import|from)\s+\w", trimmed)
    {
        return "python";
    }

    if regex_is_match(r"\b(function\s+\w+\s*\(|const\s+\w+\s*=)", trimmed)
        || trimmed.contains("=>")
        || trimmed.contains("console.log")
    {
        return "javascript";
    }

    if regex_is_match(
        r"(?m)^\s*(pub\s+)?(fn|struct|enum|impl)\b|\blet\s+mut\s+\w+",
        trimmed,
    ) {
        return "rust";
    }

    if regex_is_match(r"(?m)\b(if|then|fi|for|in|do|done)\b", trimmed) {
        return "bash";
    }

    if regex_is_match(r"(?i)\b(SELECT|INSERT|UPDATE|DELETE|CREATE)\s+", trimmed) {
        return "sql";
    }

    "text"
}

fn regex_is_match(pattern: &str, text: &str) -> bool {
    match Regex::new(pattern) {
        Ok(re) => re.is_match(text),
        Err(err) => {
            eprintln!("markdown-formatter: invalid regex pattern {pattern}: {err}");
            false
        }
    }
}

fn project_root() -> Result<PathBuf> {
    if let Ok(path) = env::var("CLAUDE_PROJECT_DIR") {
        Ok(PathBuf::from(path))
    } else {
        Ok(env::current_dir()?)
    }
}

fn run_markdownlint(project_dir: &Path, file: &Path) -> Result<()> {
    let mut command = Command::new("markdownlint-cli2");
    command.current_dir(project_dir);
    command.arg("--fix");

    let config_yaml = project_dir.join(".markdownlint-cli2.yaml");
    if config_yaml.exists() {
        command.arg("--config");
        command.arg(&config_yaml);
    }

    let config_json = project_dir.join(".markdownlint.json");
    if config_json.exists() {
        command.env("MARKDOWNLINT_CONFIG", &config_json);
    }

    command.arg(file);

    match run_command(&mut command) {
        Ok(output) => {
            if output.status.success() {
                Ok(())
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                anyhow::bail!(
                    "markdownlint-cli2 reported errors for {}: {}",
                    file.display(),
                    stderr.trim()
                )
            }
        }
        Err(err) => {
            if err
                .downcast_ref::<std::io::Error>()
                .map(|io_err| io_err.kind() == ErrorKind::NotFound)
                .unwrap_or(false)
            {
                eprintln!(
                    "markdownlint-cli2 not found; skipping lint step for {}",
                    file.display()
                );
                Ok(())
            } else {
                Err(err)
            }
        }
    }
}

#[derive(Deserialize)]
struct HookInput {
    #[serde(default)]
    tool_input: ToolInput,
}

#[derive(Default, Deserialize)]
struct ToolInput {
    #[serde(default)]
    file_path: Option<String>,
}

impl ToolInput {
    fn file_path(&self) -> Option<&str> {
        self.file_path.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::detect_language;

    #[test]
    fn detects_json() {
        assert_eq!(detect_language("{\"a\":1}"), "json");
    }

    #[test]
    fn detects_python() {
        assert_eq!(detect_language("def foo():\n    pass"), "python");
    }

    #[test]
    fn detects_javascript() {
        assert_eq!(detect_language("const foo = () => {}"), "javascript");
    }

    #[test]
    fn detects_rust() {
        assert_eq!(
            detect_language("pub fn main() {\n    let mut x = 1;\n}"),
            "rust"
        );
    }

    #[test]
    fn detects_bash() {
        assert_eq!(
            detect_language("if [ -f file ]; then\n  echo hi\nfi"),
            "bash"
        );
    }

    #[test]
    fn detects_sql() {
        assert_eq!(detect_language("SELECT * FROM table"), "sql");
    }

    #[test]
    fn defaults_to_text() {
        assert_eq!(detect_language("plain text"), "text");
    }
}
