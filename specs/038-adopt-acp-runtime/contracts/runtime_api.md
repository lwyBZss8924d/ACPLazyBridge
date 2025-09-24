# Runtime API Contracts

## Agent Trait Implementation Contract

The CodexAgent MUST implement the following methods from `agent_client_protocol::Agent`:

### initialize

**Request**:

```rust
pub struct InitializeRequest {
    pub protocol_version: Version,
    pub client_info: Option<ClientInfo>,
}
```

**Response**:

```rust
pub struct InitializeResponse {
    pub protocol_version: Version,  // MUST be V1 (integer 1)
    pub agent_capabilities: AgentCapabilities,
    pub auth_methods: Vec<AuthMethod>,  // Empty for Codex
    pub meta: Option<RawValue>,
}
```

**Contract**:

- MUST return protocol_version as integer 1
- MUST include agent_capabilities with prompt support
- auth_methods MUST be empty array
- MUST NOT fail unless protocol version unsupported

### new_session

**Request**:

```rust
pub struct NewSessionRequest {
    pub working_dir: Option<String>,  // MUST be absolute path
    pub permission_mode: Option<PermissionMode>,
    pub config: Option<RawValue>,
}
```

**Response**:

```rust
pub struct NewSessionResponse {
    pub session_id: SessionId,
    pub modes: Option<Vec<SessionMode>>,
    pub meta: Option<RawValue>,
}
```

**Contract**:

- MUST validate working_dir is absolute path
- MUST store session state with permission_mode
- MUST return unique session_id
- MUST NOT spawn process yet

### prompt

**Request**:

```rust
pub struct PromptRequest {
    pub session_id: SessionId,
    pub prompt: Vec<PromptContent>,
    pub metadata: Option<RawValue>,
}
```

**Response**:

```rust
pub struct PromptResponse {
    pub session_id: SessionId,
    pub stop_reason: StopReason,  // "end_turn" or "cancelled"
    pub metadata: Option<RawValue>,
}
```

**Contract**:

- MUST retrieve session from store
- MUST spawn Codex process with correct permissions
- MUST stream output via session_notification()
- MUST monitor notify source if configured
- MUST respect idle timeout
- MUST return appropriate stop_reason

## Runtime Lifecycle Contract

### start

**Signature**:

```rust
pub async fn start(config: RuntimeConfig) -> Result<RuntimeServer>
```

**Contract**:

- MUST create LocalSet for !Send futures
- MUST initialize AgentSideConnection
- MUST spawn notification handler task
- MUST be ready to accept requests

### stop

**Signature**:

```rust
pub async fn stop(&mut self) -> Result<()>
```

**Contract**:

- MUST cancel all active sessions
- MUST kill all child processes
- MUST clean up resources
- MUST complete within 5 seconds

## Session Management Contract

### Session States

```rust
pub enum SessionStatus {
    Created,   // Session exists, no process
    Active,    // Process running
    Ending,    // Process terminating
    Closed,    // Session complete
}
```

**State Transition Contract**:

- Created -> Active: Only via prompt()
- Active -> Ending: Via cancel(), notify, or timeout
- Ending -> Closed: After process termination
- Closed sessions MUST be removed from store

### Concurrent Session Contract

- MUST support at least 10 concurrent sessions
- MUST enforce max_sessions limit
- MUST queue or reject beyond limit
- MUST clean up expired sessions

## Process Management Contract

### spawn

**Contract**:

- MUST apply permission mode as CLI arguments
- MUST capture stdout, stderr, stdin
- MUST use line-buffered IO
- MUST handle spawn failures gracefully

### kill

**Contract**:

- MUST send SIGTERM first
- MUST send SIGKILL after 2 seconds if needed
- MUST clean up zombie processes
- MUST close all file handles

## Notification Contract

### SessionNotification Types

```rust
pub enum SessionNotification {
    AgentMessageChunk(AgentMessageChunk),
    ToolCallUpdate(ToolCallUpdate),
    PlanUpdate(PlanUpdate),
    Error(SessionError),
}
```

**Streaming Contract**:

- MUST send chunks as they arrive
- MUST preserve order
- MUST handle backpressure
- MUST NOT block on slow clients

### Notify Source Contract

**File Mode**:

- MUST poll file every 100ms
- MUST parse JSON events
- MUST handle malformed JSON

**FIFO Mode**:

- MUST block on read
- MUST reconnect if pipe breaks
- MUST timeout after 30 seconds

## Error Handling Contract

### Error Categories

```rust
pub enum ErrorCode {
    ParseError = -32700,
    InvalidRequest = -32600,
    MethodNotFound = -32601,
    InvalidParams = -32602,
    InternalError = -32603,
}
```

**Contract**:

- MUST map all errors to appropriate codes
- MUST include descriptive messages
- MUST log details to stderr
- MUST NOT expose internal state

## Performance Contract

### Message Processing

- SHOULD deliver streamed chunks with ≤150 ms end-to-end latency under local test conditions
- MUST stream without buffering entire response
- MUST flush stdout after each message

### Resource Usage

- SHOULD limit memory to <100 MB per session during interactive use
- MUST clean up within 1 second of session end
- SHOULD tolerate bursts of streaming output without dropping notifications

## Compatibility Contract

### JSONL Format

- MUST maintain exact JSONL format
- MUST preserve field names and types
- MUST support all existing methods
- MUST pass all regression tests

### Environment Variables

- MUST respect ACPLB_IDLE_TIMEOUT_MS
- MUST respect ACPLB_POLLING_INTERVAL_MS
- MUST respect ACPLB_NOTIFY_* settings
- MUST respect RUST_LOG for logging
