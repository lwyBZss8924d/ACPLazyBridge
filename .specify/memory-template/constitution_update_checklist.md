This file is [PROJECT_NAME] Constitution Update Checklist Template.
The normative Constitution Update Checklist lives at:

- dev-docs/sdd/constitution_update_checklist.md

Notes

- constitution_update_checklist.md is SDD Core Principles for the constitution.md
- Keep this template file as a pointer to avoid link rot in older references.
- All new references must point to: (dev-docs/sdd/constitution_update_checklist.md)
- Template source: [spec-kit-template-v0.0.30](https://github.com/github/spec-kit/releases/tag/v0.0.30)
- Constitution version: 0.0.30
- Constitution Update Checklist Template version: 0.0.30
- Template Last Updated: 2025-09-15

---

# Constitution Update Checklist

When amending the constitution (`/memory/constitution.md`), ensure all dependent
documents are updated to maintain consistency.

## Templates to Update

### When adding/modifying ANY article

- [ ] `/templates/plan-template.md` - Update Constitution Check section
- [ ] `/templates/spec-template.md` - Update if requirements/scope affected
- [ ] `/templates/tasks-template.md` - Update if new task types needed
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

## Validation Steps

1. **Before committing constitution changes:**
   - [ ] All templates reference new requirements
   - [ ] Examples updated to match new rules
   - [ ] No contradictions between documents

2. **After updating templates:**
   - [ ] Run through a sample implementation plan
   - [ ] Verify all constitution requirements addressed
   - [ ] Check that templates are self-contained (readable without constitution)

3. **Version tracking:**
   - [ ] Update constitution version number
   - [ ] Note version in template footers
   - [ ] Add amendment to constitution history

## Common Misses

Watch for these often-forgotten updates:

- Command documentation (`/commands/*.md`)
- Checklist items in templates
- Example code/commands
- Domain-specific variations (web vs mobile vs CLI)
- Cross-references between documents

## Template Sync Status

Last sync check: 2025-07-16

- Constitution version: 0.0.30
- Templates aligned: ❌ (missing versioning, observability details)

---

_This checklist ensures the constitution's principles are consistently applied
across all project documentation._
