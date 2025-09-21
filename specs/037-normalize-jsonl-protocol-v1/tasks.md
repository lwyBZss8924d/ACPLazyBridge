# Tasks: Normalize JSONL fixtures to ACP v1 protocolVersion

```yaml
worktree: /Users/arthur/dev-space/acplb-worktrees/037-normalize-jsonl-protocol-v1
feature_branch: chore/037-normalize-jsonl-protocol-v1
created: 2025-09-21T19:30:00Z
last_updated: 2025-09-21T20:50:06Z
status: in_progress
input: Design documents from `/specs/037-normalize-jsonl-protocol-v1/`
issue_uri: https://github.com/lwyBZss8924d/ACPLazyBridge/issues/14
spec_uri: specs/037-normalize-jsonl-protocol-v1/spec.md
plan_uri: specs/037-normalize-jsonl-protocol-v1/plan.md
tasks_uri: specs/037-normalize-jsonl-protocol-v1/tasks.md
evidence_uris: _artifacts/037-normalize-jsonl-protocol-v1/
prerequisites:
    plan: plan.md (required)
    research: Inline in spec.md
    evidence: _artifacts/037-normalize-jsonl-protocol-v1/reports/validation-summary.md
specs:
    constitution: 1.0.1
    type: tasks
    feature_number: 037
commits:
```

## Execution Flow (main)

```text
1. Load plan.md from specs/037-normalize-jsonl-protocol-v1/
   → Simple maintenance task identified
2. Generate tasks by category:
   → Setup: evidence directory, backups
   → Core: fix JSONL files by type
   → Validation: JSON structure, playback tests
3. Apply task rules:
   → Batch similar updates
   → Validate after changes
4. Number tasks sequentially (T001, T002...)
5. Return: SUCCESS (tasks ready for execution)
```

## Format: `[ID] [P?] Description`

- **[P]**: Can run in parallel (different files, no dependencies)

## Phase 3.1: Setup

- [ ] T001 Create evidence directory structure _artifacts/037-normalize-jsonl-protocol-v1/{tests,logs,reports}
- [ ] T002 [P] Backup existing JSONL fixtures to _artifacts/037-normalize-jsonl-protocol-v1/backups/
- [ ] T003 [P] Document current protocolVersion usage in _artifacts/037-normalize-jsonl-protocol-v1/reports/pre-fix-audit.txt

## Phase 3.2: Core Implementation

### Fix fixtures with "2024-11-05" string

- [ ] T004 Update test_basic_handshake.jsonl: replace the legacy string protocolVersion value (2024-11-05) with integer 1
- [ ] T005 Update prompt_with_mock_codex.jsonl: replace the legacy string protocolVersion value (2024-11-05) with integer 1
- [ ] T006 Update tool_calls_large_output.jsonl: replace the legacy string protocolVersion value (2024-11-05) with integer 1
- [ ] T007 Update tool_calls.jsonl: replace the legacy string protocolVersion value (2024-11-05) with integer 1
- [ ] T008 Update handshake.jsonl: replace the legacy string protocolVersion value (2024-11-05) with integer 1
- [ ] T009 Update tool_calls_batch.jsonl: replace the legacy string protocolVersion value (2024-11-05) with integer 1
- [ ] T010 Update basic_session.jsonl: replace the legacy string protocolVersion value (2024-11-05) with integer 1
- [ ] T011 Update prompt_and_cancel.jsonl: replace the legacy string protocolVersion value (2024-11-05) with integer 1
- [ ] T012 Update session_update_format.jsonl: replace the legacy string protocolVersion value (2024-11-05) with integer 1
- [ ] T013 Update test_prompt_session.jsonl: replace the legacy string protocolVersion value (2024-11-05) with integer 1

### Fix fixtures with "1" string

- [ ] T014 Update notify_idle.jsonl: "protocolVersion":"1" → "protocolVersion":1

### Fix test script

- [ ] T015 Update test_streaming.sh: Replace both instances of the legacy protocolVersion string (2024-11-05) with the integer literal 1

## Phase 3.3: Validation

- [ ] T016 [P] Validate all JSONL files are valid JSON using jq
- [ ] T017 [P] Verify protocolVersion is integer 1 in all fixtures
- [ ] T018 Run playback test with handshake.jsonl
- [ ] T019 Run playback test with basic_session.jsonl
- [ ] T020 Run playback test with tool_calls.jsonl

## Phase 3.4: Documentation & Reporting

- [ ] T021 [P] Generate post-fix audit report in _artifacts/037-normalize-jsonl-protocol-v1/reports/post-fix-audit.txt
- [ ] T022 [P] Create validation summary in _artifacts/037-normalize-jsonl-protocol-v1/reports/validation-summary.md
- [ ] T023 Document any issues or observations in _artifacts/037-normalize-jsonl-protocol-v1/reports/notes.md

## Dependencies

- T001 blocks all other tasks (need evidence directory)
- T002-T003 can run in parallel
- T004-T015 should run sequentially (file edits)
- T016-T017 can run in parallel after T004-T015
- T018-T020 require T016-T017 complete
- T021-T023 can run in parallel after validation

## Parallel Example

```text
# After T001, launch T002-T003 together:
Task: "Backup existing JSONL fixtures"
Task: "Document current protocolVersion usage"

# After all fixes, launch T016-T017 together:
Task: "Validate all JSONL files are valid JSON"
Task: "Verify protocolVersion is integer in all fixtures"
```

## Notes

- Use sed or direct string replacement for simple changes
- Preserve all other JSON structure exactly
- Validate JSON after each change to catch errors early
- Keep detailed logs of all changes

## Validation Checklist

_GATE: Checked before marking complete_

- [ ] All fixtures use integer protocolVersion 1
- [ ] No string protocolVersion values remain
- [ ] All files are valid JSON
- [ ] Playback tests pass
- [ ] Evidence collected in _artifacts

## Pre-PR Quality Gates

- [ ] T900: Run SDD document validation

  ```bash
  ./scripts/sdd/validate-sdd-docs.sh
  ```

  Evidence: _artifacts/037-normalize-jsonl-protocol-v1/validation/sdd_docs_$(date +%Y%m%d_%H%M%S).log

- [ ] T901: Verify all JSONL changes

  ```bash
  grep -r "protocolVersion" dev-docs/review/_artifacts/tests/*.jsonl | \
    grep -v '"protocolVersion":1' | \
    wc -l  # Should be 0
  ```

  Evidence: _artifacts/037-normalize-jsonl-protocol-v1/validation/final_check_$(date +%Y%m%d_%H%M%S).log

## IMPORTANT TECHNICAL STANDARDS

- [ACP](https://github.com/zed-industries/agent-client-protocol) - "ACPLazyBridge" follow `ACP` Protocol
- [ACP JSON Schema](https://github.com/zed-industries/agent-client-protocol/blob/main/schema/schema.json) - "ACPLazyBridge" follow `ACP` JSON Schema
- **ACP Repository local path**: (~/dev-space/agent-client-protocol)

---

⚠️ _Based on SDD CONSTITUTION: `.specify/memory/constitution.md`_
⚠️ _Follow the SDD workflow implementation: `.specify/memory/lifecycle.md`_
⚠️ _Follow the SDD rules: `sdd-rules/rules/README.md`_
