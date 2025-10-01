# MCP Tool Contract: edit_text_file

status: pending_validation

## Tool Metadata

```json
{
  "name": "edit_text_file",
  "description": "Apply a single string replacement edit to a file with diff preview",
  "inputSchema": {
    "type": "object",
    "properties": {
      "path": {
        "type": "string",
        "description": "Absolute path to the file"
      },
      "old_string": {
        "type": "string",
        "description": "Exact string to find and replace (must be unique in file)"
      },
      "new_string": {
        "type": "string",
        "description": "Replacement string"
      }
    },
    "required": ["path", "old_string", "new_string"]
  }
}
```

## Contract Specification

### Input

```typescript
interface EditTextFileParams {
  path: string;       // Absolute path
  old_string: string; // Exact match (must be unique)
  new_string: string; // Replacement
}
```

**Example Requests**:

```json
// Replace config value
{
  "path": "/test/config.toml",
  "old_string": "port = 8080",
  "new_string": "port = 3000"
}

// Add line
{
  "path": "/test/file.txt",
  "old_string": "line 1\n",
  "new_string": "line 1\nline 1.5\n"
}

// Delete line (replace with empty)
{
  "path": "/test/file.txt",
  "old_string": "line to delete\n",
  "new_string": ""
}
```

---

### Output

```typescript
interface EditTextFileResult {
  success: boolean;    // Always true on success
  diff: string;        // Unified diff format
  line_range: {        // Location of change
    start: number;     // Starting line number (1-indexed)
    end: number;       // Ending line number (1-indexed)
  };
}
```

**Example Response**:

```json
{
  "success": true,
  "diff": "--- config.toml\n+++ config.toml\n@@ -3,1 +3,1 @@\n-port = 8080\n+port = 3000\n",
  "line_range": {
    "start": 3,
    "end": 3
  }
}
```

---

### Error Handling

| Error Condition | Error Code | Message Pattern |
|-----------------|------------|-----------------|
| File not found | -32001 | "File not found: {path}" |
| Permission denied | -32002 | "Permission denied: {path}" |
| String not found | -32010 | "String not found in file: {old_string}" |
| Multiple matches | -32011 | "String appears {count} times (must be unique): {old_string}" |
| Invalid path | -32600 | "Path must be absolute: {path}" |
| Binary file | -32004 | "Cannot edit binary file: {path}" |
| Same strings | -32600 | "old_string and new_string are identical" |

**Example Error**:

```json
{
  "error": {
    "code": -32011,
    "message": "String appears 3 times (must be unique): port = 8080"
  }
}
```

---

## Implementation Contract

### ACP Client API Path

**Primary**: `acp_client.read_text_file() + apply_edit() + acp_client.write_text_file()`

```rust
async fn handle_edit_tool(
    acp_client: &dyn AcpClientApi,
    params: EditTextFileParams,
) -> Result<EditTextFileResult, McpError> {
    // 1. Validate params
    if !params.path.is_absolute() {
        return Err(McpError::invalid_params("Path must be absolute"));
    }
    if params.old_string == params.new_string {
        return Err(McpError::invalid_params("old_string and new_string are identical"));
    }

    // 2. Read original content (try ACP first, fallback to local fs)
    let original = read_file_content(acp_client, &params.path).await?;

    // 3. Find and validate occurrences
    let occurrences = original.matches(&params.old_string).count();
    if occurrences == 0 {
        return Err(McpError::string_not_found(&params.old_string));
    }
    if occurrences > 1 {
        return Err(McpError::multiple_matches(&params.old_string, occurrences));
    }

    // 4. Apply replacement
    let new_content = original.replacen(&params.old_string, &params.new_string, 1);

    // 5. Generate unified diff
    let diff = generate_unified_diff(&params.path, &original, &new_content);

    // 6. Find line range of change
    let line_range = find_change_line_range(&original, &params.old_string);

    // 7. Write new content (try ACP first, fallback to local fs)
    write_file_content(acp_client, &params.path, &new_content).await?;

    Ok(EditTextFileResult {
        success: true,
        diff,
        line_range,
    })
}
```

---

### Diff Generation

**Unified Diff Format** (standard):

```diff
--- path/to/file.txt
+++ path/to/file.txt
@@ -3,1 +3,1 @@
-old line
+new line
```

**Implementation**:

```rust
fn generate_unified_diff(path: &str, original: &str, modified: &str) -> String {
    use similar::{ChangeTag, TextDiff};

    let diff = TextDiff::from_lines(original, modified);
    let mut result = format!("--- {}\n+++ {}\n", path, path);

    for hunk in diff.unified_diff().iter_hunks() {
        result.push_str(&format!("@@ -{},{} +{},{} @@\n",
            hunk.old_range().start + 1,
            hunk.old_range().len(),
            hunk.new_range().start + 1,
            hunk.new_range().len()
        ));

        for change in hunk.iter_changes() {
            let sign = match change.tag() {
                ChangeTag::Delete => "-",
                ChangeTag::Insert => "+",
                ChangeTag::Equal => " ",
            };
            result.push_str(&format!("{}{}", sign, change.value()));
        }
    }

    result
}
```

---

## Test Cases

### TC1: Simple String Replacement

**Given**: File "/test/config.toml" contains:

```toml
[server]
host = "localhost"
port = 8080
```

**When**: edit_text_file({
  "path": "/test/config.toml",
  "old_string": "port = 8080",
  "new_string": "port = 3000"
})

**Then**:

- success = true
- diff contains `- port = 8080` and `+ port = 3000`
- line_range = {start: 3, end: 3}
- File content updated

---

### TC2: Multi-Line Replacement

**Given**: File "/test/code.rs" contains:

```rust
fn old_func() {
    println!("old");
}
```

**When**: edit_text_file({
  "path": "/test/code.rs",
  "old_string": "fn old_func() {\n    println!(\"old\");\n}",
  "new_string": "fn new_func() {\n    println!(\"new\");\n}"
})

**Then**:

- success = true
- diff shows 3-line replacement
- line_range = {start: 1, end: 3}

---

### TC3: String Not Found

**Given**: File "/test/file.txt" contains "Hello World"
**When**: edit_text_file({
  "path": "/test/file.txt",
  "old_string": "Goodbye",
  "new_string": "Hello"
})
**Then**: Error with code -32010, message "String not found in file: Goodbye"

---

### TC4: Multiple Matches (Ambiguous)

**Given**: File "/test/file.txt" contains "foo\nfoo\nfoo"
**When**: edit_text_file({
  "path": "/test/file.txt",
  "old_string": "foo",
  "new_string": "bar"
})
**Then**: Error with code -32011, message "String appears 3 times (must be unique): foo"

---

### TC5: Delete Line (Empty Replacement)

**Given**: File "/test/file.txt" contains "line 1\nline 2\nline 3\n"
**When**: edit_text_file({
  "path": "/test/file.txt",
  "old_string": "line 2\n",
  "new_string": ""
})
**Then**:

- success = true
- diff shows deletion of line 2
- File contains "line 1\nline 3\n"

---

### TC6: File Not Found

**Given**: File "/test/missing.txt" does not exist
**When**: edit_text_file({...})
**Then**: Error with code -32001, message "File not found: /test/missing.txt"

---

### TC7: Identical Strings

**Given**: N/A
**When**: edit_text_file({
  "path": "/test/file.txt",
  "old_string": "same",
  "new_string": "same"
})
**Then**: Error with code -32600, message "old_string and new_string are identical"

---

## ACP Event Mapping

When Codex CLI calls this MCP tool, codex-cli-acp MUST emit:

**On tool call begin**:

```json
{
  "method": "session/update",
  "params": {
    "sessionId": "<session-id>",
    "update": {
      "type": "toolCall",
      "id": "<tool-call-id>",
      "kind": "edit",
      "status": "pending",
      "title": "edit_text_file",
      "locations": [{"path": "/test/config.toml", "line": 3}],
      "rawInput": {
        "path": "/test/config.toml",
        "old_string": "port = 8080",
        "new_string": "port = 3000"
      }
    }
  }
}
```

**On tool call complete**:

```json
{
  "method": "session/update",
  "params": {
    "sessionId": "<session-id>",
    "update": {
      "type": "toolCallUpdate",
      "id": "<tool-call-id>",
      "fields": {
        "status": "completed",
        "rawOutput": {
          "success": true,
          "diff": "--- config.toml\n+++ config.toml\n...",
          "line_range": {"start": 3, "end": 3}
        },
        "content": [
          {
            "type": "text",
            "text": "--- config.toml\n+++ config.toml\n..."
          }
        ]
      }
    }
  }
}
```

---

⚠️ _This contract must pass before implementation begins (Test-First, Article III)_
⚠️ _Based on SDD CONSTITUTION: `.specify/memory/constitution.md`_

---
