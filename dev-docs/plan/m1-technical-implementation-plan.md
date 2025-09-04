# M1 详细技术实现开发计划（Codex Native 适配、最小可用）

目标与范围
- 产出符合 ACP 规范的最小可用 Codex 适配器（Native，经 CLI proto），严格对齐 dev-docs/requirements 与 dev-docs/design，并以 local_refs 为规范/对照来源。
- 不在本阶段实现 Proxy 与插件；仅完成 M1 范围内的 Streaming/ToolCalls/权限映射/错误码/约束校验与测试。

规范与参考（必须对照）

⚠️ ACPLazyBridge related interface design & implementation must strictly follow ACP specifications & check Codex CLI parameters!
**ACP-DocsAndSourceCodeReference**: [ACP-DocsAndSourceCodeReference.md](ACP-DocsAndSourceCodeReference.md)

- ACP 规范（local_refs/agent-client-protocol）
  - JSON-RPC 2.0、方法/通知集合、错误码（-32700/-32600/-32601/-32602/-32603）、绝对路径与 1-based 行号、JSONL 单行
  - 事件：agent_message_chunk、tool_call（pending→completed）、Plan（可选）
- Codex CLI 文档与源码（local_refs/codex；回退路径 /Users/arthur/dev-space/codex）
  - proto 事件流（agent_message、agent_message_delta、tool_call(s)、task_complete、error）
  - CLI 参数：--sandbox/-s、--ask-for-approval/-a、--full-auto、--dangerously-bypass-approvals-and-sandbox（--yolo）
  - 平台沙箱：macOS seatbelt、Linux Landlock/seccomp，不可用时的降级策略
- Zed 参考实现（local_refs/zed-acp-examples；回退路径 /Users/arthur/dev-space/zed）
  - session/update 变体全集、ToolCall/ToolCallUpdate 字段、UI 状态映射与行为

里程碑与交付物
- 里程碑
  - D1：acp-lazy-core 完成 transport/permissions/logging（具备行级 JSONL 读写、spawn、错误与日志）；acp_wire（可选）基础消息类型
  - D2：codex-cli-acp 完成 stdio 循环（initialize/new/prompt/cancel）；对接 codex proto（spawn/notify/idle fallback/去重）
  - D3：ToolCalls 标准化（单/批、local_shell 2KB stdout 预览）；错误码与约束校验；文档/示例/测试完善
- 交付物
  - 二进制：codex-cli-acp（最小可用）
  - 文档：使用手册、Zed settings.json 示例、权限映射说明
  - 测试：JSONL 回放用例、集成 smoke、日志证据

实现分解（包/模块）
- crates/acp-lazy-core（库）
  - transport
    - 需求：
      - spawn_with_stdio(cmd,args,env)：拉起 codex proto，并暴露 stdin/stdout/stderr
      - read_lines(reader, handler)：行级 JSONL 解析（跳过空行/空白），错误行日志，不阻塞后续
      - write_line(writer, json_str)：写入并换行、flush
      - 退出/清理：子进程退出监听、cancel/kill、stderr 收集（DEBUG 时转发关键信息）
    - 验收：
      - 单测：行粘包/空行/非法 JSON 不影响后续；写入 flush；spawn 与 kill 的 happy path
  - permissions
    - 需求：
      - map_acp_to_codex(mode)：ACP 模式→{approval_policy,sandbox_mode,network_access}
        - default/plan → {never, read-only, false}
        - acceptEdits    → {never, workspace-write, false}
        - bypassPerms    → {never, workspace-write, true}
      - 读取 env 覆盖（可选）：允许 turn 级别/全局覆盖，优先级：per-turn > env > default
    - 验收：
      - 单测：三模式映射正确；网络开/关一致；env 覆盖生效
  - logging
    - 需求：tracing + EnvFilter（RUST_LOG），stderr/文件可选输出
  - acp_wire（可选）
    - 需求：
      - JSON-RPC 消息结构（请求/响应/错误）与 ACP 常用消息（initialize/new/prompt/cancel/update）类型封装
      - 错误码与 data 字段
    - 验收：serde round-trip；错误码/字段完整

- crates/codex-cli-acp（二进制）
  - stdio 主循环
    - 需求：
      - 解析每行 JSON：根据 method 分发
      - 方法：initialize/new_session/prompt（必需）；cancel（通知）
      - initialize 返回：protocolVersion、capabilities.loadSession=true、promptCapabilities.image=false、serverInfo
      - new_session：生成 sessionId；维护会话映射（包含子进程句柄/状态/取消标记）
      - cancel：置取消标记；Native 路径向子进程发 interrupt（后续接入）
    - 错误：
      - Parse error: -32700
      - Method not found: -32601
      - Invalid params: -32602（缺少字段/类型错误）
      - Internal error: -32603（运行时未知错误）
  - 对接 codex proto
    - 需求：
      - 启动：codex proto + turn 级 overrides（approval_policy/sandbox_mode/network_access）
      - 读取 stdout：行级 JSON，解析 codex Event，映射：
        - agent_message      → agent_message_chunk（全量块）
        - agent_message_delta→ agent_message_chunk（增量块）
        - tool_call(s)       → tool_call(pending) → tool_call_update(completed)
        - task_complete      → 触发 turn 完成（同时做最终 chunk 去重）
        - error              → 输出 agent_message_chunk 提示，并作为内部错误处理
      - notify sink：注册 FIFO/文件，接收 agent-turn-complete 事件（若 codex 提供）；
        - 先用 notify 完成；若超时（1.2s）未收到，则 idle fallback
      - 去重：
        - lastSentChunk 与 finalized 标记，防止重复最终块
      - 取消：
        - 收到 cancel 时，向子进程发 interrupt，并清理当前 turn 状态
    - 验收：
      - 集成 smoke：agent_message/agent_message_delta/工具/无 task_complete 的 idle fallback
  - ToolCalls 标准化
    - 需求：
      - 单个 tool_call：tool_call(pending) → tool_call_update(completed)
      - 批量 tool_calls：逐一发 pending/completed
      - local_shell：title=命令；completed.content 附 stdout 预览（2KB 截断）
      - ToolCallUpdateFields：kind/status/title/content/locations/raw_input/raw_output
    - 验收：
      - 集成：用回放或真实触发验证 UI 端显示
  - 约束校验
    - 需求：
      - 绝对路径、1-based 行号、JSONL 单行
      - 参数校验：缺失/类型错误返回 -32602
    - 验收：
      - 无效参数/路径/行号用例覆盖

接口/配置
- 环境变量（示例）
  - CODEX_PATH：codex 二进制路径（默认 PATH）
  - CODEX_MODEL：模型名（默认 openai/gpt-5）
  - CODEX_APPROVAL_POLICY：默认 approval_policy（never/on-request/on-failure/untrusted）
  - CODEX_SANDBOX_MODE：read-only/workspace-write/danger-full-access
  - CODEX_IDLE_COMPLETE_MS：idle fallback 毫秒数（默认 1200）
- CLI（可选扩展）
  - --config-override/-c：透传 codex -c
  - --notify-sink：自定义 notify sink 路径

测试与证据
- JSONL 回放用例（dev-docs/review/_artifacts/tests/）
  - handshake.jsonl：initialize 基本握手
  - basic_session.jsonl：initialize + session/new
  - prompt_skeleton.jsonl：流式输出（替换 sessionId）
  - prompt_and_cancel.jsonl：流式期间 cancel 生效
  - unknown_method.jsonl：-32601
  - invalid_params.jsonl：-32602
  - large_output.jsonl：长流输出 + 去重验证（新增）
  - tool_calls.jsonl：单/批工具事件（新增）
- 真实 smoke（可选）
  - OpenRouter/Codex 可用配置；采集日志到 _artifacts/logs
- 验收证据
  - 日志快照：jq 过滤 “session/update” 与 “result.stopReason”
  - 对齐清单：traceability.csv 中 REQ/ARC ↔ SPEC/CODEX/ZED 均为 Verified/Partial，无孤儿项

与 dev-docs/requirements 与 dev-docs/design 的映射
- REQ-LAZY-0001：ACP 基线方法（stdio JSONL）→ 上述 stdio 主循环/错误码/校验
- REQ-LAZY-0002：流式分片 → agent_message_delta→chunk、去重、idle fallback
- REQ-LAZY-0003：工具事件与 2KB 预览 → ToolCalls 标准化实现
- REQ-LAZY-0004：turn 完成与兜底 → notify 优先 + idle fallback（1.2s）
- REQ-LAZY-0005：非交互权限映射 → permissions::map_acp_to_codex + per-turn override
- REQ-LAZY-0006：promptCapabilities(image=false) → initialize 返回
- ARC-LAZY-*：transport/permissions/logging/acp_wire 对应实现

与 local_refs 的配合方式
- 目录要求（建议）
  - local_refs/agent-client-protocol：ACP 规范（JSON-RPC/结构/错误码/事件）
  - local_refs/codex：Codex CLI 与协议（proto 事件、参数、平台沙箱）
  - local_refs/zed-acp-examples：Zed 参考（session/update 变体/ToolCall 字段/状态）
- 缺省回退路径（本机已索引）
  - /Users/arthur/dev-space/zed
  - /Users/arthur/dev-space/codex
  - /Users/arthur/dev-space/acp-claude-code

代码开发工作分解（交付工程师用）
- 任务板（建议）
  - core-transport-1：spawn/stdio、read_lines/write_line、stderr 收集、单测
  - core-permissions-1：map_acp_to_codex、env 覆盖、单测
  - acp-wire-0（可选）：消息与错误码封装、serde 验证
  - codex-stdio-1：initialize/new/prompt/cancel 主循环与状态机
  - codex-proto-1：spawn codex、事件解析、agent_message/agent_message_delta 转发
  - codex-notify-1：notify sink 接入、idle fallback/去重
  - codex-tools-1：tool_call 标准化（单/批）+ 2KB 预览
  - codex-errors-1：-32700/-32600/-32601/-32602/-32603 全覆盖
  - tests-jsonl-1：JSONL 回放用例补齐（错误/工具/长流/并发）
  - docs-usage-1：使用手册与 settings.json 示例

评审与验收流程（由我维护）
- PR 评审：
  - 对照 SPEC/REQ/ARC/CODEX/ZED 的映射（traceability.csv），拒绝产生孤儿条目或未达成的 Required 条目
  - 检查错误码与 data 字段、绝对路径/1-based 行号、JSONL 单行约束
  - 检查 turn 完成路径（notify+idle）、去重保护、2KB 预览
- 证据归档：
  - 日志与 jq 过滤快照存放 _artifacts/logs/
  - 测试 JSONL 与实际输出对照

分工与时间
- 建议：2 周
  - 第 1 周：core-transport/perms + stdio 主循环 + proto 对接（仅消息/流式）
  - 第 2 周：工具/错误码/约束校验 + notify/idle/去重 + 文档与测试

风险与回退
- 平台沙箱不可用：降级提示，必要时拒绝运行（never 模式不可自动提升权限）
- Provider 事件变化：解析器兼容多形态（单/批 tool_calls），宽容而显式日志
- 输出过大：统一裁剪策略（2KB 预览 + raw_output 附件/日志）

