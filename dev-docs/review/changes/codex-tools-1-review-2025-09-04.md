# codex-tools-1 Review Report — ACP v1 Alignment, Codex CLI Consistency, Best Practices

Date: 2025-09-04
Branch: feature/codex-tools-1 (scoped PR based on main)
Reviewer: Agent Mode (Warp)

Summary

- Verified ACP v1 protocol compliance in initialize/session/update.
- Confirmed Codex CLI tool definitions mapping (ShellToolCallParams) and lifecycle alignment.
- Compared against claude-code-acp best practices; validated minimal updates, dedupe, and error mapping.
- Quality gates: rustfmt OK, clippy OK, tests OK across workspace.

Highlights

- Initialize response
  - protocolVersion: 1 (integer)
  - agentCapabilities with nested promptCapabilities
  - authMethods included (empty)
  - No fs capability advertised in response
- Session/new validation
  - Requires cwd (absolute path), workingDirectory supported as fallback
  - mcpServers required and must be array
- Session/update
  - Single update object (sessionUpdate) per notification
  - ToolCall (initial) and ToolCallUpdate (subsequent) used appropriately
  - Minimal updates: omit title/kind on updates; dedupe identical status without new output
- Codex CLI consistency
  - extract_shell_command supports string and Vec<String> command with joining
  - extract_shell_params with workdir/cwd/working_directory, timeout_ms/timeout, with_escalated_permissions/sudo, justification/reason
  - raw_output preserves full result; content uses ~2KB UTF-8-safe preview
- Error mapping
  - Codex error codes mapped to semantic categories and surfaced as ToolCallUpdate (failed) when in tool context; otherwise as message chunk

Quality verification

- cargo fmt --check: PASS
- cargo clippy --workspace --all-targets --all-features -D warnings: PASS
- cargo test --workspace --all-features: PASS
  - acp-lazy-core: 19 passed
  - codex-cli-acp unit: 7 passed
  - codex-cli-acp main: 11 passed
  - playback: 5 passed
  - session_update_format: 8 passed
  - tool_calls integration: 8 passed

Scope for merge (to main)

- Include only:
  - crates/codex-cli-acp/**
  - dev-docs/** (issues, reviews, alignment records)
- Exclude unrelated files outside this scope (e.g., .claude symlink, AGENTS.md removal, top-level config unrelated to this task)

Follow-ups (separate issues on main after merge)

1) Normalize JSONL fixtures to use protocolVersion: 1 across dev-docs/review/_artifacts/tests/*.jsonl to align with ACP v1 and CI replay runner.
2) Refresh user-facing docs examples (e.g., WARP.md, CLAUDE.md) to show protocolVersion as integer 1 for clarity.

References

- codex-cli-acp/src/main.rs — initialize/session handlers and tests
- codex-cli-acp/src/codex_proto.rs — ToolCall/ToolCallUpdate mapping, error mapping, streaming
- codex-cli-acp/src/tool_calls.rs — UTF-8 preview truncation, ExtractedShellParams
- dev-docs/plan/issues/acp-v1-codex-cli-alignment-and-best-practices.md — acceptance criteria
- dev-docs/plan/issues/ci-replay-acp-v1-runner.md — future CI replay normalization
