# Spec-driven (Authoritative)

Principles
- Specification-first: WHAT/WHY (spec.md) precedes HOW (plan.md) and execution (tasks.md)
- Contracts-first where practical; write/agree on validations early (jq checks, JSON schema, contract tests)
- Single model representation; avoid unnecessary abstraction
- Integration-first (prefer real dependencies where practical)

Traceability
- Maintain forward/back links among Spec/Plan/Tasks/Issue
- Capture evidence under dev-docs/review/_artifacts/{tests,logs,jq,reports}/<task>/ and link into Spec/Plan/Tasks

Templates
- sdd-rules/spec-template.md
- sdd-rules/plan-template.md
- sdd-rules/tasks-template.md

