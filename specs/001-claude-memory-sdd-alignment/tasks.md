# Tasks: Claude Memory and SDD Rules Index Alignment

## Metadata

- Issue-URI: https://github.com/lwyBZss8924d/ACPLazyBridge/issues/26
- Tasks-URI: specs/001-claude-memory-sdd-alignment/tasks.md
- Evidence-URIs: dev-docs/review/_artifacts/{tests,logs,reports}/001-claude-memory-sdd-alignment/

## Task List

### Task 1: Update Root CLAUDE.md

**ID**: 001-01  
**Priority**: High  
**Assignee**: WARP Agent  
**Description**: Update root CLAUDE.md with SDD workflow integration and authority chain

**Subtasks**:

1. Add repository overview section
2. Define authority and governance hierarchy
3. Document SDD Developer Team workflow
4. Add project navigation structure
5. Update ACP protocol examples to use integer 1
6. Define quality gates and CI requirements

**Test Plan**:

- Verify all sections present
- Check protocol examples use `"protocolVersion": 1`
- Validate internal links

**Evidence**:

- File diff: dev-docs/review/_artifacts/tests/001-claude-memory-sdd-alignment/root-claude-diff.txt

### Task 2: Create Module CLAUDE.md Files

**ID**: 001-02  
**Priority**: High  
**Assignee**: WARP Agent  
**Description**: Create CLAUDE.md files for key modules

**Subtasks**:

1. Create crates/acp-lazy-core/CLAUDE.md
2. Create crates/codex-cli-acp/CLAUDE.md
3. Create scripts/CLAUDE.md

**Test Plan**:

- Verify file creation
- Check parent rule references
- Validate evidence paths

**Evidence**:

- File listing: dev-docs/review/_artifacts/tests/001-claude-memory-sdd-alignment/module-files.txt

### Task 3: Create Rules Index

**ID**: 001-03  
**Priority**: High  
**Assignee**: WARP Agent  
**Description**: Create sdd-rules/rules/README.md as centralized navigation

**Subtasks**:

1. Scan sdd-rules/rules/ for all rule documents
2. Categorize rules by domain
3. Create index with descriptions and links
4. Add navigation hints

**Test Plan**:

- Verify all rule files are indexed
- Check link validity
- Ensure categories are logical

**Evidence**:

- Index content: dev-docs/review/_artifacts/tests/001-claude-memory-sdd-alignment/rules-index.md

### Task 4: Update Cross-References

**ID**: 001-04  
**Priority**: Medium  
**Assignee**: WARP Agent  
**Description**: Update AGENTS.md and WARP.md with rules index links

**Subtasks**:

1. Add rules index reference to AGENTS.md
2. Add rules index reference to WARP.md (if not already present)
3. Verify bidirectional linking

**Test Plan**:

- Check references added
- Validate link paths

**Evidence**:

- Diff files: dev-docs/review/_artifacts/tests/001-claude-memory-sdd-alignment/cross-ref-diffs.txt

### Task 5: Run Local CI Validation

**ID**: 001-05  
**Priority**: High  
**Assignee**: WARP Agent  
**Description**: Execute all local CI checks and store evidence

**Subtasks**:

1. Run scripts/ci/run-sdd-structure-lint.sh
2. Run scripts/ci/check-language-policy.sh
3. Manually verify internal links
4. Store all outputs as evidence

**Test Plan**:

- All scripts exit with code 0
- No errors in output logs
- All links resolve

**Evidence**:

- Lint log: dev-docs/review/_artifacts/logs/001-claude-memory-sdd-alignment/lint.log
- Language log: dev-docs/review/_artifacts/logs/001-claude-memory-sdd-alignment/language.log
- Link check: dev-docs/review/_artifacts/tests/001-claude-memory-sdd-alignment/links.log

### Task 6: Create GitHub Issue

**ID**: 001-06  
**Priority**: High  
**Assignee**: WARP Agent  
**Description**: Create GitHub issue #26 for tracking

**Subtasks**:

1. Create issue with spec/plan/tasks links
2. Add SDD metadata block
3. Label appropriately

**Test Plan**:

- Issue created successfully
- Links resolve to spec files

**Evidence**:

- Issue URL: https://github.com/lwyBZss8924d/ACPLazyBridge/issues/26

### Task 7: Commit and Push Changes

**ID**: 001-07  
**Priority**: High  
**Assignee**: WARP Agent  
**Description**: Commit changes with proper message format

**Subtasks**:

1. Stage all changes
2. Commit with SDD-compliant message
3. Push to remote

**Test Plan**:

- Commit message follows format
- Push successful

**Evidence**:

- Commit log: dev-docs/review/_artifacts/logs/001-claude-memory-sdd-alignment/commit.log

### Task 8: Create Pull Request

**ID**: 001-08  
**Priority**: High  
**Assignee**: WARP Agent  
**Description**: Create PR with full SDD traceability

**Subtasks**:

1. Create PR with comprehensive description
2. Link to spec/plan/tasks
3. Include evidence links
4. Request review

**Test Plan**:

- PR created successfully
- All links functional
- CI checks pass

**Evidence**:

- PR URL: To be generated
- CI results: dev-docs/review/_artifacts/reports/001-claude-memory-sdd-alignment/ci-results.txt

## Dependencies

- Task 1-4 can be done in parallel
- Task 5 depends on Tasks 1-4
- Task 6 can be done anytime
- Task 7 depends on Task 5
- Task 8 depends on Task 7

## Risk Management

- If CI checks fail: Review logs, fix issues, re-run
- If links broken: Update paths, verify structure
- If protocol examples wrong: Search and replace all occurrences

## Completion Criteria

- [ ] All CLAUDE.md files created/updated
- [ ] Rules index complete and linked
- [ ] All CI checks passing
- [ ] Evidence artifacts stored
- [ ] PR created with full traceability
