# Codex CLI Protocol Analysis for ACP Bridge

**Date:** 2025-09-30
**Project:** ACPLazyBridge
**Purpose:** Complete analysis of Codex CLI protocol to inform ACP bridge implementation

---

## Overview

This directory contains comprehensive documentation of the Codex CLI protocol, event types, tool structures, and capabilities discovered through systematic analysis of the Codex codebase.

**Source Repositories:**

- Codex CLI: `/Users/arthur/dev-space/codex/`
- Analysis Output: `/Users/arthur/dev-space/ACPLazyBridge/_artifacts/reports/codex-protocol-analysis/`

---

## Documents

### 1. Complete Protocol Mapping

**File:** `codex-complete-protocol-mapping.md`

**Contents:**

- Protocol architecture and core entities (Codex, Session, Task, Turn)
- Complete enumeration of event types (Op, EventMsg)
- Tool call structures (shell, apply_patch, plan, MCP)
- Submission format and input types
- Session configuration options
- Slash commands and custom prompts
- Proto mode vs JSON mode
- Approval flows
- MCP server integration
- Notifications and turn completion

**Key Sections:**

1. Protocol Overview
2. Event Types and Schemas (50+ event types)
3. Tool Call Structures (shell, patch, MCP, web search)
4. Submission Format (UserInput, UserTurn)
5. Session Configuration
6. Slash Commands (14 built-in commands)
7. Custom Prompts
8. Proto Mode Implementation
9. Approval Flows
10. MCP Server Integration
11. Notifications and Turn Completion

**Use For:** Understanding Codex capabilities and protocol details

---

### 2. ACP Mapping Recommendations

**File:** `codex-to-acp-mapping-recommendations.md`

**Contents:**

- Design recommendations for ACP adapter implementation
- Session lifecycle mapping (session/new → Codex proto spawn)
- Event stream mapping (Codex events → ACP session/update)
- Tool call classification and parameter extraction
- State management requirements
- Approval flow handling
- Notification integration
- Error categorization
- Testing strategy
- Performance considerations
- Future enhancement proposals

**Key Sections:**

1. Session Lifecycle Mapping
2. Event Stream Mapping (25+ mapping rules)
3. Tool Call Classification
4. State Management
5. Approval Flow Handling
6. Notification Integration
7. Error Categorization
8. Testing Strategy
9. Performance Considerations
10. Future Enhancements
11. Reference Implementation Checklist

**Use For:** Implementing and extending the codex-cli-acp adapter

---

## Key Findings

### Protocol Differences: Codex vs ACP

| Aspect | Codex | ACP |
|--------|-------|-----|
| **Protocol Model** | Submission Queue / Event Queue | JSON-RPC 2.0 |
| **Transport** | JSONL over stdin/stdout | JSON-RPC over any transport |
| **Sessions** | Single session per process | Multiple sessions per connection |
| **Streaming** | Native SSE from model + delta events | session/update notifications |
| **Tool Execution** | Model-driven, server-side | Client-side via adapter |
| **Approvals** | Interactive mid-session requests | Permission mode set at session creation |
| **Turn Completion** | notify program + TaskComplete event | stopReason in response |

### Critical Implementation Points

1. **Permission Mapping:**
   - ACP's `permissionMode` must map to Codex `approval_policy` + `sandbox_mode`
   - No mid-session approval in ACP v0.4; must auto-approve or auto-deny

2. **Event Aggregation:**
   - Codex emits 40+ event types; ACP uses ~5 event types
   - Must aggregate: `AgentMessageDelta*` → single `AgentMessageChunk`
   - Must track: `ExecCommandBegin` → `ExecCommandEnd` as tool call lifecycle

3. **Tool Call Lifecycle:**
   - Track state: `pending` → `in_progress` → `completed`/`failed`
   - Extract parameters from Codex events for ACP format
   - Classify tool kinds: execute, read, edit, delete, search, fetch, think, other

4. **Turn Completion Detection:**
   - Primary: `notify` program with `agent-turn-complete` JSON
   - Fallback: `TaskComplete` event
   - Timeout: Idle detection (30s default)

5. **Proto Mode Requirements:**
   - Spawn: `codex proto -c approval_policy=... -c sandbox_mode=...`
   - Read: JSONL from stdout (protocol messages)
   - Write: JSONL to stdin (submissions)
   - Logs: stderr (ignored for protocol)

### Test Coverage Needed

**Scenarios to Test:**

1. ✅ Basic handshake (initialize)
2. ✅ Session creation (session/new)
3. ✅ Simple prompt → agent message → completion
4. ✅ Command execution with streaming output
5. ⚠️ Patch application (edit/delete)
6. ⚠️ MCP tool call
7. ⚠️ Error handling (command failure, timeout)
8. ✅ Turn completion via notify
9. ⚠️ Multiple sequential prompts in same session
10. ⚠️ Session cancellation (interrupt)

**Test Location:** `_artifacts/tests/protocol-baseline/`

---

## Usage Guide

### For Developers Implementing the Adapter

1. **Read First:**
   - `codex-complete-protocol-mapping.md` sections 1-3 (protocol basics)
   - `codex-to-acp-mapping-recommendations.md` sections 1-2 (lifecycle and event mapping)

2. **Reference During Implementation:**
   - Section 3 (Tool Call Structures) for parameter extraction
   - Section 4 (State Management) for session tracking
   - Section 8 (Testing Strategy) for test scenarios

3. **Extend Features:**
   - Section 10 (Future Enhancements) for roadmap ideas

### For Code Reviewers

1. **Verify Protocol Correctness:**
   - Compare implementation against Section 2 (Event Types and Schemas)
   - Check tool call mapping against Section 3 recommendations

2. **Verify State Management:**
   - Review state tracking against Section 4 requirements
   - Ensure approval handling matches Section 5 logic

3. **Verify Testing:**
   - Check test coverage against Section 8 checklist
   - Run JSONL scenarios from `_artifacts/tests/protocol-baseline/`

---

## Related Code Files

**Core Adapter:**

- `crates/codex-cli-acp/src/main.rs` - ACP server implementation
- `crates/codex-cli-acp/src/codex_proto.rs` - Event mapping
- `crates/codex-cli-acp/src/codex_agent.rs` - Agent wrapper
- `crates/codex-cli-acp/src/tool_calls.rs` - Tool classification

**Supporting Infrastructure:**

- `crates/codex-cli-acp/src/notify_source.rs` - Turn completion detection
- `crates/codex-cli-acp/src/validation.rs` - Request validation
- `crates/codex-cli-acp/bin/acplb_notify_forwarder.rs` - Notify helper

**Tests:**

- `crates/codex-cli-acp/tests/acp_integration_test.rs` - Integration tests
- `crates/codex-cli-acp/tests/jsonl_regression_test.rs` - JSONL scenarios
- `crates/codex-cli-acp/tests/notify_test.rs` - Notify system tests

**Test Scenarios:**

- `_artifacts/tests/protocol-baseline/handshake.jsonl`
- `_artifacts/tests/protocol-baseline/basic_session.jsonl`
- `_artifacts/tests/protocol-baseline/tool_calls.jsonl`

---

## Analysis Methodology

### Discovery Process

1. **Protocol Documentation Review:**
   - Read `codex-rs/docs/protocol_v1.md`
   - Read `docs/config.md` and `docs/advanced.md`

2. **Source Code Analysis:**
   - Protocol types: `codex-rs/protocol/src/protocol.rs`
   - Event types: `codex-rs/exec/src/exec_events.rs`
   - Proto mode: `codex-rs/cli/src/proto.rs`
   - Tool definitions: `codex-rs/core/src/openai_tools.rs`

3. **SDK Analysis:**
   - TypeScript SDK: `sdk/typescript/src/events.ts`
   - TypeScript SDK: `sdk/typescript/src/codex.ts`

4. **TUI Feature Discovery:**
   - Slash commands: `codex-rs/tui/src/slash_command.rs`
   - Custom prompts: `codex-rs/core/src/custom_prompts.rs`

### Tools Used

- **SemTools:** For document parsing and semantic search
- **ripgrep:** For code pattern searching
- **Read tool:** For detailed file analysis
- **Bash:** For directory traversal and file discovery

### Search Queries Used

1. "proto mode, protocol events, event types, streaming response"
2. "tool calls, built-in tools, MCP servers, slash commands"
3. "submission format, input context, @-mentions, file attachments"
4. "plan updates, TODO lists, approval flows, apply-patch"
5. "session configuration, model selection, sandbox mode, approval policy"
6. "headless mode, non-interactive execution, codex exec"

---

## Statistics

**Files Analyzed:** 20+
**Event Types Documented:** 50+
**Tool Types Documented:** 7
**Slash Commands Documented:** 14
**Configuration Options Documented:** 30+
**Test Scenarios Identified:** 10

**Documentation Size:**

- Complete Protocol Mapping: ~1000 lines
- ACP Mapping Recommendations: ~800 lines
- Total: ~1800 lines of structured documentation

---

## Next Steps

### Immediate (Implementation)

1. ✅ Complete `session/new` permission mapping
2. ⚠️ Implement tool call lifecycle tracking
3. ⚠️ Add patch application event handling
4. ⚠️ Add MCP tool call event handling
5. ✅ Integrate notify-based turn completion

### Short-term (Testing)

1. ⚠️ Create JSONL test scenarios for all tool types
2. ⚠️ Add regression tests for event mapping
3. ⚠️ Add integration tests for full session lifecycle
4. ⚠️ Measure and optimize event aggregation performance

### Long-term (Features)

1. Image attachment support (InputItem::LocalImage)
2. Review mode support with structured findings
3. Slash command forwarding
4. Custom prompt integration
5. Reasoning event forwarding (ACP extension)

---

## Contact & Maintenance

**Maintainer:** AI Engineer (claude)
**Last Updated:** 2025-09-30
**Review Cycle:** Update when Codex protocol changes or ACP spec evolves

**How to Update:**

1. Re-run semantic search on updated Codex source
2. Compare new findings against existing documentation
3. Update mapping recommendations for any protocol changes
4. Re-validate test scenarios

---

**End of README**
