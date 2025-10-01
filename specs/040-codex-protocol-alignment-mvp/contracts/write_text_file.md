# MCP Tool Contract: write_text_file

status: pending_validation

## Tool Metadata

```json
{
  "name": "write_text_file",
  "description": "Write complete content to a text file (creates or overwrites)",
  "inputSchema": {
    "type": "object",
    "properties": {
      "path": {
        "type": "string",
        "description": "Absolute path to the file"
      },
      "content": {
        "type": "string",
        "description": "Complete file content to write"
      }
    },
    "required": ["path", "content"]
  }
}
```

## Contract Specification

### Input

```typescript
interface WriteTextFileParams {
  path: string;    // Absolute path
  content: string; // Complete file content
}
```

**Example Requests**:

```json
// Create new file
{"path": "/test/new.txt", "content": "Hello, World!\n"}

// Overwrite existing file
{"path": "/test/existing.txt", "content": "Updated content\n"}

// Write empty file
{"path": "/test/empty.txt", "content": ""}
```

---

### Output

```typescript
interface WriteTextFileResult {
  success: boolean;     // Always true on success (otherwise error thrown)
  bytes_written: number; // Number of bytes written
  created: boolean;      // True if file was created, false if overwritten
}
```

**Example Responses**:

```json
// New file created
{
  "success": true,
  "bytes_written": 14,
  "created": true
}

// Existing file overwritten
{
  "success": true,
  "bytes_written": 20,
  "created": false
}
```

---

### Error Handling

| Error Condition | Error Code | Message Pattern |
|-----------------|------------|-----------------|
| Permission denied | -32002 | "Permission denied: {path}" |
| Invalid path | -32600 | "Path must be absolute: {path}" |
| Directory not found | -32001 | "Parent directory not found: {parent_path}" |
| Path is directory | -32003 | "{path} is a directory" |
| Disk full | -32005 | "Disk full: cannot write {bytes} bytes to {path}" |
| Read-only filesystem | -32002 | "Read-only filesystem: {path}" |

**Example Error**:

```json
{
  "error": {
    "code": -32002,
    "message": "Permission denied: /root/secret.txt"
  }
}
```

---

## Implementation Contract

### ACP Client API Path

**Primary**: `acp_client.write_text_file(path, content)`

```rust
async fn handle_write_tool(
    acp_client: &dyn AcpClientApi,
    params: WriteTextFileParams,
) -> Result<WriteTextFileResult, McpError> {
    // 1. Validate params
    if !params.path.is_absolute() {
        return Err(McpError::invalid_params("Path must be absolute"));
    }

    // 2. Check if file exists (for 'created' flag)
    let file_existed = acp_client.file_exists(&params.path).await
        .unwrap_or(false);

    // 3. Try ACP client first
    match acp_client.write_text_file(&params.path, &params.content).await {
        Ok(()) => {
            Ok(WriteTextFileResult {
                success: true,
                bytes_written: params.content.len(),
                created: !file_existed,
            })
        }
        Err(e) => {
            // 4. Fallback to local filesystem
            fallback_write_local(&params, file_existed).await
        }
    }
}
```

---

### Fallback (Local Filesystem)

**Secondary**: `tokio::fs::write(path, content)`

```rust
async fn fallback_write_local(
    params: &WriteTextFileParams,
    file_existed: bool,
) -> Result<WriteTextFileResult, McpError> {
    use tokio::fs;

    // Write content
    fs::write(&params.path, &params.content).await
        .map_err(|e| match e.kind() {
            io::ErrorKind::PermissionDenied => McpError::permission_denied(&params.path),
            io::ErrorKind::NotFound => {
                let parent = Path::new(&params.path).parent().unwrap();
                McpError::not_found(&format!("Parent directory not found: {}", parent.display()))
            }
            io::ErrorKind::IsADirectory => McpError::invalid_params(&format!("{} is a directory", params.path)),
            _ => McpError::internal(e.to_string()),
        })?;

    Ok(WriteTextFileResult {
        success: true,
        bytes_written: params.content.len(),
        created: !file_existed,
    })
}
```

---

## Test Cases

### TC1: Create New File

**Given**: File "/test/new.txt" does not exist
**When**: write_text_file({"path": "/test/new.txt", "content": "Hello\n"})
**Then**:

- success = true
- bytes_written = 6
- created = true
- File "/test/new.txt" contains "Hello\n"

---

### TC2: Overwrite Existing File

**Given**: File "/test/existing.txt" contains "Old content\n"
**When**: write_text_file({"path": "/test/existing.txt", "content": "New content\n"})
**Then**:

- success = true
- bytes_written = 12
- created = false
- File "/test/existing.txt" contains "New content\n"

---

### TC3: Write Empty File

**Given**: N/A
**When**: write_text_file({"path": "/test/empty.txt", "content": ""})
**Then**:

- success = true
- bytes_written = 0
- created = true
- File "/test/empty.txt" exists and is empty

---

### TC4: Permission Denied

**Given**: Directory "/root" requires elevated permissions
**When**: write_text_file({"path": "/root/secret.txt", "content": "data"})
**Then**: Error with code -32002, message "Permission denied: /root/secret.txt"

---

### TC5: Invalid Path (Relative)

**Given**: N/A
**When**: write_text_file({"path": "relative/path.txt", "content": "data"})
**Then**: Error with code -32600, message "Path must be absolute: relative/path.txt"

---

### TC6: Parent Directory Not Found

**Given**: Directory "/test/missing-dir" does not exist
**When**: write_text_file({"path": "/test/missing-dir/file.txt", "content": "data"})
**Then**: Error with code -32001, message "Parent directory not found: /test/missing-dir"

---

### TC7: Path Is Directory

**Given**: "/test/dir" exists and is a directory
**When**: write_text_file({"path": "/test/dir", "content": "data"})
**Then**: Error with code -32003, message "/test/dir is a directory"

---

### TC8: Large File (Performance)

**Given**: N/A
**When**: write_text_file({"path": "/test/large.txt", "content": <1MB string>})
**Then**:

- success = true
- bytes_written = 1048576
- Operation completes in <100ms

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
      "kind": "write",
      "status": "pending",
      "title": "write_text_file",
      "locations": [{"path": "/test/file.txt"}],
      "rawInput": {
        "path": "/test/file.txt",
        "content": "..." // Truncated if >10KB
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
          "bytes_written": 14,
          "created": true
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
