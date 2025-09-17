# Constitution Update Checklist

When amending the constitution (`.specify/memory/constitution.md`), ensure all
dependent documents are updated to maintain consistency.

## Templates to Update

### When adding/modifying ANY article

- [ ] `.specify/templates/plan-template.md` - Update Constitution Check section
- [ ] `.specify/templates/spec-template.md` - Update if requirements/scope affected
- [ ] `.specify/templates/tasks-template.md` - Update if new task types needed
- [ ] `/.claude/commands/plan.md` - Update if planning process changes
- [ ] `/.claude/commands/tasks.md` - Update if task generation affected
- [ ] `/CLAUDE.md` - Update runtime development guidelines
- [ ] `/WARP.md` - Update runtime development guidelines
- [ ] `/AGENTS.md` - Update runtime development guidelines (if have any)

### Article-specific updates

#### Article I (Library-First)

- [ ] Ensure templates emphasize library creation
- [ ] Update CLI command examples
- [ ] Add llms.txt documentation requirements

#### Article II (CLI Interface)

- [ ] Update CLI flag requirements in templates
- [ ] Add text I/O protocol reminders

#### Article III (Test-First)

- [ ] Update test order in all templates
- [ ] Emphasize TDD requirements
- [ ] Add test approval gates

#### Article IV (Integration Testing)

- [ ] List integration test triggers
- [ ] Update test type priorities
- [ ] Add real dependency requirements

#### Article V (Observability)

- [ ] Add logging requirements to templates
- [ ] Include multi-tier log streaming
- [ ] Update performance monitoring sections

#### Article VI (Versioning)

- [ ] Add version increment reminders
- [ ] Include breaking change procedures
- [ ] Update migration requirements

#### Article VII (Simplicity)

- [ ] Update project count limits
- [ ] Add pattern prohibition examples
- [ ] Include YAGNI reminders

## CLAUDE.md Files to Update

### All CLAUDE.md locations (12 files)

- [ ] `/CLAUDE.md` - Root project guidance
- [ ] `/.github/CLAUDE.md` - GitHub-specific guidance
- [ ] `/.specify/CLAUDE.md` - SDD operational context
- [ ] `/sdd-rules/CLAUDE.md` - Claude-specific rules
- [ ] `/scripts/CLAUDE.md` - Script automation guidance
- [ ] `/crates/CLAUDE.md` - Rust workspace guidance
- [ ] `/crates/acp-lazy-core/CLAUDE.md` - Core library guidance
- [ ] `/crates/codex-cli-acp/CLAUDE.md` - Adapter guidance
- [ ] `/dev-docs/CLAUDE.md` - Development documentation guidance
- [ ] `/specs/CLAUDE.md` - Specification guidance
- [ ] `/queries/CLAUDE.md` - Query/CodeQL guidance
- [ ] `/dev-docs/review/_artifacts/CLAUDE.md` - Evidence guidance

### CLAUDE.md Sync Requirements

- [ ] All files have consistent metadata format
- [ ] Constitution version matches across all files
- [ ] Last Updated dates are current
- [ ] Authority references are correct
- [ ] Cross-references between CLAUDE.md files are valid
- [ ] Run metadata validation: `scripts/sdd/validate-metadata.sh`
- [ ] Run consistency check: `scripts/sdd/check-sdd-consistency.sh`

## Validation Steps

1. Before committing constitution changes:
   - [ ] All templates reference new requirements
   - [ ] Examples updated to match new rules
   - [ ] No contradictions between documents
   - [ ] All CLAUDE.md files updated with new Constitution version

2. After updating templates:
   - [ ] Run through a sample implementation plan
   - [ ] Verify all constitution requirements addressed
   - [ ] Check that templates are self-contained (readable without constitution)
   - [ ] Run metadata validation: `./scripts/sdd/validate-metadata.sh`
   - [ ] Run consistency check: `./scripts/sdd/check-sdd-consistency.sh`

3. Version tracking:
   - [ ] Update constitution version number
   - [ ] Note version in template footers
   - [ ] Add amendment to constitution history
   - [ ] Update all CLAUDE.md file metadata

## Automated Validation

### Metadata Management Tools

The following tools ensure SDD documentation consistency:

#### Metadata Validation

```bash
# Validate all document metadata
./scripts/sdd/validate-metadata.sh

# Check specific file
./scripts/sdd/validate-metadata.sh --file CLAUDE.md --verbose

# Check constitution version consistency
./scripts/sdd/validate-metadata.sh --check-consistency --strict

# Output JSON for CI
./scripts/sdd/validate-metadata.sh --format json > validation-report.json
```

#### Document Querying

```bash
# Find all Claude memory files
./scripts/sdd/query-metadata.sh --type claude-memory

# Find outdated documents (not updated in 30 days)
./scripts/sdd/query-metadata.sh --outdated 30

# Find documents with old constitution version
./scripts/sdd/query-metadata.sh --constitution-version 1.0.0
```

#### Consistency Checking

```bash
# Full consistency check with details
./scripts/sdd/check-sdd-consistency.sh --verbose

# Generate JSON consistency report
./scripts/sdd/check-sdd-consistency.sh --format json > consistency-report.json

# Check without dependency validation (faster)
./scripts/sdd/check-sdd-consistency.sh --no-dependencies
```

#### Metadata Migration

```bash
# Dry run to preview changes
./scripts/sdd/migrate-to-yaml-metadata.sh --dry-run

# Migrate all files to unified YAML format
./scripts/sdd/migrate-to-yaml-metadata.sh

# Migrate specific file
./scripts/sdd/migrate-to-yaml-metadata.sh --file sdd-rules/AGENTS.md
```

### CI Integration

The validation script is automatically run in CI:

- On every PR that modifies Constitution or CLAUDE.md files
- After merge to main branch
- During scheduled weekly consistency checks

## Common Misses

- Outdated links in AGENTS.md and WARP.md
- Missing addition in rules index (sdd-rules/rules/README.md)
- Language policy violations in normative artifacts
- Markdown lint issues: multiple H1s, missing blank lines, long lines without breaks
- Inconsistent CLAUDE.md metadata formats across directories
- Missing Constitution version updates in CLAUDE.md files
- Command documentation (`/commands/*.md`)
- Checklist items in templates
- Example code/commands
- Domain-specific variations (web vs mobile vs CLI)
- Cross-references between documents

## Template Sync Status

Last sync check: 2025-09-17

- Constitution version: 1.0.1
- Templates aligned: ✓ All templates updated to Constitution 1.0.1
- Commands aligned: ✓ All .specify/commands updated with frontmatter
- Claude commands enabled: ✓ Available in .claude/commands
- Scripts vendored: ✓ upstream/lib/common.sh from spec-kit
- Language check coverage: ✓ Extended to all normative directories
- CLAUDE.md files synchronized: ✓ All 12 files synchronized with Constitution 1.0.1

---

_This checklist ensures the constitution's principles are consistently applied
across all project documentation._

---

```yaml
constitution:
    version: "1.0.1"
    last_checked: "2025-09-17T12:00:00Z"
document:
    type: "constitution-checklist"
    path: ".specify/memory/constitution_update_checklist.md"
    version: "1.0.1"
    last_updated: "2025-09-17T12:00:00Z"
    last_sync_check: "2025-09-17"
    dependencies:
        - ".specify/memory/constitution.md"
```
