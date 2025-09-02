# ACPLazyBridge 合规/对齐评审报告（ACP × Codex × Zed）

版本：草案 v0.1
修订：见 _artifacts/REVISION.txt
环境：见 _artifacts/ENV.txt

目录
- 摘要与结论
- 方法与范围
- 规范对照与合规评估（ACP）
- 架构评审（Streaming/ToolCalls/Permissions）
- 权限映射与安全策略（Codex）
- Zed 参考实现差异点
- 追踪矩阵摘要
- 差距与整改建议
- 测试建议与样例
- 附录（证据与命令清单）

1) 摘要与结论
- 总体合规度：初步 Partial（最小 ACP 服务已实现握手/新会话/流式/取消通知；错误码/字段级约束与 Codex 集成待补）
- 关键风险：
  - JSON-RPC 错误码与 data 字段未在需求/架构显式约束（建议补充并测试覆盖）
  - session/load 与 authenticate 能力声明与实现一致性（若暴露 loadSession: true 则需实现/说明）
  - 平台沙箱差异（seatbelt/Landlock）与降级策略
  - 事件 payload 的字段级 schema 需与 Zed 示例对齐

2) 方法与范围
- 文档：dev-docs/requirements/*、dev-docs/design/*、WARP.md
- 参考：local_refs/{agent-client-protocol,codex,zed-acp-examples}（本仓库被 .gitignore 忽略，回退到本机已索引仓库作为对照：/Users/arthur/dev-space/{zed,codex,acp-claude-code}）
- 代码实现范围：crates/*（后续在 IMPL.csv 汇总）
- 工具：parse/search（semtools）、jq、cargo、git

3) 规范对照与合规评估（ACP 摘要）
- JSON-RPC 2.0：请求/响应/错误（-32700/-32600/-32601/-32602/-32603、data 字段） → 状态：TBD
- 方法集：initialize/new|load?/prompt/cancel、session/update（通知） → 状态：TBD
- 流式：agent_message_delta → agent_message_chunk；去重与结尾守卫；idle fallback → 状态：TBD
- 工具事件：tool_call（pending → completed）、批量 → 状态：TBD
- 约束：绝对路径、1-based 行号、JSONL 单行 → 状态：TBD

4) 架构评审（Streaming/ToolCalls/Permissions）
- transport 行级解析与队列、lastSentChunk/finalized 去重 → 评估：TBD
- tool_call stdout 预览（2KB）与错误回显策略 → 评估：TBD
- turn 完成：notify 优先 + idle fallback（1.2s） → 评估：TBD

5) 权限映射与安全策略（Codex）
- default/plan → approval=never + sandbox=read-only
- acceptEdits → approval=never + sandbox=workspace-write
- bypassPerms → approval=never + sandbox=workspace-write + network_access=true
- YOLO/danger → --dangerously-bypass-approvals-and-sandbox（显式警告）
- 平台差异与降级：seatbelt/Landlock 不可用时的回退策略

6) Zed 参考实现差异点
- agent_message_chunk 的事件命名、字段形态
- tool_call 事件的 pending/completed payload 对齐
- notify("agent-turn-complete") 行为与缺省兜底

7) 追踪矩阵摘要
- 全量见 _artifacts/traceability.{md,csv}

8) 差距与整改建议（示例）
- GAP-ACP-ERRCODES：未明确错误码使用与 data 字段 → 高 → 更新需求/架构与测试
- GAP-LOAD-IMPL：loadSession 能力未落地实现/策略 → 中 → 明确是否支持或隐藏能力
- GAP-SANDBOX-FALLBACK：平台沙箱降级策略未定义 → 中 → 增加运行时检测与提示

9) 测试建议与样例
- 基础握手（initialize）与能力/版本
- 会话创建/提示（new/prompt）流式与去重
- 工具调用全链路（pending→completed，stdout 预览）
- cancel 通知中断
- 权限模式与网络开关
- 压测：长流/多工具/并发会话
- 见 _artifacts/tests/*.jsonl 与 WARP.md 示例命令

10) 附录
- 证据：_artifacts/logs/*、IMPL.csv
- 命令清单：WARP.md、test 脚本

