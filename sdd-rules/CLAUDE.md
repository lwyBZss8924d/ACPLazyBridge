# CLAUDE.md (Authoritative)

This document defines Claude Code–specific rules in ACPLazyBridge. It inherits the global rules from CONTRIBUTING.md and sdd-rules/AGENTS.md.

Authority and scope
- Normative authority: CONTRIBUTING.md, sdd-rules/lifecycle.md, sdd-rules/AGENTS.md
- This file provides Claude-specific clarifications and must remain consistent with the above.

Key rules (Claude Code)
- Worktree-first; branch categories: feature | fix | perf | chore | docs
- Stdout strictly JSONL; logs to stderr only
- Evidence stored under dev-docs/review/_artifacts/{tests,logs,jq,reports}/<task>/
- Non-interactive permission mapping (typical defaults): approval_policy=never; sandbox_mode per task; network access only when explicitly required
- Protocol examples MUST use ACP v1: "protocolVersion": 1

Submission checklist (Claude PRs)
- Links to Spec/Plan/Tasks (specs/<NNN>-<slug>/)
- Evidence links (tests/logs/jq/reports)
- Risks/rollback section
- CI summary (fmt/clippy/test/replay)

References
- CONTRIBUTING.md
- sdd-rules/AGENTS.md
- sdd-rules/lifecycle.md

