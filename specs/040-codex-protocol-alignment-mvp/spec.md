```yaml
worktree: /Users/arthur/dev-space/acplb-worktrees/040-codex-protocol-alignment-mvp
feature_branch: feature/040-codex-protocol-alignment-mvp
date: 2025-10-01T04:22:17Z
created: 2025-09-30T15:35:21Z
last_updated: 2025-10-01T04:22:17Z
status: validated
input: GitHub Issue #52 - Codex Protocol Alignment MVP
issue_uri: https://github.com/lwyBZss8924d/ACPLazyBridge/issues/52
spec_uri: specs/040-codex-protocol-alignment-mvp/spec.md
plan_uri: specs/040-codex-protocol-alignment-mvp/plan.md
tasks_uri: specs/040-codex-protocol-alignment-mvp/tasks.md
evidence_uris: _artifacts/040-codex-protocol-alignment-mvp/
specs:
    constitution: "1.0.1"
    type: spec
    feature_number: 040
```

---

# Feature Specification: Codex Protocol Alignment MVP

## Execution Flow (main)

```text
1. Parse user description from Input ✅
   - Issue #52: Complete Codex adapter migration for ACP clients
   - 34 hours of research complete (8,600+ lines of documentation)
2. Extract key concepts from description ✅
   - Actors: Zed IDE (ACP client), Codex CLI (agent), MCP servers
   - Actions: File operations, tool calls, approvals, slash commands
   - Data: Event streams, session state, tool lifecycle
   - Constraints: ACP protocol compliance, 100% event coverage
3. For each unclear aspect: ✅
   - No [NEEDS CLARIFICATION] - comprehensive research completed
4. Fill User Scenarios & Testing section ✅
5. Generate Functional Requirements ✅
6. Identify Key Entities ✅
7. Run Review Checklist ✅
8. Return: SUCCESS (spec ready for planning) ✅
```

---

## ⚡ Quick Guidelines

- ✅ Focus on WHAT users need and WHY
- ❌ Avoid HOW to implement (no tech stack, APIs, code structure)
- 👥 Written for business stakeholders, not developers

---

## User Scenarios & Testing

### Primary User Story

**As a** Zed IDE user leveraging Codex CLI for AI-assisted development
**I want** complete workflow support including file operations, approvals, and status commands
**So that** I can use all Codex CLI features seamlessly within my IDE without switching contexts

### Acceptance Scenarios

1. **Given** a Zed project with files to edit, **When** Codex proposes file changes, **Then** I see real-time diffs and can approve/reject each operation within Zed's UI

2. **Given** Codex needs to execute a shell command, **When** approval is required, **Then** I receive an approval prompt with command details and can approve/deny without leaving the IDE

3. **Given** I'm in an active Codex session, **When** I type `/status` as a slash command, **Then** I receive current session information (model, approval policy, sandbox mode) displayed in the chat

4. **Given** Codex wants to read a file using MCP tools, **When** the file operation executes, **Then** the operation transparently uses Zed's native file system capabilities via the bridge

5. **Given** Codex generates a plan with multiple steps, **When** the plan updates, **Then** I see structured plan entries with status (pending/in_progress/completed) and priorities in Zed's UI

6. **Given** Codex produces reasoning text, **When** reasoning sections stream, **Then** I see accumulated reasoning with proper aggregation and de-duplication

7. **Given** a long-running Codex operation, **When** Codex emits tool progress updates, **Then** I see real-time status transitions (pending → in_progress → completed/failed) with output previews

### Edge Cases

- **What happens when** the MCP bridge TCP server encounters a port conflict?
  → System automatically retries with a different OS-assigned port

- **What happens when** Codex CLI crashes mid-session?
  → Bridge cleanup occurs, session terminates gracefully, error reported to user

- **What happens when** a tool output exceeds 10KB?
  → Output is truncated with metadata flags indicating truncation and total size

- **What happens when** user submits an invalid slash command like `/unknown`?
  → System responds with available commands list and usage hints

- **What happens when** ACP client filesystem API is unavailable?
  → MCP bridge falls back to local filesystem operations with proper error handling

- **What happens when** approval flow times out?
  → Operation fails gracefully, timeout logged, user notified

---

## Requirements

### Functional Requirements

#### Event Coverage

- **FR-001**: System MUST map all 25 targeted Codex CLI `EventMsg` variants to corresponding ACP `SessionUpdate` payloads
- **FR-002**: System MUST implement 14 missing event handlers: ApprovalRequired, ApprovalResponse, ToolProgress, ToolCallUpdate (enhanced), McpServerConnected, McpToolCall, PlanChunk, PlanComplete, SlashCommandInvoked, SlashCommandResult, ReasoningSection, ReasoningComplete, ContextAdded, ContextRemoved
- **FR-003**: System MUST achieve 100% event mapping coverage (currently 44%)
- **FR-004**: System MUST correctly map tool status transitions (pending → in_progress → completed/failed)
- **FR-005**: System MUST preserve Codex metadata (tool raw I/O, stop reasons, timing)

#### File Operation Bridge

- **FR-006**: System MUST enable Codex file operations to work seamlessly with ACP client capabilities
- **FR-007**: System MUST support read, write, edit, and multi-edit operations on text files
- **FR-008**: System MUST attempt file operations through the ACP client first, falling back to direct filesystem access when unavailable
- **FR-009**: System MUST provide diff previews for multi-step edit operations before applying changes
- **FR-010**: System MUST avoid port conflicts when establishing inter-process communication
- **FR-011**: System MUST automatically clean up bridge resources when sessions terminate

#### Slash Command Support

- **FR-012**: System MUST support built-in commands: status queries, model information, approval controls, output compaction, and review workflows
- **FR-013**: System MUST display available commands to users when sessions start
- **FR-014**: System MUST recognize commands prefixed with slash (/) in user input
- **FR-015**: System MUST provide command execution results to users in real-time
- **FR-016**: System MUST guide users with helpful error messages when invalid commands are entered

#### Session Management

- **FR-017**: System MUST maintain separate tracking for client sessions and internal bridge connections
- **FR-018**: System MUST ensure all resources are released when sessions end
- **FR-019**: System MUST track and report token consumption for each session
- **FR-020**: System MUST consolidate reasoning output, removing duplicate content

#### Submission Context

- **FR-021**: System MUST capture working directory, security settings, approval policies, model selection, and reasoning preferences
- **FR-022**: System MUST make submission context visible to ACP clients
- **FR-023**: System MUST extract structured annotations and file references from user input
- **FR-024**: System MUST preserve context metadata in a format ACP clients can understand

#### Tool Lifecycle

- **FR-025**: System MUST notify clients when patch operations start and complete
- **FR-026**: System MUST notify clients when file operations start and complete
- **FR-027**: System MUST preserve original tool inputs and outputs with file locations
- **FR-028**: System MUST limit output preview sizes while indicating when truncation occurs
- **FR-029**: System MUST accurately classify tool operations by their purpose (execute, read, write, edit, delete, move, search, think, fetch, other)

#### Quality & Testing

- **FR-030**: System MUST work with multiple Codex CLI communication formats
- **FR-031**: System MUST include test fixtures covering all event types
- **FR-032**: System MUST demonstrate bridge connectivity and file operation correctness through automated tests
- **FR-033**: System MUST verify command parsing and execution through automated tests
- **FR-034**: System MUST collect test evidence and validation artifacts for review

#### Documentation

- **FR-035**: System MUST update project documentation to reflect completed work
- **FR-036**: System MUST indicate which previous work items are replaced by this feature
- **FR-037**: System MUST provide visual architecture documentation
- **FR-038**: System MUST include step-by-step validation procedures for users

### Non-Functional Requirements

- **NFR-001**: Bridge overhead MUST be less than 5ms per operation
- **NFR-002**: System MUST handle bridge process crashes with automatic cleanup
- **NFR-003**: System MUST provide clear error messages for MCP protocol incompatibilities
- **NFR-004**: System MUST never log secrets or sensitive information
- **NFR-005**: System MUST use structured logging to stderr (stdout reserved for JSON-RPC)
- **NFR-006**: System MUST comply with SDD Constitution principles (Test-First, Library-First, Integration-First)

### Key Entities

- **File Operation Bridge**: Communication channel enabling Codex to access ACP client file capabilities
    - Attributes: Connection state, available operations, lifecycle status
    - Relationships: Managed by session, handles file tool requests

- **File Operation Server**: Service exposing filesystem tools to Codex
    - Attributes: Available tools, edit staging state
    - Relationships: Communicates with ACP client, maintains edit history

- **Session Tracking**: Enhanced session state for dual-protocol coordination
    - Attributes: Client session ID, bridge session ID, reasoning accumulator, pending approvals, token counts
    - Relationships: Manages bridge lifecycle, tracks tool execution

- **Event Mapping**: Translation layer between Codex events and ACP updates
    - Attributes: Event type, payload data, timing metadata
    - Relationships: Converts Codex events to ACP session updates

- **Tool Execution Record**: Tracking state for individual tool invocations
    - Attributes: Unique ID, operation type, status, file locations, inputs/outputs, result content
    - Relationships: Created on tool start, updated until completion

- **Built-in Command**: System command available via slash syntax
    - Attributes: Command name, description, execution handler
    - Relationships: Registered at session start, triggered by user input

- **Edit Staging**: Temporary file state for multi-step modifications
    - Attributes: Target file, original content, staged changes, applied edits
    - Relationships: Enables diff generation before committing changes

---

## Review & Acceptance Checklist

### Content Quality

- [x] No implementation details (languages, frameworks, APIs)
- [x] Focused on user value and business needs
- [x] Written for non-technical stakeholders
- [x] All mandatory sections completed

### Requirement Completeness

- [x] No [NEEDS CLARIFICATION] markers remain
- [x] Requirements are testable and unambiguous
- [x] Success criteria are measurable
- [x] Scope is clearly bounded
- [x] Dependencies and assumptions identified

---

## Execution Status

- [x] User description parsed (GitHub Issue #52)
- [x] Key concepts extracted (bridge architecture, event coverage, slash commands)
- [x] Ambiguities marked (none - 34 hours of research complete)
- [x] User scenarios defined (7 scenarios + 6 edge cases)
- [x] Requirements generated (38 functional + 6 non-functional)
- [x] Entities identified (7 key entities)
- [x] Review checklist passed

---

## Dependencies and Assumptions

### Dependencies

- **Issue #44**: Core runtime adoption (completed)
- **Issue #45**: Streaming alignment and session notifications (completed)
- **ACP Rust Library**: `agent-client-protocol = "0.4.4"` (latest)
- **Codex CLI**: OpenAI Codex CLI with proto mode support
- **Research Documentation**: 34 hours of analysis in `dev-docs/_requirements/040-codex-protocol-alignment-mvp/`

### Assumptions

- Zed IDE provides complete ACP Client API implementation
- Codex CLI MCP client implementation is stable and documented
- TCP localhost connections are reliable and low-latency
- OS port assignment (bind to 0) provides conflict-free ports
- ACP protocol remains stable at v0.4.4 schema during implementation

### Out of Scope

- Adding new Codex CLI capabilities beyond current spec
- Shipping adapters for Claude or Gemini (separate tasks)
- Introducing persistence or background workers
- Implementing custom MCP servers beyond filesystem bridge
- Performance optimization beyond <5ms bridge overhead target

---

## Success Metrics

### Coverage Metrics

- Event mapping coverage: 44% → 100% (14 new events)
- Tool lifecycle completeness: Partial → Complete (all status transitions)
- Slash command support: 0 → 5 built-in commands

### Quality Metrics

- All tests pass (fmt/clippy/test)
- Bridge integration tests pass
- JSONL regression tests pass
- SDD validation passes (validate-sdd-docs, validate-metadata, check-sdd-consistency)

### Performance Metrics

- Bridge overhead: <5ms per operation
- Session startup time: <100ms including bridge spawn
- Memory overhead: <10MB for bridge infrastructure

### User Value Metrics

- Complete Codex workflow support in Zed IDE
- Zero context switching for approvals and status
- Real-time tool progress visibility
- Seamless file operation experience

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
