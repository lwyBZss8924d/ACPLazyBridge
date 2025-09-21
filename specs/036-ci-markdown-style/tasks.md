# Tasks: CI Markdown Style Verification

```yaml
worktree: /Users/arthur/dev-space/acplb-worktrees/028-markdown-ci
feature_branch: docs/028-markdown-ci-verification
created: 2025-09-20
last_updated: 2025-09-21
status: completed
merged_pr: https://github.com/lwyBZss8924d/ACPLazyBridge/pull/37
merge_date: 2025-09-21
merge_commit: 2a4d0a98afffeba61fc6155d39e979b03f50e611
input: Design documents from `/specs/036-ci-markdown-style/`
issue_uri: https://github.com/lwyBZss8924d/ACPLazyBridge/issues/28
pr_uri: https://github.com/lwyBZss8924d/ACPLazyBridge/pull/37
spec_uri: specs/036-ci-markdown-style/spec.md
plan_uri: specs/036-ci-markdown-style/plan.md
tasks_uri: specs/036-ci-markdown-style/tasks.md
evidence_uris: _artifacts/036-ci-markdown-style/
prerequisites:
    plan: plan.md (required),
    research: research.md
    quickstart: quickstart.md
specs:
    constitution: 1.0.1
    type: tasks
    feature_number: 036
commits:
    merge_commit: 2a4d0a98afffeba61fc6155d39e979b03f50e611
    pr_number: 37
```

## Execution Flow (main)

```text
1. Load plan.md from feature directory
   → Found: Tech stack (YAML, Bash), structure plan
2. Load optional design documents:
   → research.md: Loaded - decisions documented
   → quickstart.md: Loaded - user guide ready
3. Generate tasks by category:
   → Setup: Configuration updates
   → Core: Workflow creation
   → Integration: Documentation updates
   → Polish: Testing and transition planning
4. Apply task rules:
   → Different files = mark [P] for parallel
   → Configuration before workflow
   → Documentation can be parallel
5. Number tasks sequentially (T001, T002...)
6. Generate dependency graph
7. Create parallel execution examples
8. Validate task completeness:
   → Configuration task: ✓
   → Workflow creation: ✓
   → Documentation updates: ✓
9. Return: SUCCESS (tasks ready for execution)
```

## Format: `[ID] [P?] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- Include exact file paths in descriptions

## Phase 3.1: Setup

- [ ] T001 Update .markdownlint.json to disable MD013 (line length rule)
- [ ] T002 [P] Create evidence directory _artifacts/036-ci-markdown-style/

## Phase 3.2: Core Implementation

- [ ] T003 Create .github/workflows/docs-style.yml with report-only mode
- [ ] T004 Add path filtering for **/*.md in workflow triggers
- [ ] T005 Configure workflow to use scripts/ci/run-markdown-style.sh

## Phase 3.3: Documentation Updates

- [ ] T006 [P] Update WARP.md to document docs-style CI job
- [ ] T007 [P] Update .github/CLAUDE.md with verification-only approach details

## Phase 3.4: Testing & Validation

- [ ] T008 Create test branch with sample markdown violations
- [ ] T009 Verify workflow runs in report-only mode
- [ ] T010 Capture workflow output as evidence in _artifacts/

## Phase 3.5: Polish

- [ ] T011 Document transition plan from report-only to enforcement
- [ ] T012 Update quickstart.md with troubleshooting section if needed

## Dependencies

- T001 before T003 (config before workflow)
- T003-T005 before T008-T010 (workflow before testing)
- T008-T010 before T011 (testing before transition plan)
- Documentation (T006-T007) can run parallel with workflow creation

## Parallel Example

```text
# Launch T006-T007 together:
Task: "Update WARP.md to document docs-style CI job"
Task: "Update .github/CLAUDE.md with verification-only approach details"
```

## Notes

- Configuration change (MD013) reduces friction immediately
- Report-only mode ensures non-disruptive rollout
- Clear documentation helps team adoption
- Transition to enforcement after team adapts

## Task Details

### T001: Update .markdownlint.json

```json
{
  "MD013": false
}
```

Add comment explaining rationale for disabling line length.

### T003: Create .github/workflows/docs-style.yml

```yaml
name: Markdown Style Verification
on:
  pull_request:
    paths:
      - '**/*.md'

jobs:
  verify-markdown:
    runs-on: ubuntu-latest
    continue-on-error: true  # Report-only mode
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: '20'
      - name: Run markdown style check
        run: |
          if ! ./scripts/ci/run-markdown-style.sh; then
            echo "::error::Markdown violations found. Run './scripts/ci/run-local-ci.sh' locally"
            exit 1
          fi
      - name: Upload results
        if: always()
        uses: actions/upload-artifact@v4
        with:
          name: markdown-lint-results
          path: |
            markdown-lint-report.json
            markdown-style.log
          if-no-files-found: ignore
```

### T006: Update WARP.md

Add to CI section:

- docs-style job description
- Report-only initial mode
- Transition plan timeline

### T007: Update .github/CLAUDE.md

Add to Required Checks section:

- Markdown Style Verification (report-only initially)
- Local-first approach documentation
- Reference to quickstart.md

## Validation Checklist

_GATE: Checked by main() before returning_

- [x] Configuration task defined (T001)
- [x] Workflow creation tasks defined (T003-T005)
- [x] Documentation tasks defined (T006-T007)
- [x] Testing tasks included (T008-T010)
- [x] Each task specifies exact file path
- [x] Parallel tasks truly independent

## Pre-PR Quality Gates

### T900-T999: Pre-PR Validation [P]

- [ ] T900: Run SDD document validation [P]

  ```bash
  ./scripts/sdd/validate-sdd-docs.sh
  ```

  Evidence: _artifacts/036-ci-markdown-style/validation/sdd_docs_$(date +%Y%m%d_%H%M%S).log

- [ ] T901: Test workflow YAML syntax

  ```bash
  # Validate YAML syntax
  yq eval '.' .github/workflows/docs-style.yml > /dev/null
  ```

  Evidence: Clean validation required

- [ ] T902: Run local markdown checks to establish baseline

  ```bash
  ./scripts/ci/run-markdown-style.sh 2>&1 | tee _artifacts/036-ci-markdown-style/baseline.log
  ```

  Evidence: _artifacts/036-ci-markdown-style/baseline.log

## IMPORTANT TECHNICAL STANDARDS

- [ACP](https://github.com/zed-industries/agent-client-protocol) - "ACPLazyBridge" follow `ACP` Protocol
- [ACP JSON Schema](https://github.com/zed-industries/agent-client-protocol/blob/main/schema/schema.json) - "ACPLazyBridge" follow `ACP` JSON Schema
- **ACP Repository local path**: (~/dev-space/agent-client-protocol)

---

⚠️ _Based on SDD CONSTITUTION: `.specify/memory/constitution.md`_
⚠️ _Fllow the SDD workflow implementation: `.specify/memory/lifecycle.md`_
⚠️ _Follow the SDD rules: `sdd-rules/rules/README.md`_
