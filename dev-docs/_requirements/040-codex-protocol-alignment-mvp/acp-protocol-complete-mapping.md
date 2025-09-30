# ACP Protocol - Complete Type & Message Mapping for Adapters

**Document Date:** 2025-09-30
**Source:** agent-client-protocol/docs & schema.json (v0.4.3 / Protocol Version 1)
**Purpose:** Complete enumeration of all ACP types, messages, and requirements for adapter implementations

---

## Table of Contents

1. [Protocol Overview](#protocol-overview)
2. [SessionUpdate Enum - Complete Variants](#sessionupdate-enum---complete-variants)
3. [ToolKind Enum - All Categories](#toolkind-enum---all-categories)
4. [ToolCallStatus Lifecycle](#toolcallstatus-lifecycle)
5. [ContentBlock Types](#contentblock-types)
6. [StopReason Enum](#stopreason-enum)
7. [PermissionMode & Options](#permissionmode--options)
8. [SessionMode Capabilities](#sessionmode-capabilities)
9. [MCP Server Integration](#mcp-server-integration)
10. [Terminal Management Protocol](#terminal-management-protocol)
11. [File System Operations](#file-system-operations)
12. [Agent Methods (Required & Optional)](#agent-methods-required--optional)
13. [Client Methods (Required & Optional)](#client-methods-required--optional)
14. [Validation Rules & Constraints](#validation-rules--constraints)
15. [Best Practices from Protocol](#best-practices-from-protocol)
16. [Experimental & Unstable Features](#experimental--unstable-features)

---

## Protocol Overview

### Core Communication Model

- **Protocol**: JSON-RPC 2.0
- **Current Version**: 1 (integer, not string)
- **Message Types**:
    - **Methods**: Request-response with result/error
    - **Notifications**: One-way, no response expected

### Message Flow

1. **Initialization**: `initialize` → optional `authenticate`
2. **Session Setup**: `session/new` or `session/load`
3. **Prompt Turn**: `session/prompt` → streaming `session/update` notifications → response with `stopReason`

### Key Protocol Rules

- **All file paths MUST be absolute**
- **Line numbers are 1-based**
- **Protocol version MUST be integer** (not string `"1"`)
- **stdout = JSON-RPC only, stderr = logs**

---

## SessionUpdate Enum - Complete Variants

All variants that can appear in `session/update` notifications:

### 1. `user_message_chunk`

**Purpose**: Stream chunks of user's message

```json
{
  "sessionUpdate": "user_message_chunk",
  "content": ContentBlock
}
```

**Required Fields**: `sessionUpdate`, `content`

---

### 2. `agent_message_chunk`

**Purpose**: Stream chunks of agent's response

```json
{
  "sessionUpdate": "agent_message_chunk",
  "content": ContentBlock
}
```

**Required Fields**: `sessionUpdate`, `content`

---

### 3. `agent_thought_chunk`

**Purpose**: Stream agent's internal reasoning

```json
{
  "sessionUpdate": "agent_thought_chunk",
  "content": ContentBlock
}
```

**Required Fields**: `sessionUpdate`, `content`

---

### 4. `tool_call`

**Purpose**: Initial tool call creation

```json
{
  "sessionUpdate": "tool_call",
  "toolCallId": "call_001",
  "title": "Reading configuration file",
  "kind": "read",
  "status": "pending",
  "content": [ToolCallContent],
  "locations": [ToolCallLocation],
  "rawInput": {},
  "rawOutput": {}
}
```

**Required Fields**: `sessionUpdate`, `toolCallId`, `title`
**Optional Fields**: `kind`, `status`, `content`, `locations`, `rawInput`, `rawOutput`

---

### 5. `tool_call_update`

**Purpose**: Update existing tool call

```json
{
  "sessionUpdate": "tool_call_update",
  "toolCallId": "call_001",
  "status": "in_progress",
  "title": "Updated title",
  "kind": "read",
  "content": [ToolCallContent],
  "locations": [ToolCallLocation],
  "rawInput": {},
  "rawOutput": {}
}
```

**Required Fields**: `sessionUpdate`, `toolCallId`
**Optional Fields**: All others (only include what's changing)

---

### 6. `plan`

**Purpose**: Agent's execution plan

```json
{
  "sessionUpdate": "plan",
  "entries": [
    {
      "content": "Check for syntax errors",
      "priority": "high",
      "status": "pending"
    }
  ]
}
```

**Required Fields**: `sessionUpdate`, `entries`

**PlanEntry Fields**:

- `content` (string, required): Task description
- `priority` (PlanEntryPriority, required): `high` | `medium` | `low`
- `status` (PlanEntryStatus, required): `pending` | `in_progress` | `completed`

---

### 7. `available_commands_update`

**Purpose**: Notify of available slash commands

```json
{
  "sessionUpdate": "available_commands_update",
  "availableCommands": [
    {
      "name": "create_plan",
      "description": "Create an execution plan",
      "input": {
        "hint": "Enter plan description"
      }
    }
  ]
}
```

**Required Fields**: `sessionUpdate`, `availableCommands`

---

### 8. `current_mode_update`

**Purpose**: Agent changed its mode

```json
{
  "sessionUpdate": "current_mode_update",
  "currentModeId": "code"
}
```

**Required Fields**: `sessionUpdate`, `currentModeId`

---

## ToolKind Enum - All Categories

Complete enumeration with semantic meanings:

| Value | Description | Use Case |
|-------|-------------|----------|
| `read` | Reading files or data | File read, data fetch |
| `edit` | Modifying files or content | File edit, content update |
| `delete` | Removing files or data | File/data deletion |
| `move` | Moving or renaming files | File operations |
| `search` | Searching for information | Code search, grep |
| `execute` | Running commands or code | Shell commands, scripts |
| `think` | Internal reasoning or planning | Analysis, planning |
| `fetch` | Retrieving external data | API calls, web requests |
| `switch_mode` | Switching session mode | Mode transitions |
| `other` | Other tool types (default) | Catch-all |

**Usage**: Helps clients choose appropriate icons and UI treatment

---

## ToolCallStatus Lifecycle

### Valid Status Values

| Status | Meaning | When to Use |
|--------|---------|-------------|
| `pending` | Not started yet | Input streaming or awaiting approval |
| `in_progress` | Currently running | Execution started |
| `completed` | Finished successfully | No errors |
| `failed` | Error occurred | Execution failed |

### Valid Transitions

```txt
pending → in_progress → completed
                     ↘ failed

pending → failed (e.g., permission denied)
```

### Rules

- Tool calls can skip `pending` and go directly to `in_progress`
- `completed` and `failed` are terminal states
- Updates after terminal states are allowed (e.g., adding more content)

---

## ContentBlock Types

All content types that can appear in messages:

### 1. Text (REQUIRED - All agents MUST support)

```json
{
  "type": "text",
  "text": "The actual text content",
  "annotations": {
    "audience": ["user", "assistant"],
    "priority": 1.0,
    "lastModified": "2025-09-30T00:00:00Z"
  }
}
```

**Required**: `type`, `text`
**Optional**: `annotations`

---

### 2. Image (Requires `image` capability)

```json
{
  "type": "image",
  "data": "base64-encoded-image-data",
  "mimeType": "image/png",
  "uri": "file:///path/to/image.png",
  "annotations": {}
}
```

**Required**: `type`, `data`, `mimeType`
**Optional**: `uri`, `annotations`

---

### 3. Audio (Requires `audio` capability)

```json
{
  "type": "audio",
  "data": "base64-encoded-audio-data",
  "mimeType": "audio/wav",
  "annotations": {}
}
```

**Required**: `type`, `data`, `mimeType`
**Optional**: `annotations`

---

### 4. Resource Link (REQUIRED - All agents MUST support)

```json
{
  "type": "resource_link",
  "name": "config.json",
  "uri": "file:///home/user/project/config.json",
  "title": "Configuration File",
  "description": "Main configuration",
  "mimeType": "application/json",
  "size": 1024,
  "annotations": {}
}
```

**Required**: `type`, `name`, `uri`
**Optional**: `title`, `description`, `mimeType`, `size`, `annotations`

---

### 5. Embedded Resource (Requires `embeddedContext` capability)

```json
{
  "type": "resource",
  "resource": {
    "uri": "file:///home/user/project/config.json",
    "text": "{\n  \"key\": \"value\"\n}",
    "mimeType": "application/json"
  },
  "annotations": {}
}
```

**Text Resource Required**: `uri`, `text`
**Blob Resource Required**: `uri`, `blob`
**Optional**: `mimeType`, `annotations`

---

## StopReason Enum

Why an agent stops processing a prompt turn:

| Value | Description | When to Return |
|-------|-------------|----------------|
| `end_turn` | Normal completion | LLM finished without more tool calls |
| `max_tokens` | Token limit reached | Hit token limit |
| `max_turn_requests` | Too many model requests | Exceeded turn request limit |
| `refusal` | Agent refused to continue | Safety/policy violation |
| `cancelled` | Client cancelled turn | Received `session/cancel` |

### Critical: Cancellation Handling

When receiving `session/cancel`:

1. Abort all LLM requests and tool invocations **immediately**
2. Send any pending `session/update` notifications
3. **MUST return `cancelled` stop reason** (not an error)
4. Catch and handle API exceptions - don't propagate as errors

**Why**: Clients display unrecognized errors to users; cancellation isn't an error.

---

## PermissionMode & Options

### Permission Request Flow

```json
{
  "method": "session/request_permission",
  "params": {
    "sessionId": "sess_abc123",
    "toolCall": {
      "toolCallId": "call_001",
      "title": "Write to config.json",
      "kind": "edit"
    },
    "options": [
      {
        "optionId": "allow-once",
        "name": "Allow once",
        "kind": "allow_once"
      },
      {
        "optionId": "allow-always",
        "name": "Allow and remember",
        "kind": "allow_always"
      },
      {
        "optionId": "reject-once",
        "name": "Reject",
        "kind": "reject_once"
      },
      {
        "optionId": "reject-always",
        "name": "Reject and remember",
        "kind": "reject_always"
      }
    ]
  }
}
```

### PermissionOptionKind Values

| Kind | Meaning | UI Suggestion |
|------|---------|---------------|
| `allow_once` | Allow this operation only | Green button |
| `allow_always` | Allow and remember | Green button with "Always" |
| `reject_once` | Reject this operation only | Red button |
| `reject_always` | Reject and remember | Red button with "Always" |

### Response Outcomes

**Selected**:

```json
{
  "outcome": {
    "outcome": "selected",
    "optionId": "allow-once"
  }
}
```

**Cancelled** (when turn is cancelled):

```json
{
  "outcome": {
    "outcome": "cancelled"
  }
}
```

**MUST**: Respond with `cancelled` outcome when client sends `session/cancel`

---

## SessionMode Capabilities

### Session Mode Structure

```json
{
  "modes": {
    "currentModeId": "ask",
    "availableModes": [
      {
        "id": "ask",
        "name": "Ask",
        "description": "Request permission before changes"
      },
      {
        "id": "code",
        "name": "Code",
        "description": "Full tool access without prompts"
      }
    ]
  }
}
```

### Mode Switching

**From Client** (via `session/set_mode`):

```json
{
  "method": "session/set_mode",
  "params": {
    "sessionId": "sess_abc123",
    "modeId": "code"
  }
}
```

**From Agent** (via `current_mode_update`):

```json
{
  "method": "session/update",
  "params": {
    "sessionId": "sess_abc123",
    "update": {
      "sessionUpdate": "current_mode_update",
      "currentModeId": "code"
    }
  }
}
```

### Common Mode Patterns

- **Ask Mode**: Request permission for every operation
- **Code Mode**: Auto-approve most operations
- **Architect Mode**: Planning only, no implementation

---

## MCP Server Integration

### Required Support: Stdio Transport

**All agents MUST support**:

```json
{
  "name": "filesystem",
  "command": "/path/to/mcp-server",
  "args": ["--stdio"],
  "env": [
    {
      "name": "API_KEY",
      "value": "secret123"
    }
  ]
}
```

**Fields**:

- `name` (string, required): Human-readable identifier
- `command` (string, required): Absolute path to executable
- `args` (string[], required): Command arguments
- `env` (EnvVariable[], required): Environment variables

---

### Optional: HTTP Transport

**Requires `mcpCapabilities.http: true`**:

```json
{
  "type": "http",
  "name": "api-server",
  "url": "https://api.example.com/mcp",
  "headers": [
    {
      "name": "Authorization",
      "value": "Bearer token123"
    }
  ]
}
```

**Fields**:

- `type` (string, required): Must be `"http"`
- `name` (string, required): Server identifier
- `url` (string, required): Server URL
- `headers` (HttpHeader[], required): HTTP headers

---

### Optional: SSE Transport (Deprecated)

**Requires `mcpCapabilities.sse: true`**:

```json
{
  "type": "sse",
  "name": "event-stream",
  "url": "https://events.example.com/mcp",
  "headers": []
}
```

**Note**: SSE transport is deprecated by MCP spec. New implementations should focus on HTTP.

---

## Terminal Management Protocol

### Capability Check

**MUST check** `clientCapabilities.terminal: true` before using any `terminal/*` methods.

---

### 1. Create Terminal (`terminal/create`)

```json
{
  "method": "terminal/create",
  "params": {
    "sessionId": "sess_abc123",
    "command": "npm",
    "args": ["test", "--coverage"],
    "env": [
      {
        "name": "NODE_ENV",
        "value": "test"
      }
    ],
    "cwd": "/home/user/project",
    "outputByteLimit": 1048576
  }
}
```

**Response**:

```json
{
  "result": {
    "terminalId": "term_xyz789"
  }
}
```

**Fields**:

- `sessionId` (SessionId, required)
- `command` (string, required): Command to execute
- `args` (string[], optional): Command arguments
- `env` (EnvVariable[], optional): Environment variables
- `cwd` (string, optional): Working directory (absolute path)
- `outputByteLimit` (uint64, optional): Max bytes to retain

---

### 2. Get Output (`terminal/output`)

```json
{
  "method": "terminal/output",
  "params": {
    "sessionId": "sess_abc123",
    "terminalId": "term_xyz789"
  }
}
```

**Response**:

```json
{
  "result": {
    "output": "Running tests...\n✓ All tests passed",
    "truncated": false,
    "exitStatus": {
      "exitCode": 0,
      "signal": null
    }
  }
}
```

**Fields**:

- `output` (string, required): Current output
- `truncated` (boolean, required): Whether truncated
- `exitStatus` (optional): Present only if exited
    - `exitCode` (uint32 | null): Process exit code
    - `signal` (string | null): Termination signal

---

### 3. Wait for Exit (`terminal/wait_for_exit`)

**Blocks until command completes**:

```json
{
  "method": "terminal/wait_for_exit",
  "params": {
    "sessionId": "sess_abc123",
    "terminalId": "term_xyz789"
  }
}
```

**Response**:

```json
{
  "result": {
    "exitCode": 0,
    "signal": null
  }
}
```

---

### 4. Kill Command (`terminal/kill`)

**Terminates command without releasing terminal**:

```json
{
  "method": "terminal/kill",
  "params": {
    "sessionId": "sess_abc123",
    "terminalId": "term_xyz789"
  }
}
```

**Use Case**: Implement timeouts - kill command, then get final output

---

### 5. Release Terminal (`terminal/release`)

**MUST call when done**:

```json
{
  "method": "terminal/release",
  "params": {
    "sessionId": "sess_abc123",
    "terminalId": "term_xyz789"
  }
}
```

**Behavior**: Kills command if still running, invalidates terminal ID

---

### Embedding Terminals in Tool Calls

```json
{
  "sessionUpdate": "tool_call",
  "toolCallId": "call_002",
  "title": "Running tests",
  "kind": "execute",
  "content": [
    {
      "type": "terminal",
      "terminalId": "term_xyz789"
    }
  ]
}
```

**Client behavior**: Display live output, continue displaying after release

---

## File System Operations

### Capability Check

Must check `clientCapabilities.fs` fields before using.

---

### 1. Read Text File (`fs/read_text_file`)

**Requires `fs.readTextFile: true`**:

```json
{
  "method": "fs/read_text_file",
  "params": {
    "sessionId": "sess_abc123",
    "path": "/home/user/project/config.json",
    "line": 1,
    "limit": 100
  }
}
```

**Response**:

```json
{
  "result": {
    "content": "{\n  \"key\": \"value\"\n}"
  }
}
```

**Fields**:

- `sessionId` (SessionId, required)
- `path` (string, required): Absolute path
- `line` (uint32, optional): Start line (1-based)
- `limit` (uint32, optional): Max lines to read

---

### 2. Write Text File (`fs/write_text_file`)

**Requires `fs.writeTextFile: true`**:

```json
{
  "method": "fs/write_text_file",
  "params": {
    "sessionId": "sess_abc123",
    "path": "/home/user/project/config.json",
    "content": "{\n  \"key\": \"new_value\"\n}"
  }
}
```

**Response**:

```json
{
  "result": {}
}
```

**Fields**:

- `sessionId` (SessionId, required)
- `path` (string, required): Absolute path
- `content` (string, required): File content to write

---

## Agent Methods (Required & Optional)

### Required Methods

| Method | Purpose | Returns |
|--------|---------|---------|
| `initialize` | Negotiate protocol & capabilities | `InitializeResponse` |
| `session/new` | Create new session | `sessionId` |
| `session/prompt` | Process user prompt | `stopReason` |

---

### Optional Methods

| Method | Purpose | Capability Required |
|--------|---------|---------------------|
| `authenticate` | Authenticate client | `authMethods` non-empty |
| `session/load` | Load existing session | `loadSession: true` |
| `session/set_mode` | Change agent mode | `modes` returned |
| `session/set_model` | Select model (UNSTABLE) | `models` returned |

---

### Notifications (Agent Receives)

| Notification | Purpose | Response Expected |
|--------------|---------|-------------------|
| `session/cancel` | Cancel ongoing turn | No (notification) |

---

## Client Methods (Required & Optional)

### Required Methods

| Method | Purpose | Returns |
|--------|---------|---------|
| `session/request_permission` | Get user authorization | `RequestPermissionOutcome` |

---

### Optional Methods

| Method | Purpose | Capability Required |
|--------|---------|---------------------|
| `fs/read_text_file` | Read file | `fs.readTextFile: true` |
| `fs/write_text_file` | Write file | `fs.writeTextFile: true` |
| `terminal/create` | Execute command | `terminal: true` |
| `terminal/output` | Get terminal output | `terminal: true` |
| `terminal/wait_for_exit` | Wait for completion | `terminal: true` |
| `terminal/kill` | Kill command | `terminal: true` |
| `terminal/release` | Release terminal | `terminal: true` |

---

### Notifications (Client Receives)

| Notification | Purpose | Response Expected |
|--------------|---------|-------------------|
| `session/update` | Session progress updates | No (notification) |

---

## Validation Rules & Constraints

### Path Validation

- **MUST be absolute paths** (e.g., `/home/user/project/file.txt`)
- **NOT relative** (e.g., `../file.txt` is invalid)
- Applied to: `cwd`, file paths in all operations

---

### Line Numbers

- **1-based indexing** (not 0-based)
- Applied to: `fs/read_text_file.line`, `ToolCallLocation.line`

---

### Protocol Version

- **MUST be integer** (e.g., `1`)
- **NOT string** (e.g., `"1"` is invalid)
- Version negotiation: client sends highest supported, agent responds with chosen version

---

### Session IDs

- **Type**: String
- **Example**: `"sess_abc123def456"`
- **Usage**: Must be included in all session-related requests

---

### Tool Call IDs

- **Type**: String
- **Example**: `"call_001"`
- **Scope**: Unique within a session
- **Usage**: Reference tool calls in updates

---

### Terminal IDs

- **Type**: String
- **Example**: `"term_xyz789"`
- **Lifecycle**: Valid from `create` until `release`

---

### Content Block Constraints

**Text & Resource Link**: REQUIRED support (all agents)

**Image**: Requires `promptCapabilities.image: true`

**Audio**: Requires `promptCapabilities.audio: true`

**Embedded Resource**: Requires `promptCapabilities.embeddedContext: true`

---

### Terminal Output Limits

- `outputByteLimit` controls max bytes retained
- **Client MUST truncate at character boundaries**
- Truncation from **beginning** of output (FIFO)
- `truncated` field indicates if truncation occurred

---

## Best Practices from Protocol

### 1. Streaming Updates

**Do**: Send frequent `session/update` notifications for real-time feedback

**Don't**: Buffer all output and send one large update

---

### 2. Tool Call Reporting

**Do**: Report tool calls immediately when LLM requests them

**Don't**: Wait until execution completes

---

### 3. Permission Requests

**Do**: Request permission for sensitive operations

**Don't**: Auto-approve everything without user control

---

### 4. Error Handling

**Do**: Catch API exceptions and return semantic stop reasons (e.g., `cancelled`)

**Don't**: Let exceptions propagate as JSON-RPC errors for non-error conditions

---

### 5. Cancellation

**Do**:

- Abort all operations immediately
- Send pending updates
- Return `cancelled` stop reason

**Don't**:

- Ignore cancellation
- Return errors instead of `cancelled`
- Leave operations running

---

### 6. Terminal Lifecycle

**Do**:

- Always call `terminal/release` when done
- Use `terminal/kill` for timeouts, then get final output

**Don't**:

- Leave terminals unreleased
- Call methods after release

---

### 7. MCP Server Connections

**Do**:

- Connect to all specified MCP servers
- Handle stdio transport (required)
- Check capabilities before using HTTP/SSE

**Don't**:

- Silently skip MCP servers
- Assume HTTP support without checking

---

### 8. Session Loading

**Do**:

- Stream entire conversation history via `session/update`
- Respond to `session/load` only after all history is streamed

**Don't**:

- Skip conversation history
- Send partial history

---

### 9. Content Types

**Do**:

- Always support text and resource links
- Check capabilities before sending images/audio

**Don't**:

- Send unsupported content types
- Assume all content types are supported

---

### 10. Stop Reasons

Use appropriate stop reasons:

| Situation | Stop Reason |
|-----------|-------------|
| Normal completion | `end_turn` |
| Out of tokens | `max_tokens` |
| Too many requests | `max_turn_requests` |
| Safety/policy | `refusal` |
| User cancelled | `cancelled` |

---

## Experimental & Unstable Features

### Session Model Selection (UNSTABLE)

**Warning**: May be removed or changed

```json
{
  "method": "session/set_model",
  "params": {
    "sessionId": "sess_abc123",
    "modelId": "model_gpt4"
  }
}
```

**Fields**:

- `models` in session responses
- `SessionModelState` type
- `ModelId` and `ModelInfo` types

**Usage**: Check documentation for current status before implementing

---

## ToolCallContent Types

Complete enumeration of content types in tool calls:

### 1. Content Block

```json
{
  "type": "content",
  "content": {
    "type": "text",
    "text": "Analysis complete"
  }
}
```

---

### 2. Diff

```json
{
  "type": "diff",
  "path": "/home/user/project/config.json",
  "oldText": "{\n  \"debug\": false\n}",
  "newText": "{\n  \"debug\": true\n}"
}
```

**Fields**:

- `type` (required): `"diff"`
- `path` (string, required): Absolute file path
- `oldText` (string | null): Original content (null for new files)
- `newText` (string, required): New content

---

### 3. Terminal

```json
{
  "type": "terminal",
  "terminalId": "term_xyz789"
}
```

**Fields**:

- `type` (required): `"terminal"`
- `terminalId` (string, required): Terminal ID from `terminal/create`

**Note**: Terminal must be added before calling `terminal/release`

---

## ToolCallLocation Structure

Used for "follow-along" features:

```json
{
  "path": "/home/user/project/src/main.py",
  "line": 42
}
```

**Fields**:

- `path` (string, required): Absolute file path
- `line` (uint32, optional): Line number (1-based)

**Purpose**: Enable clients to track which files agent is working with

---

## Complete Initialization Flow

### Client → Agent: `initialize`

```json
{
  "jsonrpc": "2.0",
  "id": 0,
  "method": "initialize",
  "params": {
    "protocolVersion": 1,
    "clientCapabilities": {
      "fs": {
        "readTextFile": true,
        "writeTextFile": true
      },
      "terminal": true
    }
  }
}
```

### Agent → Client: Response

```json
{
  "jsonrpc": "2.0",
  "id": 0,
  "result": {
    "protocolVersion": 1,
    "agentCapabilities": {
      "loadSession": true,
      "promptCapabilities": {
        "image": true,
        "audio": false,
        "embeddedContext": true
      },
      "mcpCapabilities": {
        "http": true,
        "sse": false
      }
    },
    "authMethods": []
  }
}
```

---

## Complete Session Setup Flow

### Create New Session

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "session/new",
  "params": {
    "cwd": "/home/user/project",
    "mcpServers": [
      {
        "name": "filesystem",
        "command": "/usr/local/bin/mcp-server",
        "args": ["--stdio"],
        "env": []
      }
    ]
  }
}
```

**Response**:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "sessionId": "sess_abc123def456",
    "modes": {
      "currentModeId": "ask",
      "availableModes": [
        {
          "id": "ask",
          "name": "Ask",
          "description": "Request permission before changes"
        },
        {
          "id": "code",
          "name": "Code",
          "description": "Full access"
        }
      ]
    }
  }
}
```

---

## Prompt Turn Complete Example

### 1. Client sends prompt

```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "session/prompt",
  "params": {
    "sessionId": "sess_abc123",
    "prompt": [
      {
        "type": "text",
        "text": "What's in config.json?"
      }
    ]
  }
}
```

### 2. Agent sends updates

**Agent message**:

```json
{
  "jsonrpc": "2.0",
  "method": "session/update",
  "params": {
    "sessionId": "sess_abc123",
    "update": {
      "sessionUpdate": "agent_message_chunk",
      "content": {
        "type": "text",
        "text": "Let me check the config file..."
      }
    }
  }
}
```

**Tool call**:

```json
{
  "jsonrpc": "2.0",
  "method": "session/update",
  "params": {
    "sessionId": "sess_abc123",
    "update": {
      "sessionUpdate": "tool_call",
      "toolCallId": "call_001",
      "title": "Reading config.json",
      "kind": "read",
      "status": "pending"
    }
  }
}
```

### 3. Agent requests permission

```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "session/request_permission",
  "params": {
    "sessionId": "sess_abc123",
    "toolCall": {
      "toolCallId": "call_001"
    },
    "options": [
      {
        "optionId": "allow",
        "name": "Allow",
        "kind": "allow_once"
      },
      {
        "optionId": "reject",
        "name": "Reject",
        "kind": "reject_once"
      }
    ]
  }
}
```

### 4. Client responds

```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "result": {
    "outcome": {
      "outcome": "selected",
      "optionId": "allow"
    }
  }
}
```

### 5. Agent executes tool

```json
{
  "jsonrpc": "2.0",
  "method": "session/update",
  "params": {
    "sessionId": "sess_abc123",
    "update": {
      "sessionUpdate": "tool_call_update",
      "toolCallId": "call_001",
      "status": "in_progress"
    }
  }
}
```

### 6. Agent completes tool

```json
{
  "jsonrpc": "2.0",
  "method": "session/update",
  "params": {
    "sessionId": "sess_abc123",
    "update": {
      "sessionUpdate": "tool_call_update",
      "toolCallId": "call_001",
      "status": "completed",
      "content": [
        {
          "type": "content",
          "content": {
            "type": "text",
            "text": "{\"database\": \"production\", \"debug\": false}"
          }
        }
      ]
    }
  }
}
```

### 7. Agent responds with final message

```json
{
  "jsonrpc": "2.0",
  "method": "session/update",
  "params": {
    "sessionId": "sess_abc123",
    "update": {
      "sessionUpdate": "agent_message_chunk",
      "content": {
        "type": "text",
        "text": "The config file contains database and debug settings."
      }
    }
  }
}
```

### 8. Agent completes turn

```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": {
    "stopReason": "end_turn"
  }
}
```

---

## Summary: Critical Implementation Requirements

### MUST Support (Required)

1. ✅ Protocol version 1 (integer)
2. ✅ `initialize`, `session/new`, `session/prompt` methods
3. ✅ Text and resource link content blocks
4. ✅ MCP stdio transport
5. ✅ Absolute file paths
6. ✅ 1-based line numbers
7. ✅ All 5 stop reasons
8. ✅ Cancellation with `cancelled` stop reason
9. ✅ Permission request flow
10. ✅ Session update notifications
11. ✅ HTTP MCP transport (for modern MCP servers)
12. ✅ Terminal operations (if client supports)
13. ✅ File system operations (if client supports)
14. ✅ Session modes
15. ✅ Agent plans
16. ✅ Tool call reporting with all ToolKind values
17. ✅ Real-time streaming updates

### SHOULD Support (Optional)

1. ✅ Session loading (`loadSession` capability)
2. ✅ Image content (`image` capability)
3. ✅ Audio content (`audio` capability)
4. ✅ Embedded resources (`embeddedContext` capability)
5. ✅ SSE MCP transport (deprecated, low priority)
6. ✅ Authentication methods
7. ✅ Slash commands

---

## Appendix: JSON-RPC Error Codes

Standard error codes from JSON-RPC 2.0 spec:

| Code | Meaning | When to Use |
|------|---------|-------------|
| -32700 | Parse error | Invalid JSON received |
| -32600 | Invalid Request | JSON-RPC structure invalid |
| -32601 | Method not found | Unknown method |
| -32602 | Invalid params | Parameter validation failed |
| -32603 | Internal error | Server error |

**Custom Errors**: Can use codes outside standard range

---

## References

- **Protocol Docs**: https://agentclientprotocol.com/protocol
- **Schema**: https://agentclientprotocol.com/protocol/schema
- **JSON-RPC 2.0**: https://www.jsonrpc.org/specification
- **Rust Library**: `agent-client-protocol = "0.4.3"`
- **TypeScript Library**: `@zed-industries/agent-client-protocol`

---

**Document End**
