# Milestone 0.1.0 – Technical Implementation Playbook

```yaml
Milestone: 0.1.0 (Core Runtime & Zed ↔ Codex MVP)
Spec-URIs: see `_issues_drafts/runtime-adoption-core-loop.md`, `streaming-alignment-session-notifications.md`, `protocol-cleanup-official-models.md`
Plan-URI: dev-docs/_requirements/m1-technical-implementation-plan.md
Tasks-URI: specs/<NNN>-<slug>/tasks.md (per issue draft)
Evidence-URIs: _artifacts/<task>/{tests,logs,jq,reports}/
```

## 1. Objective

Deliver the first production-ready ACPLazyBridge runtime by:

- Migrating all Codex ACP traffic to `agent-client-protocol::AgentSideConnection`.
- Emitting ACP-native streaming notifications and tool-call lifecycle messages.
- Removing the bespoke protocol mirror in `acp-lazy-core`.

## 2. Workstreams

| Workstream | Issue Draft | Key Deliverables |
| --- | --- | --- |
| Runtime adoption | `_issues_drafts/runtime-adoption-core-loop.md` | Shared runtime crate, LocalSet orchestration, Codex adapter migration |
| Streaming alignment | `_issues_drafts/streaming-alignment-session-notifications.md` | Official ACP notification models, dedupe safeguards, notify/timeout parity |
| Protocol cleanup | `_issues_drafts/protocol-cleanup-official-models.md` | Removal of `acp-lazy-core::protocol`, upstream error/response usage |

## 3. Implementation Breakdown

1. **Create shared runtime module**
   - Wrap `AgentSideConnection` setup, session store, process transport, notify handling.
   - Ensure `LocalSet` hosts all `!Send` futures.
2. **Port Codex adapter**
   - Replace handcrafted loop with runtime module (initialize/new/prompt/cancel).
   - Snapshot CLI arguments pre/post migration to preserve permission overrides.
3. **Adopt official streaming models**
   - Map Codex events → `SessionNotification`, `ContentBlock`, `ToolCall`, `ToolCallUpdate`.
   - Retain dedupe/idle timeout semantics and notify-forwarder support.
4. **Deprecate internal protocol mirror**
   - Swap all references to `acp-lazy-core::protocol` with upstream types.
   - Remove legacy module and adjust tests/docs accordingly.
5. **Evidence capture**
   - JSONL replays stored under `_artifacts/tests/protocol-baseline/`.
   - Logs and reports under `_artifacts/logs/runtime-adoption/` and `_artifacts/reports/runtime-adoption/`.

## 4. Test Matrix

| Layer | Tests | Evidence Path |
| --- | --- | --- |
| Unit | `cargo test -p acp-lazy-core -- transport::*` | `_artifacts/tests/runtime-adoption/` |
| Integration | `cargo test -p codex-cli-acp playback::*` with baseline JSONL | `_artifacts/tests/protocol-baseline/` |
| JSONL Replay | handshake, basic_session, unknown_method, invalid_params, cancel | `_artifacts/tests/protocol-baseline/` |
| Manual | Zed custom agent smoke (initialize → prompt) | `_artifacts/logs/runtime-adoption/` |

## 5. Quality Gates

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-features --locked`
- `scripts/ci/run-local-ci.sh`
- JSONL replays clean (no stderr errors, stdout strict JSONL)
- Evidence in `_artifacts/<task>/...`

## 6. Risks & Mitigations

| Risk | Mitigation |
| --- | --- |
| Tokio LocalSet misconfiguration | Prototype in feature branch; add integration test that exercises notify + idle timeout |
| CLI argument drift | Capture before/after snapshots; add regression assertions in tests |
| Schema mismatch | Add serde snapshot tests for `SessionNotification` JSON |
| Legacy protocol references lingering | Run `rg` for `acp-lazy-core::protocol` after cleanup |

## 7. Dependencies

- `agent-client-protocol` crate ≥ 0.4.0
- Codex CLI availability (`CODEX_CMD` env)
- Notify forwarder binary (`acplb-notify-forwarder`) accessible via PATH or target directory

## 8. Deliverables Checklist

- [ ] Runtime module merged and documented in `core_servers/acplb-core-runtime.md`
- [ ] Codex adapter uses shared runtime without regressions
- [ ] Streaming notifications validated via snapshot tests
- [ ] Legacy protocol module removed; docs updated (`architecture/`, `requirements/`)
- [ ] Evidence stored under `_artifacts/<task>/...`
- [ ] Changelog entry created (`dev-docs/changelogs/`)

## 9. Follow-Up (Post 0.1.0)

- Prepare composer runtime extensions (Milestone 0.2.0)
- Draft Claude/Gemini adapter issue briefs building on shared runtime
- Schedule architecture review for plugin pipeline
