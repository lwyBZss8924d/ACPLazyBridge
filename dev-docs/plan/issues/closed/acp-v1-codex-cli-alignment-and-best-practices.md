# ACPLB: codex-cli-acp Alignment with ACP v1, Codex CLI, and claude-code-acp Best Practices

Status: Proposed
Owner: ACPLazyBridge
Blocking: Gate codex-proto-1 PR merge until this issue is completed

Summary
Align codex-cli-acp with:
- ACP v1 protocol (responses, requests, and notifications)
- Codex CLI tool definitions and event semantics (codex-rs)
- Best practices from claude-code-acp agent client adapter

This ensures strict compatibility with ACP IDE clients (e.g., Zed), minimizes integration surprises with Codex backends, and standardizes behavior for tool calls and streaming updates.

A. ACP v1 Protocol Alignment (Spec compliance)
Recommendations
1) initialize response
- protocolVersion must be integer 1
- Use agentCapabilities (not capabilities) and nest promptCapabilities inside it
- Include authMethods (empty array if none)
- Do not expose fs capability in the agent’s initialize response

2) session/new params
- Accept cwd (absolute path required) per ACP schema
- Support workingDirectory as a fallback alias only; reject if both missing
- Validate cwd is absolute; validate mcpServers is an array when present

3) Notifications: session/update
- Use a single update object with discriminator field (e.g., sessionUpdate)
- Provide AgentMessageChunk for agent text, ToolCall for initial tool start, ToolCallUpdate for subsequent state changes

4) Methods scope
- Remove fs/* request handlers on the agent side (they are client methods in ACP)

Acceptance Criteria
- initialize response exactly matches ACP v1 (protocolVersion=1, agentCapabilities, authMethods, no fs)
- session/new enforces cwd absolute path; validates mcpServers array; supports workingDirectory alias without defaulting to “.”
- All session/update events conform to schema; tests cover AgentMessageChunk, ToolCall, ToolCallUpdate

Evidence
- Unit tests verifying initialize response shape
- Tests validating session/new parameter rules
- JSON schema checks or jq assertions for session/update event shapes

B. Codex CLI Tool Definitions Consistency (codex-rs integration)
Recommendations
1) Shell tool params mapping
- Support command as Vec<String> (ShellToolCallParams.command) in addition to string; when building titles, join array with spaces
- Honor workdir when present; ensure absolute-path validations align with ACP session cwd expectations

2) Output preservation
- Maintain full rawOutput for completed/failed ToolCalls (stdout, stderr, exit_code)
- Keep preview content to ~2KB and UTF-8 safe (prefix/suffix + marker)

3) Lifecycle mapping
- Map Codex’s begin/end events cleanly to Pending/InProgress → Completed/Failed in ACP ToolCall/ToolCallUpdate
- Deduplicate initial events and suppress redundant updates

Acceptance Criteria
- extract_shell_command handles both string and array command inputs
- Completed/failed ToolCalls include rawOutput; preview uses UTF-8 safe truncation
- ToolCall lifecycle transitions verified via integration tests (including batch scenarios)

Evidence
- Integration tests for array-form commands, lifecycle, and output preservation
- Cross-checks with codex-rs models (protocol/src/models.rs ShellToolCallParams)

C. Best Practices from claude-code-acp
Recommendations
1) Event structure and minimal payloads
- Use ToolCall for initial, ToolCallUpdate for status updates
- Avoid repeating title/kind in updates unless changed
- Keep update messages concise, only include fields that changed

2) Robust streaming
- Deduplicate identical chunks; respect finalized state to avoid trailing noise
- Graceful error mapping to ToolCallUpdate (status=failed) when errors are known to be tool-contextual

3) IDs and correlation
- Ensure stable tool_call_id across updates; avoid duplicate pending events for same id

Acceptance Criteria
- Tests confirm no duplicate pending ToolCalls and only meaningful updates are emitted
- Errors within a tool context surface as ToolCallUpdate with status=failed (where applicable)

Evidence
- Session update format tests (ToolCall/ToolCallUpdate field presence and minimality)
- Logs demonstrating deduplication and correct finalization behavior

Task Breakdown (Proposed naming)
- acp-v1-init-and-session-alignment
  - Implement initialize response normalization and session/new strict validation
- codex-cli-shell-params-and-output-alignment
  - Support Vec<String> commands for titles; preserve rawOutput; validate lifecycle mapping
- claude-acp-best-practices-streaming-and-updates
  - Enforce minimal updates, dedupe chunks, stable tool_call_id handling, error mapping improvements

References
- ACP v1 spec and Zed references (local refs)
- Codex repos: /Users/arthur/dev-space/codex, /Users/arthur/dev-space/codex/codex-cli, /Users/arthur/dev-space/codex/codex-rs (see protocol/src/models.rs ShellToolCallParams)
- claude-code-acp: /Users/arthur/dev-space/claude-code-acp/
- Existing tests: crates/codex-cli-acp/tests/* and tests/session_update_format.rs

Notes
- Do not merge .worktrees/codex-proto-1 PR until this issue’s tasks are completed by Claude Code
- After completion, consider enabling the CI JSONL replay runner (see ci-replay-acp-v1-runner.md)

