# MCP Tool Contract: multi_edit_text_file

status: pending_validation

## Tool Metadata

```json
{
  "name": "multi_edit_text_file",
  "description": "Apply multiple string replacement edits to a file with cumulative diff",
  "inputSchema": {
    "type": "object",
    "properties": {
      "path": {
        "type": "string",
        "description": "Absolute path to the file"
      },
      "edits": {
        "type": "array",
        "description": "Array of edit operations to apply sequentially",
        "items": {
          "type": "object",
          "properties": {
            "old_string": {
              "type": "string",
              "description": "Exact string to find and replace"
            },
            "new_string": {
              "type": "string",
              "description": "Replacement string"
            }
          },
          "required": ["old_string", "new_string"]
        }
      }
    },
    "required": ["path", "edits"]
  }
}
```

## Contract Specification

### Input

```typescript
interface MultiEditTextFileParams {
  path: string;       // Absolute path
  edits: EditOp[];    // Array of edit operations
}

interface EditOp {
  old_string: string; // Exact match
  new_string: string; // Replacement
}
```

**Example Requests**:

```json
// Multiple related edits in one file
{
  "path": "/test/config.toml",
  "edits": [
    {"old_string": "port = 8080", "new_string": "port = 3000"},
    {"old_string": "host = \"localhost\"", "new_string": "host = \"0.0.0.0\""},
    {"old_string": "debug = false", "new_string": "debug = true"}
  ]
}
```

---

### Output

```typescript
interface MultiEditTextFileResult {
  success: boolean;     // Always true on success
  diff: string;         // Cumulative unified diff (original → final)
  applied_count: number; // Number of edits successfully applied
  line_ranges: Array<{  // Location of each change
    edit_index: number; // Which edit (0-indexed)
    start: number;      // Starting line (1-indexed)
    end: number;        // Ending line (1-indexed)
  }>;
}
```

**Example Response**:

```json
{
  "success": true,
  "diff": "--- config.toml\n+++ config.toml\n@@ -2,3 +2,3 @@\n [server]\n-host = \"localhost\"\n+host = \"0.0.0.0\"\n-port = 8080\n+port = 3000\n@@ -5,1 +5,1 @@\n-debug = false\n+debug = true\n",
  "applied_count": 3,
  "line_ranges": [
    {"edit_index": 0, "start": 3, "end": 3},
    {"edit_index": 1, "start": 4, "end": 4},
    {"edit_index": 2, "start": 6, "end": 6}
  ]
}
```

---

### Error Handling

| Error Condition | Error Code | Message Pattern |
|-----------------|------------|-----------------|
| File not found | -32001 | "File not found: {path}" |
| Permission denied | -32002 | "Permission denied: {path}" |
| String not found | -32010 | "Edit {index}: String not found: {old_string}" |
| Multiple matches | -32011 | "Edit {index}: String appears {count} times: {old_string}" |
| Invalid path | -32600 | "Path must be absolute: {path}" |
| Empty edits array | -32600 | "Edits array cannot be empty" |
| Conflicting edits | -32012 | "Edit {index} conflicts with edit {prior_index}" |
| Binary file | -32004 | "Cannot edit binary file: {path}" |

**Example Error**:

```json
{
  "error": {
    "code": -32010,
    "message": "Edit 1: String not found: host = \"localhost\""
  }
}
```

---

## Implementation Contract

### Staged Edits Pattern

**Key Concept**: Apply edits sequentially to in-memory staged content, then generate cumulative diff between original and final.

```rust
async fn handle_multi_edit_tool(
    acp_client: &dyn AcpClientApi,
    params: MultiEditTextFileParams,
) -> Result<MultiEditTextFileResult, McpError> {
    // 1. Validate params
    if !params.path.is_absolute() {
        return Err(McpError::invalid_params("Path must be absolute"));
    }
    if params.edits.is_empty() {
        return Err(McpError::invalid_params("Edits array cannot be empty"));
    }

    // 2. Read original content
    let original = read_file_content(acp_client, &params.path).await?;

    // 3. Initialize staged content
    let mut staged = StagedFile::new(original.clone());

    // 4. Apply edits sequentially
    let mut line_ranges = Vec::new();
    for (index, edit) in params.edits.iter().enumerate() {
        // Check uniqueness in current staged content
        let occurrences = staged.content.matches(&edit.old_string).count();
        if occurrences == 0 {
            return Err(McpError::string_not_found_with_context(
                &edit.old_string,
                index
            ));
        }
        if occurrences > 1 {
            return Err(McpError::multiple_matches_with_context(
                &edit.old_string,
                occurrences,
                index
            ));
        }

        // Apply edit to staged content
        let line_range = staged.apply_edit(edit)?;
        line_ranges.push(LineRange {
            edit_index: index,
            start: line_range.start,
            end: line_range.end,
        });
    }

    // 5. Generate cumulative diff (original → staged final)
    let diff = generate_unified_diff(&params.path, &original, &staged.content);

    // 6. Write final content
    write_file_content(acp_client, &params.path, &staged.content).await?;

    Ok(MultiEditTextFileResult {
        success: true,
        diff,
        applied_count: params.edits.len(),
        line_ranges,
    })
}
```

---

### StagedFile Implementation

```rust
struct StagedFile {
    original: String,    // Never modified
    content: String,     // Updated with each edit
    edits: Vec<EditOp>,  // History of applied edits
}

impl StagedFile {
    fn new(original: String) -> Self {
        Self {
            content: original.clone(),
            original,
            edits: Vec::new(),
        }
    }

    fn apply_edit(&mut self, edit: &EditOp) -> Result<LineRange, McpError> {
        // Find line range BEFORE applying edit
        let line_range = find_change_line_range(&self.content, &edit.old_string);

        // Apply replacement
        self.content = self.content.replacen(&edit.old_string, &edit.new_string, 1);

        // Store edit in history
        self.edits.push(edit.clone());

        Ok(line_range)
    }

    fn generate_diff(&self, path: &str) -> String {
        generate_unified_diff(path, &self.original, &self.content)
    }
}
```

---

## Test Cases

### TC1: Multiple Edits in Same File

**Given**: File "/test/config.toml" contains:

```toml
[server]
host = "localhost"
port = 8080

[app]
debug = false
```

**When**: multi_edit_text_file({
  "path": "/test/config.toml",
  "edits": [
    {"old_string": "port = 8080", "new_string": "port = 3000"},
    {"old_string": "host = \"localhost\"", "new_string": "host = \"0.0.0.0\""},
    {"old_string": "debug = false", "new_string": "debug = true"}
  ]
})

**Then**:

- success = true
- applied_count = 3
- diff shows all 3 changes in cumulative format
- line_ranges = [{0, 3, 3}, {1, 2, 2}, {2, 6, 6}]

---

### TC2: Sequential Edits (Order Matters)

**Given**: File "/test/file.txt" contains "AAA"

**When**: multi_edit_text_file({
  "path": "/test/file.txt",
  "edits": [
    {"old_string": "AAA", "new_string": "BBB"},
    {"old_string": "BBB", "new_string": "CCC"}
  ]
})

**Then**:

- success = true
- applied_count = 2
- File contains "CCC" (edits applied sequentially)
- diff shows `- AAA` and `+ CCC`

---

### TC3: String Not Found in Edit 2

**Given**: File "/test/file.txt" contains "line 1\nline 2\n"

**When**: multi_edit_text_file({
  "path": "/test/file.txt",
  "edits": [
    {"old_string": "line 1", "new_string": "LINE 1"},
    {"old_string": "line 3", "new_string": "LINE 3"}  // Doesn't exist
  ]
})

**Then**: Error with code -32010, message "Edit 1: String not found: line 3"

---

### TC4: Empty Edits Array

**Given**: File "/test/file.txt" exists
**When**: multi_edit_text_file({"path": "/test/file.txt", "edits": []})
**Then**: Error with code -32600, message "Edits array cannot be empty"

---

### TC5: Duplicate Edits (Same String Twice)

**Given**: File "/test/file.txt" contains "foo"

**When**: multi_edit_text_file({
  "path": "/test/file.txt",
  "edits": [
    {"old_string": "foo", "new_string": "bar"},
    {"old_string": "foo", "new_string": "baz"}  // "foo" no longer exists after edit 0
  ]
})

**Then**: Error with code -32010, message "Edit 1: String not found: foo"

---

### TC6: Large Number of Edits

**Given**: File with 100 unique strings
**When**: multi_edit_text_file with 100 edits
**Then**:

- success = true
- applied_count = 100
- Operation completes in <500ms

---

### TC7: Edits Creating Ambiguity

**Given**: File "/test/file.txt" contains "A"

**When**: multi_edit_text_file({
  "path": "/test/file.txt",
  "edits": [
    {"old_string": "A", "new_string": "AA"},  // Now file has "AA"
    {"old_string": "A", "new_string": "B"}    // Which "A"? Ambiguous!
  ]
})

**Then**: Error with code -32011, message "Edit 1: String appears 2 times: A"

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
      "title": "multi_edit_text_file (3 edits)",
      "locations": [{"path": "/test/config.toml"}],
      "rawInput": {
        "path": "/test/config.toml",
        "edits": [...]
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
          "diff": "...",
          "applied_count": 3,
          "line_ranges": [...]
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
