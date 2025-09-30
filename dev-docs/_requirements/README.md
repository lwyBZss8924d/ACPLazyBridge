# Requirements & Planning Documents

This directory contains high-level requirements, project plans, and detailed research for ACPLazyBridge development.

---

## Directory Structure

### Project-Wide Requirements
- **[Roadmap.md](./Roadmap.md)** - Product roadmap and milestones
- **[acp-lazybridge-requirements.md](./acp-lazybridge-requirements.md)** - Core requirements
- **[acp-lazybridge-project-plan.md](./acp-lazybridge-project-plan.md)** - Overall project plan

### Milestone 1 (0.1.0) Planning
- **[m1-issue-list.md](./m1-issue-list.md)** - Issue tracking for M1
- **[m1-technical-implementation-plan.md](./m1-technical-implementation-plan.md)** - M1 technical plan

---

## Issue/Task-Specific Requirements

### Task #040: Codex Protocol Alignment MVP
**Directory:** [040-codex-protocol-alignment-mvp/](./040-codex-protocol-alignment-mvp/)

**Status:** Requirements Complete ✅ → Ready for SDD Initialization

**Description:** Complete Codex CLI protocol support for ACP adapter

**Contents:**
- Comprehensive research report (3,500+ lines)
- Detailed gap analysis (2,500+ lines)
- ACP protocol requirements (800+ lines)
- Codex protocol analysis (1,800+ lines)
- Implementation roadmap (3 weeks)

**Key Metrics:**
- **Current Coverage:** 44% → Target: 100%
- **Missing Events:** 14 critical events
- **Code Changes:** ~1,150 lines
- **Effort:** 2-3 weeks (one developer)

**Next Action:** Run `/sdd-task 50` to initialize SDD workflow

---

## How to Use This Directory

### For Project Planning
1. Start with **Roadmap.md** for overall direction
2. Review **acp-lazybridge-requirements.md** for core needs
3. Check milestone-specific plans (m1-*, m2-*, etc.)

### For Feature Implementation
1. Navigate to task-specific directory (e.g., `040-codex-protocol-alignment-mvp/`)
2. Read the README.md for overview
3. Review research report for context
4. Use gap analysis for implementation details
5. Run `/sdd-task <number>` to initialize workflow

### For Requirements Updates
When adding new requirements or research:
1. Create directory: `<NNN>-<feature-name>/`
2. Add README.md with overview
3. Add detailed research/analysis documents
4. Update this index

---

## SDD Integration

Requirements in this directory feed into the SDD workflow:

```
dev-docs/_requirements/<NNN>-<feature>/
    ↓
/sdd-task <NNN>
    ↓
specs/<NNN>-<feature>/
    ├── spec.md      (from research report)
    ├── plan.md      (from roadmap/analysis)
    └── tasks.md     (from gap analysis)
```

---

## Document Types

### Research Reports
Comprehensive analysis of problem space:
- Background and context
- Technology evaluation
- Gap analysis
- Implementation recommendations

### Gap Analysis
Detailed comparison of current vs required state:
- What exists today
- What's missing
- Mapping tables
- Code change estimates

### Protocol Mappings
Reference documentation for protocols:
- Complete type enumerations
- Message flows
- Validation rules
- Best practices

### Roadmaps
High-level implementation plans:
- Phased approach
- Dependencies
- Timeline estimates
- Risk assessment

---

## Related Directories

- **specs/** - SDD specifications (generated from requirements)
- **dev-docs/architecture/** - Architecture documentation
- **dev-docs/references/** - Technical references
- **_artifacts/** - Evidence and test results

---

**Last Updated:** 2025-09-30T13:45:40Z