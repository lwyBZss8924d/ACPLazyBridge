---

Issue status: "closed"

---

# feature/codex-tools-1 — ToolCalls standardization + 2KB preview

## 背景 / 需求

- 将 Codex 的工具调用事件标准化为 ACP 规范期望的结构：
    - initial `tool_call`（pending）；后续 `tool_call_update`（in_progress → completed/failed）
    - 字段：toolCallId/title/kind/status/content[](ContentBlock)/locations/rawInput/rawOutput
- 对于本地命令（local_shell 等），在 `completed` 时提供 stdout 预览（2KB 截断），便于 IDE 侧 UI 快速呈现。

⚠️ ACPLazyBridge related interface design & implementation must strictly follow ACP specifications & check Codex CLI parameters!
**ACP-DocsAndSourceCodeReference**: [ACP-DocsAndSourceCodeReference.md](ACP-DocsAndSourceCodeReference.md)

参考（需求/架构/计划）

- dev-docs/requirements/acp-lazybridge-requirements.md
- dev-docs/design/acp-lazybridge-architecture.md
- dev-docs/plan/acp-lazybridge-project-plan.md
- dev-docs/plan/m1-technical-implementation-plan.md
- dev-docs/plan/issues/m1-issue-list.md（codex-tools-1）

## 技术方案

- 在 `crates/codex-cli-acp/src/codex_proto.rs`：
    - 定义 `ContentBlock`（已存在：Text），后续可扩展为 image/audio；
    - 初次事件使用 `SessionUpdateContent::ToolCall`（status=pending），包含 rawInput；
    - 后续状态采用 `SessionUpdateContent::ToolCallUpdate`（status=in_progress/completed/failed），rawOutput 在完成/失败时提供；
    - 裁剪预览：对 `raw_output.stdout` 截断到 2048 bytes（UTF-8 安全截断）。
- `kind` 推断：基于 name（read/get→read；write/edit/update→edit；delete/remove→delete；search/find→search；exec/run/shell→execute；默认 other）。
- 去重：已有 `last_sent_chunk`/`finalized` 逻辑，扩展到 ToolCall 路径，避免重复 update。

## local_refs 引用

- (local_refs/agent-client-protocol) @/Users/arthur/dev-space/ACPLazyBridge/local_refs/agent-client-protocol/schema/schema.json
- (local_refs/zed-acp-examples) @/Users/arthur/dev-space/ACPLazyBridge/local_refs/zed-acp-examples/agent_servers/src/agent_servers.rs
- (local_refs/codex) @/Users/arthur/dev-space/ACPLazyBridge/local_refs/codex/docs

## 对应的 dev-docs/review 条目

- SPEC: SPEC-ACP-STREAM-0003（tool_call/tool_call_update）
- REQ: REQ-LAZY-0003（工具调用标准化、预览）
- ARC: ARC-LAZY-0001
- ZED: ZED-REF-0002/0005/0006（工具事件与 UI 展示）

## 任务拆分

1) 数据结构与序列化
   - `ToolCall` vs `ToolCallUpdate` 字段校对；content 使用 `Vec<ContentBlock>`
2) 预览截断实现
   - UTF-8 安全 2KB 截断；只在 completed/failed 时附带 rawOutput 预览
3) kind 推断与 title 规范
4) 去重与终态处理
   - 防止重复 `completed`/`failed` 重放；`finalized` 写保护
5) 测试（单测+结构校验）
   - 补充 `tests/session_update_format.rs` 的 ToolCall/ToolCallUpdate 覆盖
   - JSONL 演示：`_artifacts/tests/tool_calls.jsonl`

## 验收标准（DoD）

- 单测：ToolCall/ToolCallUpdate 的结构校验通过；
- 预览：对大输出的截断逻辑单测覆盖；
- JSONL 回放：新增 `tool_calls.jsonl`，CI 绿；
- 文档：记录 2KB 截断策略与字段约束。

## Worktree-first

- 分支：feature/codex-tools-1
- 初始化：git worktree add ../acplb-worktrees/codex-tools-1 feature/codex-tools-1
- 合并：PR 方式；traceability.csv 更新

## 提交说明模板

- Commit 标题
    - feat(codex-tools-1): standardize ToolCall events and add 2KB stdout preview
- PR 描述要点
    - 字段与状态迁移；2KB 截断策略；测试与 JSONL 证明；去重与终态保护
