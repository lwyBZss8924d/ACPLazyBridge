# feature/codex-notify-1 — Notify sink integration + idle fallback + dedup

## 背景 / 需求
- 在 Codex proto 流式过程中，若缺少 `task_complete`（或等价结束信号），ACP 侧仍需可靠结束一次 turn，避免卡死。
- 将 Codex 的通知（notify sink，如文件/FIFO/管道或未来 CLI 事件）纳入统一的流式更新通道，配合去重与空闲超时，保证 session/update 输出稳定。
- 满足 ACP v1 规范的 session/update 结构，避免重复最终块；可配置 idle 超时。

参考（需求/架构/计划）
- dev-docs/requirements/acp-lazybridge-requirements.md
- dev-docs/design/acp-lazybridge-architecture.md
- dev-docs/plan/acp-lazybridge-project-plan.md
- dev-docs/plan/m1-technical-implementation-plan.md
- dev-docs/plan/issues/m1-issue-list.md（codex-notify-1）

## 技术方案
- 在 `crates/codex-cli-acp/src/codex_proto.rs` 基础上，新增 NotifySource 抽象：
  - 模式 A（现状）: 仅解析 Codex stdout 的 JSON 事件（agent_message/agent_message_delta/tool_call/task_complete/error）。
  - 模式 B（增量）: 监听 notify sink（文件/管道/FIFO，路径通过 env 配置），将关键事件（如 idle 提示/完成信号）转为统一的 SessionUpdate。
- 在 `AcpServer::handle_session_prompt` 处：
  - 启动 Codex 进程后，除 stdout 解析外，启用可选 notify 监听任务；
  - 使用 `ACPLB_IDLE_TIMEOUT_MS`（已存在）作为空闲超时，周期由 `ACPLB_POLLING_INTERVAL_MS` 决定；
  - 去重：保留 `last_sent_chunk`，对相同内容/相同终态不重复发送；设置 `finalized` 标志，防止重复最终块；
  - 结束条件：
    - 收到 `task_complete`；
    - 或 idle 超时；
    - 或 notify sink 发出完成信号。
- 配置项：
  - ACPLB_IDLE_TIMEOUT_MS（默认 1200）
  - ACPLB_POLLING_INTERVAL_MS（默认 100）
  - ACPLB_NOTIFY_PATH（可选，若存在则启用通知监听）
- 错误处理与日志：
  - 监听任务容错（IO 错误不崩溃，只记录 error，并按 idle 流程结束）；
  - 关键阶段 structured logging（session 开始/结束、notify 收到、idle 触发、去重命中）。

## local_refs 引用
- (local_refs/agent-client-protocol) @/Users/arthur/dev-space/ACPLazyBridge/local_refs/agent-client-protocol/schema/schema.json
- (local_refs/zed-acp-examples) @/Users/arthur/dev-space/ACPLazyBridge/local_refs/zed-acp-examples/agent_servers/src/acp.rs
- (local_refs/codex) @/Users/arthur/dev-space/ACPLazyBridge/local_refs/codex/docs

## 对应的 dev-docs/review 条目
- SPEC: SPEC-ACP-STREAM-0003（session/update 流式、结束行为）
- REQ: REQ-LAZY-0004（idle fallback）
- ARC: ARC-LAZY-0001（传输/并发/资源管控）
- CODEX: CODEX-CLI-0001/0002/0003（权限/网络/执行参数）
- ZED: ZED-REF-0001（参考实现行为）

## 任务拆分
1) 配置 & 文档
   - 在 README/CLAUDE.md/CONTRIBUTING.md 补充 ACPLB_NOTIFY_PATH、idle/polling 参数说明
2) NotifySource 抽象
   - traits + 文件/FIFO 监听实现；容错与关闭
3) 集成主循环
   - 在 handle_session_prompt 合并 notify 与 stdout 两路事件；保持单一 `SessionUpdate` 出口
4) 去重/最终块
   - 针对 `AgentMessageChunk`/`ToolCall(_Update)` 与 `finalized` 逻辑，增加冪等
5) 测试（单测+回放）
   - mock notify 文件：不写 `task_complete` 时，依然在 idle 时间窗后结束；
   - 保证无重复最终块；
   - CI JSONL 保持稳定（时序允许的抖动内）。

## 验收标准（DoD）
- 单测：
  - idle 超时触发 end_turn；
  - 有 `task_complete` 时立即结束；
  - 打开 ACPLB_NOTIFY_PATH，收到通知能立即结束（无需等待 idle）；
  - 无重复最终块；
- JSONL 回放：新增 `notify_idle.jsonl` 场景，CI 绿；
- 日志证据：dev-docs/review/_artifacts/logs/ 下提供一次运行日志与 jq 摘要；
- 文档更新：CLAUDE.md/CONTRIBUTING.md 增加配置说明。

## Worktree-first
- 分支：feature/codex-notify-1
- 初始化：git worktree add ../acplb-worktrees/codex-notify-1 feature/codex-notify-1
- 合并：PR 方式；提交前确保 traceability.csv 更新，无孤儿条目

## 提交说明模板
- Commit 标题
  - feat(codex-notify-1): integrate notify sink with idle fallback and dedup
- PR 描述要点
  - 背景与目标；
  - 配置项说明（ACPLB_IDLE_TIMEOUT_MS/ACPLB_POLLING_INTERVAL_MS/ACPLB_NOTIFY_PATH）；
  - 关键实现（去重/结束条件/容错）；
  - 测试与证据（JSONL 场景、日志与 jq）；

