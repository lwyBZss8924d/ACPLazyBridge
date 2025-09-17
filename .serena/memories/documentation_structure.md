# ACPLazyBridge Documentation Structure

## Documentation Hierarchy

```tree
ACPLazyBridge/
├── README.md                 # Project overview
├── CLAUDE.md                 # Claude Code agent instructions (normative)
├── AGENTS.md                 # Links to sdd-rules/AGENTS.md
├── WARP.md                   # WARP agent instructions
├── CONTRIBUTING.md           # Engineering ground rules (normative)
├── ROADMAP.md               # Project roadmap
│
├── sdd-rules/               # NORMATIVE: SDD governance
│   ├── spec-driven.md       # Core SDD principles
│   ├── lifecycle.md         # SDD lifecycle phases
│   ├── AGENTS.md           # Team AI agents rules
│   ├── commands/           # SDD command docs
│   │   ├── specify.md      # /specify command
│   │   ├── plan.md         # /plan command
│   │   └── tasks.md        # /tasks command
│   ├── templates/          # SDD templates
│   │   ├── spec-template.md
│   │   ├── plan-template.md
│   │   └── tasks-template.md
│   └── rules/              # Categorized rules
│       ├── README.md       # Rules index
│       ├── documentation-style/
│       │   ├── sdd-rules-documentation-style.md
│       │   ├── Google-developer-documentation-style-guide.md
│       │   └── markdownlint-config.md
│       ├── git/
│       │   ├── worktree/   # Worktree management
│       │   ├── pr/         # Pull request rules
│       │   ├── issues/     # Issue tracking
│       │   └── comments/   # Commit messages
│       ├── ci/             # CI/CD requirements
│       ├── tests/          # Testing standards
│       ├── code-analysis/  # Code quality
│       ├── tools-cli/      # CLI guidelines
│       ├── tools-mcp/      # MCP guidelines
│       ├── research/       # Research methodology
│       └── changelog/      # Changelog standards
│
├── specs/                   # SDD specifications
│   ├── 000-example/        # Example spec structure
│   └── <NNN>-<slug>/       # Feature specifications
│       ├── spec.md         # Specification
│       ├── plan.md         # Technical plan
│       └── tasks.md        # Task breakdown
│
├── dev-docs/               # Development documentation
│   ├── references/         # Centralized reference hub
│   │   ├── acp.md         # Agent Client Protocol
│   │   ├── zed_ide.md     # Zed IDE integration
│   │   ├── acp_adapters/  # ACP adapter references
│   │   │   └── claude_code_acp.md
│   │   └── cli_agents/    # CLI agent documentation
│   │       ├── ClaudeCode/
│   │       ├── codex.md
│   │       └── gemini.md
│   │
│   ├── engineering/        # Engineering practices (non-normative)
│   │   ├── workflow.md    # Reference maintenance
│   │   └── codeql.md      # Security analysis
│   │
│   ├── requirements/       # Project requirements
│   │   └── acp-lazybridge-requirements.md
│   │
│   ├── design/            # Architecture & design
│   │   └── acp-lazybridge-architecture.md
│   │
│   ├── plan/              # Planning documents
│   │   ├── issues/        # Task tracking
│   │   └── m1-technical-implementation-plan.md
│   │
│   ├── review/            # Review artifacts
│   │   └── _artifacts/    # Evidence storage
│   │       ├── tests/     # Test scenarios
│   │       ├── logs/      # Execution logs
│   │       ├── jq/        # JSON filters
│   │       ├── reports/   # Test reports
│   │       ├── IMPL.csv   # Symbol mapping
│   │       └── traceability.csv
│   │
│   └── zh-CN/             # Chinese documentation (non-normative)
│       └── (with disclaimer)
│
└── scripts/               # Automation scripts
    ├── ci/               # CI scripts
    └── sdd/              # SDD validation
```

## Documentation Categories

### Normative Documents (Authoritative)

These documents define the rules and must be followed:

1. **CLAUDE.md** - Primary AI agent instructions
2. **CONTRIBUTING.md** - Engineering ground rules
3. **sdd-rules/** - All SDD governance documents
   - `spec-driven.md` - Core principles
   - `lifecycle.md` - Development phases
   - `rules/` - Categorized rules
4. **specs/** - Feature specifications

### Non-Normative Documents (Informative)

These provide guidance but aren't authoritative:

1. **dev-docs/engineering/** - Links back to normative sources
2. **dev-docs/zh-CN/** - Chinese documentation with disclaimer
3. **dev-docs/references/** - Vendored external documentation

## SDD Documentation Requirements

### Specification Documents (`specs/<NNN>-<slug>/`)

1. **spec.md** - Feature specification
   - Overview and context
   - Requirements (functional/non-functional)
   - User stories
   - Acceptance criteria

2. **plan.md** - Technical plan
   - Architecture decisions
   - Component design
   - Integration points
   - Risk assessment

3. **tasks.md** - Task breakdown
   - Executable task list
   - Dependencies
   - Time estimates

### Evidence Documents (`dev-docs/review/_artifacts/`)

For each task, maintain:

- `tests/` - JSONL test scenarios
- `logs/` - Execution logs with timestamps
- `reports/` - Test results and coverage
- `IMPL.csv` - Symbol to requirement mapping
- `traceability.csv` - Requirement verification

## Documentation Standards

### Markdown Style

Enforced by `.markdownlint.json`:

- Consistent heading levels
- Proper list formatting
- Code block language tags
- No trailing whitespace
- Line length limits (where applicable)

### Language Policy

**English Required (Normative)**:

- Specifications (`specs/`)
- Plans and tasks
- Issues and PRs
- Commit messages
- Code comments
- SDD rules

**Any Language (Non-Normative)**:

- Development notes
- Team discussions
- Chinese docs under `dev-docs/zh-CN/`

### Cross-References

- Use relative links for internal docs
- Include version/date in external references
- Link non-normative docs back to authority
- Maintain reference hub in `dev-docs/references/`

## Maintenance Workflow

### Adding Documentation

1. Determine if normative or non-normative
2. Place in appropriate directory
3. Follow templates if available
4. Add to relevant index/README
5. Update serena memories if structural

### Updating Documentation

1. Check if document is normative
2. If normative, follow SDD process
3. Update version and date stamps
4. Verify cross-references still valid
5. Run markdown lint checks

### Quality Checks

```bash
# Markdown style validation
markdownlint . --config .markdownlint.json

# SDD structure validation
scripts/ci/run-sdd-structure-lint.sh

# Language policy check
scripts/ci/check-language-policy.sh

# Full documentation check
scripts/ci/run-local-ci.sh
```

## Key Documentation Paths

- **SDD Rules**: `sdd-rules/rules/`
- **Specifications**: `specs/<NNN>-<slug>/`
- **References**: `dev-docs/references/`
- **Evidence**: `dev-docs/review/_artifacts/<task>/`
- **Templates**: `sdd-rules/templates/`

---

```yaml
constitution:
    version: "1.0.1"
    last_checked: "2025-09-17T04:32:00Z"
document:
    type: "serena-memories"
    memories: "documentation_structure"
    status: "expired"
    path: ".serena/memories/documentation_structure.md"
    version: "1.0.1"
    last_updated: "2025-09-14T08:26:00Z"
```
