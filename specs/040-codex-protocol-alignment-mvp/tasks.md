```yaml
worktree: /Users/arthur/dev-space/acplb-worktrees/040-codex-protocol-alignment-mvp
feature_branch: feature/040-codex-protocol-alignment-mvp
date: 2025-10-01T04:22:17Z
created: 2025-09-30T15:35:21Z
last_updated: 2025-10-01T04:22:17Z
status: validated
input: Design documents from specs/040-codex-protocol-alignment-mvp/
spec_uri: specs/040-codex-protocol-alignment-mvp/spec.md
plan_uri: specs/040-codex-protocol-alignment-mvp/plan.md
tasks_uri: specs/040-codex-protocol-alignment-mvp/tasks.md
evidence_uris: _artifacts/040-codex-protocol-alignment-mvp/
prerequisites:
    plan: plan.md (required)
    research: research.md
    data-model: data-model.md
    contracts: contracts/
specs:
    constitution: "1.0.1"
    type: tasks
    feature_number: 040
commits:
    commit: []
    last_commit: ""
    pr: ""
    merge_date: ""
    merge_commit: ""
```

---

# Tasks: Codex Protocol Alignment MVP

## Execution Flow (main)

```text
1. Load plan.md from feature directory ✅
   - Extracted: Rust workspace, TCP bridge + MCP server, 14 events, 5 commands
2. Load optional design documents: ✅
   - data-model.md: 8 entities extracted
   - contracts/: 4 files → 4 contract test suites
   - research.md: Decisions and rationale loaded
3. Generate tasks by category: ✅
   - Setup: workspace verification, dependencies
   - Tests: contract tests (28 cases), integration tests
   - Core: McpBridge, acp_mcp_server, event handlers, commands
   - Integration: session management, lifecycle
   - Polish: JSONL scenarios, documentation, evidence
4. Apply task rules: ✅
   - Different files = mark [P] for parallel
   - Same file = sequential (no [P])
   - Tests before implementation (TDD)
5. Number tasks sequentially (T001, T002...) ✅
6. Generate dependency graph ✅
7. Create parallel execution examples ✅
8. Validate task completeness: ✅
9. Return: SUCCESS (tasks ready for execution) ✅
```

## Format: `[ID] [P?] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- Include exact file paths in descriptions
- Tests MUST fail before implementation (RED phase)

## Path Conventions

- **Core library**: `crates/acp-lazy-core/src/`
- **Adapter**: `crates/codex-cli-acp/src/`
- **Tests**: `crates/*/tests/`
- Evidence: `_artifacts/040-codex-protocol-alignment-mvp/`

---

## Phase 3.1: Setup (Days 0-1)

- [ ] T001 Verify Cargo workspace structure and dependencies
- [ ] T002 [P] Add `agent-client-protocol = "0.4.4"` to Cargo.toml if needed
- [ ] T003 [P] Create evidence directory structure: `_artifacts/040-codex-protocol-alignment-mvp/{tests,logs,reports,jq}/`
- [ ] T004 [P] Document baseline event coverage (44% - 11/25 events) in `_artifacts/040-../reports/baseline_coverage.md`
- [ ] T005 [P] Run initial benchmarks for comparison in `_artifacts/040-../reports/baseline_performance.md`

---

## Phase 3.2: Tests First - MCP Bridge (TDD) ⚠️ MUST COMPLETE BEFORE 3.3

**CRITICAL: These tests MUST be written and MUST FAIL before ANY implementation**

### T006-T010: McpBridge Contract Tests [P]

- [ ] T006 [P] Create `crates/acp-lazy-core/tests/mcp_bridge_test.rs`
- [ ] T007 [P] Test: McpBridge::start() binds to 127.0.0.1:0 and returns address
- [ ] T008 [P] Test: McpBridge accepts TCP connection from MCP client
- [ ] T009 [P] Test: McpBridge spawns acp_mcp_server binary with correct args
- [ ] T010 [P] Test: McpBridge::shutdown() cleans up server and process

### T011-T024: MCP Tool Contract Tests [P]

**From contracts/read_text_file.md**:

- [ ] T011 [P] Create `crates/codex-cli-acp/tests/mcp_tools_read_test.rs`
- [ ] T012 [P] Test TC1: Read entire file
- [ ] T013 [P] Test TC2: Read with pagination
- [ ] T014 [P] Test TC3: File not found error
- [ ] T015 [P] Test TC4: Invalid path (relative) error

**From contracts/write_text_file.md**:

- [ ] T016 [P] Create `crates/codex-cli-acp/tests/mcp_tools_write_test.rs`
- [ ] T017 [P] Test TC1: Create new file
- [ ] T018 [P] Test TC2: Overwrite existing file
- [ ] T019 [P] Test TC3: Permission denied error

**From contracts/edit_text_file.md**:

- [ ] T020 [P] Create `crates/codex-cli-acp/tests/mcp_tools_edit_test.rs`
- [ ] T021 [P] Test TC1: Simple string replacement with diff
- [ ] T022 [P] Test TC3: String not found error
- [ ] T023 [P] Test TC4: Multiple matches (ambiguous) error

**From contracts/multi_edit_text_file.md**:

- [ ] T024 [P] Create `crates/codex-cli-acp/tests/mcp_tools_multi_edit_test.rs`
- [ ] T025 [P] Test TC1: Multiple edits in same file with cumulative diff
- [ ] T026 [P] Test TC2: Sequential edits (order matters)
- [ ] T027 [P] Test TC3: String not found in edit 2 error

### T028-T042: Event Mapping Contract Tests [P]

- [ ] T028 [P] Create `crates/codex-cli-acp/tests/event_mapping_test.rs`

**Execution Tool Lifecycle** (5 events):

- [ ] T029 [P] Test: ExecCommandBegin → ToolCall (kind: execute, status: pending)
- [ ] T030 [P] Test: ExecCommandStdout → ToolCallUpdate (status: in_progress, incremental)
- [ ] T031 [P] Test: ExecCommandStderr → ToolCallUpdate (status: in_progress, stderr)
- [ ] T032 [P] Test: ExecCommandEnd → ToolCallUpdate (status: completed, exit_code)
- [ ] T033 [P] Test: ExecApprovalRequest → ToolCall with approval hint

**Patch Application** (4 events):

- [ ] T034 [P] Test: PatchApplyBegin → ToolCall (kind: edit, diff preview)
- [ ] T035 [P] Test: PatchApplyEnd → ToolCallUpdate (status: completed)
- [ ] T036 [P] Test: PatchApprovalRequest → ToolCall with approval hint
- [ ] T037 [P] Test: PatchApproved → approval state cleared

**MCP & Other** (3 events):

- [ ] T038 [P] Test: McpToolCallBegin → ToolCall (kind: inferred)
- [ ] T039 [P] Test: McpToolCallEnd → ToolCallUpdate
- [ ] T040 [P] Test: ToolProgress → ToolCallUpdate (metadata)

**Plan & Reasoning** (2 events):

- [ ] T041 [P] Test: ReasoningSection → AgentThoughtChunk (accumulated)
- [ ] T042 [P] Test: TokenUsageUpdate → session state tracking

### T043-T048: Slash Command Tests [P]

- [ ] T043 [P] Create `crates/codex-cli-acp/tests/commands_test.rs`
- [ ] T044 [P] Test: Parse "/" prefix and extract command name
- [ ] T045 [P] Test: /status command returns session info
- [ ] T046 [P] Test: /model command emits CurrentModeUpdate
- [ ] T047 [P] Test: Invalid command returns error with available commands
- [ ] T048 [P] Test: AvailableCommandsUpdate emitted on session init

**Test Count**: 48 tests written ✅ All MUST fail (RED phase) ✅

---

## Phase 3.3: Core Implementation (ONLY after tests are failing)

### T049-T054: McpBridge Implementation

- [ ] T049 Create `crates/acp-lazy-core/src/mcp_bridge.rs` (~400 lines)
- [ ] T050 Implement McpBridge::start() - bind TCP 127.0.0.1:0, get OS port
- [ ] T051 Implement spawn_mcp_server() - launch acp_mcp_server binary
- [ ] T052 Implement accept_connection() - handle incoming MCP client
- [ ] T053 Implement McpBridge::shutdown() - cleanup TCP + process
- [ ] T054 Run T006-T010 tests → GREEN ✅

### T055-T063: acp_mcp_server Binary

- [ ] T055 Create `crates/codex-cli-acp/src/bin/acp_mcp_server.rs` (~850 lines)
- [ ] T056 Implement main() - parse args, start MCP JSON-RPC server
- [ ] T057 Implement handle_read_tool() - read_text_file with pagination
- [ ] T058 Implement handle_write_tool() - write_text_file with created flag
- [ ] T059 Implement handle_edit_tool() - edit_text_file with diff generation
- [ ] T060 Implement handle_multi_edit_tool() - multi_edit with staged edits
- [ ] T061 Implement StagedEditsManager - in-memory file versioning
- [ ] T062 Implement ACP Client API integration - read/write via ACP, fallback to tokio::fs
- [ ] T063 Run T011-T027 tests → GREEN ✅

### T064-T070: Event Coverage (14 New Events)

- [ ] T064 Update `crates/codex-cli-acp/src/codex_proto.rs` (~400 lines of changes)

**Execution Tool Lifecycle**:

- [ ] T065 Implement ExecCommandBegin handler → ToolCall mapping
- [ ] T066 Implement ExecCommandStdout/Stderr handlers → ToolCallUpdate (incremental)
- [ ] T067 Implement ExecCommandEnd handler → ToolCallUpdate (final)
- [ ] T068 Implement ExecApprovalRequest handler → store in SessionState::current_approval

**Patch Application**:

- [ ] T069 Implement PatchApplyBegin handler → ToolCall with diff
- [ ] T070 Implement PatchApplyEnd handler → ToolCallUpdate
- [ ] T071 Implement PatchApprovalRequest handler → approval tracking
- [ ] T072 Run T028-T037 tests → GREEN ✅

**MCP & Other**:

- [ ] T073 Implement McpToolCallBegin/End handlers → ToolCall + Update
- [ ] T074 Implement ToolProgress handler → ToolCallUpdate with metadata
- [ ] T075 Implement ReasoningSection handler → accumulate in SessionState
- [ ] T076 Implement TokenUsageUpdate handler → track token counts
- [ ] T077 Run T038-T042 tests → GREEN ✅

### T078-T083: Slash Commands

- [ ] T078 Create `crates/codex-cli-acp/src/commands.rs` (~300 lines)
- [ ] T079 Implement parse_slash_command() - detect "/" prefix, extract name
- [ ] T080 Implement SlashCommand registry - 5 built-in commands
- [ ] T081 Implement command handlers: /status, /model, /approvals, /compact, /review
- [ ] T082 Integrate command parser into codex_agent.rs - check user messages
- [ ] T083 Run T043-T048 tests → GREEN ✅

### T084-T088: Session Integration

- [ ] T084 Update `crates/codex-cli-acp/src/codex_agent.rs` (~250 lines of changes)
- [ ] T085 Add SessionState fields: fs_session_id, mcp_bridge, reasoning_sections, current_approval, token_usage
- [ ] T086 Integrate McpBridge lifecycle: spawn on session/new, cleanup on session end
- [ ] T087 Pass bridge address to Codex CLI spawn args: `--mcp-server=127.0.0.1:{port}`
- [ ] T088 Implement dual session tracking: log both acp_session_id and fs_session_id

### T089-T091: Tool Call Enhancements

- [ ] T089 Update `crates/codex-cli-acp/src/tool_calls.rs` (~50 lines of changes)
- [ ] T090 Add MCP tool kind categorization: read/write/edit → ToolKind
- [ ] T091 Enhance extract_shell_params for new ExecCommand events

---

## Phase 3.4: Integration & Polish

### T092-T096: JSONL Regression Scenarios [P]

- [ ] T092 [P] Create `_artifacts/tests/protocol-baseline/approval_flows.jsonl` - ExecApproval + PatchApproval sequences
- [ ] T093 [P] Create `_artifacts/tests/protocol-baseline/mcp_integration.jsonl` - Bridge + MCP tool calls
- [ ] T094 [P] Create `_artifacts/tests/protocol-baseline/slash_commands.jsonl` - Command parsing and execution
- [ ] T095 [P] Create `_artifacts/tests/protocol-baseline/reasoning_tracking.jsonl` - ReasoningSection accumulation
- [ ] T096 [P] Run all JSONL scenarios through playback binary, verify output

### T097-T100: Documentation Updates [P]

- [ ] T097 [P] Update `CLAUDE.md` - add MCP bridge section, document new modules
- [ ] T098 [P] Update `README.md` - mention Issue #52, MCP bridge architecture
- [ ] T099 [P] Update `dev-docs/_requirements/Roadmap.md` - mark Issue #52 complete, supersede #46
- [ ] T100 [P] Update 5 other requirement docs with Issue #52 references

### T101-T104: Performance & Evidence

- [ ] T101 Run bridge overhead benchmark - verify <5ms target
- [ ] T102 Run session startup benchmark - verify <100ms target
- [ ] T103 Run memory profiling - verify <10MB per session
- [ ] T104 Collect all test logs in `_artifacts/040-../logs/` with timestamps

---

## Phase 3.5: Manual Validation & Pre-PR

### T105-T110: Quickstart Manual Validation

- [ ] T105 Execute Test Suite 1: MCP Bridge Lifecycle (2 tests)
- [ ] T106 Execute Test Suite 2: MCP Filesystem Tools (4 tests)
- [ ] T107 Execute Test Suite 3: Event Coverage (4 tests)
- [ ] T108 Execute Test Suite 4: Slash Commands (4 tests)
- [ ] T109 Execute Test Suite 5: Integration & Edge Cases (3 tests)
- [ ] T110 Execute Test Suite 6: Performance Validation (2 tests)
- [ ] T111 Capture screenshots of key behaviors (diffs, approvals, plans) in `_artifacts/040-../screenshots/`

### T112-T120: Pre-PR Quality Gates [P]

- [ ] T112 [P] Run `cargo fmt --all -- --check` → PASS
- [ ] T113 [P] Run `cargo clippy --workspace --all-targets --all-features -- -D warnings` → PASS
- [ ] T114 [P] Run `cargo test --workspace --all-features --locked` → PASS (all 48+ tests)
- [ ] T115 [P] Run `ast-grep scan -c sgconfig.yml` → no new violations
- [ ] T116 [P] Run `scripts/sdd/validate-sdd-docs.sh` → PASS
- [ ] T117 [P] Run `scripts/sdd/validate-metadata.sh` → PASS
- [ ] T118 [P] Run `scripts/sdd/check-sdd-consistency.sh` → PASS
- [ ] T119 [P] Run `scripts/ci/run-local-ci.sh` → all gates PASS
- [ ] T120 Store evidence: test logs, benchmark results, validation output in `_artifacts/040-../`

### T121-T125: PR Preparation

- [ ] T121 Create PR summary from spec.md + plan.md
- [ ] T122 Link evidence artifacts in PR description
- [ ] T123 Verify all acceptance criteria met (from spec.md)
- [ ] T124 Tag reviewers, link Issue #52
- [ ] T125 After merge: run SDD consistency refresh, update CLAUDE.md if needed

---

## Dependencies

**Critical Path**:

1. Setup (T001-T005) before all tests
2. Tests (T006-T048) before implementation (T049-T091)
3. McpBridge (T049-T054) before acp_mcp_server (T055-T063)
4. acp_mcp_server (T055-T063) before event integration (T064-T077)
5. Core implementation (T049-T091) before JSONL scenarios (T092-T096)
6. All implementation before pre-PR gates (T112-T120)

**Parallel Opportunities**:

- T002-T005: All setup tasks
- T006-T010: Bridge tests
- T011-T027: MCP tool tests (4 suites in parallel)
- T028-T042: Event tests (5 categories)
- T043-T048: Command tests
- T092-T096: JSONL scenarios
- T097-T100: Documentation
- T112-T119: Pre-PR gates

---

## Parallel Example

```bash
# Week 1, Day 1: Setup + Bridge Tests
Task: "Verify Cargo workspace structure and dependencies" # T001
Task: "Add agent-client-protocol = 0.4.4 to Cargo.toml" # T002 [P]
Task: "Create evidence directory structure" # T003 [P]
Task: "Document baseline event coverage" # T004 [P]
Task: "Run initial benchmarks" # T005 [P]

# Week 1, Day 2: MCP Tool Contract Tests (parallel)
Task: "Create mcp_tools_read_test.rs with 5 test cases" # T011-T015 [P]
Task: "Create mcp_tools_write_test.rs with 3 test cases" # T016-T019 [P]
Task: "Create mcp_tools_edit_test.rs with 3 test cases" # T020-T023 [P]
Task: "Create mcp_tools_multi_edit_test.rs with 3 test cases" # T024-T027 [P]

# Week 1, Day 5: Verify RED Phase
Task: "Run all 48 tests → verify ALL FAIL (RED phase complete)" # Validation

# Week 2, Day 1: Bridge Implementation (sequential)
Task: "Implement McpBridge::start()" # T050
Task: "Implement spawn_mcp_server()" # T051
Task: "Run bridge tests → GREEN" # T054

# Week 3, Day 5: Pre-PR Gates (parallel)
Task: "Run cargo fmt check" # T112 [P]
Task: "Run cargo clippy" # T113 [P]
Task: "Run cargo test" # T114 [P]
Task: "Run ast-grep scan" # T115 [P]
Task: "Run SDD validation scripts" # T116-T118 [P]
```

---

## Notes

**TDD Discipline**:

- ⚠️ **CRITICAL**: T006-T048 MUST fail before T049-T091
- Verify RED phase completion before implementation
- Commit after each GREEN test
- Refactor with passing tests

**Evidence Trail**:

- Every test run: save log to `_artifacts/040-../logs/test_<timestamp>.log`
- Every benchmark: save results to `_artifacts/040-../reports/bench_<timestamp>.txt`
- Every validation: save output to `_artifacts/040-../reports/validation_<timestamp>.log`

**Slash Command Integration**:

- Commands execute within SessionState context
- Emit AgentMessageChunk with formatted responses
- Surface via AvailableCommandsUpdate on session init

**Bridge Lifecycle**:

- Spawn on session/new
- Pass address to Codex CLI: `--mcp-server=127.0.0.1:{port}`
- Cleanup on session end (even if Codex crashes)

---

## Task Generation Rules

_Applied during main() execution_

1. **From Contracts**: Each contract file → test suite + implementation
2. **From Data Model**: Each entity → struct + methods
3. **From Event Mappings**: Each new event → handler + test
4. **Ordering**: Setup → Tests (RED) → Implementation (GREEN) → Integration → Pre-PR

---

## Validation Checklist

_GATE: Checked by main() before returning_

- [x] All contracts have corresponding test tasks
- [x] All entities have implementation tasks
- [x] All tests come before implementation
- [x] Parallel tasks truly independent (different files)
- [x] Each task specifies exact file path or test case
- [x] No task modifies same file as another [P] task
- [x] Pre-PR quality gates included (T112-T120)

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
