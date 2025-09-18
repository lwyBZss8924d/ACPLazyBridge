# Implementation Plan: Fix ast-grep Rust Warnings

```yaml
worktree: /acplb-worktrees/fix-ast-grep-unwrap-mutex
feature_branch: fix/ast-grep-unwrap-mutex
created: 2025-09-18
last_updated: 2025-09-19
status: done
input: Feature specification from `/specs/031-fix-ast-grep-unwrap-mutex/spec.md`
spec_uri: specs/031-fix-ast-grep-unwrap-mutex/spec.md
plan_uri: specs/031-fix-ast-grep-unwrap-mutex/plan.md
tasks_uri: specs/031-fix-ast-grep-unwrap-mutex/tasks.md
evidence_uris: _artifacts/reports/fix-ast-grep-unwrap-mutex/
specs:
    constitution: 1.0.1
    type: plan
    feature_number: 031
```

## Execution Flow (/plan command scope)

```text
1. Load feature spec from Input path
   → Found: specs/031-fix-ast-grep-unwrap-mutex/spec.md
2. Fill Technical Context
   → Detect Project Type: Rust workspace (library + CLI)
   → Set Structure Decision: Single project
3. Fill the Constitution Check section
4. Evaluate Constitution Check section
   → PASS: All gates satisfied
5. Execute Phase 0 → research.md
6. Execute Phase 1 → contracts, data-model.md, quickstart.md
7. Re-evaluate Constitution Check
   → PASS: Design remains compliant
8. Plan Phase 2 → Task generation approach
9. STOP - Ready for /tasks command
```

## Summary

Fix ast-grep false positives by updating rules to exclude inline test code using AST-based patterns, then refactor production code to properly handle errors without unwrap()/expect() except where explicitly justified with context.

## Technical Context

**Language/Version**: Rust 1.89
**Primary Dependencies**: ast-grep 0.30.0, anyhow 1.0 (for error context)
**Storage**: N/A (configuration and code changes only)
**Testing**: cargo test
**Target Platform**: Linux/macOS/Windows (cross-platform Rust)
**Project Type**: single (Rust workspace)
**Performance Goals**: No regression in runtime performance
**Constraints**: Maintain backward compatibility, no breaking changes
**Scale/Scope**: ~89 ast-grep warnings to resolve (56 false positives, 33 legitimate)

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

**Simplicity**:

- Projects: 2 (acp-lazy-core, codex-cli-acp)
- Using framework directly? YES (ast-grep rules, Rust std error handling)
- Single data model? N/A (no data model changes)
- Avoiding patterns? YES (no unnecessary abstractions)

**Architecture**:

- EVERY feature as library? N/A (bug fix, not new feature)
- Libraries listed: N/A
- CLI per library: N/A
- Library docs: N/A

**Testing (NON-NEGOTIABLE)**:

- RED-GREEN-Refactor cycle enforced? YES (tests must pass after changes)
- Git commits show tests before implementation? N/A (fixing existing code)
- Order: Contract→Integration→E2E→Unit strictly followed? YES (run all existing tests)
- Real dependencies used? YES (no mocks)
- Integration tests for changes? YES (existing test suite validates changes)
- FORBIDDEN: Implementation before test - N/A (tests already exist)

**Observability**:

- Structured logging included? EXISTING (no changes needed)
- Frontend logs → backend? N/A
- Error context sufficient? YES (adding context via anyhow)

**Versioning**:

- Version number assigned? N/A (bug fix, patch version)
- BUILD increments on every change? YES
- Breaking changes handled? NONE (backward compatible)

## Project Structure

### Documentation (this feature)

```tree
specs/031-fix-ast-grep-unwrap-mutex/
├── spec.md              # Feature specification (complete)
├── plan.md              # This file
├── research.md          # Phase 0 output
├── quickstart.md        # Phase 1 output (validation steps)
└── tasks.md             # Phase 2 output (/tasks command)
```

### Source Code (repository root)

```tree
sdd-rules/rules/code-analysis/ast-grep/rust/
├── no-unwrap.yml        # Update to exclude inline tests
├── rust-mutex-lock.yml  # Update to exclude inline tests
└── todo-comment.yml     # No changes needed

crates/
├── acp-lazy-core/src/
│   ├── transport.rs     # Fix ~30 unwrap() calls
│   └── protocol.rs      # Fix 6 unwrap() calls
└── codex-cli-acp/src/
    ├── main.rs          # Fix 3 unwrap() calls
    ├── notify_source.rs # Fix 1 unwrap() call
    └── bin/playback.rs  # Fix 3 unwrap() calls
```

## Phase 0: Outline & Research

1. **Research AST-grep pattern syntax**:
   - How to use `not:` with `inside:` for exclusion
   - AST node types for Rust attributes
   - Pattern matching for `cfg(test)` and `test` attributes

2. **Research error handling patterns**:
   - Best practices for `?` operator usage
   - When to use `anyhow::Context` vs custom errors
   - Proper Mutex poisoning handling

3. **Generate research.md** with findings:
   - AST-grep exclusion patterns validated
   - Error handling approach confirmed
   - No breaking changes identified

**Output**: research.md with AST patterns and error handling guidelines

## Phase 1: Design & Contracts

_Prerequisites: research.md complete_

1. **Design rule updates**:
   - AST pattern for excluding `#[cfg(test)]` modules
   - AST pattern for excluding `#[test]` functions
   - Ensure patterns work with nested attributes

2. **Design code refactoring approach**:
   - Categorize unwrap() usage: can use `?`, needs context, must remain
   - Define context messages for expect() calls
   - Mutex lock handling strategy

3. **Create validation quickstart**:
   - Commands to run ast-grep before/after
   - Commands to run quality gates
   - Evidence collection steps

**Output**: quickstart.md with validation steps

## Phase 2: Task Planning Approach

_This section describes what the /tasks command will do_

**Task Generation Strategy**:

- Setup tasks: Create evidence directory, capture initial state
- Rule update tasks: Modify YAML files (parallel)
- Code refactoring tasks: Fix each file (some parallel)
- Validation tasks: Run tests, capture final state

**Ordering Strategy**:

- Capture initial evidence first
- Update rules before testing their effect
- Refactor code after rules are validated
- Final validation and evidence collection

**Estimated Output**: 18-20 numbered tasks in tasks.md

## Phase 3+: Future Implementation

_These phases are beyond the scope of the /plan command_

**Phase 3**: Task execution (execute tasks.md)
**Phase 4**: Validation (run tests, ast-grep scan, collect evidence)
**Phase 5**: PR creation with evidence links

## Complexity Tracking

_No violations - all changes follow constitutional principles_

## Progress Tracking

**Phase Status**:

- [x] Phase 0: Research complete
- [x] Phase 1: Design complete
- [x] Phase 2: Task planning complete (approach defined)
- [ ] Phase 3: Tasks generated (/tasks command)
- [ ] Phase 4: Implementation complete
- [ ] Phase 5: Validation passed

**Gate Status**:

- [x] Initial Constitution Check: PASS
- [x] Post-Design Constitution Check: PASS
- [x] All NEEDS CLARIFICATION resolved
- [x] Complexity deviations documented (none)

## IMPORTANT TECHNICAL STANDARDS

- [ACP](https://github.com/zed-industries/agent-client-protocol) - "ACPLazyBridge" follow `ACP` Protocol
- [ACP JSON Schema](https://github.com/zed-industries/agent-client-protocol/blob/main/schema/schema.json) - "ACPLazyBridge" follow `ACP` JSON Schema
- **ACP Repository local path**: (~/dev-space/agent-client-protocol)

---

⚠️ _Based on SDD CONSTITUTION: `.specify/memory/constitution.md`_
⚠️ _Follow the SDD workflow implementation: `.specify/memory/lifecycle.md`_
⚠️ _Follow the SDD rules: `sdd-rules/rules/README.md`_
