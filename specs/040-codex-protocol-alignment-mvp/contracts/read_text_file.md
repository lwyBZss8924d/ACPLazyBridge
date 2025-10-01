# MCP Tool Contract: read_text_file

status: pending_validation

## Tool Metadata

```json
{
  "name": "read_text_file",
  "description": "Read complete or partial content of a text file with optional pagination",
  "inputSchema": {
    "type": "object",
    "properties": {
      "path": {
        "type": "string",
        "description": "Absolute path to the file"
      },
      "line": {
        "type": "integer",
        "minimum": 1,
        "description": "Starting line number (1-indexed). If omitted, read from line 1"
      },
      "limit": {
        "type": "integer",
        "minimum": 1,
        "description": "Maximum number of lines to return. If omitted, return all remaining lines"
      }
    },
    "required": ["path"]
  }
}
```

## Contract Specification

### Input

```typescript
interface ReadTextFileParams {
  path: string;         // Absolute path
  line?: number;        // Optional: starting line (1-indexed)
  limit?: number;       // Optional: max lines to return
}
```

**Example Requests**:

```json
// Read entire file
{"path": "/test/file.txt"}

// Read from line 10 onwards
{"path": "/test/file.txt", "line": 10}

// Read lines 10-19 (10 lines starting at line 10)
{"path": "/test/file.txt", "line": 10, "limit": 10}
```

---

### Output

```typescript
interface ReadTextFileResult {
  content: string;      // File content (with newlines preserved)
  _meta: {
    total_lines: number;    // Total lines in file
    returned_lines: number; // Lines in this response
    has_more: boolean;      // True if more lines available
    next_line?: number;     // Starting line for next request (if has_more)
  }
}
```

**Example Responses**:

```json
// Full file (3 lines)
{
  "content": "line 1\nline 2\nline 3\n",
  "_meta": {
    "total_lines": 3,
    "returned_lines": 3,
    "has_more": false
  }
}

// Paginated (lines 10-19 of 100-line file)
{
  "content": "line 10\n...line 19\n",
  "_meta": {
    "total_lines": 100,
    "returned_lines": 10,
    "has_more": true,
    "next_line": 20
  }
}
```

---

### Error Handling

| Error Condition | Error Code | Message Pattern |
|-----------------|------------|-----------------|
| File not found | -32001 | "File not found: {path}" |
| Permission denied | -32002 | "Permission denied: {path}" |
| Not a file | -32003 | "{path} is not a file" |
| Invalid path | -32600 | "Path must be absolute: {path}" |
| Invalid line number | -32600 | "Line number must be >= 1: {line}" |
| Invalid limit | -32600 | "Limit must be >= 1: {limit}" |
| Binary file | -32004 | "Cannot read binary file: {path}" |

**Example Error**:

```json
{
  "error": {
    "code": -32001,
    "message": "File not found: /test/missing.txt"
  }
}
```

---

## Implementation Contract

### ACP Client API Path

**Primary**: `acp_client.read_text_file(path, line, limit)`

```rust
async fn handle_read_tool(
    acp_client: &dyn AcpClientApi,
    params: ReadTextFileParams,
) -> Result<ReadTextFileResult, McpError> {
    // 1. Validate params
    if !params.path.is_absolute() {
        return Err(McpError::invalid_params("Path must be absolute"));
    }

    // 2. Try ACP client first
    match acp_client.read_text_file(&params.path, params.line, params.limit).await {
        Ok(content) => {
            // 3. Calculate metadata
            let lines: Vec<&str> = content.lines().collect();
            let returned_lines = lines.len();

            // If pagination params provided, calculate total_lines
            let total_lines = if params.line.is_some() || params.limit.is_some() {
                // For paginated requests, need to query total line count
                // This may require a separate stat call or assume returned_lines
                returned_lines  // Simplified for contract
            } else {
                returned_lines
            };

            let has_more = params.limit.is_some() && returned_lines == params.limit.unwrap();
            let next_line = if has_more {
                Some(params.line.unwrap_or(1) + returned_lines)
            } else {
                None
            };

            Ok(ReadTextFileResult {
                content,
                _meta: ReadMeta {
                    total_lines,
                    returned_lines,
                    has_more,
                    next_line,
                },
            })
        }
        Err(e) => {
            // 4. Fallback to local filesystem
            fallback_read_local(&params).await
        }
    }
}
```

---

### Fallback (Local Filesystem)

**Secondary**: `tokio::fs::read_to_string(path)` with pagination logic

```rust
async fn fallback_read_local(params: &ReadTextFileParams) -> Result<ReadTextFileResult, McpError> {
    use tokio::fs;

    // Read entire file
    let full_content = fs::read_to_string(&params.path).await
        .map_err(|e| match e.kind() {
            io::ErrorKind::NotFound => McpError::file_not_found(&params.path),
            io::ErrorKind::PermissionDenied => McpError::permission_denied(&params.path),
            _ => McpError::internal(e.to_string()),
        })?;

    // Apply pagination
    let lines: Vec<&str> = full_content.lines().collect();
    let total_lines = lines.len();

    let start_line = params.line.unwrap_or(1).saturating_sub(1);  // Convert to 0-indexed
    let end_line = if let Some(limit) = params.limit {
        (start_line + limit).min(total_lines)
    } else {
        total_lines
    };

    let selected_lines = &lines[start_line..end_line];
    let content = selected_lines.join("\n") + "\n";
    let returned_lines = selected_lines.len();
    let has_more = end_line < total_lines;
    let next_line = if has_more { Some((end_line + 1) as u32) } else { None };

    Ok(ReadTextFileResult {
        content,
        _meta: ReadMeta {
            total_lines: total_lines as u32,
            returned_lines: returned_lines as u32,
            has_more,
            next_line,
        },
    })
}
```

---

## Test Cases

### TC1: Read Entire File

**Given**: File "/test/hello.txt" contains "Hello\nWorld\n"
**When**: read_text_file({"path": "/test/hello.txt"})
**Then**:

- content = "Hello\nWorld\n"
- total_lines = 2
- returned_lines = 2
- has_more = false
- next_line = None

---

### TC2: Read with Pagination

**Given**: File "/test/numbers.txt" contains lines "1\n2\n3\n...100\n"
**When**: read_text_file({"path": "/test/numbers.txt", "line": 10, "limit": 5})
**Then**:

- content = "10\n11\n12\n13\n14\n"
- total_lines = 100
- returned_lines = 5
- has_more = true
- next_line = 15

---

### TC3: File Not Found

**Given**: File "/test/missing.txt" does not exist
**When**: read_text_file({"path": "/test/missing.txt"})
**Then**: Error with code -32001, message "File not found: /test/missing.txt"

---

### TC4: Invalid Path (Relative)

**Given**: N/A
**When**: read_text_file({"path": "relative/path.txt"})
**Then**: Error with code -32600, message "Path must be absolute: relative/path.txt"

---

### TC5: Binary File

**Given**: File "/test/image.png" is binary
**When**: read_text_file({"path": "/test/image.png"})
**Then**: Error with code -32004, message "Cannot read binary file: /test/image.png"

---

### TC6: Empty File

**Given**: File "/test/empty.txt" exists but is empty
**When**: read_text_file({"path": "/test/empty.txt"})
**Then**:

- content = ""
- total_lines = 0
- returned_lines = 0
- has_more = false

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
      "kind": "read",
      "status": "pending",
      "title": "read_text_file",
      "locations": [{"path": "/test/file.txt"}],
      "rawInput": {"path": "/test/file.txt", "line": 10, "limit": 10}
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
          "content": "...",
          "_meta": {...}
        }
      }
    }
  }
}
```

---

⚠️ _This contract must pass before implementation begins (Test-First, Article III)_
⚠️ _Based on SDD CONSTITUTION: `.specify/memory/constitution.md`_

---
