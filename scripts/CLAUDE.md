# CLAUDE.md (scripts/)

Authority
- See ../sdd-rules/CLAUDE.md and ../sdd-rules/AGENTS.md

Scripts overview
- ci/: CI helpers (SDD structure lint, language policy)
- sdd/: SDD document checks (language, lint_docs, semantic checks)
- setup-plan.sh: bootstrap a new feature plan/specs using templates
- common.sh, get-feature-paths.sh: shared utilities

Usage
- Do not echo secrets; pass via environment variables
- Prefer non-interactive, non-paged commands

References
- sdd-rules/commands/specify.md, sdd-rules/commands/plan.md, sdd-rules/commands/tasks.md

