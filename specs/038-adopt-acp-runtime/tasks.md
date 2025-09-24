# Tasks: Adopt Official ACP Runtime

```yaml
worktree: ../acplb-worktrees/038-adopt-acp-runtime
feature_branch: feature/038-adopt-acp-runtime
created: 2025-09-23T07:23:14Z
last_updated: 2025-09-24T07:33:37Z
status: ready_for_review
input: Design documents from specs/038-adopt-acp-runtime/
spec_uri: specs/038-adopt-acp-runtime/spec.md
plan_uri: specs/038-adopt-acp-runtime/plan.md
tasks_uri: specs/038-adopt-acp-runtime/tasks.md
evidence_uris: _artifacts/038-adopt-acp-runtime/
prerequisites:
    plan: plan.md (complete)
    research: research.md (complete)
    data-model: data-model.md (complete)
specs:
    constitution: 1.0.1
    type: tasks
    feature_number: 038
commits:
    commit: # To be filled after implementation
    last_commit: # To be filled after PR
    pr: # To be filled after PR creation
    merge_date: # To be filled after merge
    merge_commit: # To be filled after merge
```

## Execution Flow (main)

```text
1. Load plan.md from feature directory
   → Found and loaded successfully
2. Load optional design documents:
   → data-model.md: Extracted entities
   → research.md: Extracted decisions
3. Generate tasks by category:
   → Setup: dependencies, project structure
   → Tests: Agent trait, session lifecycle, JSONL
   → Core: runtime module, CodexAgent
   → Integration: Wire into main.rs
   → Polish: cleanup, documentation
4. Apply task rules:
   → Different files = mark [P] for parallel
   → Same file = sequential (no [P])
   → Tests before implementation (TDD)
5. Number tasks sequentially (T001, T002...)
6. Generate dependency graph
7. Create parallel execution examples
8. Validate task completeness:
   → All Agent methods have tests? ✓
   → All entities have implementations? ✓
   → All scenarios covered? ✓
9. Return: SUCCESS (tasks ready for execution)
```

## Format: `[ID] [P?] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- Include exact file paths in descriptions

## Path Conventions

- **Single project**: `crates/*/src/`, `crates/*/tests/` at repository root
- Paths shown below for Rust workspace structure

## Phase 3.1: Setup

- [x] T001 Add agent-client-protocol dependency to the workspace `Cargo.toml`
- [x] T002 Create runtime module scaffolding in `crates/acp-lazy-core/src/runtime/mod.rs`
- [x] T003 Create test file structure in `crates/acp-lazy-core/tests/runtime_test.rs`
- [x] T004 [P] Create integration test file in `crates/codex-cli-acp/tests/acp_integration_test.rs`

## Phase 3.2: Tests First (TDD) ⚠️ MUST COMPLETE BEFORE 3.3

**CRITICAL: These tests MUST be written and MUST FAIL before ANY implementation**

- [x] T005 Contract test for `Agent::initialize` in `crates/acp-lazy-core/tests/runtime_test.rs`
- [x] T006 Contract test for `Agent::new_session` in `crates/acp-lazy-core/tests/runtime_test.rs`
- [x] T007 Contract test for `Agent::prompt` in `crates/acp-lazy-core/tests/runtime_test.rs`
- [x] T008 Contract test for `Agent::authenticate` default handling in `crates/acp-lazy-core/tests/runtime_test.rs`
- [x] T009 Contract test for `Agent::load_session` default handling in `crates/acp-lazy-core/tests/runtime_test.rs`
- [x] T010 Contract test for `Agent::set_session_mode` updates in `crates/acp-lazy-core/tests/runtime_test.rs`
- [x] T011 Contract test for `Agent::cancel` notification routing in `crates/acp-lazy-core/tests/runtime_test.rs`
- [x] T012 Contract test for `Agent::ext_method` unsupported response in `crates/acp-lazy-core/tests/runtime_test.rs`
- [x] T013 Contract test for `Agent::ext_notification` unsupported response in `crates/acp-lazy-core/tests/runtime_test.rs`
- [x] T014 Integration test for session lifecycle in `crates/codex-cli-acp/tests/acp_integration_test.rs`
- [x] T015 Integration test for notify handling in `crates/codex-cli-acp/tests/acp_integration_test.rs`
- [x] T016 Integration test for timeout behavior in `crates/codex-cli-acp/tests/acp_integration_test.rs`
- [x] T017 [P] Regression test for JSONL scenarios in `crates/codex-cli-acp/tests/jsonl_regression_test.rs`

## Phase 3.3: Core Implementation (ONLY after tests are failing)

- [x] T018 Implement `RuntimeServer` struct in `crates/acp-lazy-core/src/runtime/server.rs`
- [x] T019 Implement `SessionStore` in `crates/acp-lazy-core/src/runtime/session.rs`
- [x] T020 Define `ProviderAdapter` trait in `crates/acp-lazy-core/src/runtime/adapter.rs`
- [x] T021 Implement `CodexAgent` trait wrapper in `crates/codex-cli-acp/src/codex_agent.rs`
- [x] T022 Wire `Agent::initialize` behavior in `crates/codex-cli-acp/src/codex_agent.rs`
- [x] T023 Wire `Agent::new_session` storage in `crates/codex-cli-acp/src/codex_agent.rs`
- [x] T024 Wire `Agent::prompt` spawning and streaming in `crates/codex-cli-acp/src/codex_agent.rs`
- [x] T025 Implement session notification chunking in `crates/codex-cli-acp/src/codex_agent.rs`
- [x] T026 Implement `Agent::authenticate` default response in `crates/codex-cli-acp/src/codex_agent.rs`
- [x] T027 Implement `Agent::load_session` unsupported path in `crates/codex-cli-acp/src/codex_agent.rs`
- [x] T028 Implement `Agent::set_session_mode` updates in `crates/codex-cli-acp/src/codex_agent.rs`
- [x] T029 Implement `Agent::cancel` termination flow in `crates/codex-cli-acp/src/codex_agent.rs`
- [x] T030 Implement `Agent::ext_method` handling in `crates/codex-cli-acp/src/codex_agent.rs`
- [x] T031 Implement `Agent::ext_notification` handling in `crates/codex-cli-acp/src/codex_agent.rs`
- [x] T032 Integrate notify-source wiring in `crates/acp-lazy-core/src/runtime/server.rs`
- [x] T033 Implement idle-timeout handling in `crates/acp-lazy-core/src/runtime/server.rs`

## Phase 3.4: Integration

- [x] T034 Refactor `crates/codex-cli-acp/src/main.rs` to use `RuntimeServer` with `LocalSet`
- [x] T035 Connect `CodexAgent` to existing permission mapper in `crates/codex-cli-acp/src/main.rs`
- [x] T036 Preserve environment variable configuration in `crates/codex-cli-acp/src/main.rs`
- [x] T037 Maintain stdout/stderr discipline across `crates/codex-cli-acp/src/main.rs` and `crates/codex-cli-acp/src/lib.rs`
- [x] T038 Wire cancellation handling through `AgentSideConnection` in `crates/acp-lazy-core/src/runtime/server.rs`
- [x] T039 Add telemetry and evidence hooks in `crates/acp-lazy-core/src/runtime/server.rs`

## Phase 3.5: Polish

_Note: configure `ACPLB_EVIDENCE_PATH` before tackling T043+ to collect runtime telemetry into SDD evidence bundles._

- [x] T040 Remove legacy JSON-RPC handling code in `crates/codex-cli-acp/src/transport.rs` and related modules (legacy transport module retired during runtime migration)
- [x] T041 Update module documentation in `crates/acp-lazy-core/src/lib.rs`
- [x] T042 Add runtime README at `crates/acp-lazy-core/README.md`
- [x] T043 Run JSONL regression scenarios via `cargo test --test jsonl_regression_test` (evidence: `_artifacts/038-adopt-acp-runtime/tests/jsonl_regression_20250924T043915Z.log`)
- [x] T044 Record performance validation results in `_artifacts/038-adopt-acp-runtime/tests/perf.log` (captured via playback run at `_artifacts/038-adopt-acp-runtime/tests/perf_20250924T043947Z.log`)
- [x] T045 Update `CHANGELOG.md` with migration notes

## Phase 3.6: Pre-PR Validation

- [x] T046 Run `cargo fmt --all -- --check`
- [x] T047 Run `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [x] T048 Run `cargo test --workspace --all-features --locked` (evidence: `_artifacts/038-adopt-acp-runtime/tests/full_suite_20250924T044327Z.log`, `_artifacts/038-adopt-acp-runtime/tests/cargo_test_20250924T062749Z.log`)
- [x] T049 Run `scripts/sdd/validate-sdd-docs.sh` (evidence: `_artifacts/038-adopt-acp-runtime/tests/sdd_validate_20250924T050031Z.log`, `_artifacts/038-adopt-acp-runtime/tests/sdd_validate_20250924T063333Z.log`)
- [x] T050 Collect evidence in `_artifacts/038-adopt-acp-runtime/`

## Dependencies

- T001 blocks all other tasks (dependency needed)
- T005-T017 (tests) before T018-T033 (implementation)
- T018-T020 before T021 (CodexAgent depends on runtime types)
- T021-T031 before T034-T039 (integration consumes Agent wiring)
- T034-T039 before T040-T045 (cleanup after integration)
- All implementation tasks before T046-T050 (validation)

## Parallel Example

```text
# After T001-T004, launch test writing in parallel:
Task: "Contract test for Agent::initialize"
Task: "Contract test for Agent::prompt"
Task: "Contract test for Agent::cancel"
Task: "Integration test for session lifecycle"
Task: "Integration test for timeout behavior"
```

## Notes

- [P] tasks = different files, no dependencies
- Verify tests fail before implementing
- Commit after each task group
- Keep JSONL compatibility paramount

## Task Generation Rules

_Applied during main() execution_

1. **From Contracts**:
   - Each Agent method → contract test task
   - Each integration scenario → dedicated integration test

2. **From Data Model**:
   - Each entity → implementation task
   - RuntimeServer, SessionStore → core tasks
   - CodexAgent → adapter implementation

3. **From User Stories**:
   - JSONL regression → test suite
   - Notify handling → integration test
   - Timeout behavior → integration test

4. **Ordering**:
   - Setup → Tests → Core → Integration → Polish → Validation
   - Dependencies block parallel execution

## Validation Checklist

_GATE: Checked by main() before returning_

- [x] All Agent methods have corresponding tests
- [x] All entities have implementation tasks
- [x] All tests come before implementation
- [x] Parallel tasks truly independent
- [x] Each task specifies exact file path
- [x] No task modifies same file as another [P] task

## Pre-PR Quality Gates

### T900-T999: Pre-PR Validation [P]

- [x] T900: Run SDD document validation [P]

  ```bash
  ./scripts/sdd/validate-sdd-docs.sh
  ```

  Evidence: `_artifacts/038-adopt-acp-runtime/tests/sdd_validate_20250924T063333Z.log`

- [x] T901: Run Rust quality gates [P]

  ```bash
  cargo fmt --all -- --check && \
  cargo clippy --workspace --all-targets --all-features -- -D warnings && \
  cargo test --workspace --all-features --locked
  ```

  Evidence: `_artifacts/038-adopt-acp-runtime/tests/cargo_test_20250924T062749Z.log`

- [x] T902: Verify no unresolved markers

  ```bash
  ! grep -r "NEEDS CLARIFICATION\|PLACEHOLDER\|TODO\|FIXME" specs/038-adopt-acp-runtime/ --include="*.md"
  ```

  Evidence: `grep -r` run on 2025-09-24 (no matches; see dev log 2025-09-24T06:32:33Z)

## IMPORTANT TECHNICAL STANDARDS

- [ACP](https://github.com/zed-industries/agent-client-protocol) - "ACPLazyBridge" follow `ACP` Protocol
- [ACP JSON Schema](https://github.com/zed-industries/agent-client-protocol/blob/main/schema/schema.json) - "ACPLazyBridge" follow `ACP` JSON Schema
- **ACP Repository local path**: (~/dev-space/agent-client-protocol)

---

⚠️ _Based on SDD CONSTITUTION: `.specify/memory/constitution.md`_
⚠️ _Follow the SDD workflow implementation: `.specify/memory/lifecycle.md`_
⚠️ _Follow the SDD rules: `sdd-rules/rules/README.md`_
