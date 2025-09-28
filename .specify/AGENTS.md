# AGENTS.md - Codex Agent SDD Integration

## Purpose

This document captures my (Codex) understanding and operational context within the ACPLazyBridge Specification-Driven Development (SDD) framework.
It serves as a reference for consistent SDD workflow execution and ensures alignment with project constitution and rules.

## SDD Workflow Understanding

### Core Flow

```text
Idea → /specify → spec.md → /plan → plan.md → /tasks → tasks.md → Implementation → Evidence → PR → Merge
```

📌 Wen AI-Engineer SDD-TASKs Cooking Workflow can follow the BASELINE TEMPLATES work in (specs/): [AI-Engineer-SDD-Workflow-Baseline-templates](.specify/memory/AI-Engineer-SDD-Workflow-Baseline-templates.txt)

### Phase Responsibilities

#### 0. Issue Initialization Phase (`/sdd-task`)

- **Trigger**: User provides GitHub issue number or URL
- **My Actions**:
  1. Execute `gh issue view "$1" --json title,body,number,url,state,labels`
  2. Analyze issue labels to determine branch type (feature/fix/docs/chore/perf)
  3. Create worktree: `git worktree add ../acplb-worktrees/<NNN-slug> origin/main -b <branch-type>/<NNN-slug>`
  4. Initialize SDD workflow phases (/specify → /plan → /tasks)
  5. Link all artifacts to the GitHub issue

#### 1. Specify Phase (`/specify`)

- **Trigger**: User provides feature description
- **My Actions**:
  1. Execute `scripts/sdd/create-new-feature.sh --json "$ARGUMENTS"`
  2. Parse JSON for BRANCH_NAME and SPEC_FILE
  3. Load `.specify/templates/spec-template.md`
  4. Generate spec.md focusing on WHAT/WHY (no HOW)
  5. Mark ambiguities with `[NEEDS CLARIFICATION: question]`

#### 2. Plan Phase (`/plan`)

- **Trigger**: Spec exists, user requests planning
- **My Actions**:
  1. Execute `scripts/sdd/setup-plan.sh --json`
  2. Parse JSON for paths (FEATURE_SPEC, IMPL_PLAN, SPECS_DIR)
  3. Read constitution for compliance checks
  4. Execute plan template phases:
     - Phase 0: Generate `research.md`
     - Phase 1: Generate `data-model.md`, `contracts/`, `quickstart.md`
  5. Stop at Phase 2 description (tasks generation is separate)

#### 3. Tasks Phase (`/tasks`)

- **Trigger**: Plan exists, user requests task breakdown
- **My Actions**:
  1. Execute `scripts/sdd/check-task-prerequisites.sh --json`
  2. Parse available documents (plan.md, data-model.md, contracts/)
  3. Generate tasks following TDD order:
     - Setup tasks
     - Test tasks [P] (must fail first)
     - Implementation tasks
     - Integration tasks
     - Polish tasks [P]
  4. Mark parallel tasks with [P]
  5. Number tasks sequentially (T001, T002...)

## Constitutional Gate Checks

I enforce these constitutional articles at each phase:

### Article I: Library-First ✓

- Every feature must start as a library/crate
- Check: Ensure plan includes library structure
- Validation: `src/lib.rs` or equivalent exists

### Article III: Test-First (NON-NEGOTIABLE) ✓

- Tests written and must fail before implementation
- Check: Tasks.md lists tests before implementation
- Validation: RED→GREEN→REFACTOR cycle enforced

### Article VII: Simplicity ✓

- Maximum 3 projects in implementation
- Check: Count projects in plan.md structure
- Validation: Reject if >3 projects without justification

### Article VIII: Anti-Abstraction ✓

- Use framework features directly
- Check: No unnecessary wrapper classes in design
- Validation: Single model representation

### Article IX: Integration-First ✓

- Contracts defined before implementation
- Check: `contracts/` exists with specifications
- Validation: Contract tests precede implementation

## Command Execution Patterns

### Script Integration

All scripts support JSON mode for tool consumption:

```bash
# Specify
scripts/sdd/create-new-feature.sh --json "feature description"
# Returns: {"BRANCH_NAME": "...", "SPEC_FILE": "...", "FEATURE_NUMBER": "..."}

# Plan
scripts/sdd/setup-plan.sh --json
# Returns: {"FEATURE_SPEC": "...", "IMPL_PLAN": "...", "SPECS_DIR": "...", "BRANCH": "..."}

# Tasks
scripts/sdd/check-task-prerequisites.sh --json
# Returns: {"FEATURE_DIR": "...", "AVAILABLE_DOCS": [...]}
```

### Template Processing

1. Load template from `.specify/templates/`
2. Execute `Execution Flow (main)` function
3. Replace placeholders with concrete values
4. Validate against constitutional gates
5. Handle errors and warnings appropriately

### Path Resolution

Always use absolute paths:

- Repository root: `git rev-parse --show-toplevel`
- Specs directory: `$REPO_ROOT/specs/`
- Templates: `$REPO_ROOT/.specify/templates/`

## Evidence Collection Standards

### Directory Structure

```tree
_artifacts/
├── tests/<task>/      # Test execution logs
├── logs/<task>/       # General logs
├── reports/<task>/    # Analysis reports
└── jq/<task>/         # JSON validation

_artifacts/legacy/  # Legacy location
```

### Evidence Requirements

- Every task produces evidence
- Timestamp all artifacts: `$(date +%Y%m%d_%H%M%S)`
- Link evidence in PR descriptions
- Store JSONL replay scenarios

### Quality Gates

```bash
# Must pass before PR
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features --locked
scripts/ci/run-local-ci.sh
```

## Sub-agent Coordination

### Available Sub-agents

Located in `~/.claude/agents/`:

#### document-retriever

- **Purpose**: High-signal document retrieval
- **Usage**: When searching sdd-rules/, dev-docs/
- **Delegation**: Scope paths + keywords

#### code-retriever

- **Purpose**: AST-aware code search
- **Usage**: Finding code patterns (e.g., unwrap(), console.log)
- **Delegation**: Language + AST pattern + scope

#### code-analyzer

- **Purpose**: Repository-wide rule audits
- **Usage**: Running ast-grep scan with sgconfig.yml
- **Delegation**: Rule filter + output format

#### sdd-doc-validator

- **Purpose**: SDD documentation validation and fixing
- **Usage**: Markdown validation, SDD compliance, auto-fixing
- **Delegation**: Validation type + fix mode + scope

### Delegation Decision Tree

```text
Need documents? → document-retriever
Need code patterns? → code-retriever
Need rule audit? → code-analyzer
Need markdown validation? → sdd-doc-validator
Need multiple? → Launch in parallel
```

## Validation Procedures

### Pre-Implementation Checklist

- [ ] Specification complete (no [NEEDS CLARIFICATION])
- [ ] Plan passes constitutional gates
- [ ] Tasks ordered by TDD principle
- [ ] Worktree created from origin/main
- [ ] Evidence directory prepared
- [ ] Metadata validation passes (`scripts/sdd/validate-metadata.sh`)
- [ ] Document consistency verified (`scripts/sdd/check-sdd-consistency.sh`)

### During Implementation

- [ ] Follow RED→GREEN→REFACTOR strictly
- [ ] Collect evidence continuously
- [ ] Run local CI after each phase
- [ ] Update task status in tasks.md
- [ ] Document decisions in comments

### Post-Implementation

- [ ] All tests pass
- [ ] Quality gates satisfied
- [ ] Evidence complete and linked
- [ ] PR template filled completely
- [ ] Specs updated if design changed

## Memory & State Management

### What I Track

- Current feature branch and worktree
- Active spec/plan/tasks paths
- Constitutional violations found
- Evidence artifacts created
- Test execution status

### Serena Integration

- Read memories for project context
- Update memories when SDD rules change
- Sync paths if structure changes
- Maintain consistency with constitution

### State Persistence

- Use TodoWrite for task tracking
- Reference previous decisions in specs
- Link related issues and PRs
- Maintain evidence trail

## Operational Guidelines

### Handling Ambiguity

When encountering unclear requirements:

1. Mark with `[NEEDS CLARIFICATION: specific question]`
2. Don't guess or make assumptions
3. Escalate to user for clarification
4. Document resolution in spec

### Error Recovery

- Script failures: Check permissions and paths
- Template errors: Validate placeholder replacement
- Gate violations: Document justification or simplify
- Evidence gaps: Re-run tests with logging

### Performance Optimization

- Use JSON mode for all script calls
- Batch read operations when possible
- Delegate heavy searches to sub-agents
- Cache frequently accessed templates

## Common Patterns & Anti-patterns

### ✅ Good Patterns

- Spec before plan before tasks before code
- Tests fail before implementation
- Evidence collected continuously
- Constitutional gates checked early
- Parallel tasks marked appropriately

### ❌ Anti-patterns

- Implementation before tests
- Skipping RED phase
- >3 projects without justification
- Wrapper classes around frameworks
- Missing evidence links

## Integration with CLAUDE.md Files

This document complements:

- `/CLAUDE.md` - Repository-wide Claude instructions
- `/sdd-rules/CLAUDE.md` - SDD-specific Claude rules
- `/.serena/memories/` - Serena's project knowledge

## Quick Reference

### Commands

```bash
/specify "feature description"  # Create spec
/plan "technical context"        # Generate plan
/tasks "additional context"      # Create tasks
/sdd-task <issue-number>        # Initialize from GitHub issue
```

### Key Files

- Constitution: `.specify/memory/constitution.md`
- Lifecycle: `.specify/memory/lifecycle.md`
- Alignment: `.specify/ALIGNMENT.md`
- Templates: `.specify/templates/`
- Scripts: `scripts/sdd/`

### Validation Commands

```bash
scripts/sdd/validate-sdd-docs.sh
scripts/sdd/check_language.sh
scripts/sdd/run_semantic_checks.sh
scripts/sdd/validate-metadata.sh
scripts/sdd/check-sdd-consistency.sh
scripts/ci/run-local-ci.sh
```

---

```yaml
constitution:
    version: "1.0.1"
    last_checked: "2025-09-22T15:20:00Z"
document:
    type: "codex-memory"
    path: ".specify/AGENTS.md"
    version: "1.0.4"
    last_updated: "2025-09-27T10:21:00Z"
    dependencies:
        - ".specify/memory/constitution.md"
        - ".specify/memory/lifecycle.md"
        - ".specify/memory/AI-Engineer-SDD-Workflow-Baseline-templates.txt"
        - "sdd-rules/rules/README.md"
        - ".specify/templates/spec-template.md"
        - ".specify/templates/plan-template.md"
        - ".specify/templates/tasks-template.md"
        - "./AGENTS.md"
```
