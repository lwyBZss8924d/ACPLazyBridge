---

Issue status: "waiting"
Issue number: [ # ]
Issue title: CI Replay Runner for ACP v1 Strict Tests
Issue URL: [CI Replay Runner for ACP v1 Strict Tests](https://github.com/lwyBZss8924d/ACPLazyBridge/issues/#)
Issue type: "ci"
Issue owner: "lwyBZss8924d", "claude", "claude[bot]", "warp-agent"

---

# ISSUE: CI Replay Runner for ACP v1 Strict Tests

Status: Draft (do not schedule until core features land)

Summary
- Build a deterministic JSONL replay runner for ACP v1 to run in CI.
- Normalize protocolVersion usage to strict ACP v1 (integer 1) in all CI replay fixtures.
- Validate agent outputs against ACP v1 schemas and behavior checks (jq and JSON Schema).

Why
- We currently have JSONL examples used for local playback and evidence, but they mix versions of initialize params (date-string vs v1 integer) and are not wired into CI.
- A standardized CI replay ensures protocol compliance regressions are caught early and reproducibly.

Scope
- Add a small binary (or test harness) that:
  - Reads a JSONL request script (client → agent) with ACP v1 messages
  - Feeds it to the agent over stdio
  - Captures all responses and notifications to JSONL logs
  - Validates the transcript:
    * initialize response: protocolVersion == 1, agentCapabilities shape
    * session/new params validation errors/success
    * session/update event shapes: AgentMessageChunk, ToolCall, ToolCallUpdate
    * error codes and JSON-RPC 2.0 envelope correctness
  - Emits machine-friendly summary and exits non-zero on failure

Requirements
- Fixtures:
  - Store under dev-docs/review/_artifacts/tests-ci/*.jsonl
  - All initialize params use { "protocolVersion": 1 }
- Validation:
  - JSON Schema validation for known responses and notifications
  - jq assertions for counts and presence (e.g., number of ToolCallUpdate completed == expected)
- GH Actions integration:
  - New workflow job (matrix on macos-latest and ubuntu-latest):
    * cargo build --workspace
    * Run replay harness against each *.jsonl fixture
    * Upload logs on failure as artifacts
- Security:
  - No secrets consumed; runner must not access network or filesystem beyond working dir unless mocked

Acceptance Criteria
- CI job fails if any fixture run violates ACP v1 initialize, session/new, or session/update schema or expected behaviors
- CI job prints concise failure diff and stores the full transcript under artifacts
- Docs: Add a short README with how to run locally via `cargo test -p codex-cli-acp -- --ignored ci:replay`

Out of Scope (for this issue)
- Adding or refactoring agent features; this is a harness and fixtures task only
- Non-v1 protocol fixtures

Open Questions
- Where to locate schemas for validation (vendored vs generated)?
- Single unified replay binary vs. a test-only harness behind `#[ignore]`

Next Steps
- Confirm fixture set and expected outcomes
- Implement replay harness (Rust or minimal Node/Python if preferred), prefer Rust for easy CI integration
- Wire GH Actions job (allow-failure initially until stabilized)

