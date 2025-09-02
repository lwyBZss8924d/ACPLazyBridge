# ACPLazyBridge

ACP bridge for agents / agent-tools plugin Hub connects IDEs, other types of editors, etc.

## Purpose

Provide a reusable, IDE-agnostic ACP bridge that:
- Adopts Zed’s official ACP integration patterns (agent_servers/agent_ui) as best practice.
- Hosts external CLI agent adapters (Claude, Gemini, Codex, …) with a consistent capability surface.
- Ensures non-interactive approvals by default for IDEs without a UI approval flow.

## Layout

- local_refs/
  - zed-acp-examps/
    - agent_servers/ (vendored reference)
    - agent_ui/ (vendored reference)
  - codex/ (docs gathered during Codex adapter work)
  - agent-client-protocol/ (ACP docs, if any)

## Planned adapters

- @zed-industries/claude-code-acp (use Zed’s official adapter directly)
- @google/gemini-cli (use Zed’s official adapter directly via --experimental-acp)
- @zed-industries/codex-cli-acp (new adapter implemented here; aligns ACP stream, tool_calls, and non-interactive approvals)

## Non‑interactive approvals (recommended defaults)

To avoid stalling tool_calls in IDEs with no approval UI, map permission modes to:
- default:  approval_policy=never, sandbox_mode=read-only,      network_access=false
- plan:     approval_policy=never, sandbox_mode=read-only,      network_access=false
- acceptEdits:       approval_policy=never, sandbox_mode=workspace-write, network_access=false
- bypassPermissions: approval_policy=never, sandbox_mode=workspace-write, network_access=true

You can expose a YOLO profile (danger-full-access) as an explicit opt-in only.

## Roadmap (high level)

1) Codex ACP adapter (codex-cli-acp)
- Stream: robust stdout line queue; forward agent_message_delta as agent_message_chunk in real time
- Tooling: map single + batched tool_calls; improve titles/kinds; show stdout preview in tool_call_update
- Turn completion: prefer notify agent-turn-complete; idle fallback only
- Capabilities: return promptCapabilities in initialize

2) Shared adapter skeleton
- Provide base spawn/handshake/stream utilities (based on Zed’s agent_servers patterns)
- Add approval policy + sandbox policy bridge helpers

3) Tests & smoke tools
- Mocked stdout event sequences (streaming + tool_calls + errors)
- Non-interactive turn completion & duplicate-chunk guards

## License

MIT

