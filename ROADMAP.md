# ACPLazyBridge – Implementation Plan

## Milestone 1 – Bootstrap repo (done)
- Clone empty repo and vendor references under local_refs/
- Include zed-acp-examps/agent_servers and agent_ui for best-practice reference

## Milestone 2 – Codex adapter skeleton (@zed-industries/codex-cli-acp)
- Structure
  - adapters/codex-cli-acp/
    - src/index.ts – ACP server entry (stdio)
    - src/codex.ts – spawn + handshake + stream line-queue
    - src/mapping.ts – ACP mode → Codex approval/sandbox mapping
    - src/tools.ts – normalize tool_calls (single + batch) → ACP tool events
    - src/notify.ts – agent-turn-complete integration (FIFO/file)
  - scripts/smoke-codex.ts – standalone smoke test
- Behavior
  - Non-interactive approvals by default (approval_policy=never)
  - Sandbox workspace-write for acceptEdits/bypassPermissions; plan/default read-only
  - Network only in bypassPermissions
  - Streaming: emit agent_message_chunk for deltas; de-dup final chunk; idle fallback
  - Tool calls: pending → completed with stdout preview (2KB cap)
- Initialize: return promptCapabilities (image=false)

## Milestone 3 – Shared utilities
- adapters/_shared/
  - spawn.ts – portable spawn via path/args/env
  - line_reader.ts – robust queue-based JSON line reader
  - end_of_turn.ts – notify + idle fallback combiner
  - permissions.ts – ACP → adapter mapping helpers

## Milestone 4 – Tests & examples
- tests/unit/line_reader.test.ts
- tests/unit/tools_mapping.test.ts
- tests/integration/codex_stream.test.ts
- examples/zed/settings.json snippets

## Milestone 5 – Optional adapters
- wrappers for existing Zed agents to demonstrate external embedding
- MCP server compatibility notes

## Notes
- Keep adapters non-interactive by default to avoid IDE stalls
- Offer explicit YOLO profile (danger-full-access) as opt-in only
- Prefer small, inspectable JSON contracts on stdio; log stderr to aid diagnostics

