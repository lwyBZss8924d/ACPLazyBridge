```yaml
worktree: /Users/arthur/dev-space/acplb-worktrees/040-codex-protocol-alignment-mvp
feature_branch: feature/040-codex-protocol-alignment-mvp
date: 2025-10-01T04:22:17Z
created: 2025-09-30T15:35:21Z
last_updated: 2025-10-01T04:22:17Z
status: validated
input: Feature specification from specs/040-codex-protocol-alignment-mvp/spec.md
spec_uri: specs/040-codex-protocol-alignment-mvp/spec.md
plan_uri: specs/040-codex-protocol-alignment-mvp/plan.md
tasks_uri: specs/040-codex-protocol-alignment-mvp/tasks.md
evidence_uris: _artifacts/040-codex-protocol-alignment-mvp/
specs:
    constitution: "1.0.1"
    type: plan
    feature_number: 040
```

---

# Implementation Plan: Codex Protocol Alignment MVP

## Execution Flow (/plan command scope)

```text
1. Load feature spec from Input path ✅
   - Loaded: specs/040-codex-protocol-alignment-mvp/spec.md
2. Fill Technical Context ✅
   - Project Type: Rust workspace (existing)
   - Structure Decision: Library + Binary (existing structure)
3. Fill Constitution Check section ✅
4. Evaluate Constitution Check section ✅
   - No violations: PASS
   - Complexity Tracking: N/A (within limits)
   - Update Progress Tracking: Initial Constitution Check PASS
5. Execute Phase 0 → research.md ✅
   - Created: 34 hours of prior research referenced
   - No NEEDS CLARIFICATION remain
6. Execute Phase 1 → contracts, data-model.md, quickstart.md ✅
   - Created: 4 MCP tool contracts
   - Created: data-model.md with 8 entities
   - Created: quickstart.md with 20+ test cases
7. Re-evaluate Constitution Check section ✅
   - No new violations: PASS
   - Update Progress Tracking: Post-Design Constitution Check PASS
8. Plan Phase 2 → Describe task generation approach ✅
9. STOP - Ready for /tasks command ✅
```

---

## Summary

Complete Codex CLI adapter migration to achieve 100% event coverage and implement MCP-to-ACP bridge architecture. This enables full Codex workflow support in ACP clients (Zed IDE) including file operations, approval flows, slash commands, and real-time streaming updates.

**Key Components**:

- MCP Bridge infrastructure (~1,250 lines: TCP server + MCP binary)
- Event coverage completion (~650 lines: 14 new events + commands)
- Session management enhancement (~250 lines: dual tracking + lifecycle)

**Total Scope**: ~2,250 lines new/modified code + ~500 lines tests + comprehensive documentation

---

## Technical Context

**Language/Version**: Rust 1.89+ (existing workspace)

**Primary Dependencies**:

- `agent-client-protocol = "0.4.4"` (ACP Rust library)
- `tokio = "1.0"` (async runtime)
- `serde = "1.0"` + `serde_json` (serialization)
- `anyhow` + `thiserror` (error handling)
- `tracing` (structured logging)
- `uuid` (session IDs)

**Storage**: N/A (stateless session management)

**Testing**: `cargo test` (unit + integration), JSONL regression scenarios

**Target Platform**: Linux/macOS (stdio-based ACP protocol)

**Performance Goals**:

- Bridge overhead: <5ms per operation
- Session startup: <100ms
- Memory per session: <10MB

**Constraints**:

- stdout reserved for JSON-RPC (all logs to stderr)
- No breaking changes to existing ACP surface
- Must work with Codex CLI (unmodified)
- Real-time streaming required

**Scale/Scope**:

- 25 events (11 existing + 14 new)
- 4 MCP filesystem tools
- 5 built-in slash commands
- Target: Production-ready MVP for Milestone 0.1.0

---

## Constitution Check

**Simplicity**:

- Projects: 2 (core library + adapter binary) (max 3) ✅
- Using framework directly? Yes (ACP types, no wrappers) ✅
- Single data model? Yes (SessionState, no DTOs) ✅
- Avoiding patterns? Yes (no Repository/UoW, direct implementations) ✅

**Architecture**:

- EVERY feature as library? ✅
    - McpBridge: library in `acp-lazy-core/src/mcp_bridge.rs`
    - acp_mcp_server: binary in `codex-cli-acp/src/bin/acp_mcp_server.rs`
- Libraries listed:
  1. `acp-lazy-core`: MCP bridge infrastructure
  2. `codex-cli-acp`: Adapter + event mapping + tools
- CLI per library: ✅
    - `codex-cli-acp` binary: ACP agent entry point
    - `acp_mcp_server` binary: MCP filesystem server
- Library docs: Yes (inline docs + CLAUDE.md updates planned) ✅

**Testing (NON-NEGOTIABLE)**:

- RED-GREEN-Refactor cycle enforced? Yes (contracts → tests → impl) ✅
- Git commits show tests before implementation? Yes (will verify) ✅
- Order: Contract→Integration→E2E→Unit strictly followed? Yes ✅
- Real dependencies used? Yes (real Codex CLI subprocess, real ACP client API) ✅
- Integration tests for: new libraries (McpBridge), contract changes (4 MCP tools), shared schemas (SessionState) ✅
- FORBIDDEN: Implementation before test, skipping RED phase ⛔

**Observability**:

- Structured logging included? Yes (`tracing` to stderr) ✅
- Frontend logs → backend? N/A (server-side only) ✅
- Error context sufficient? Yes (anyhow error chains) ✅

**Versioning**:

- Version number assigned? Yes (v0.4.0 targeting ACP 0.4.4) ✅
- BUILD increments on every change? Yes (Cargo.toml bumped per commit) ✅
- Breaking changes handled? N/A (no public API changes) ✅

---

## Project Structure

### Documentation (this feature)

```tree
specs/040-codex-protocol-alignment-mvp/
├── plan.md              # This file (/plan command output)
├── spec.md              # Phase 0: Feature specification
├── research.md          # Phase 0: 34 hours of research (complete)
├── data-model.md        # Phase 1: 8 entities + event mappings
├── quickstart.md        # Phase 1: Manual validation (20+ tests)
├── contracts/           # Phase 1: MCP tool contracts
│   ├── read_text_file.md
│   ├── write_text_file.md
│   ├── edit_text_file.md
│   └── multi_edit_text_file.md
└── tasks.md             # Phase 2: /tasks command output (NOT created by /plan)
```

### Source Code (repository root)

**Existing Structure** (Rust workspace):

```tree
crates/
├── acp-lazy-core/              # Shared library
│   ├── src/
│   │   ├── lib.rs
│   │   ├── permissions.rs
│   │   ├── protocol.rs
│   │   ├── runtime/
│   │   │   ├── adapter.rs
│   │   │   ├── mod.rs
│   │   │   ├── server.rs
│   │   │   └── session.rs
│   │   ├── transport.rs
│   │   └── mcp_bridge.rs       # NEW (Phase 1)
│   └── tests/
│       ├── runtime_test.rs
│       └── mcp_bridge_test.rs  # NEW (Phase 1)
└── codex-cli-acp/              # Adapter binary
    ├── src/
    │   ├── main.rs
    │   ├── codex_agent.rs      # MODIFIED (bridge integration)
    │   ├── codex_proto.rs      # MODIFIED (14 new events)
    │   ├── tool_calls.rs       # MODIFIED (MCP tools)
    │   ├── commands.rs         # NEW (slash commands)
    │   ├── validation.rs
    │   └── bin/
    │       ├── acplb_notify_forwarder.rs
    │       ├── playback.rs
    │       └── acp_mcp_server.rs  # NEW (MCP server binary)
    └── tests/
        ├── acp_integration_test.rs
        ├── mcp_bridge_test.rs     # NEW
        ├── commands_test.rs       # NEW
        └── event_mapping_test.rs  # NEW
```

**Structure Decision**: Use existing Rust workspace structure (2 projects: core library + adapter)

---

## Phase 0: Outline & Research

**Research Status**: ✅ COMPLETE

**Research Investment**: 34 hours, 8,600+ lines of documentation

**Key Research Documents**:

1. `dev-docs/_requirements/040-codex-protocol-alignment-mvp/issue-50-research-report.md` (3,500 lines)
2. `dev-docs/_requirements/040-codex-protocol-alignment-mvp/issue-50-gap-analysis.md` (2,500 lines)
3. `dev-docs/_requirements/040-codex-protocol-alignment-mvp/acp-protocol-complete-mapping.md` (800 lines)
4. `dev-docs/_requirements/040-codex-protocol-alignment-mvp/codex-protocol-analysis/` (1,800+ lines)

**Research Consolidated in**: `specs/040-codex-protocol-alignment-mvp/research.md`

**Key Decisions Made**:

- ✅ TCP MCP Bridge architecture (validated via codex-acp reference impl)
- ✅ 4 MCP tools required (read/write/edit/multi_edit)
- ✅ Staged edits pattern for diff generation
- ✅ Dual session tracking (ACP + bridge)
- ✅ 5 built-in slash commands
- ✅ Layered testing strategy (4 layers)

**No NEEDS CLARIFICATION Remain**: All research questions answered with documented decisions

---

## Phase 1: Design & Contracts

**Status**: ✅ COMPLETE

### Design Documents Created

1. **data-model.md**: 8 core entities with validation rules
   - McpBridge (TCP server lifecycle)
   - McpServerProcess (acp_mcp_server binary)
   - SessionState (enhanced for dual tracking)
   - ToolCallState (lifecycle + metadata)
   - StagedEditsManager (in-memory versioning)
   - CodexEvent mappings (25 events)
   - AskForApproval (pending approval tracking)
   - SlashCommand (built-in command definitions)

2. **contracts/**: 4 MCP tool contracts with test cases
   - `read_text_file.md`: 6 test cases
   - `write_text_file.md`: 8 test cases
   - `edit_text_file.md`: 7 test cases
   - `multi_edit_text_file.md`: 7 test cases
   - Total: 28 contract test cases

3. **quickstart.md**: Manual validation guide
   - 6 test suites (20+ individual tests)
   - Evidence collection procedures
   - Performance validation steps
   - Troubleshooting guide

### Contract Tests Generated

**Test Files to Create** (Phase 2):

```rust
// crates/acp-lazy-core/tests/mcp_bridge_test.rs
#[test]
async fn test_mcp_bridge_lifecycle() {
    // MUST FAIL initially
    let bridge = McpBridge::start().await?;
    assert!(bridge.is_listening());
    // ...
}

// crates/codex-cli-acp/tests/mcp_bridge_test.rs
#[test]
async fn test_read_tool_contract() {
    // MUST FAIL initially (from contracts/read_text_file.md TC1)
    let result = handle_read_tool(params).await?;
    assert_eq!(result.content, "Hello\nWorld\n");
    // ...
}

// 28 total contract tests from contracts/*.md
```

### Agent Context Update

**CLAUDE.md Updates Planned**:

- Add MCP bridge architecture section
- Document new modules (mcp_bridge.rs, commands.rs, acp_mcp_server.rs)
- Update event coverage (44% → 100%)
- Add slash command reference

**Update Strategy**: Incremental O(1) operation via `scripts/sdd/update-agent-context.sh claude`

---

## Phase 2: Task Planning Approach

_This section describes what the /tasks command will do - DO NOT execute during /plan_

**Task Generation Strategy**:

1. **From Contracts** (contracts/*.md):
   - Each contract file → 1 contract test file creation task [P]
   - Each test case → 1 test implementation task [P]
   - Each tool → 1 implementation task (sequential)
   - Total: ~32 tasks (4 contracts + 28 tests + 4 implementations)

2. **From Data Model** (data-model.md):
   - Each entity → 1 struct definition task [P]
   - Each entity → 1 method implementation task
   - Total: ~16 tasks (8 entities × 2)

3. **From Event Mappings** (data-model.md section 6):
   - Each new event → 1 event handler task
   - Event categories → integration test tasks
   - Total: ~20 tasks (14 events + 6 integration tests)

4. **From Bridge Architecture**:
   - McpBridge TCP server → 1 implementation task
   - acp_mcp_server binary → 1 implementation task
   - Bridge lifecycle → 1 integration test task
   - Total: ~3 tasks

5. **From Slash Commands**:
   - Command parser → 1 implementation task
   - Each command handler → 1 task [P]
   - Command tests → 1 task
   - Total: ~7 tasks (1 parser + 5 commands + 1 test)

6. **Integration & Polish**:
   - Session integration → 1 task
   - JSONL regression scenarios → 1 task [P]
   - Documentation updates → 1 task [P]
   - Evidence collection → 1 task
   - Total: ~4 tasks

**Ordering Strategy**:

- TDD order: Contracts → Tests (RED) → Implementation (GREEN) → Refactor
- Dependency order: McpBridge → acp_mcp_server → event handlers → integration
- Mark [P] for parallel execution (independent files/tests)

**Estimated Output**: ~80-90 numbered, ordered tasks in tasks.md

**IMPORTANT**: This phase is executed by the /tasks command, NOT by /plan

---

## Pre-PR Validation

**SDD Document Validation**:

- [ ] Run `scripts/sdd/validate-sdd-docs.sh` to check:
    - YAML frontmatter syntax and required fields ✅
    - Document structure matches templates ✅
    - No unresolved [NEEDS CLARIFICATION] markers ✅
    - No placeholder values ✅
- [ ] Run `scripts/sdd/validate-metadata.sh` for metadata consistency
- [ ] Run `scripts/sdd/check-sdd-consistency.sh` for global consistency

**Code Quality Gates**:

- [ ] All tests pass (`cargo test --workspace`) - including 28+ contract tests
- [ ] Format check (`cargo fmt --all -- --check`)
- [ ] Lint check (`cargo clippy --workspace --all-targets -- -D warnings`)
- [ ] AST-grep scan (`ast-grep scan -c sgconfig.yml`)

**Performance Validation**:

- [ ] Bridge overhead <5ms (benchmark suite)
- [ ] Session startup <100ms (measurement)
- [ ] Memory per session <10MB (profiling)

**Evidence Collection**:

- [ ] JSONL regression tests pass (5 new scenarios)
- [ ] Quickstart manual validation complete (20+ tests)
- [ ] Logs stored in `_artifacts/040-codex-protocol-alignment-mvp/`

---

## Phase 3+: Future Implementation

_These phases are beyond the scope of the /plan command_

**Phase 3**: Task execution (/tasks command creates tasks.md)

**Phase 4**: Implementation sequence

1. Week 1: Bridge Infrastructure (McpBridge + acp_mcp_server)
2. Week 2: Event Coverage (14 new events + slash commands)
3. Week 3: Testing & Evidence (JSONL scenarios + quickstart validation)

**Phase 5**: Validation

- Run all tests (`cargo test --workspace`)
- Execute quickstart.md (20+ manual tests)
- Performance validation (benchmarks)
- Collect evidence artifacts

**Phase 6**: Pre-PR validation

- Run all SDD checks (validate-sdd-docs, validate-metadata, check-sdd-consistency)
- Run all quality gates (fmt, clippy, test, ast-grep)
- Store evidence in `_artifacts/040-codex-protocol-alignment-mvp/`

---

## Complexity Tracking

_Fill ONLY if Constitution Check has violations that must be justified_

**Status**: NO VIOLATIONS ✅

No complexity deviations to document. All constitutional principles satisfied:

- Within 3-project limit (2 projects)
- No unnecessary abstractions
- Direct framework usage
- Single model representation
- Test-first enforced

---

## Progress Tracking

**Phase Status**:

- [x] Phase 0: Research complete (/plan command)
- [x] Phase 1: Design complete (/plan command)
- [x] Phase 2: Task planning complete (/plan command - approach described)
- [ ] Phase 3: Tasks generated (/tasks command)
- [ ] Phase 4: Implementation complete
- [ ] Phase 5: Validation passed

**Gate Status**:

- [x] Initial Constitution Check: PASS
- [x] Post-Design Constitution Check: PASS
- [x] All NEEDS CLARIFICATION resolved
- [x] Complexity deviations documented (none)

**Design Artifacts**:

- [x] spec.md (feature specification)
- [x] research.md (34 hours consolidated)
- [x] data-model.md (8 entities)
- [x] contracts/ (4 tool contracts, 28 test cases)
- [x] quickstart.md (20+ manual tests)
- [x] plan.md (this file)
- [ ] tasks.md (/tasks command output)

---

## Risk Mitigation Plan

| Risk | Impact | Mitigation Status |
|------|--------|-------------------|
| TCP port conflicts | Medium | ✅ Mitigated (bind to port 0, retry logic designed) |
| Bridge crashes | High | ✅ Mitigated (health checks + cleanup patterns designed) |
| MCP protocol incompatibility | High | ✅ Mitigated (version check + error messages designed) |
| Event mapping errors | High | ✅ Mitigated (JSONL regression tests planned) |
| Performance regression | Medium | ✅ Mitigated (benchmark suite designed, targets validated in reference impl) |
| Memory leaks | High | ✅ Mitigated (RAII patterns + lifecycle tests designed) |

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
