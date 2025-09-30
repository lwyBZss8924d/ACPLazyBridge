# Milestone 0.1.0 – Technical Implementation Playbook

```yaml
Milestone: 0.1.0 (Core Runtime & Zed ↔ Codex MVP)
Spec-URIs: `specs/038-adopt-acp-runtime/` (Issue #44 completed via PR #47, commit 7ae2628)
Deferred-URIs: `_issues_drafts/open/#45-streaming-alignment-session-notifications.md`, `_issues_drafts/open/#50-codex-protocol-alignment-mvp.md`
Superseded-URIs: `_issues_drafts/open/#46-protocol-cleanup-official-models.md` (replaced by Issue #50)
Plan-URI: dev-docs/_requirements/m1-technical-implementation-plan.md
Tasks-URI: specs/038-adopt-acp-runtime/tasks.md
Evidence-URIs: _artifacts/038-adopt-acp-runtime/{tests,logs,jq,reports}/
```

## 1. Objective

Deliver the first production-ready ACPLazyBridge runtime by:

- Migrating all Codex ACP traffic to `agent-client-protocol::AgentSideConnection`.
- Emitting ACP-native streaming notifications and tool-call lifecycle messages.
- Removing the bespoke protocol mirror in `acp-lazy-core`.

## 2. Workstreams

| Workstream | Issue Draft | Status | Key Deliverables |
| --- | --- | --- | --- |
| Runtime adoption | `specs/038-adopt-acp-runtime/` | ✅ **Completed** | Shared runtime crate, LocalSet orchestration, Codex adapter migration (PR #47) |
| Streaming alignment | `_issues_drafts/open/#45-streaming-alignment-session-notifications.md` | ✅ **Completed** to Phase 4 | Official ACP notification models, dedupe safeguards, notify/timeout parity |
| Codex protocol alignment | `_issues_drafts/open/#50-codex-protocol-alignment-mvp.md` | 🔄 Planned | Complete ACP submission/event coverage, retire `acp-lazy-core::protocol`; Issue #50 supersedes Issue #46 |

## 3. Implementation Breakdown

### Phase 3.1-3.3: Completed (SDD Task 038)

(1) **Create shared runtime module** ✅

- Wrap `AgentSideConnection` setup, session store, process transport, notify handling.
- Ensure `LocalSet` hosts all `!Send` futures.

(2) **Port Codex adapter** ✅

- Replace handcrafted loop with runtime module (initialize/new/prompt/cancel).
- Snapshot CLI arguments pre/post migration to preserve permission overrides.

(3) **Evidence capture** ✅

- JSONL replays stored under `_artifacts/tests/protocol-baseline/`.
- Logs and reports under `_artifacts/038-adopt-acp-runtime/{tests,logs,reports}/`.

### Phase 3.4-3.5: Deferred to Follow-up Issues

(4) **Adopt official streaming models** ✅

- Map Codex events → `SessionNotification`, `ContentBlock`, `ToolCall`, `ToolCallUpdate`.
- Retain dedupe/idle timeout semantics and notify-forwarder support (Issue #45).

(5) **Deprecate internal protocol mirror** 🔄

- Swap all references to `acp-lazy-core::protocol` with upstream types and surface complete ACP metadata.
- Remove legacy module and adjust tests/docs accordingly (Issue #50 – supersedes Issue #46).

## 4. Test Matrix

| Layer | Tests | Evidence Path | Status |
| --- | --- | --- | --- |
| Unit | `cargo test -p acp-lazy-core -- runtime::*` | `_artifacts/038-adopt-acp-runtime/tests/` | ✅ Completed |
| Integration | `cargo test -p codex-cli-acp playback::*` with baseline JSONL | `_artifacts/038-adopt-acp-runtime/tests/` | ✅ Completed |
| JSONL Replay | handshake, basic_session, unknown_method, invalid_params, cancel | `_artifacts/038-adopt-acp-runtime/tests/` | ✅ Completed |
| Manual | Zed custom agent smoke (initialize → prompt) | `_artifacts/038-adopt-acp-runtime/logs/` | ✅ Completed |

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

### Completed (SDD Task 038)

- [x] Runtime module merged and documented in `core_servers/acplb-core-runtime.md`
- [x] Codex adapter uses shared runtime without regressions (PR #47)
- [x] Evidence stored under `_artifacts/038-adopt-acp-runtime/...`
- [x] Changelog entry created (`dev-docs/changelogs/038-adopt-acp-runtime.md`)

### Deferred to Follow-up Issues

- [x] Streaming notifications validated via snapshot tests (Issue #45)
- [ ] Legacy protocol module removed; docs updated (`architecture/`, `requirements/`) (Issue #50)

## 9. Follow-Up (Post 0.1.0)

- **Issue #45**: (specs/039-streaming-alignment-session-notifications) Phase 3.6 (T041-T064) transferred to new issues pending release [## Phase 3.6: Gap Remediation (Future Work)](specs/039-streaming-alignment-session-notifications/tasks.md) (Milestone 0.1.0 - Phase 4)
- **Issue #50**: Codex protocol alignment MVP — complete ACP mappings and retire the legacy protocol module (Milestone 0.1.0 - Phase 5)
- **Legacy notice**: `_issues_drafts/open/#46-protocol-cleanup-official-models.md` remains for audit history but should not receive further updates.
- Planing for (Milestone 0.1.0 First Release) other MVP features (Phase 6) `[TODO]: ("to be updated")`
- Prepare composer runtime extensions (Milestone 0.2.0)
- Draft Claude/Gemini adapter issue briefs building on shared runtime
- Schedule architecture review for plugin pipeline
