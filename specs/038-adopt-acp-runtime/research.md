# Research: ACP Runtime Adoption

## AgentSideConnection API Analysis

### Decision: Use AgentSideConnection with Agent trait

**Rationale**: Official implementation maintained by Zed team, provides protocol compliance
**Alternatives considered**:

- Custom JSON-RPC layer (rejected - duplicates upstream work)
- Direct RPC without abstraction (rejected - loses type safety)

### Key Findings

1. **AgentSideConnection Construction**

   ```rust
   // From examples/agent.rs
   let (conn, handle_io) = AgentSideConnection::new(
       agent_impl,     // Our Agent trait implementation
       outgoing,       // AsyncWrite (stdout)
       incoming,       // AsyncRead (stdin)
       |fut| tokio::task::spawn_local(fut)  // Spawner for !Send futures
   );
   ```

2. **LocalSet Execution Pattern**
   - All ACP futures are !Send (not thread-safe)
   - Must use tokio::task::LocalSet with spawn_local
   - Run entire server within LocalSet::run_until

3. **Agent Trait Methods Required**
   - `initialize()` - Protocol handshake
   - `authenticate()` - Optional auth
   - `new_session()` - Create session
   - `load_session()` - Resume session
   - `prompt()` - Handle user prompts
   - `set_session_mode()` - Change permissions
   - `ext_method()` - Extension methods
   - `ext_notification()` - Extension notifications

4. **Session Notification Flow**
   - Agent sends updates via `conn.session_notification()`
   - Must be called from within LocalSet context
   - Supports streaming chunks for real-time updates

## Notify Integration Pattern

### Current Implementation

- File-based: Poll file for JSON events
- FIFO-based: Blocking read from named pipe
- Event format: `{"type":"agent-turn-complete"}`

### Integration with AgentSideConnection

- Keep notify source as separate concern
- Check notify events between session notifications
- Use select! to multiplex notify and timeout

## Permission Mapping Strategy

### Current Mapping

```text
ACP PermissionMode -> Codex CLI Arguments
- Unrestricted -> approval_policy=auto
- SilentDeny -> approval_policy=never, sandbox_mode=all
- PromptAlways -> approval_policy=always
```

### Preserve in New Runtime

- Extract mapping to shared function
- Apply during process spawn
- Keep environment variable overrides

## Session State Management

### Required State

- Session ID
- Working directory
- Permission mode
- Process handle
- Notify source

### Storage Pattern

```rust
type SessionStore = Arc<RwLock<HashMap<SessionId, SessionState>>>;
```

## Error Handling

### JSON-RPC Error Codes

- -32700: Parse error
- -32600: Invalid request
- -32601: Method not found
- -32602: Invalid params
- -32603: Internal error

### Preserve Current Behavior

- Map ACP errors to JSON-RPC codes
- Log errors to stderr
- Never expose internal errors to stdout

## Testing Strategy

### Contract Tests

- Test each Agent trait method in isolation
- Use mock process for deterministic behavior
- Verify correct JSON-RPC responses

### Integration Tests

- Full session lifecycle with real agent-client-protocol
- JSONL scenario replay
- Notify event handling
- Timeout behavior

### Regression Tests

- All existing JSONL files must pass
- Output must match baseline (excluding timestamps)
- Performance must not degrade

## Migration Path

### Phase 1: Add Runtime Module

- Create acp-lazy-core::runtime module
- Implement CodexAgent trait
- Keep old code path active

### Phase 2: Wire Runtime

- Switch main.rs to use new runtime
- Run regression tests
- Fix any deviations

### Phase 3: Remove Legacy

- Delete old JSON-RPC handling
- Clean up unused code
- Update documentation

## Performance Considerations

### LocalSet Overhead

- Single-threaded execution
- Context switches for spawn_local
- Acceptable for CLI use case

### Message Processing

- Stream chunks as they arrive
- Don't buffer entire response
- Flush stdout after each message

### Resource Management

- Clean up sessions on disconnect
- Kill child processes on cancel
- Limit concurrent sessions if needed
