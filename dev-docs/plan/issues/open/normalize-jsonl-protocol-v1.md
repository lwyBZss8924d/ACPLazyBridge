---

Issue status: "open"
Issue number: [#14]
Issue title: Normalize JSONL fixtures to ACP v1 protocolVersion (1)
Issue URL: [Normalize JSONL fixtures to ACP v1 protocolVersion (1)](https://github.com/lwyBZss8924d/ACPLazyBridge/issues/14)
Issue type: "ci"
Issue owner: "lwyBZss8924d", "claude", "claude[bot]", "warp-agent"

---

# ISSUE: Normalize JSONL fixtures to ACP v1 protocolVersion (1)

Status: Small follow-up (to be opened on main after PR merge)

Summary
Normalize all dev-docs/review/_artifacts/tests/*.jsonl fixtures to use integer protocolVersion 1 in initialize requests. This aligns fixtures with ACP v1 and the planned CI replay runner.

Scope

- Update JSONL fixtures under dev-docs/review/_artifacts/tests/ that currently show protocolVersion as a date string (e.g., "2024-11-05").
- Ensure requests use: { "jsonrpc": "2.0", "id": 1, "method": "initialize", "params": { "protocolVersion": 1, ... } }
- Leave any clientCapabilities references intact if present (they are part of requests).

Acceptance Criteria

- All JSONL fixtures in the tests directory use integer protocolVersion 1.
- Local playback tests and documentation remain consistent and pass.
- CI replay runner (when enabled) will operate on normalized fixtures without protocol mismatches.

References

- dev-docs/plan/issues/ci-replay-acp-v1-runner.md
- dev-docs/review/_artifacts/tests/*.jsonl
