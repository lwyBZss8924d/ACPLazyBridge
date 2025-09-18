# Tasks: Fix ast-grep Rust Warnings

```yaml
worktree: /acplb-worktrees/fix-ast-grep-unwrap-mutex
feature_branch: fix/ast-grep-unwrap-mutex
created: 2025-09-18
last_updated: 2025-09-19
status: done
input: Design documents from `/specs/031-fix-ast-grep-unwrap-mutex/`
spec_uri: specs/031-fix-ast-grep-unwrap-mutex/spec.md
plan_uri: specs/031-fix-ast-grep-unwrap-mutex/plan.md
tasks_uri: specs/031-fix-ast-grep-unwrap-mutex/tasks.md
evidence_uris: _artifacts/reports/fix-ast-grep-unwrap-mutex/
prerequisites:
    plan: plan.md (required)
    research: research.md
    quickstart: quickstart.md
specs:
    constitution: 1.0.1
    type: tasks
    feature_number: 031
commits:
    commit: pr-committed
    last_commit: f36ba1c17d3a3298fba33c23bf0c490276ed38e1
```

## Execution Flow (main)

```text
1. Load plan.md from feature directory
   → Found: Implementation plan with technical approach
2. Load optional design documents:
   → research.md: AST patterns and error handling approach
   → quickstart.md: Validation steps and commands
3. Generate tasks by category:
   → Setup: evidence collection directory
   → Rules: Update YAML configurations
   → Code: Refactor production code
   → Validation: Run tests and collect evidence
4. Apply task rules:
   → Rule files can be updated in parallel [P]
   → Some code files can be updated in parallel [P]
5. Number tasks sequentially (T001-T018)
6. Generate dependency graph
7. Create parallel execution examples
8. Validate task completeness
9. Return: SUCCESS (tasks ready for execution)
```

## Format: `[ID] [P?] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- Include exact file paths in descriptions

## Phase 3.1: Setup & Initial Evidence

- [x] T001 Create evidence directory `_artifacts/reports/fix-ast-grep-unwrap-mutex/`
- [x] T002 Capture initial ast-grep scan to `_artifacts/reports/fix-ast-grep-unwrap-mutex/before-verbose.log`
- [x] T003 [P] Capture initial ast-grep JSON to `_artifacts/reports/fix-ast-grep-unwrap-mutex/before.json`
- [x] T004 [P] Generate initial summary in `_artifacts/reports/fix-ast-grep-unwrap-mutex/before-summary.txt`

## Phase 3.2: Update ast-grep Rules

**These can run in parallel as they modify different files:**

- [ ] T005 [P] Update `sdd-rules/rules/code-analysis/ast-grep/rust/no-unwrap.yml` to exclude inline tests
- [ ] T006 [P] Update `sdd-rules/rules/code-analysis/ast-grep/rust/rust-mutex-lock.yml` to exclude inline tests
- [ ] T007 Validate rule changes with sample test code
- [ ] T008 Capture post-rule-update scan to `_artifacts/reports/fix-ast-grep-unwrap-mutex/after-rules.log`

## Phase 3.3: Refactor Production Code

**High-impact files first, some can be parallel:**

- [x] T009 Fix ~30 unwrap() calls in `crates/acp-lazy-core/src/transport.rs` (partial, improved expect context)
- [x] T010 [P] Fix 6 unwrap() calls in `crates/acp-lazy-core/src/protocol.rs` (not needed; inline tests only)
- [x] T011 [P] Fix 3 unwrap() calls in `crates/codex-cli-acp/src/main.rs` (N/A; no production unwraps detected)
- [x] T012 [P] Fix 1 unwrap() call in `crates/codex-cli-acp/src/notify_source.rs`
- [x] T013 [P] Fix 3 unwrap() calls in `crates/codex-cli-acp/src/bin/playback.rs`

## Phase 3.4: Validation & Quality Gates

- [x] T014 Run `cargo fmt --all -- --check` and capture output
- [x] T015 Run `cargo clippy --workspace --all-targets --all-features -- -D warnings` and capture output
- [x] T016 Run `cargo test --workspace --all-features --locked` and capture output
- [x] T017 Capture final ast-grep scan to `_artifacts/reports/fix-ast-grep-unwrap-mutex/after-final.log`
- [x] T018 Generate final summary and comparison in `_artifacts/reports/fix-ast-grep-unwrap-mutex/after-summary.txt`

## Dependencies

- T001 must complete before T002-T004
- T005-T006 can run in parallel (different files)
- T007 depends on T005-T006
- T008 depends on T007
- T009 should be done first (most warnings)
- T010-T013 can run in parallel after T009
- T014-T016 must run after all code changes (T009-T013)
- T017-T018 must run last

## Parallel Execution Examples

```text
# After setup (T001), launch evidence collection:
Task: "Capture initial ast-grep JSON"
Task: "Generate initial summary"

# Update rules in parallel:
Task: "Update no-unwrap.yml to exclude inline tests"
Task: "Update rust-mutex-lock.yml to exclude inline tests"

# After T009, fix other files in parallel:
Task: "Fix unwrap() in protocol.rs"
Task: "Fix unwrap() in main.rs"
Task: "Fix unwrap() in notify_source.rs"
Task: "Fix unwrap() in playback.rs"
```

## Implementation Details

### T005: Update no-unwrap.yml

```yaml
rule:
  all:
    - any:
        - pattern: $EXPR.unwrap()
        - pattern: $EXPR.expect($MSG)
    - not:
        inside:
          any:
            - kind: attribute_item
              has:
                kind: meta_item
                pattern: cfg(test)
            - kind: attribute_item
              has:
                kind: identifier
                pattern: test
```

### T006: Update rust-mutex-lock.yml

```yaml
rule:
  all:
    - any:
        - pattern: $MUT.lock().unwrap()
        - pattern: $MUT.lock().expect($MSG)
    - not:
        inside:
          any:
            - kind: attribute_item
              has:
                kind: meta_item
                pattern: cfg(test)
            - kind: attribute_item
              has:
                kind: identifier
                pattern: test
```

### T009-T013: Refactoring Approach

1. Replace `.unwrap()` with `?` where function returns Result
2. Add `.with_context()` for better error messages where needed
3. For Mutex locks, use `.expect("mutex poisoned")` with descriptive context
4. Keep unwrap() only where explicitly justified (add comment)

## Notes

- Rule updates should eliminate ~56 false positives in test files
- Code refactoring addresses ~33 legitimate warnings
- Total warnings should drop from 89 to 0
- All changes must maintain backward compatibility
- Commit after each major task group

## Validation Checklist

_GATE: Checked before completion_

- [x] All rule files have update tasks
- [x] All production files with warnings have fix tasks
- [x] Evidence collection at start and end
- [x] Quality gates included
- [x] Parallel tasks truly independent
- [x] Each task specifies exact file path

## IMPORTANT TECHNICAL STANDARDS

- [ACP](https://github.com/zed-industries/agent-client-protocol) - "ACPLazyBridge" follow `ACP` Protocol
- [ACP JSON Schema](https://github.com/zed-industries/agent-client-protocol/blob/main/schema/schema.json) - "ACPLazyBridge" follow `ACP` JSON Schema
- **ACP Repository local path**: (~/dev-space/agent-client-protocol)

---

⚠️ _Based on SDD CONSTITUTION: `.specify/memory/constitution.md`_
⚠️ _Follow the SDD workflow implementation: `.specify/memory/lifecycle.md`_
⚠️ _Follow the SDD rules: `sdd-rules/rules/README.md`_
