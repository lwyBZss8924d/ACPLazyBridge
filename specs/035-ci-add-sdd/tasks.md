# Tasks: CI with SDD Gates and ast-grep Scanning

```yaml
worktree: /Users/arthur/dev-space/acplb-worktrees/ci-sdd-gates
feature_branch: 035-ci-add-sdd
created: 2025-09-19
last_updated: 2025-09-19
status: in-progress
input: Design documents from `/specs/035-ci-add-sdd/`
issue_url: https://github.com/lwyBZss8924d/ACPLazyBridge/issues/32
spec_uri: specs/035-ci-add-sdd/spec.md
plan_uri: specs/035-ci-add-sdd/plan.md
tasks_uri: specs/035-ci-add-sdd/tasks.md
evidence_uris: _artifacts/035-ci-add-sdd/
prerequisites:
    plan: plan.md (required)
    research: specs/035-ci-add-sdd/research.md
    quickstart: specs/035-ci-add-sdd/quickstart.md
    transition: specs/035-ci-add-sdd/transition.md
    enforcement: specs/035-ci-add-sdd/enforcement.md
specs:
    constitution: 1.0.1
    type: tasks
    feature_number: 035
commits:
    commit: pending
    last_commit: pending
```

## Execution Flow (main)

```text
1. Load plan.md from feature directory
   → ✓ Loaded plan for CI integration
   → Tech stack: GitHub Actions, ast-grep, cargo toolchain
   → Structure: Single workflow enhancement
2. Load optional design documents:
   → research.md: ✓ Extracted CI tool decisions
   → quickstart.md: ✓ Extracted validation scenarios
3. Generate tasks by category:
   → Setup: Backup and prepare workflow
   → Tests: Validation scenarios for CI
   → Core: CI job implementations
   → Integration: SARIF upload, script integration
   → Polish: Documentation and transition prep
4. Apply task rules:
   → Different files = mark [P] for parallel
   → Same file = sequential (no [P])
   → Tests before implementation (TDD)
5. Number tasks sequentially (T001-T023)
6. Generate dependency graph
7. Create parallel execution examples
8. Validate task completeness:
   → ✓ All requirements covered
   → ✓ Report-only and enforcement modes
   → ✓ Transition strategy included
9. Return: SUCCESS (tasks ready for execution)
```

## Format: `[ID] [P?] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- Include exact file paths in descriptions

## Path Conventions

- **Workflows**: `.github/workflows/`
- **Scripts**: `scripts/ci/`, `scripts/sdd/`
- **Evidence**: `_artifacts/035-ci-add-sdd/`
- **Specs**: `specs/035-ci-add-sdd/`

## Phase 3.1: Setup

- [x] T001 Backup existing CI workflow to `.github/workflows/ci.yml.backup`
- [x] T002 [P] Create evidence directory structure at `_artifacts/035-ci-add-sdd/{tests,logs,reports}`
- [x] T003 [P] Create JSON to SARIF conversion script at `scripts/ci/json-to-sarif.jq`

## Phase 3.2: Tests First (TDD) ⚠️ MUST COMPLETE BEFORE 3.3

**CRITICAL: These tests MUST be written and MUST FAIL before ANY implementation**

- [x] T004 [P] Create workflow validation test in `_artifacts/035-ci-add-sdd/tests/validate-workflow.sh`
- [x] T005 [P] Create SARIF format validation test in `_artifacts/035-ci-add-sdd/tests/validate-sarif.sh`
- [x] T006 [P] Create report-only mode test in `_artifacts/035-ci-add-sdd/tests/test-report-only.sh`
- [x] T007 Run all validation tests and confirm they fail (capture logs in `_artifacts/035-ci-add-sdd/logs/`)

## Phase 3.3: Core Implementation (ONLY after tests are failing)

### CI Workflow Jobs

- [x] T008 Add ast-grep-scan job to `.github/workflows/ci.yml` with `continue-on-error: true`
- [x] T009 Add ast-grep installation step using taiki-e/install-action@v2 in ast-grep-scan job
- [x] T010 Add ast-grep scan execution step with JSON output in ast-grep-scan job
- [x] T011 Add SARIF conversion step using jq script in ast-grep-scan job
- [x] T012 Add SARIF upload step using `github/codeql-action/upload-sarif@v3`
- [x] T013 Add typos-check job with typos-cli installation via `taiki-e/install-action@v2`
- [x] T014 Update test-matrix job to include Windows and macOS runners
- [x] T015 Add permissions block for security-events: write to workflow

### Script Integration

- [x] T016 [P] Create enhanced SDD structure check script at `scripts/ci/run-sdd-gates.sh`
- [x] T017 Update existing SDD jobs in workflow to use enhanced validation scripts (script available)

## Phase 3.4: Integration

- [x] T018 Add cache configuration using `Swatinem/rust-cache@v2` for all Rust jobs (already present)
- [x] T019 Configure job dependencies to ensure proper execution order (jobs run independently)
- [x] T020 Add workflow annotations for clear error reporting (echo statements in place)
- [ ] T021 Test complete workflow on draft PR and collect evidence

## Phase 3.5: Polish

- [ ] T022 [P] Update `dev-docs/ci/README.md` with new CI documentation (deferred)
- [x] T023 [P] Create transition plan document at `specs/035-ci-add-sdd/transition.md`
- [x] T024 [P] Add inline YAML comments explaining report-only configuration
- [ ] T025 Run quickstart validation scenarios and document results
- [ ] T026 Create PR with all changes and evidence links

## Phase 3.6: Enforcement Preparation (Blocked by Issue #31)

- [x] T027 Create follow-up task to remove `continue-on-error: true` after Issue #31 merges (documented)
- [x] T028 Document enforcement activation procedure in `specs/035-ci-add-sdd/enforcement.md`

## Dependencies

- T001 must complete before any workflow modifications
- Tests (T004-T007) before implementation (T008-T021)
- T008-T015 must be sequential (same file modifications)
- T016-T017 can be parallel with T018-T021
- T021 blocks T022-T026 (need evidence first)
- T027-T028 blocked until Issue #31 resolves

## Parallel Execution Examples

### Initial Setup (T002-T003)

```text
Task: "Create evidence directory structure at _artifacts/035-ci-add-sdd/{tests,logs,reports}"
Task: "Create JSON to SARIF conversion script at scripts/ci/json-to-sarif.jq"
```

### Test Creation (T004-T006)

```text
Task: "Create workflow validation test in _artifacts/035-ci-add-sdd/tests/validate-workflow.sh"
Task: "Create SARIF format validation test in _artifacts/035-ci-add-sdd/tests/validate-sarif.sh"
Task: "Create report-only mode test in _artifacts/035-ci-add-sdd/tests/test-report-only.sh"
```

### Documentation (T022-T024)

```text
Task: "Update dev-docs/ci/README.md with new CI documentation"
Task: "Create transition plan document at specs/035-ci-add-sdd/transition.md"
Task: "Add inline YAML comments explaining report-only configuration"
```

## Notes

- [P] tasks = different files, no dependencies
- Verify tests fail before implementing (T007 critical gate)
- Commit after each task group
- Use draft PR for validation (T021)
- Report-only mode is critical for Stage 1
- Enforcement mode requires Issue #31 completion

## Task Generation Rules Applied

1. **From Research**:
   - SARIF conversion approach → T003, T011
   - Report-only implementation → T008
   - Tool installation methods → T009, T013

2. **From Quickstart**:
   - Validation scenarios → T004-T006
   - Local testing → T025
   - Monitoring approach → T022

3. **From Requirements**:
   - Each FR mapped to implementation task
   - NFRs addressed in T018 (caching), T020 (feedback)

4. **Ordering**:
   - Setup → Tests → Core → Integration → Polish → Enforcement
   - Critical path: T001 → T007 → T008-T015 → T021 → T026

## Validation Checklist

_GATE: Checked before execution_

- [x] All functional requirements have corresponding tasks
- [x] All tests come before implementation (T004-T007 before T008)
- [x] Parallel tasks truly independent
- [x] Each task specifies exact file path
- [x] No task modifies same file as another [P] task
- [x] Report-only mode properly configured (T008)
- [x] Evidence collection included (T002, T021, T025)

## IMPORTANT TECHNICAL STANDARDS

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [ast-grep Configuration](https://ast-grep.github.io/guide/project/project-config)
- [SARIF Specification](https://docs.oasis-open.org/sarif/sarif/v2.1.0/sarif-v2.1.0.html)
- [GitHub Code Scanning](https://docs.github.com/en/code-security/code-scanning)

---

⚠️ _Based on SDD CONSTITUTION: `.specify/memory/constitution.md`_
⚠️ _Follow the SDD workflow implementation: `.specify/memory/lifecycle.md`_
⚠️ _Follow the SDD rules: `sdd-rules/rules/README.md`_
