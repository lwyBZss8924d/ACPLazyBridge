# acp-lazy-core Runtime Module

 provides the shared runtime foundation used by ACPLazyBridge adapters. The runtime wraps the official Agent Client Protocol (ACP) implementation and offers a stable surface for IDE-facing agents.

## Key Components

- : Orchestrates ACP requests, tracks session state, and records telemetry.
- : Trait adapters implement to plug provider-specific behavior (Codex, Claude, Gemini, etc.).
- : Lightweight in-memory store for session metadata and permission modes.
- : Helper for spawning provider CLIs with JSONL stdio.

## Configuration

 reads operational settings from environment variables at startup:

| Variable | Description | Default |
| -------- | ----------- | ------- |
|  | Idle timeout before a prompt finishes. |  |
|  | Polling interval for idle and notify checks. |  |
|  | Optional path to a file or FIFO for notify integration. | unset |
|  |  (polling) or  (blocking). |  |
|  | JSONL file that receives runtime telemetry. | unset |

When  is set,  appends structured JSON records for initialize/new-session/prompt/cancel events, enabling SDD evidence capture ().

## Adapter Lifecycle

1. Create a  implementation that knows how to spawn and stream the provider process.
2. Instantiate  and expose it through an ACP  implementation.
3. Use  to forward  events back to  so IDE clients receive  streams.

Adapters can hook into lifecycle methods:

-  – perform provider-specific setup.
-  – spawn the provider process, stream stdout, and honor notify/timeout semantics.
-  – terminate active prompts.
-  – react to permission updates (e.g., rebuild CLI overrides).

## Testing

Contract tests live in  and document expected runtime behavior across initialize, session management, prompts, and extensions. Integration adapters should add end-to-end coverage (see ).

Run the full workspace test suite:

/Users/arthur

## feature/038-adopt-acp-runtime

 M Cargo.toml
 M _artifacts/tests/protocol-baseline/basic_session.jsonl
 M _artifacts/tests/protocol-baseline/prompt_and_cancel.jsonl
 M_artifacts/tests/protocol-baseline/prompt_with_mock_codex.jsonl
 M_artifacts/tests/protocol-baseline/session_update_format.jsonl
 M _artifacts/tests/protocol-baseline/test_basic_handshake.jsonl
 M_artifacts/tests/protocol-baseline/test_prompt_session.jsonl
 M crates/acp-lazy-core/Cargo.toml
 M crates/acp-lazy-core/src/lib.rs
A  crates/acp-lazy-core/src/runtime/adapter.rs
A  crates/acp-lazy-core/src/runtime/mod.rs
AM crates/acp-lazy-core/src/runtime/server.rs
A  crates/acp-lazy-core/src/runtime/session.rs
AM crates/acp-lazy-core/tests/runtime_test.rs
 M crates/codex-cli-acp/Cargo.toml
A  crates/codex-cli-acp/src/codex_agent.rs
 M crates/codex-cli-acp/src/lib.rs
 M crates/codex-cli-acp/src/main.rs
 M crates/codex-cli-acp/src/notify_source.rs
A  crates/codex-cli-acp/tests/acp_integration_test.rs
A  crates/codex-cli-acp/tests/jsonl_regression_test.rs
 M crates/codex-cli-acp/tests/notify_test.rs
 M crates/codex-cli-acp/tests/playback.rs
AM specs/038-adopt-acp-runtime/SDD-TASKs-038-DevLogs
A  specs/038-adopt-acp-runtime/contracts/runtime_api.md
A  specs/038-adopt-acp-runtime/data-model.md
AM specs/038-adopt-acp-runtime/plan.md
AM specs/038-adopt-acp-runtime/quickstart.md
A  specs/038-adopt-acp-runtime/research.md
AM specs/038-adopt-acp-runtime/spec.md
AM specs/038-adopt-acp-runtime/tasks.md
?? crates/acp-lazy-core/README.md
_artifacts
AGENTS.md
Cargo.lock
Cargo.toml
CLAUDE.md
coderabbit.yaml
CONTRIBUTING.md
crates
dev-docs
LICENSE
queries
README.md
ROADMAP.md
rust-toolchain.toml
scripts
sdd-rules
sgconfig.yml
specs
target
WARP.md
0
_artifacts
AGENTS.md
Cargo.lock
Cargo.toml
CLAUDE.md
coderabbit.yaml
CONTRIBUTING.md
crates
dev-docs
LICENSE
queries
README.md
ROADMAP.md
rust-toolchain.toml
scripts
sdd-rules
sgconfig.yml
specs
target
WARP.md

To capture evidence while running tests or playback scenarios, supply an evidence path:



## Next Steps

- Implement additional  variants for upcoming roadmap items (FR-0201+).
- Extend contract tests as new ACP capabilities (e.g., slash commands, terminals) are adopted.
