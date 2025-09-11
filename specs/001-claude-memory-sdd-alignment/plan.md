# Implementation Plan: Claude Memory and SDD Rules Index Alignment

## Metadata
- Issue-URI: https://github.com/lwyBZss8924d/ACPLazyBridge/issues/26
- Plan-URI: specs/001-claude-memory-sdd-alignment/plan.md
- Evidence-URIs: dev-docs/review/_artifacts/logs/001-claude-memory-sdd-alignment/

## Architecture Overview

### Component Structure
```
Repository Root
├── CLAUDE.md (repository-level memory - updated)
├── crates/
│   ├── acp-lazy-core/
│   │   └── CLAUDE.md (core module memory - new)
│   └── codex-cli-acp/
│       └── CLAUDE.md (CLI adapter memory - new)
├── scripts/
│   └── CLAUDE.md (scripts memory - new)
└── sdd-rules/
    ├── AGENTS.md (updated with rules link)
    ├── WARP.md (updated with rules link)
    └── rules/
        └── README.md (rules index - new)
```

### Information Hierarchy
1. **Root CLAUDE.md**: Repository governance, SDD workflow, authority chain
2. **Module CLAUDE.md**: Local conventions, build/test specifics, inherits from root
3. **Rules Index**: Centralized navigation, categorized by domain

## Technical Design

### Root CLAUDE.md Structure
```markdown
# CLAUDE.md

## Repository Overview
[Project description and purpose]

## Authority and Governance
- Normative: CONTRIBUTING.md, sdd-rules/spec-driven.md, sdd-rules/lifecycle.md
- Team Rules: sdd-rules/AGENTS.md
- Rules Index: sdd-rules/rules/README.md

## SDD Developer Team Workflow
[Specification-driven development process]

## Project Navigation
[Directory structure and key paths]

## ACP Protocol Implementation
- Version: 1 (integer)
- Examples: [Corrected JSONL examples]

## Quality Gates
[CI checks, testing requirements]
```

### Module CLAUDE.md Template
```markdown
# CLAUDE.md - [Module Name]

## Module Overview
[Purpose and responsibilities]

## Parent Rules
Inherits from: [parent CLAUDE.md]

## Local Conventions
- Build: [specific commands]
- Test: [test approach]
- Evidence: dev-docs/review/_artifacts/[module]/

## Protocol Specifics
[Module-specific protocol details]
```

### Rules Index Structure
```markdown
# SDD Rules Index

## Categories

### Git Workflow
- [worktree/sdd-rules-worktrees.md](git/worktree/sdd-rules-worktrees.md)
- [commit/sdd-rules-commit.md](git/commit/sdd-rules-commit.md)

### API Design
- [guidelines/sdd-rules-api-guidelines.md](api/guidelines/sdd-rules-api-guidelines.md)

[Additional categories...]
```

## Implementation Steps

### Phase 1: Documentation Updates
1. Update root CLAUDE.md with SDD workflow integration
2. Create module-specific CLAUDE.md files
3. Create rules index README.md

### Phase 2: Cross-References
1. Update AGENTS.md with rules index link
2. Update WARP.md with rules index link
3. Verify all internal links resolve

### Phase 3: Validation
1. Run SDD structure lint
2. Run language policy check
3. Verify protocol examples use integer 1
4. Store evidence artifacts

## Testing Strategy

### Local Validation
```bash
# Structure lint
scripts/ci/run-sdd-structure-lint.sh

# Language policy
scripts/ci/check-language-policy.sh

# Link validation (manual)
grep -r "sdd-rules/rules/README.md" sdd-rules/
```

### Evidence Collection
- Lint output: dev-docs/review/_artifacts/logs/001-claude-memory-sdd-alignment/lint.log
- Language check: dev-docs/review/_artifacts/logs/001-claude-memory-sdd-alignment/language.log
- Link validation: dev-docs/review/_artifacts/tests/001-claude-memory-sdd-alignment/links.log

## Trade-offs and Decisions

### Decision: Hierarchical CLAUDE.md Structure
- Pro: Clear inheritance, module-specific context
- Con: Multiple files to maintain
- Rationale: Aligns with Claude Code's memory hierarchy model

### Decision: Centralized Rules Index
- Pro: Single navigation point, easier discovery
- Con: Requires maintenance when rules change
- Rationale: Improves developer and agent efficiency

## Rollback Plan
1. Revert branch merge if issues detected
2. CLAUDE.md files are documentation-only, no runtime impact
3. Rules index is additive, no breaking changes

## Success Metrics
- All CI checks pass
- No broken internal links
- Consistent protocol version usage (integer 1)
- Evidence artifacts properly stored
