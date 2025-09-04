# ISSUE: Refresh docs examples to ACP v1 protocolVersion (1)

Status: Small follow-up (to be opened on main after PR merge)

Summary
Refresh user-facing documentation examples to use integer protocolVersion 1 (instead of string dates). This improves clarity and prevents confusion for ACP v1 clients.

Scope
- Update examples in WARP.md, CLAUDE.md, and any other docs where initialize/session examples include protocolVersion.
- Ensure examples show: { "jsonrpc": "2.0", "id": 1, "method": "initialize", "params": { "protocolVersion": 1 } }

Acceptance Criteria
- All referenced docs display ACP v1 examples with integer protocolVersion.
- No extraneous semantic changes to docs outside example blocks.

References
- dev-docs/plan/issues/ci-replay-acp-v1-runner.md
- dev-docs/review/changes/codex-tools-1-review-2025-09-04.md
