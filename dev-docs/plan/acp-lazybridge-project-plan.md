# ACPLazyBridge 项目实施规划

本文档给出里程碑、任务分解、时间安排、风险管理与验收标准。

⚠️ ACPLazyBridge 相关接口设计 & 实现时必须严格遵循 ACP 规范 & 查询 Codex CLI 参数!

- (local_refs/agent-client-protocol)
- (local_refs/codex)
- (local_refs/zed-acp-examples)

## 1. 里程碑总览

- M0（已完成）Workspace 初始化
  - 建立 Rust workspace（acp-lazy-core / codex-cli-acp），cargo build 通过
  - 引入 Zed 官方参考到 local_refs（agent_servers/agent_ui）
  - README / ROADMAP 初稿

- M1 Codex Native 适配（最小可用）
  - ACP stdio 循环：initialize/new_session/prompt/cancel
  - spawn codex proto；逐行读取；notify + idle 结束；错误日志
  - 流式：agent_message_delta → agent_message_chunk；去重 guard
  - 工具：tool_call(s) 标准化；local_shell stdout 预览（2KB）
  - initialize 返回 promptCapabilities（image=false）
  - smoke 测试 + 示例 settings.json

- M2 Proxy 适配（Claude/Gemini 包装）
  - crates/acp-proxy：作为 ACP Server，对接下游 ACP Server（claude-code-acp / gemini --experimental-acp）
  - 统一审批策略（非交互）与 sandbox/network；统一流式与工具事件
  - 健康检查（--version/--help）、版本/能力日志

- M3 插件系统（agent_subservers）v0
  - 插件框架：inbound/outbound；toml/yaml 配置
  - 子代理调用：以 ACP Client 调用另一个 ACP Server 获取结果
  - 示例插件：翻译、提示词优化

- M4 Native 扩展（可选）
  - claude-code-acp-native / gemini-acp-native（减少对第三方二进制依赖）
  - 更丰富的工具语义（按类型渲染摘要）

- M5 生态拓展
  - acp-http-bridge：为非 ACP 编辑器提供 HTTP/SSE 接入
  - 更多 CLI 适配（cursor-agent、AMP…）

## 2. 任务分解（M1~M3 详列）

### M1 Codex（2 周）
- 核心：
  - [ ] ACP 初始化/会话/提示/取消 stdio 循环
  - [ ] spawn codex proto（PATH/env），stderr 与 notify 处理
  - [ ] 行队列解析 + 流式 chunk 转发 + 去重
  - [ ] tool_calls 单/批 标准化，local_shell stdout 预览
  - [ ] 权限映射：approval=never + sandbox/network 分档
  - [ ] promptCapabilities（image=false）
- 测试/文档：
  - [ ] 单元（行队列、工具映射、权限映射、结尾守卫）
  - [ ] 集成 smoke（OpenRouter 可用配置）
  - [ ] examples/zed/settings.json + 使用手册

### M2 Proxy（2 周）
- 核心：
  - [ ] ACP→ACP 代理：上行/下行透传（可加策略与插件）
  - [ ] 健康检查（--version/--help）、版本能力日志
  - [ ] 非交互审批与 sandbox/network 的覆盖策略
  - [ ] 事件标准化（确保下游与上游语义一致）
- 测试/文档：
  - [ ] 单元（协议透传、策略覆盖）
  - [ ] 集成（claude/gemini），Zed 端流式与工具事件一致

### M3 插件（2 周）
- 核心：
  - [ ] Plugin trait 与可插拔管线（inbound/outbound）
  - [ ] 子代理（Sub-Agent）调用框架：ACP Client 调另一 ACP Server
  - [ ] 翻译/提示词优化插件（示例）
- 测试/文档：
  - [ ] 插件生效与禁用配置测试
  - [ ] 子代理稳定性与超时/预算

## 3. 时间安排（建议）
- 第 1~2 周：M1 Codex 最小可用
- 第 3~4 周：M2 Proxy（Claude/Gemini 包装）
- 第 5~6 周：M3 插件系统 v0（翻译/提示词优化）

> 备注：Native（Claude/Gemini）可作为 M4 延伸；非 ACP 编辑器桥接作为 M5。

## 4. 依赖/资源
- 依赖：tokio/serde/tracing，Rust 1.76+；Codex/Claude/Gemini CLI 二进制；OpenRouter/Anthropic/Google API Key
- 资源：本地/CI 构建机器；测试用 OpenRouter 账户

## 5. 风险与缓解
- Provider 行为变化 → 宽松解析 + 明确降级路径（日志提示）
- 工具输出过大 → 统一裁剪（2KB 预览）+ 文件链接/附件策略（后续）
- 审批策略误配置 → 默认为 never；YOLO 仅显式启用
- 安全 → 日志脱敏；危险模式强提示；沙箱默认 workspace-write/只读

## 6. 验收标准（阶段性）
- M1：
  - Zed 端可见 Codex 流式输出，tool_calls 不再 pending；finish_reason=tool_calls 后仍继续执行直至 turn 完成
  - smoke 成功（stream + tool_calls + 无 task_complete 也能结束）
- M2：
  - 通过 Proxy 连接 Claude/Gemini，与直接连接体验一致；非交互审批生效
- M3：
  - 开关插件 + 子代理调用可用；翻译/提示词优化在 Zed 端可见

## 7. 交付物
- 二进制：codex-cli-acp、acp-proxy
- 文档：
  - dev-docs/requirements/*.md
  - dev-docs/design/*.md
  - dev-docs/plan/*.md
  - 使用手册、示例 settings.json、插件配置样例
