# ACPLazyBridge Documentation Structure

## Documentation Hierarchy
```
ACPLazyBridge/
├── README.md                 # Project overview (cleaned up)
├── CLAUDE.md                 # AI agent instructions
├── AGENTS.md                 # Linked to CLAUDE.md
├── CONTRIBUTING.md           # Development guidelines
├── ROADMAP.md               # Project roadmap
│
├── dev-docs/                # Development documentation
│   ├── references/          # NEW: Centralized reference hub
│   │   ├── acp.md          # Agent Client Protocol
│   │   ├── zed_ide.md      # Zed IDE integration
│   │   ├── acp_adapters/   # ACP adapter references
│   │   └── cli_agents/     # CLI agent documentation
│   │
│   ├── engineering/         # Engineering practices
│   │   ├── workflow.md     # NEW: Reference maintenance workflow
│   │   └── codeql.md       # Security analysis
│   │
│   ├── requirements/        # Project requirements
│   │   └── acp-lazybridge-requirements.md
│   │
│   ├── design/             # Architecture & design
│   │   └── acp-lazybridge-architecture.md
│   │
│   ├── plan/               # Planning documents
│   │   ├── issues/         # Task tracking
│   │   └── m1-technical-implementation-plan.md
│   │
│   └── review/             # Review artifacts
│       └── _artifacts/     # Test results, logs, evidence
│
└── local_refs/             # Vendored reference materials
    ├── agent-client-protocol/
    ├── codex/
    └── zed-acp-examples/
```

## Recent Changes (feat/add-dev-docs-hub)

### Added
1. **dev-docs/references/** - New centralized reference hub
   - Replaces informal tracking of third-party dependencies
   - All references use public, permanent URLs
   - Organized by category (protocols, agents, adapters)

2. **dev-docs/engineering/workflow.md** - Reference maintenance guide
   - How to add new references
   - Update procedures for upstream changes
   - Quality standards for documentation

### Modified
- **README.md** - Cleaned up and streamlined
  - Removed redundant information
  - Better focus on core purpose
  - Links to new reference documentation

## Documentation Standards

### Reference Documents
- Must contain only public, permanent URLs
- Should include version information where applicable
- Cross-reference with local_refs/ for vendored copies
- Update when upstream changes occur

### Development Docs
- Chinese documentation in dev-docs/ for implementation details
- English for API documentation and public interfaces
- Markdown format with clear structure
- Include examples where helpful

### AI Agent Instructions
- CLAUDE.md as primary source
- AGENTS.md links to CLAUDE.md
- Keep synchronized across AI tools
- Include workflow and quality gates

## Maintenance Workflow

### Adding New References
1. Create appropriate file in dev-docs/references/
2. Follow template structure from existing files
3. Include official URLs and documentation links
4. Update this memory if structure changes

### Updating Existing References
1. Check upstream for changes regularly
2. Update URLs if repositories move
3. Note version changes
4. Keep local_refs/ synchronized if vendored

### Quality Checks
- Verify all URLs are accessible
- Ensure no internal/private links
- Check cross-references are valid
- Update related memories in Serena