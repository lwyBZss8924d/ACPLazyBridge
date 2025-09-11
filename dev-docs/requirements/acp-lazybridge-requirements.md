# ACPLazyBridge 项目需求描述

本文档定义 ACPLazyBridge 的目标、范围、用户画像与需求清单，指导后续技术设计与实施。

⚠️ ACPLazyBridge related interface design & implementation must strictly follow ACP specifications & check Codex CLI parameters!
**ACP-DocsAndSourceCodeReference**: [ACP-DocsAndSourceCodeReference.md](ACP-DocsAndSourceCodeReference.md)

## 1. 背景与愿景

- 背景：各类 Coding Agent CLI（Claude Code、Gemini CLI、Codex CLI、未来的 Cursor/AMP 等）在不同 IDE/编辑器中的集成方式不一致，审批与权限模型也不统一，工具事件（tool_calls）在缺乏 UI 审批时容易“卡住”。
- 愿景：打造一个独立于特定 IDE 的 ACP（Agent Client Protocol）服务端适配器集合（acp-agent-server），用统一语义对接任意 ACP 客户端（如 Zed 的 agent_ui），并提供可插拔的“子代理/插件（agent_subservers）”能力，在不修改下游 CLI 的情况下实现跨产品一致体验与快速扩展。

## 2. 范围与不做清单

- 在范围内
  - 复刻 Zed 官方 Claude/Gemini 的 ACP 语义与最佳实践（复刻其 crates/agent_servers 的实现，但不直接作为代码依赖导入）。
  - 新增 Codex 的 ACP 适配器（原生 Rust 实现），Codex CLI 多轮响应的流式转发、Codex 各种响应事件的适配、tool_calls 适配、非交互审批策略。
  - 统一权限模型：通过 sandbox/network/approval_policy 的非交互映射，规避无审批 UI 的卡顿。
  - 插件系统（agent_subservers）：对输入/输出做翻译、提示词优化、会话压缩、计划/任务管理等可扩展处理；可调用外部 ACP Server 作为“子代理”。_后续 0.2.x 迭代增加插件系统（agent_subservers）_
  - 兼容多编辑器：默认支持 Zed（ACP v1），可拓展到其它支持 ACP 协议的客户端；对于非 ACP 编辑器，后续 ACPLazyBridge 提供增加第三方 API HTTP → ACP 的桥接。
- 不在范围内（初期）
  - 直接把 Zed 内部 crates/agent_servers 作为编译依赖（直接依赖的工程维护和后续兼容复杂度高）。
  - IO 级别的沙箱实现（遵循被集成 CLI 的沙箱机制与配置）；我们只做策略传递/映射与约束。

## 3. 角色与用户画像

- 终端用户：使用 Zed/其它编辑器的开发者，期望“开箱即用”的 CLI Coding Agent 集成体验，不被审批打断。
- 第三方各种类型编辑器软件的集成工程师：希望将自家编辑器/平台对接 ACP；或将外部 CLI（Claude/Gemini/Codex）统一接入。

## 4. 目标、KPI 与验收标准

- 目标
  1) 提供三个“等价范式”的核心 ACP 服务端：Claude/Gemini（Proxy 或 Native）、Codex（Native）。
  2) 默认非交互审批策略：tool_calls 在无审批 UI 的环境下不会卡住。
  3) 稳定流式：消息分片（agent_message_delta）实时转发，避免合并到末尾才显示。
  4) 工具事件一致：单个/批量 tool_calls 标准化为 tool_call/tool_call_update；local_shell 提供 stdout 预览。
  5) Session-SubAgent 专用子线程插件(外部"gemini-cli" (也可以是 "claude-code") 专用配置的 Sub ACP Agent)：至少 2 个示例（翻译插件、提示词优化插件），可插拔配置开关。
- KPI/验收
  - 流式延迟：< 150ms（同机环境）
  - 卡死率：0（无审批 UI 时 tool_calls 也能完成）
  - 失败重试/超时：具备基本兜底（idle/notify），异常有清晰日志
  - Zed 端与 OpenRouter/Codex 端可对齐：finish_reason=tool_calls 后能继续执行直到 turn 完成

## 5. 需求清单（功能性）

1) ACP Server（stdio）：initialize/new_session/prompt/cancel 的端到端实现
2) 流式转发：行队列解析，agent_message_delta → agent_message_chunk
3) 工具事件：tool_call + tool_call_update；支持批量 tool_calls；local_shell stdout 预览
4) turn 完成：优先 notify("agent-turn-complete"），idle fallback 兜底
5) 权限映射（非交互）：
   - default/plan → approval=never, sandbox=read-only,   net=false
   - acceptEdits    → approval=never, sandbox=workspace, net=false
   - bypassPerms    → approval=never, sandbox=workspace, net=true
6) 初始化能力位：promptCapabilities（image=false）
7) 插件系统：入站/出站管线；子代理（通过 ACP 客户端调用外部 ACP server）
8) 版本/健康检查：--version/--help（Proxy 场景）、proto 握手（Native）
9) 安全：敏感变量不落盘明文；danger-full-access 仅显式启用；日志脱敏

## 6. 需求清单（非功能性）

- 可移植：仅依赖 tokio/serde/tracing，拒绝耦合 Zed 内部 crate
- 可测试：模拟 stdout 事件、集成 smoke；CI 通过
- 文档化：使用手册、settings.json 示例、插件配置样例
- 监控/日志：stderr/notify/事件日志可控，ACP_DEBUG 控制详细级别

## 7. 约束与风险

- Provider 行为变更（Codex proto/notify 格式变化）
- OS 沙箱差异（macOS seatbelt / Linux landlock）
- API Key 安全注入（Keychain/env），避免打印
- 性能瓶颈（大输出时的行队列/预览截断）

## 8. 依赖与外部接口

- ACP v1（stdio JSON 行）
- 外部 CLI：codex、claude-code-acp、gemini --experimental-acp
- OpenRouter/Anthropic/Google 等云端 API（经 CLI 或 SDK）

## 9. ACP / Codex / ZED 开发文档 & 集成代码目录

**所有所需查询的 ACP 规范, Codex CLI 交互参数定义 , ZED-ACP 参考代码目录:**

⚠️ ACPLazyBridge 相关接口设计 & 实现时必须严格遵循 ACP 规范 & 查询 Codex CLI 参数!

path: (dev-docs/references/)
