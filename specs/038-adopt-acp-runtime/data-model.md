# Data Model: ACP Runtime

## Core Entities

### RuntimeServer

**Purpose**: Owns LocalSet and manages AgentSideConnection lifecycle

**Fields**:

- local_set: tokio::task::LocalSet
- agent_connection: AgentSideConnection
- session_store: Arc<RwLock<SessionStore>>
- config: RuntimeConfig

**Relationships**:

- Creates and owns AgentSideConnection
- Manages SessionStore lifecycle
- Spawns tasks on LocalSet

### SessionStore

**Purpose**: Tracks active sessions and their state

**Type**: `HashMap<SessionId, SessionState>`

**Operations**:

- insert(session_id, state)
- get(session_id) -> Option<SessionState>
- remove(session_id)
- list_active() -> Vec<SessionId>

### SessionState

**Purpose**: Holds per-session runtime data

**Fields**:

- id: SessionId
- working_dir: PathBuf
- permission_mode: PermissionMode
- process: Option<ProcessTransport>
- notify_source: Option<Box<dyn NotifySource>>
- created_at: SystemTime
- last_activity: SystemTime

**States**:

- Created: Session exists, no process
- Active: Process running
- Ending: Process terminating
- Closed: Session complete

### CodexAgent

**Purpose**: Implements agent_client_protocol::Agent trait for Codex

**Fields**:

- session_store: Arc<RwLock<SessionStore>>
- config: CodexConfig
- notification_tx: mpsc::Sender<SessionNotification>

**Trait Implementation**:

- initialize() -> Protocol version and capabilities
- new_session() -> Create session, store state
- prompt() -> Spawn Codex process, stream output
- authenticate() -> Not implemented (returns default)
- load_session() -> Not implemented (returns error)
- set_session_mode() -> Update permission mode

### PermissionMode

**Purpose**: ACP permission settings

**Enum Variants**:

- Unrestricted
- SilentDeny
- PromptAlways

**Mapping to Codex**:

- Unrestricted -> approval_policy=auto
- SilentDeny -> approval_policy=never, sandbox_mode=all
- PromptAlways -> approval_policy=always

### ProcessTransport

**Purpose**: Manages child process lifecycle

**Fields**:

- child: tokio::process::Child
- stdin: ChildStdin
- stdout: Lines<BufReader<ChildStdout>>
- stderr: ChildStderr

**Methods**:

- spawn(command, args) -> Result<Self>
- write_line(json) -> Result<()>
- read_line() -> Result<Option<String>>
- kill() -> Result<()>

### NotifySource

**Purpose**: External turn completion signals

**Trait Methods**:

- poll() -> Option<NotifyEvent>
- close()

**Implementations**:

- FileNotifySource: Polls file for JSON
- FifoNotifySource: Reads from named pipe

### NotifyEvent

**Purpose**: Turn completion signal

**Fields**:

- type: String ("agent-turn-complete")
- timestamp: Option<SystemTime>

### SessionNotification

**Purpose**: Updates sent to client during session

**Types** (from agent_client_protocol):

- AgentMessageChunk: Text output
- ToolCallUpdate: Tool execution status
- PlanUpdate: Planning information
- Error: Error conditions

### RuntimeConfig

**Purpose**: Server configuration

**Fields**:

- idle_timeout_ms: u64 (default: 1200)
- polling_interval_ms: u64 (default: 100)
- max_sessions: usize (default: 100)
- codex_binary: String (default: "codex")

## Entity Relationships

```tree
RuntimeServer
    ├── owns LocalSet
    ├── owns AgentSideConnection
    │   └── uses CodexAgent (impl Agent)
    │       └── accesses SessionStore
    └── owns SessionStore
        └── contains SessionState[]
            ├── has ProcessTransport
            ├── has NotifySource
            └── has PermissionMode
```

## Data Flow

1. **Session Creation**
   - Client calls new_session
   - CodexAgent creates SessionState
   - Stores in SessionStore
   - Returns SessionId

2. **Prompt Processing**
   - Client sends prompt
   - CodexAgent retrieves SessionState
   - Spawns ProcessTransport with Codex
   - Streams output as SessionNotifications
   - Monitors NotifySource for completion

3. **Session Cleanup**
   - On completion/cancel/error
   - Kill ProcessTransport
   - Close NotifySource
   - Remove from SessionStore

## Validation Rules

- SessionId must be unique
- Working directory must be absolute path
- Only one active process per session
- Sessions expire after inactivity timeout
- Maximum concurrent sessions enforced

## State Transitions

```text
None -> Created: new_session()
Created -> Active: prompt() with process spawn
Active -> Ending: cancel() or notify event
Active -> Ending: idle timeout
Ending -> Closed: process terminated
Closed -> None: removed from store
```
