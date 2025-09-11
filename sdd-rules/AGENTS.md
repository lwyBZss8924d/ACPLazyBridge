# AGENTS.md (Authoritative)

This document defines the unified development rules for human engineers and AI coding agents (Claude Code, WARP, Gemini, Codex, etc.). It complements CONTRIBUTING.md and sdd-rules/lifecycle.md.

Authority and scope
- Normative authority:
  - CONTRIBUTING.md (engineering ground rules)
  - sdd-rules/lifecycle.md (SDD-driven lifecycle)
- This file applies to all contributors (human and AI). Agent-specific files (e.g., CLAUDE.md, WARP.md) must align with this file.

Language policy
- All normative artifacts (PRDs/specs/plans/issues/templates) MUST be in English.
- Chinese documents are allowed as non-normative references under dev-docs/zh-CN/ with a disclaimer.

Branch and worktree policy
- Branch categories (canonical): feature | fix | perf | chore | docs (kebab-case)
- Create a new worktree and branch from origin/main (worktree-first):
  git -C /Users/arthur/dev-space/ACPLazyBridge worktree add /Users/arthur/dev-space/acplb-worktrees/<task-dir> origin/main -b <branch>
- Optional IDE navigation:
  ln -sfn /Users/arthur/dev-space/acplb-worktrees/<task-dir> /Users/arthur/dev-space/ACPLazyBridge/.worktrees/<task-dir>

SDD compliance (must do per task)
- Create specs/<NNN>-<slug>/:
  - spec.md (WHAT/WHY; requirements and acceptance)
  - plan.md (technical plan; architecture and trade-offs)
  - tasks.md (subtasks; responsibilities; test plan)
- Metadata block at the top of each file (and mirrored in the GitHub Issue body):
  - Issue-URI: <link to the GitHub issue>
  - Spec-URI / Plan-URI / Tasks-URI: <self links>
  - Evidence-URIs: dev-docs/review/_artifacts/{tests|logs|jq|reports}/<task>/...
- PR description MUST include links to Spec/Plan/Tasks, evidence (tests/logs/jq/reports), risks/rollback, and CI summary.

Quality gates
- cargo fmt --all -- --check
- cargo clippy --workspace --all-targets --all-features -- -D warnings
- cargo test --workspace --all-features --locked
- Protocol JSONL scenarios (if present) replay cleanly; stdout is valid JSONL
- Code scanning via GitHub Code Scanning is enabled; local custom CodeQL queries are allowed (see dev-docs/engineering/codeql.md)

Evidence & traceability
- Evidence root: dev-docs/review/_artifacts/{tests,logs,jq,reports}/<task>/
- Maintain traceability files (when used by the task):
  - dev-docs/review/_artifacts/IMPL.csv (symbol → file:line → mapped IDs)
  - dev-docs/review/_artifacts/traceability.csv (REQ/ARC ↔ SPEC/CODEX/ZED status)

Protocol discipline (ACP)
- stdout strictly JSONL; logs to stderr
- ACP v1 examples MUST use "protocolVersion": 1
- Use standard JSON-RPC 2.0 error codes; include descriptive messages and optional data

Security
- Never log secrets; use environment variables and GitHub secrets
- Avoid running untrusted code or scripts without review

References
- CONTRIBUTING.md
- sdd-rules/lifecycle.md
- sdd-rules/spec-template.md, sdd-rules/plan-template.md, sdd-rules/tasks-template.md

