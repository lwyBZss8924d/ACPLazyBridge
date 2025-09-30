# [Draft] Tasks: Complete Codex Protocol Alignment for MVP (Issue #50 | Task #040)

```yaml
worktree: ../acplb-worktrees/040-codex-protocol-alignment-mvp
feature_branch: feature/040-codex-protocol-alignment-mvp
created: 2025-09-29T21:24:56Z
last_updated: 2025-09-29T21:24:56Z
status: draft
input: Design documents from `specs/040-codex-protocol-alignment-mvp/`
spec_uri: specs/drafts-040-codex-protocol-alignment-mvp/spec.md
plan_uri: specs/drafts-040-codex-protocol-alignment-mvp/plan.md
tasks_uri: specs/drafts-040-codex-protocol-alignment-mvp/tasks.md
evidence_uris:
  - _artifacts/040-codex-protocol-alignment-mvp/tests/
  - _artifacts/040-codex-protocol-alignment-mvp/logs/
  - _artifacts/040-codex-protocol-alignment-mvp/reports/
prerequisites:
  plan: specs/drafts-040-codex-protocol-alignment-mvp/plan.md
  research: specs/drafts-040-codex-protocol-alignment-mvp/research.md
  data-model: specs/drafts-040-codex-protocol-alignment-mvp/data-model.md
  contracts: specs/drafts-040-codex-protocol-alignment-mvp/contracts/
specs:
  constitution: 1.0.1
  type: tasks
  feature_number: 040
commits:
  commit: TBD
  last_commit: TBD
  pr: TBD
  merge_date: TBD
  merge_commit: TBD
```

> ⚠️ Tests (RED) must be authored and run before any implementation task that satisfies them. Marked [P] tasks can run in parallel when they touch disjoint files.

## Phase 0 – Research & Context Capture

- [ ] **T040-001** Gather Codex `EventMsg` definitions and document ACP mapping targets in `specs/040-codex-protocol-alignment-mvp/research.md`.
- [ ] **T040-002 [P]** Audit ACP schema (`agent-client-protocol` v0.4.3) and record required fields for targeted `SessionUpdate` variants in `research.md`.
- [ ] **T040-003 [P]** Inventory documentation touchpoints (Roadmap, requirements, project plan, milestone index) that reference Issue #46; note update plan in `research.md`.

## Phase 1 – Design Artefacts

- [ ] **T040-010** Author `data-model.md` detailing `SessionContext` and tool lifecycle state machine.
- [ ] **T040-011 [P]** Produce mapping table (`contracts/event-mapping.md`) linking Codex events to ACP updates with validation rules.
- [ ] **T040-012 [P]** Draft regression scenario outlines in `contracts/jsonl/` (one file per scenario) capturing plan, tool, approval, MCP, slash-command, and experimental JSON cases.
- [ ] **T040-013** Write `quickstart.md` describing Codex proto vs experimental JSON workflows, notify setup, and Zed smoke validation steps.

## Phase 2 – Tests First (RED)

- [ ] **T040-020** Implement failing unit tests for submission parsing and content block conversion in `crates/codex-cli-acp/tests/submission_tests.rs`.
- [ ] **T040-021 [P]** Add failing lifecycle tests for plan and tool events in `crates/codex-cli-acp/tests/event_mapping_tests.rs`.
- [ ] **T040-022 [P]** Add failing regression harness JSONL fixtures under `_artifacts/040-codex-protocol-alignment-mvp/tests/baseline/` covering approvals, MCP, and slash commands.
- [ ] **T040-023** Extend schema validation tests ensuring serialized ACP payloads validate against `agent-client-protocol` (`crates/codex-cli-acp/tests/schema_validation.rs`).

## Phase 3 – Implementation (GREEN)

- [ ] **T040-030** Create `crates/codex-cli-acp/src/submission_handler.rs` with parsing, annotation, and session context helpers; integrate with `main.rs` / `codex_agent.rs`.
- [ ] **T040-031 [P]** Refactor `codex_proto.rs` to cover all `EventMsg` variants, routing through shared builders and emitting ACP updates.
- [ ] **T040-032 [P]** Expand `tool_calls.rs` to classify apply_patch and MCP tools, attach locations, and manage approval transitions.
- [ ] **T040-033** Implement optional experimental JSON ingestion path reusing mapping logic; gate via configuration flag.
- [ ] **T040-034** Ensure notify and stop-reason propagation includes idle timeout and manual notify events with ACP-compliant metadata.
- [ ] **T040-035 [P]** Wire slash-command listings and @-mention resource links into ACP updates (`AvailableCommandsUpdate`, content annotations).

## Phase 4 – Documentation & Evidence

- [ ] **T040-040** Replace placeholder file with full issue draft `dev-docs/_issues_drafts/open/#50-codex-protocol-alignment-mvp.md`; mark Issue #46 as superseded.
- [ ] **T040-041 [P]** Update requirements documents (`Roadmap.md`, `m1-technical-implementation-plan.md`, `m1-issue-list.md`, `acp-lazybridge-requirements.md`, `acp-lazybridge-project-plan.md`, `migration-blueprint-project-management-plan.md`) with Issue #50 scope.
- [ ] **T040-042 [P]** Refresh milestone index tables to reference Issue #50 and archive Issue #46 within `_issues_drafts/` and supporting docs.
- [ ] **T040-043** Capture manual Zed smoke test evidence and Codex CLI runs in `_artifacts/040-codex-protocol-alignment-mvp/tests/` with timestamped logs.

## Phase 5 – Verification & Cleanup

- [ ] **T040-050** Run JSONL regression suite (proto + experimental) and store outputs under `_artifacts/040-codex-protocol-alignment-mvp/tests/baseline/`.
- [ ] **T040-051 [P]** Execute schema validation scripts and archive results in `_artifacts/040-codex-protocol-alignment-mvp/reports/`.
- [ ] **T040-052 [P]** Perform Zed IDE smoke test and document findings in `quickstart.md` and `_artifacts/.../logs/`.
- [ ] **T040-053** Update changelog or migration notes if required (`dev-docs/changelogs/` or migration blueprint appendix).

## Phase 6 – Pre-PR Validation

- [ ] **T040-900 [P]** Run `scripts/sdd/validate-sdd-docs.sh`; save log to `_artifacts/040-codex-protocol-alignment-mvp/reports/sdd_docs_$(date +%Y%m%d_%H%M%S).log`.
- [ ] **T040-901 [P]** Run `scripts/sdd/validate-metadata.sh --check-consistency`; archive log alongside above.
- [ ] **T040-902 [P]** Execute Rust quality gates:

  ```bash
  cargo fmt --all -- --check && \
  cargo clippy --workspace --all-targets --all-features -- -D warnings && \
  cargo test --workspace --all-features --locked
  ```

  Store combined log in `_artifacts/040-codex-protocol-alignment-mvp/tests/quality_gates_$(date +%Y%m%d_%H%M%S).log`.
- [ ] **T040-903 [P]** Run `ast-grep scan -c sgconfig.yml --json > _artifacts/040-codex-protocol-alignment-mvp/reports/ast_grep_$(date +%Y%m%d_%H%M%S).json`.
- [ ] **T040-904** Verify no unresolved markers:

  ```bash
  ! rg "NEEDS CLARIFICATION|PLACEHOLDER|TODO|FIXME" specs/040-codex-protocol-alignment-mvp -g"*.md"
  ```

## Dependencies & Parallelism

- Phase 2 tests (T040-020–T040-023) must fail before Phase 3 implementation tasks begin.
- Documentation updates (T040-040–T040-042) depend on research/design outputs but can run alongside late implementation once scope finalized.
- Validation tasks (T040-900+) require completion of implementation and documentation updates.

## Notes

- Maintain evidence discipline: every script/test log stored under `_artifacts/040-codex-protocol-alignment-mvp/` with ISO8601 timestamps.
- Coordinate documentation edits with other milestone work to avoid merge conflicts; communicate supersession of Issue #46 clearly.
- Keep raw tool payload truncation thresholds documented in research/design artefacts for future reference.

---

⚠️ _Based on SDD CONSTITUTION: `.specify/memory/constitution.md`_
⚠️ _Follow the SDD workflow implementation: `.specify/memory/lifecycle.md`_
⚠️ _Follow the SDD rules: `sdd-rules/rules/README.md`_
