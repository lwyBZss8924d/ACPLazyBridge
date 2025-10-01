```yaml
worktree: /Users/arthur/dev-space/acplb-worktrees/040-codex-protocol-alignment-mvp
feature_branch: feature/040-codex-protocol-alignment-mvp
date: 2025-10-01T04:22:17Z
created: 2025-09-30T15:35:21Z
last_updated: 2025-10-01T04:22:17Z
status: validated
issue_uri: https://github.com/lwyBZss8924d/ACPLazyBridge/issues/52
specs:
    constitution: "1.0.1"
    type: quickstart
    feature_number: 040
```

---

# Quickstart: Manual Validation for Codex Protocol Alignment MVP

## Purpose

This quickstart provides step-by-step manual validation procedures to verify the Codex Protocol Alignment MVP implementation. All tests should be performed in Zed IDE using Codex CLI as the agent.

---

## Prerequisites

### Required Software

- ✅ **Zed IDE** - Latest stable release with External Agents support
- ✅ **Codex CLI** - OpenAI Codex CLI with proto mode
- ✅ **codex-cli-acp** - Built from this feature branch
- ✅ **Rust toolchain** - For building the adapter

### Environment Setup

```bash
# 1. Build the codex-cli-acp adapter
cd /Users/arthur/dev-space/acplb-worktrees/040-codex-protocol-alignment-mvp
cargo build --release -p codex-cli-acp

# 2. Verify binary exists
ls -lh target/release/codex-cli-acp
ls -lh target/release/acp_mcp_server

# 3. Configure Zed to use the adapter
# Edit ~/.config/zed/settings.json:
{
  "external_agents": {
    "codex": {
      "command": "/path/to/codex-cli-acp",
      "args": []
    }
  }
}

# 4. Verify Codex CLI is installed
codex --version

# 5. Create test workspace
mkdir -p /tmp/codex-mvp-test
cd /tmp/codex-mvp-test
git init
```

---

## Test Suite 1: MCP Bridge Lifecycle

### Test 1.1: Bridge Initialization

**Goal**: Verify McpBridge spawns correctly and listens on random port

**Steps**:

1. Open Zed IDE
2. Open folder: `/tmp/codex-mvp-test`
3. Activate Codex agent (Cmd+K or external agent panel)
4. Send message: "Hello, are you ready?"
5. Check codex-cli-acp logs (stderr)

**Expected**:

- ✅ McpBridge spawns with address like `127.0.0.1:5XXXX`
- ✅ acp_mcp_server binary spawns successfully
- ✅ Codex CLI connects to bridge
- ✅ Agent responds normally

**Log Evidence**:

```log
[INFO] McpBridge starting on 127.0.0.1:0
[INFO] McpBridge listening on 127.0.0.1:54321
[INFO] acp_mcp_server spawned (pid: 12345)
[INFO] Codex CLI connected to bridge
```

---

### Test 1.2: Bridge Cleanup on Session End

**Goal**: Verify bridge resources are cleaned up when session ends

**Steps**:

1. In active Codex session, send: "Thank you, goodbye"
2. Close Codex chat panel
3. Check process list: `ps aux | grep acp_mcp_server`

**Expected**:

- ✅ acp_mcp_server process terminates
- ✅ TCP port is released
- ✅ No zombie processes

---

## Test Suite 2: MCP Filesystem Tools

### Test 2.1: read_text_file

**Goal**: Verify file reading through MCP → ACP bridge

**Steps**:

(1). Create test file:

```bash
echo -e "line 1\nline 2\nline 3" > /tmp/codex-mvp-test/test.txt
```

(2). In Zed Codex chat: "Read the file test.txt and tell me how many lines it has"

**Expected**:

- ✅ Codex reads file via MCP bridge
- ✅ Zed UI shows ToolCall (kind: read, status: pending)
- ✅ Zed UI shows ToolCallUpdate (status: completed)
- ✅ Agent responds: "The file has 3 lines"

**ACP Events to Verify** (check Zed logs):

```json
{"type": "toolCall", "kind": "read", "title": "read_text_file", "locations": [{"path": "/tmp/codex-mvp-test/test.txt"}]}
{"type": "toolCallUpdate", "status": "completed"}
```

---

### Test 2.2: write_text_file

**Goal**: Verify file creation/overwriting through bridge

**Steps**:

In Zed Codex chat: "Create a new file called hello.txt with the content 'Hello, World!'"

**Expected**:

- ✅ Codex creates file via MCP bridge
- ✅ Zed UI shows ToolCall (kind: write)
- ✅ File `/tmp/codex-mvp-test/hello.txt` exists with correct content
- ✅ Agent confirms creation

**Verification**:

```bash
cat /tmp/codex-mvp-test/hello.txt
# Should output: Hello, World!
```

---

### Test 2.3: edit_text_file

**Goal**: Verify single string replacement with diff preview

**Steps**:

(1). Create config file:

```bash
cat > /tmp/codex-mvp-test/config.toml <<EOF
[server]
host = "localhost"
port = 8080
EOF
```

(2). In Zed Codex chat: "Change the port from 8080 to 3000 in config.toml"

**Expected**:

- ✅ Codex proposes edit via MCP bridge
- ✅ Zed UI shows ToolCall with diff preview:

  ```diff
  - port = 8080
  + port = 3000
  ```

- ✅ If approval required, user approves
- ✅ File updated correctly

**Verification**:

```bash
grep "port = 3000" /tmp/codex-mvp-test/config.toml
# Should match
```

---

### Test 2.4: multi_edit_text_file

**Goal**: Verify multiple edits with cumulative diff

**Steps**:

In Zed Codex chat: "In config.toml, change host to '0.0.0.0' and add 'debug = true' after the port line"

**Expected**:

- ✅ Codex applies multiple edits via MCP bridge
- ✅ Zed UI shows cumulative diff:

  ```diff
  - host = "localhost"
  + host = "0.0.0.0"
    port = 3000
  + debug = true
  ```

- ✅ All edits applied correctly

---

## Test Suite 3: Event Coverage

### Test 3.1: Execution Tool Lifecycle

**Goal**: Verify ExecCommand* events map correctly

**Steps**:

1. In Zed Codex chat: "Run 'ls -la' in the current directory"

**Expected**:

- ✅ ExecCommandBegin → ToolCall (kind: execute, status: pending)
- ✅ ExecCommandStdout → ToolCallUpdate (status: in_progress, incremental output)
- ✅ ExecCommandEnd → ToolCallUpdate (status: completed, final output)
- ✅ Zed UI shows streaming output in real-time

**ACP Events**:

```json
{"type": "toolCall", "kind": "execute", "title": "ls -la"}
{"type": "toolCallUpdate", "status": "inProgress", "content": [...]}
{"type": "toolCallUpdate", "status": "completed"}
```

---

### Test 3.2: Approval Flow

**Goal**: Verify ExecApprovalRequest → approval prompt → ExecApproved

**Prerequisites**: Configure Codex with approval required for commands

**Steps**:

1. Set Zed permission mode to "ask"
2. In Zed Codex chat: "Delete the file test.txt"

**Expected**:

- ✅ ExecApprovalRequest → ToolCall (status: pending) with approval hint
- ✅ Zed shows approval UI: "Approve command: rm test.txt?"
- ✅ User clicks "Approve"
- ✅ ExecApproved → command executes
- ✅ ExecCommandEnd → ToolCallUpdate (status: completed)

---

### Test 3.3: Plan Streaming

**Goal**: Verify PlanChunk/PlanComplete events

**Steps**:

1. In Zed Codex chat: "Create a simple Node.js Express server with routes for /health and /api/users"

**Expected**:

- ✅ Codex generates plan
- ✅ PlanChunk → Plan update (incremental)
- ✅ Zed UI shows plan steps with status (pending/in_progress/completed)
- ✅ PlanComplete → final plan

**ACP Events**:

```json
{"type": "plan", "entries": [
  {"step": "Create package.json", "status": "pending"},
  {"step": "Install express", "status": "pending"},
  ...
]}
```

---

### Test 3.4: Reasoning Tracking

**Goal**: Verify ReasoningSection accumulation

**Steps**:

1. In Zed Codex chat: "Analyze the complexity of implementing a rate limiter middleware"

**Expected**:

- ✅ Codex emits reasoning sections
- ✅ ReasoningSection → AgentThoughtChunk (accumulated)
- ✅ Zed UI shows reasoning in thought bubbles
- ✅ ReasoningComplete → final reasoning

**ACP Events**:

```json
{"type": "agentThoughtChunk", "text": "First, I need to consider..."}
{"type": "agentThoughtChunk", "text": "Next, we should think about..."}
```

---

## Test Suite 4: Slash Commands

### Test 4.1: /status Command

**Steps**:

1. In active Codex session, type: "/status"

**Expected**:

- ✅ Agent responds with session info:

  ```txt
  Session: <session-id>
  Model: gpt-5-codex
  Approval: ask
  Sandbox: default
  Tools: 5 active
  ```

- ✅ Response appears as AgentMessageChunk

---

### Test 4.2: /model Command

**Steps**:

1. In active session, type: "/model"

**Expected**:

- ✅ Agent responds: "Current model: gpt-5-codex"
- ✅ Or emits CurrentModeUpdate with model info

---

### Test 4.3: /approvals Command

**Steps**:

1. In active session, type: "/approvals"

**Expected**:

- ✅ Agent responds with approval policy:

  ```txt
  Approval policy: ask
  - Commands require approval
  - File edits require approval
  ```

---

### Test 4.4: Invalid Slash Command

**Steps**:

1. In active session, type: "/unknown"

**Expected**:

- ✅ Agent responds with error and available commands:

  ```txt
  Unknown command: /unknown
  Available commands: /status, /model, /approvals, /compact, /review
  ```

---

## Test Suite 5: Integration & Edge Cases

### Test 5.1: Large File Operation

**Goal**: Verify performance with 1MB file

**Steps**:

(1). Create large file:

```bash
yes "This is line $(date +%s)" | head -n 50000 > /tmp/codex-mvp-test/large.txt
```

(2). In Zed: "Count the lines in large.txt"

**Expected**:

- ✅ Operation completes in <1 second
- ✅ Output truncated at 10KB with metadata flag
- ✅ Agent responds with line count: "50000 lines"

---

### Test 5.2: Concurrent Tool Calls

**Goal**: Verify multiple tool calls can execute in parallel

**Steps**:

In Zed: "Read both test.txt and hello.txt and compare their lengths"

**Expected**:

- ✅ Two read_text_file calls execute
- ✅ Zed shows 2 ToolCall notifications
- ✅ Both complete successfully
- ✅ Agent provides comparison

---

### Test 5.3: Bridge Recovery After Error

**Goal**: Verify bridge recovers from Codex CLI crash

**Steps**:

1. During active session, kill Codex CLI: `pkill -9 codex`
2. Try to send another message in Zed

**Expected**:

- ✅ Error reported to user: "Agent process terminated"
- ✅ Bridge resources cleaned up
- ✅ Can start new session without issues

---

## Test Suite 6: Performance Validation

### Test 6.1: Bridge Overhead Measurement

**Goal**: Verify bridge adds <5ms overhead

**Steps**:

1. Run benchmark:

```bash
cargo run --release -p codex-cli-acp -- --benchmark-mcp-bridge
```

**Expected**:

- ✅ TCP localhost: <1ms (p99)
- ✅ Tool call round-trip: <5ms (p99)
- ✅ Memory per session: <10MB

---

### Test 6.2: Session Startup Time

**Goal**: Verify session starts in <100ms

**Steps**:

1. Measure time from session/new to first prompt ready
2. Use Zed's developer tools or log timestamps

**Expected**:

- ✅ Session initialization: <100ms
- ✅ Includes bridge spawn and MCP server start

---

## Evidence Collection

### Logs to Capture

**During each test**:

1. **codex-cli-acp stderr**: `~/.codex/logs/acp-adapter.log`
2. **Zed debug logs**: `~/.local/share/zed/zed.log` (Linux) or `~/Library/Logs/Zed/zed.log` (macOS)
3. **Network traffic** (optional): `sudo tcpdump -i lo0 port 54321`

**Store evidence in**:

```tree
_artifacts/040-codex-protocol-alignment-mvp/
├── logs/
│   ├── quickstart_run_<timestamp>.log
│   ├── zed_<timestamp>.log
│   └── network_<timestamp>.pcap
├── screenshots/
│   ├── test_2.3_edit_diff.png
│   ├── test_3.2_approval_ui.png
│   └── test_3.3_plan_streaming.png
└── reports/
    └── quickstart_summary_<timestamp>.md
```

---

## Success Criteria

**Must Pass**:

- ✅ All 20+ test cases pass
- ✅ No crashes or hangs
- ✅ Performance targets met (<5ms bridge overhead, <100ms session start)
- ✅ All ACP events conform to schema

**Evidence Required**:

- ✅ Screenshots of key UI behaviors (diffs, approvals, plans)
- ✅ Log files showing correct event sequences
- ✅ Benchmark results showing performance targets met

---

## Troubleshooting

### Issue: Bridge fails to start

**Symptoms**: Error "Failed to bind to 127.0.0.1:0"

**Solutions**:

1. Check firewall settings
2. Verify no other process is using loopback
3. Check SELinux/AppArmor policies

---

### Issue: MCP server not spawning

**Symptoms**: Error "acp_mcp_server binary not found"

**Solutions**:

1. Verify binary exists: `ls target/release/acp_mcp_server`
2. Check binary permissions: `chmod +x target/release/acp_mcp_server`
3. Verify PATH includes target/release/

---

### Issue: Codex CLI not connecting to bridge

**Symptoms**: Timeout waiting for MCP connection

**Solutions**:

1. Check Codex CLI MCP client configuration
2. Verify bridge address is passed correctly to Codex spawn
3. Check network logs for connection attempts

---

### Issue: Events not appearing in Zed

**Symptoms**: Tool calls execute but no UI updates

**Solutions**:

1. Verify ACP event JSON schema compliance
2. Check Zed version supports External Agents
3. Enable Zed debug logging to see received events

---

## IMPORTANT TECHNICAL STANDARDS

- [ACP Protocol](https://github.com/zed-industries/agent-client-protocol) - ACPLazyBridge follows ACP Protocol
- [ACP JSON Schema](https://github.com/zed-industries/agent-client-protocol/blob/main/schema/schema.json) - ACPLazyBridge follows ACP JSON Schema
- **ACP Repository local path**: ~/dev-space/agent-client-protocol
- **ACP Rust Library Version**: `agent-client-protocol = "0.4.4"`

---

⚠️ _Based on SDD CONSTITUTION: `.specify/memory/constitution.md`_
⚠️ _Follow the SDD workflow implementation: `.specify/memory/lifecycle.md`_
⚠️ _Follow the SDD rules: `sdd-rules/rules/README.md`_

---
