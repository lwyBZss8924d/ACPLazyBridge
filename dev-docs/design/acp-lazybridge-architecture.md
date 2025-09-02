# ACPLazyBridge 技术设计方案

本文档描述 ACPLazyBridge 的整体架构、模块划分、协议映射、扩展机制与安全策略。

⚠️ ACPLazyBridge 相关接口设计 & 实现时必须严格遵循 ACP 规范 & 查询 Codex CLI 参数!

- (local_refs/agent-client-protocol)
- (local_refs/codex)
- (local_refs/zed-acp-examples)

## 1. 架构总览

- 模式：ACP 服务端适配器（stdio 行分隔 JSON）。
- 统一范式：每个 Agent 以一个独立的二进制（acp-agent-server）出现，暴露统一的 ACP 接口；编辑器作为 ACP 客户端连接。
- 两条实现路径：
  - Proxy：包装已有 ACP Server（claude-code-acp、gemini --experimental-acp），提供统一策略/插件能力；
  - Native：直接对接 Provider API/CLI（codex 首先落地），复刻官方最佳实践但不直接将其内部代码作为 Rust 依赖注入，而是通过解耦其内部依赖&符合 ACPLazyBridge 后重构实现。

## 2. 目录结构与仓库组织

- crates/
  - acp-lazy-core（库）：
    - permissions：ACP 模式 → non-interactive approvals + sandbox/network 映射
    - transport：spawn/stdio、逐行读取（队列）、写行
    - logging：tracing 初始化
    - （可选）acp_wire：最小 ACP 消息定义（serde）
  - codex-cli-acp（二进制）：Native 适配器
  - acp-proxy（可选二进制）：包装现有 ACP Server（claude/gemini）
- local_refs/
  - zed-acp-examps/agent_servers, agent_ui（仅作参考）

## 3. ACP 协议与事件映射

- 初始化（initialize）：
  - 返回：protocol_version，agent_capabilities（loadSession: true），prompt_capabilities（image: false）
- 新会话（new_session）：
  - 参数：cwd, mcpServers（原样透传/忽略）
  - 实现：Native 场景 spawn 子进程（如 codex proto），Proxy 场景直连下游 ACP Server
- 提示（prompt）：
  - 输入：Prompt blocks（text/image…）
  - 流式：
    - provider agent_message → assistant_message
    - provider agent_message_delta → assistant_message_delta → Zed 的 agent_message_chunk
  - 工具：
    - 单个 tool_call：转发为 tool_call(pending) → tool_call_update(completed)
    - 批量 tool_calls：逐一转发
    - local_shell：title=命令；completed.content 附 stdout 预览（2KB 截断）；stderr 关键行在 DEBUG 时输出文本 chunk（可选）
  - 结束：
    - notify("agent-turn-complete") 优先；
    - idle fallback（如 1.2s）兜底，不会因 provider 不发最终事件而挂起。
- 取消（cancel）：
  - Native 场景：发 interrupt 给子进程；Proxy 场景：发 cancel 给下游 ACP Server

## 4. 权限与安全

- 非交互审批（避免工具卡住）：approval_policy=never
- ACP 模式映射：
  - default/plan → sandbox=read-only,    net=false
  - acceptEdits  → sandbox=workspace-write, net=false
  - bypassPerms  → sandbox=workspace-write, net=true
  - YOLO（仅显式）：danger-full-access + net=true
- 密钥注入：env/keychain；日志脱敏；避免打印敏感值；仅在必要时探测

## 5. Streaming 实现细节

- 逐行读取：
  - 基于 tokio BufReader::lines，行入队列 pending；每次从队列取一行 JSON 解析→事件
  - 解析失败记录日志，不阻塞后续行
- 去重与结尾：
  - lastSentChunk 与 finalized 标记，防止 notify/fallback 等重复发送

## 6. ToolCalls 规范

- 支持单/批 tool_calls：
  - name 可能缺失时使用 "Tool"；rawInput 只放结构化参数（避免全文），stdout/片段显示在 completed 内容
- 工具类别（mapToolKind）：read/edit/delete/move/search/execute/fetch/think/other（复用 Claude 的规则）

## 7. 插件系统（agent_subservers）

- 目标：在会话线程流中插入 inbound/outbound 处理；支持调用“子代理”（Sub-Agents）。
- 接口：
  - trait Plugin {
    - async fn on_user_prompt(&mut Prompt) -> Result<()>
    - async fn on_agent_update(&mut SessionUpdate) -> Result<()>
  }
- 执行：
  - inbound 顺序链：翻译 → 提示词优化 → 主代理
  - outbound 顺序链：主代理 → 工具回显整理 → 翻译回用户语言
- 子代理：
  - 以 ACP 客户端身份调用另一个 acp-agent-server（如专用 gemini-cli 翻译），返回结果再注入主会话
- 配置：
  - toml/yaml 声明：插件启用、顺序、子代理地址、预算/超时/重试

## 8. Proxy 与 Native 的协同

- Proxy：
  - 好处：即时复用官方实现；统一审批策略与插件能力；无需关心 provider 细节
  - 用途：claude/gemini 初期接入；未来任何第三方 ACP server
- Native：
  - 好处：可深度优化（如更好的工具预览/错误处理/诊断）；可统一发布到非 Zed 生态
  - 用途：Codex 优先；中期将 Claude/Gemini 逐步 Native 化

## 9. 日志与诊断

- tracing + EnvFilter（RUST_LOG）
- ACP_DEBUG：输出事件/工具解析摘要；stderr 关键行
- notify sink：FIFO/文件写 JSON，一并记录路径

## 10. 测试策略

- 单元：
  - 行队列解析、工具映射、权限映射、去重/结尾守卫
- 集成：
  - 模拟 stdout 事件序列（文本/工具/错误/无 task_complete）
  - smoke：spawn codex/gemini/claude（或代理），发 prompt 看 Zed 侧行为
- 兼容性：
  - 版本检测（--version/--help），能力缺失时降级提示

## 11. 交付工件与示例

- 二进制：codex-cli-acp、acp-proxy（可选）
- 示例：examples/zed/settings.json（三 agent 并排）
- 文档：使用手册、插件样例、FAQ

⚠️ ACPLazyBridge 相关接口设计 & 实现时必须严格遵循 ACP 规范 & 查询 Codex CLI 参数!

- (local_refs/agent-client-protocol)
- (local_refs/codex)
- (local_refs/zed-acp-examples)
