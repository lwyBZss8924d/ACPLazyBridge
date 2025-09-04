//! Tool call utilities for mapping Codex events to ACP protocol
//!
//! This module provides utilities for:
//! - Mapping tool names to ACP ToolKind categories
//! - Truncating tool output for preview (2KB limit)
//! - Formatting tool calls for ACP protocol compliance

use serde_json::Value;
use tracing::debug;

/// Maximum size for tool output preview in bytes (2KB)
pub const MAX_OUTPUT_PREVIEW_BYTES: usize = 2048;

/// Extracted shell tool parameters matching Codex's ShellToolCallParams
#[derive(Debug, Clone, Default)]
pub struct ExtractedShellParams {
    pub command: Option<String>,
    pub workdir: Option<String>,
    pub timeout_ms: Option<u64>,
    pub with_escalated_permissions: Option<bool>,
    pub justification: Option<String>,
}

/// Map a tool name to an ACP ToolKind category
///
/// This function categorizes tools based on their names to help clients
/// choose appropriate icons and UI treatment.
pub fn map_tool_kind(tool_name: &str) -> String {
    let name_lower = tool_name.to_lowercase();

    // Check for fetch operations first (network tools often have specific prefixes)
    if (name_lower.contains("fetch") && !name_lower.contains("fetch_file"))
        || name_lower.contains("download")
        || name_lower.contains("curl")
        || name_lower.contains("wget")
        || name_lower.contains("http")
    {
        return "fetch".to_string();
    }

    // Check for search operations (before read, as some search tools contain "get")
    if name_lower.contains("search")
        || name_lower.contains("find")
        || name_lower.contains("grep")
        || name_lower.contains("locate")
        || name_lower.contains("query")
    {
        return "search".to_string();
    }

    // Check for read operations
    if name_lower.contains("read")
        || name_lower.contains("get")
        || name_lower.contains("fetch_file")
        || name_lower.contains("view")
        || name_lower.contains("cat")
        || name_lower.contains("list")
    {
        return "read".to_string();
    }

    // Check for edit operations
    if name_lower.contains("write")
        || name_lower.contains("edit")
        || name_lower.contains("update")
        || name_lower.contains("modify")
        || name_lower.contains("patch")
        || name_lower.contains("change")
        || name_lower.contains("set")
    {
        return "edit".to_string();
    }

    // Check for delete operations
    if name_lower.contains("delete")
        || name_lower.contains("remove")
        || name_lower.starts_with("rm")
    {
        return "delete".to_string();
    }

    // Check for move operations
    if name_lower.contains("move") || name_lower.contains("rename") || name_lower.starts_with("mv")
    {
        return "move".to_string();
    }

    // Check for execute operations
    if name_lower.contains("exec")
        || name_lower.contains("run")
        || name_lower.contains("shell")
        || name_lower.contains("cmd")
        || name_lower.contains("command")
        || name_lower.contains("execute")
        || name_lower.contains("local_shell")
        || name_lower.contains("bash")
        || name_lower.contains("python")
    {
        return "execute".to_string();
    }

    // Check for think operations
    if name_lower.contains("think")
        || name_lower.contains("reason")
        || name_lower.contains("plan")
        || name_lower.contains("analyze")
        || name_lower.contains("consider")
    {
        return "think".to_string();
    }

    // Default to other
    "other".to_string()
}

/// Truncate output to a maximum size while preserving beginning and end
///
/// For outputs larger than the limit, keeps the first 75% and last 25%
/// with a truncation marker in the middle.
pub fn truncate_output(output: &str, max_bytes: usize) -> String {
    let output_len = output.len();

    if output_len <= max_bytes {
        return output.to_string();
    }

    // Reserve space for the truncation marker
    let marker = "...[truncated]...";
    let available_bytes = max_bytes.saturating_sub(marker.len());

    // Keep first 75% and last 25% of available space
    let prefix_size = (available_bytes * 3) / 4;
    let suffix_size = available_bytes / 4;

    // Find character boundaries for clean UTF-8 truncation
    let prefix_end = find_char_boundary(output, prefix_size);
    let suffix_start = output_len - find_char_boundary_reverse(output, suffix_size);

    let truncated_bytes = output_len - prefix_end - (output_len - suffix_start);
    let marker_with_info = format!("...[truncated {} bytes]...", truncated_bytes);

    debug!(
        "Truncating output from {} bytes to ~{} bytes (prefix: {}, suffix: {})",
        output_len,
        max_bytes,
        prefix_end,
        output_len - suffix_start
    );

    format!(
        "{}{}{}",
        &output[..prefix_end],
        marker_with_info,
        &output[suffix_start..]
    )
}

/// Find the nearest valid UTF-8 character boundary at or before the target position
fn find_char_boundary(s: &str, target: usize) -> usize {
    let mut pos = target.min(s.len());
    while pos > 0 && !s.is_char_boundary(pos) {
        pos -= 1;
    }
    pos
}

/// Find the nearest valid UTF-8 character boundary from the end
fn find_char_boundary_reverse(s: &str, target: usize) -> usize {
    let mut pos = target.min(s.len());
    while pos > 0 {
        let start = s.len() - pos;
        if s.is_char_boundary(start) {
            break;
        }
        pos -= 1;
    }
    pos
}

/// Extract the command from tool arguments for shell executions
///
/// For local_shell or similar tools, extracts the command to use as title.
/// Supports both string and Vec<String> command formats per Codex ShellToolCallParams.
pub fn extract_shell_command(tool_name: &str, arguments: &Value) -> Option<String> {
    let name_lower = tool_name.to_lowercase();

    if name_lower.contains("shell")
        || name_lower.contains("exec")
        || name_lower.contains("run")
        || name_lower.contains("cmd")
        || name_lower.contains("bash")
    {
        // Try common argument patterns
        // First check for 'command' field (can be string or array)
        if let Some(cmd_value) = arguments.get("command") {
            if let Some(cmd_str) = cmd_value.as_str() {
                return Some(cmd_str.to_string());
            } else if let Some(cmd_array) = cmd_value.as_array() {
                // Handle Vec<String> command format from Codex
                let cmd_parts: Vec<String> = cmd_array
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();
                if !cmd_parts.is_empty() {
                    return Some(cmd_parts.join(" "));
                }
            }
        }

        // Try other common field names (usually strings)
        if let Some(cmd) = arguments.get("cmd").and_then(|v| v.as_str()) {
            return Some(cmd.to_string());
        }
        if let Some(script) = arguments.get("script").and_then(|v| v.as_str()) {
            return Some(script.to_string());
        }
        if let Some(code) = arguments.get("code").and_then(|v| v.as_str()) {
            return Some(code.to_string());
        }
    }

    None
}

/// Extract all shell tool parameters from arguments
///
/// Extracts command, workdir, timeout, and permission fields per Codex ShellToolCallParams
pub fn extract_shell_params(tool_name: &str, arguments: &Value) -> ExtractedShellParams {
    let name_lower = tool_name.to_lowercase();

    // Only extract for shell-like tools
    if !name_lower.contains("shell")
        && !name_lower.contains("exec")
        && !name_lower.contains("run")
        && !name_lower.contains("cmd")
        && !name_lower.contains("bash")
    {
        return ExtractedShellParams::default();
    }

    // Extract command (string or array)
    let command = extract_shell_command(tool_name, arguments);

    // Extract workdir (check multiple field names for compatibility)
    let workdir = arguments
        .get("workdir")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .or_else(|| {
            arguments
                .get("cwd")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        })
        .or_else(|| {
            arguments
                .get("working_directory")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        });

    // Extract timeout_ms (also check "timeout" alias)
    let timeout_ms = arguments
        .get("timeout_ms")
        .and_then(|v| v.as_u64())
        .or_else(|| arguments.get("timeout").and_then(|v| v.as_u64()));

    // Extract escalated permissions flag
    let with_escalated_permissions = arguments
        .get("with_escalated_permissions")
        .and_then(|v| v.as_bool())
        .or_else(|| arguments.get("sudo").and_then(|v| v.as_bool()));

    // Extract justification for escalated permissions
    let justification = arguments
        .get("justification")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .or_else(|| {
            arguments
                .get("reason")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        });

    ExtractedShellParams {
        command,
        workdir,
        timeout_ms,
        with_escalated_permissions,
        justification,
    }
}

/// Format tool output for display in content blocks
///
/// Handles special formatting for different tool types
pub fn format_tool_output(_tool_name: &str, output: &Value, max_preview_bytes: usize) -> String {
    // Handle string output
    if let Some(text) = output.as_str() {
        return truncate_output(text, max_preview_bytes);
    }

    // Handle structured output with common fields
    if let Some(stdout) = output.get("stdout").and_then(|v| v.as_str()) {
        let mut result = truncate_output(stdout, max_preview_bytes);

        // Add stderr if present and there's room
        if let Some(stderr) = output.get("stderr").and_then(|v| v.as_str()) {
            if !stderr.is_empty() {
                let remaining = max_preview_bytes.saturating_sub(result.len());
                if remaining > 100 {
                    result.push_str("\n[stderr]:\n");
                    result.push_str(&truncate_output(stderr, remaining - 12));
                }
            }
        }

        // Add exit code if present
        if let Some(code) = output.get("exit_code").and_then(|v| v.as_i64()) {
            if code != 0 {
                result.push_str(&format!("\n[exit code: {}]", code));
            }
        }

        return result;
    }

    // Handle array output (e.g., list of items)
    if let Some(array) = output.as_array() {
        let items: Vec<String> = array
            .iter()
            .take(10)
            .map(|v| {
                if let Some(s) = v.as_str() {
                    s.to_string()
                } else {
                    format!("{}", v)
                }
            })
            .collect();

        let mut result = items.join("\n");
        if array.len() > 10 {
            result.push_str(&format!("\n... and {} more items", array.len() - 10));
        }

        return truncate_output(&result, max_preview_bytes);
    }

    // Fallback to JSON representation
    let json_str = serde_json::to_string_pretty(output).unwrap_or_else(|_| output.to_string());

    truncate_output(&json_str, max_preview_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_map_tool_kind() {
        // Read operations
        assert_eq!(map_tool_kind("read_file"), "read");
        assert_eq!(map_tool_kind("get_content"), "read");
        assert_eq!(map_tool_kind("fetch_file_data"), "read");
        assert_eq!(map_tool_kind("view_document"), "read");
        assert_eq!(map_tool_kind("cat"), "read");
        assert_eq!(map_tool_kind("list_files"), "read");

        // Edit operations
        assert_eq!(map_tool_kind("write_file"), "edit");
        assert_eq!(map_tool_kind("edit_content"), "edit");
        assert_eq!(map_tool_kind("update_config"), "edit");
        assert_eq!(map_tool_kind("modify_data"), "edit");
        assert_eq!(map_tool_kind("patch_file"), "edit");

        // Delete operations
        assert_eq!(map_tool_kind("delete_file"), "delete");
        assert_eq!(map_tool_kind("remove_item"), "delete");
        assert_eq!(map_tool_kind("rm"), "delete");

        // Move operations
        assert_eq!(map_tool_kind("move_file"), "move");
        assert_eq!(map_tool_kind("rename_document"), "move");
        assert_eq!(map_tool_kind("mv"), "move");

        // Search operations
        assert_eq!(map_tool_kind("search_content"), "search");
        assert_eq!(map_tool_kind("find_files"), "search");
        assert_eq!(map_tool_kind("grep_pattern"), "search");

        // Execute operations
        assert_eq!(map_tool_kind("execute_command"), "execute");
        assert_eq!(map_tool_kind("run_script"), "execute");
        assert_eq!(map_tool_kind("local_shell"), "execute");
        assert_eq!(map_tool_kind("bash_command"), "execute");

        // Think operations
        assert_eq!(map_tool_kind("think_about"), "think");
        assert_eq!(map_tool_kind("reason_through"), "think");
        assert_eq!(map_tool_kind("plan_approach"), "think");

        // Fetch operations
        assert_eq!(map_tool_kind("fetch_url"), "fetch");
        assert_eq!(map_tool_kind("download_resource"), "fetch");
        assert_eq!(map_tool_kind("curl_request"), "fetch");

        // Other operations
        assert_eq!(map_tool_kind("unknown_tool"), "other");
        assert_eq!(map_tool_kind("custom_operation"), "other");
    }

    #[test]
    fn test_truncate_output() {
        // Small output - no truncation
        let small = "Hello, world!";
        assert_eq!(truncate_output(small, 100), small);

        // Large output - truncation needed
        let large = "a".repeat(1000);
        let truncated = truncate_output(&large, 100);
        assert!(truncated.len() <= 120); // Some allowance for truncation marker
        assert!(truncated.contains("[truncated"));
        assert!(truncated.starts_with("aaa"));
        assert!(truncated.ends_with("aaa"));

        // UTF-8 handling
        let utf8 = "Hello 世界 ".repeat(50);
        let truncated_utf8 = truncate_output(&utf8, 100);
        assert!(truncated_utf8.len() <= 120);
        assert!(truncated_utf8.contains("[truncated"));
    }

    #[test]
    fn test_extract_shell_command() {
        // Test string command
        let args1 = json!({"command": "ls -la"});
        assert_eq!(
            extract_shell_command("local_shell", &args1),
            Some("ls -la".to_string())
        );

        // Test Vec<String> command (Codex ShellToolCallParams format)
        let args_vec = json!({"command": ["ls", "-la", "/tmp"]});
        assert_eq!(
            extract_shell_command("local_shell", &args_vec),
            Some("ls -la /tmp".to_string())
        );

        // Test empty Vec<String> command
        let args_empty_vec = json!({"command": []});
        assert_eq!(extract_shell_command("shell", &args_empty_vec), None);

        // Test Vec with non-string elements (should filter them out)
        let args_mixed = json!({"command": ["echo", 123, "hello", null, "world"]});
        assert_eq!(
            extract_shell_command("bash", &args_mixed),
            Some("echo hello world".to_string())
        );

        // Test other field names
        let args2 = json!({"cmd": "pwd"});
        assert_eq!(
            extract_shell_command("exec", &args2),
            Some("pwd".to_string())
        );

        let args3 = json!({"script": "echo hello"});
        assert_eq!(
            extract_shell_command("run_bash", &args3),
            Some("echo hello".to_string())
        );

        // Test non-shell tool
        let args4 = json!({"command": "value"});
        assert_eq!(extract_shell_command("read_file", &args4), None);

        // Test missing command field
        let args5 = json!({"other": "value"});
        assert_eq!(extract_shell_command("shell", &args5), None);
    }

    #[test]
    fn test_extract_shell_params() {
        // Test comprehensive parameter extraction
        let args_full = json!({
            "command": ["npm", "test"],
            "workdir": "/project",
            "timeout_ms": 30000,
            "with_escalated_permissions": true,
            "justification": "Need to install global packages"
        });

        let params = extract_shell_params("local_shell", &args_full);
        assert_eq!(params.command, Some("npm test".to_string()));
        assert_eq!(params.workdir, Some("/project".to_string()));
        assert_eq!(params.timeout_ms, Some(30000));
        assert_eq!(params.with_escalated_permissions, Some(true));
        assert_eq!(
            params.justification,
            Some("Need to install global packages".to_string())
        );

        // Test alternative field names
        let args_alt = json!({
            "command": "ls -la",
            "cwd": "/tmp",
            "timeout": 5000,
            "sudo": false,
            "reason": "List files"
        });

        let params_alt = extract_shell_params("bash", &args_alt);
        assert_eq!(params_alt.command, Some("ls -la".to_string()));
        assert_eq!(params_alt.workdir, Some("/tmp".to_string()));
        assert_eq!(params_alt.timeout_ms, Some(5000));
        assert_eq!(params_alt.with_escalated_permissions, Some(false));
        assert_eq!(params_alt.justification, Some("List files".to_string()));

        // Test non-shell tool returns defaults
        let args_other = json!({
            "command": "test",
            "workdir": "/project"
        });

        let params_none = extract_shell_params("read_file", &args_other);
        assert_eq!(params_none.command, None);
        assert_eq!(params_none.workdir, None);
    }

    #[test]
    fn test_format_tool_output() {
        // String output
        let string_output = json!("Simple text output");
        assert_eq!(
            format_tool_output("any_tool", &string_output, 100),
            "Simple text output"
        );

        // Structured output with stdout
        let structured = json!({
            "stdout": "Command output here",
            "stderr": "Warning message",
            "exit_code": 1
        });
        let formatted = format_tool_output("shell", &structured, 200);
        assert!(formatted.contains("Command output here"));
        assert!(formatted.contains("[stderr]"));
        assert!(formatted.contains("Warning message"));
        assert!(formatted.contains("[exit code: 1]"));

        // Array output
        let array = json!(["item1", "item2", "item3"]);
        let formatted_array = format_tool_output("list", &array, 100);
        assert!(formatted_array.contains("item1"));
        assert!(formatted_array.contains("item2"));
        assert!(formatted_array.contains("item3"));

        // Large array - truncated
        let large_array: Vec<String> = (0..20).map(|i| format!("item{}", i)).collect();
        let large_json = json!(large_array);
        let formatted_large = format_tool_output("list", &large_json, 200);
        assert!(formatted_large.contains("item0"));
        assert!(formatted_large.contains("... and 10 more items"));
    }
}
