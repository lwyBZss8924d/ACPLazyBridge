This file is [PROJECT_NAME] Constitution Template.
The ratified, normative Constitution lives at:

- dev-docs/sdd/constitution.md

Notes

- constitution.md is SDD Core Principles with the constitution_update_checklist.md
- Keep this template file as a pointer to avoid link rot in older references.
- All new references must point to: (dev-docs/sdd/constitution_update_checklist.md)
- Template source: [spec-kit-template-v0.0.30](https://github.com/github/spec-kit/releases/tag/v0.0.30)
- Constitution version: 0.0.30
- Constitution Update Checklist Template version: 0.0.30
- Template Last Updated: 2025-09-15

---

# [PROJECT_NAME] Constitution
<!-- Example: Spec Constitution, TaskFlow Constitution, etc. -->

## Core Principles

### [PRINCIPLE_1_NAME]
<!-- Example: I. Library-First -->
[PRINCIPLE_1_DESCRIPTION]
<!-- Example: Every feature starts as a standalone library; Libraries must be
self-contained, independently testable, documented;
Clear purpose required - no organizational-only libraries -->

### [PRINCIPLE_2_NAME]
<!-- Example: II. CLI Interface -->
[PRINCIPLE_2_DESCRIPTION]
<!-- Example: Every library exposes functionality via CLI; Text in/out protocol:
stdin/args → stdout, errors → stderr; Support JSON + human-readable formats -->

### [PRINCIPLE_3_NAME]
<!-- Example: III. Test-First (NON-NEGOTIABLE) -->
[PRINCIPLE_3_DESCRIPTION]
<!-- Example: TDD mandatory: Tests written → User approved → Tests fail → Then
implement; Red-Green-Refactor cycle strictly enforced -->

### [PRINCIPLE_4_NAME]
<!-- Example: IV. Integration Testing -->
[PRINCIPLE_4_DESCRIPTION]
<!-- Example: Focus areas requiring integration tests: New library contract tests,
Contract changes, Inter-service communication, Shared schemas -->

### [PRINCIPLE_5_NAME]
<!-- Example: V. Observability, VI. Versioning & Breaking Changes
VII. Simplicity -->
[PRINCIPLE_5_DESCRIPTION]
<!-- Example: Text I/O ensures debuggability; Structured logging required;
Or: MAJOR.MINOR.BUILD format; Or: Start simple, YAGNI principles -->

## [SECTION_2_NAME]
<!-- Example: Additional Constraints, Security Requirements,
Performance Standards, etc. -->

[SECTION_2_CONTENT]
<!-- Example: Technology stack requirements, compliance standards,
deployment policies, etc. -->

## [SECTION_3_NAME]
<!-- Example: Development Workflow, Review Process, Quality Gates, etc. -->

[SECTION_3_CONTENT]
<!-- Example: Code review requirements, testing gates, 
eployment approval process, etc. -->

## Governance
<!-- Example: Constitution supersedes all other practices; Amendments require
documentation, approval, migration plan -->

[GOVERNANCE_RULES]
<!-- Example: All PRs/reviews must verify compliance; Complexity must be
justified; Use [GUIDANCE_FILE] for runtime development guidance -->

```yaml
Version: [CONSTITUTION_VERSION] 
Ratified: [RATIFICATION_DATE]
Last Amended: [LAST_AMENDED_DATE]
Redirected: [REDIRECTED_DATE]
Last sync check: [LAST_SYNC_CHECK_DATE]
```
