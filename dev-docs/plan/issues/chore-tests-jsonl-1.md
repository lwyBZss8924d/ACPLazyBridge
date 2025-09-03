# chore/tests-jsonl-1 — JSONL scenarios & log evidence

## 背景 / 需求
- 在 CI 中通过 JSONL 回放验证 ACP 协议交互的正确性与鲁棒性；
- 为评审留存可验证的日志证据与 jq 过滤脚本，保证 traceability。

参考（需求/架构/计划）
- dev-docs/requirements/acp-lazybridge-requirements.md
- dev-docs/design/acp-lazybridge-architecture.md
- dev-docs/plan/acp-lazybridge-project-plan.md
- dev-docs/plan/m1-technical-implementation-plan.md
- dev-docs/plan/issues/m1-issue-list.md（tests-jsonl-1）

## 技术方案
- 在 `dev-docs/review/_artifacts/tests/` 增补：
  - `large_output.jsonl`：长输出 + 分块验证；
  - `tool_calls.jsonl`：涵盖 ToolCall/ToolCallUpdate 全生命周期；
  - `parse_error.jsonl`：刻意破坏一行（测试 -32700）；
  - `invalid_request.jsonl`：缺字段/结构不合法（测试 -32600）。
- 在 `dev-docs/review/_artifacts/jq/` 新增 jq 模板：
  - `errors-summary.jq`：统计各错误码出现次数；
  - `stop-reason.jq`：抽取 stopReason 汇总；
  - `updates-brief.jq`：提取 session/update 的关键信息（类型、toolCallId、status、text 预览）。
- CI：沿用现有 workflow（ci.yml）JSONL 回放步骤，回放所有 `*.jsonl` 并上传输出。

## local_refs 引用
- (local_refs/agent-client-protocol) @/Users/arthur/dev-space/ACPLazyBridge/local_refs/agent-client-protocol/schema/schema.json
- (local_refs/zed-acp-examples)

## 对应的 dev-docs/review 条目
- SPEC: SPEC-ACP-JSONRPC-0001 / SPEC-ACP-STREAM-0003
- REQ: traceability（完整覆盖，避免孤儿条目）
- ARC: 日志与可追溯性

## 任务拆分
1) 新增 JSONL 场景文件
2) 新增 jq 模板与 README 示例
3) CI 验证与产物上传
4) traceability.csv 补充映射

## 验收标准（DoD）
- `*.jsonl` 在本地与 CI 均可稳定回放；
- 上传 `protocol-outputs` 产物；
- jq 模板可一键生成错误码与 stopReason 摘要；
- traceability.csv 完整映射。

## Worktree-first
- 分支：chore/tests-jsonl-1
- 初始化：git worktree add ../acplb-worktrees/tests-jsonl-1 chore/tests-jsonl-1
- 合并：PR 方式；traceability.csv 更新

## 提交说明模板
- Commit 标题
  - chore(tests-jsonl-1): add JSONL scenarios and jq templates for CI evidence
- PR 描述要点
  - 场景说明、jq 使用说明、CI 截图/链接、产物位置

