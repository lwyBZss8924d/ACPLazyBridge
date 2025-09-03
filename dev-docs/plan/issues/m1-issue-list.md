# M1 任务清单（ISSUE 草案）

说明
- 每个 ISSUE 按可测试的合理范围拆分，包含：需求、技术方案、local_refs 引用、对应的 review 条目（SPEC/REQ/ARC/CODEX/ZED）、验收标准、Worktree 指南。
- 提交要求遵循 Worktree-first 规范（见下方模板与 CLAUDE.md）。

---

## ISSUE: core-transport-1 — 行级 JSONL 传输与子进程管控
- 需求
  - 提供 spawn_with_stdio(cmd,args,env)、read_lines(reader,handler)、write_line(writer,json_str)
  - 子进程退出监听、stderr 收集（DEBUG 可转发关键行）
- 技术方案
  - tokio::process::Command 管理子进程；tokio::io::BufReader::lines 做行解析
  - 忽略空行，非法 JSON 记录日志但不阻塞后续
- local_refs 引用
  - (local_refs/agent-client-protocol)
  - (local_refs/zed-acp-examples)
- 对应 review 条目
  - ARC-LAZY-0001；SPEC-ACP-CONSTRAINTS-0005
- 验收标准
  - 单测覆盖：空行、粘包拆分、非法 JSON 不阻塞；写入 flush
- 状态：✅ 已完成（acp-lazy-core::transport 实现 + 单测通过；stderr 日志分级与退出尾日志保留）
- Worktree 指南
  - 分支名：feature/core-transport-1
  - git worktree add ../acplb-core-transport-1 feature/core-transport-1

## ISSUE: core-permissions-1 — ACP→Codex 权限映射
- 需求
  - map_acp_to_codex(mode)：default/plan/acceptEdits/bypassPerms 映射 approval/sandbox/network
  - 支持 env 覆盖（turn 级优先）
- 技术方案
  - 纯函数映射 + 可选 env 解析
- local_refs 引用
  - (local_refs/codex)
- 对应 review 条目
  - REQ-LAZY-0005；CODEX-CLI-0001/0002/0003
- 验收标准
  - 单测：三模式映射正确；network 开/关一致；env 覆盖生效
- 状态：✅ 已完成（map_acp_to_codex + env 覆盖；tests 覆盖 prefix/CLI 生成）
- Worktree 指南
  - 分支名：feature/core-permissions-1

## ISSUE: acp-wire-0 — ACP/JSON-RPC 线协议封装（可选）
- 需求
  - 封装请求/响应/错误结构，统一错误码与 data 字段
- 技术方案
  - serde + enum 封装；提供 Err->JSON-RPC 错误映射
- local_refs 引用
  - (local_refs/agent-client-protocol)
- 对应 review 条目
  - SPEC-ACP-JSONRPC-0001；SPEC-ACP-METHODS-0002
- 验收标准
  - serde round-trip；错误码覆盖到 -32700/-32600/-32601/-32602/-32603
- 状态：✅ 已完成（protocol.rs 提供 JSON-RPC 类型与错误映射；单测覆盖）
- Worktree 指南
  - 分支名：feature/acp-wire-0

## ISSUE: codex-stdio-1 — ACP 主循环（initialize/new/prompt/cancel）
- 需求
  - 解析每行 JSON；根据 method 分发；返回协议化结果或错误
- 技术方案
  - 复用 acp-wire（若已完成）或直接 json! 手动拼装
- local_refs 引用
  - (local_refs/agent-client-protocol)
- 对应 review 条目
  - REQ-LAZY-0001；SPEC-ACP-METHODS-0002/STREAM-0003/CANCEL-0008
- 验收标准
  - JSONL 回放：handshake/basic_session/prompt_skeleton/prompt_and_cancel
- 状态：🚧 进行中（主循环/方法分发/错误映射已实现；需用回放用例验证 initialize/new/prompt/cancel 路径）
- Worktree 指南
  - 分支名：feature/codex-stdio-1

## ISSUE: codex-proto-1 — 对接 codex proto（消息/流式）
- 需求
  - spawn codex；解析 agent_message/agent_message_delta → agent_message_chunk
- 技术方案
  - transport::spawn_with_stdio + read_lines；宽松解析
- local_refs 引用
  - (local_refs/codex)
- 对应 review 条目
  - REQ-LAZY-0002；ZED-REF-0001；SPEC-ACP-STREAM-0003
- 验收标准
  - 长流/增量输出；无阻塞；去重前置
- 状态：进行中（已 spawn codex 并发送 prompt；未实现 stdout 解析与 agent_message(_delta)→agent_message_chunk 流式转发）
- Worktree 指南
  - 分支名：feature/codex-proto-1

## ISSUE: codex-notify-1 — notify + idle fallback + 去重
- 需求
  - 接入 codex notify sink；1.2s idle fallback；lastSentChunk/finalized 去重
- 技术方案
  - FIFO/文件监听；定时器；最终块去重
- local_refs 引用
  - (local_refs/codex)
- 对应 review 条目
  - REQ-LAZY-0004；SPEC-ACP-STREAM-0003
- 验收标准
  - 无 task_complete 情况也能 EndTurn；不重复最终块
- 状态：❌ 未开始
- Worktree 指南
  - 分支名：feature/codex-notify-1

## ISSUE: codex-tools-1 — ToolCalls 标准化与 2KB 预览
- 需求
  - 单/批工具：pending → completed；local_shell stdout 预览（2KB）
- 技术方案
  - 标准化结构（title/kind/status/raw_input/raw_output/content/locations）
- local_refs 引用
  - (local_refs/zed-acp-examples)
- 对应 review 条目
  - REQ-LAZY-0003；ZED-REF-0002/0005/0006
- 验收标准
  - 批量用例与 UI 显示正确；2KB 裁剪
- 状态：❌ 未开始
- Worktree 指南
  - 分支名：feature/codex-tools-1

## ISSUE: codex-errors-1 — 错误码/约束校验
- 需求
  - -32700/-32600/-32601/-32602/-32603；绝对路径/1-based 行号/JSONL 单行
- 技术方案
  - 参数校验与错误映射；路径/行号检查
- local_refs 引用
  - (local_refs/agent-client-protocol)
- 对应 review 条目
  - SPEC-ACP-JSONRPC-0001；SPEC-ACP-CONSTRAINTS-0005
- 验收标准
  - 回放 invalid_request/invalid_params/parse_error 等用例均按期望返回
- 状态：🚧 进行中（错误码定义完整；参数/路径/行号约束校验与回放用例待补）
- Worktree 指南
  - 分支名：feature/codex-errors-1

## ISSUE: tests-jsonl-1 — JSONL 用例与日志证据
- 需求
  - 新增 large_output/tool_calls 用例；完善 logs/README 与 jq 过滤
- 技术方案
  - 统一在 _artifacts/tests 归档；输出快照存 logs
- local_refs 引用
  - (local_refs/zed-acp-examples)
- 对应 review 条目
  - traceability.csv 全量覆盖，无孤儿项
- 验收标准
  - jq 过滤脚本可重现评审证据
- 状态：❌ 未开始（_artifacts/ 结构与 traceability 已建立；需新增 JSONL 用例与 jq 模板）
- Worktree 指南
  - 分支名：chore/tests-jsonl-1

## ISSUE: docs-usage-1 — 使用手册与 settings.json 示例
- 需求
  - 面向 Zed 的配置与说明，含权限模式/网络开关
- 技术方案
  - README/USAGE 文档；examples/zed/settings.json
- local_refs 引用
  - (local_refs/zed-acp-examples)
- 对应 review 条目
  - REQ-LAZY-0006（promptCapabilities 交付可在文档中说明）
- 验收标准
  - 新手可按文档跑通 smoke
  - 状态：❌ 未开始
 - Worktree 指南
  - 分支名：docs/usage-1

---

## 补充：测试环境与流程配置（待合并后执行）

### ISSUE: tests-env-zed-config-1 — Zed settings.json 配置 ACPLazyBridge
- 需求
  - 在 `~/.config/zed/settings.json` 中添加 ACPLazyBridge (Codex) 项（绝对路径指向 `target/release/codex-cli-acp`）
  - 预留 ACPLazyBridge (Claude/Gemini) 条目，待二进制落地后启用
- 技术方案
  - 提供示例 JSON 片段与占位符密钥（`{{ANTHROPIC_API_KEY}}`, `{{GEMINI_API_KEY}}`）
- 引用
  - `local_refs/zed-acp-examples/agent_servers/src/*`
- 验收标准
  - Zed 端可连接 Codex 适配器并完成一次最小冒烟（initialize/new/prompt）
- Worktree 指南
  - 分支名：docs/zed-config-1

### ISSUE: tests-env-codex-cli-setup-1 — 验证 Codex CLI 安装与配置
- 需求
  - 确认 `codex` 在 PATH；校验 `~/.codex/config.toml` 基本可用
- 技术方案
  - 文档列出检查命令与常见问题排查（不回显密钥）
- 引用
  - `local_refs/codex/docs`, `~/.codex/config.toml`
- 验收标准
  - 本地能成功启动 `codex proto` 并返回初始化响应（由适配器驱动）
- Worktree 指南
  - 分支名：docs/codex-setup-1

### ISSUE: warp-script-nonmock-1 — WARP-Agent 非mock 脚本化测试入口
- 需求
  - 在文档中给出脚本化运行命令与日志归档规范；可选地在 `scripts/smoke/` 放置示例脚本
- 技术方案
  - 统一日志到 `dev-docs/review/_artifacts/logs/`；引用 jq 过滤模板
- 引用
  - `WARP.md`, `dev-docs/review/_artifacts/logs/README.md`
- 验收标准
  - 按文档执行能获得稳定可审计的日志快照
- Worktree 指南
  - 分支名：chore/warp-script-nonmock-1

### ISSUE: claude-proxy-acplb-1 — Claude Code 代理二进制与冒烟配置
- 需求
  - 落地 `claude-code-acplb`，支持 ACP 透传；提供 Zed 配置与脚本化运行说明
- 技术方案
  - 参考 `local_refs/zed-acp-examples/agent_servers/src/claude.rs`；环境变量 `ANTHROPIC_API_KEY`
- 引用
  - `CLAUDE.md`, `local_refs/agent-client-protocol/`
- 验收标准
  - Zed 与脚本化均可完成 initialize/new/prompt/cancel 的最小闭环
- Worktree 指南
  - 分支名：feature/claude-proxy-acplb-1

### ISSUE: gemini-proxy-acplb-1 — Gemini CLI 代理二进制与冒烟配置
- 需求
  - 落地 `gemini-cli-acplb`，默认追加 `--experimental-acp`；提供 Zed 配置与脚本化运行说明
- 技术方案
  - 参考 `local_refs/zed-acp-examples/agent_servers/src/gemini.rs`；环境变量 `GEMINI_API_KEY`
- 引用
  - `CONTRIBUTING.md`, `local_refs/agent-client-protocol/`
- 验收标准
  - Zed 与脚本化均可完成最小冒烟
- Worktree 指南
  - 分支名：feature/gemini-proxy-acplb-1

### ISSUE: logs-jq-templates-1 — jq 过滤模板与错误码用例完善
- 需求
  - 增补 -32600/-32602 等错误用例与 jq 过滤示例
- 技术方案
  - 在 `dev-docs/review/_artifacts/jq/` 下补充模板与示例
- 引用
  - `dev-docs/review/_artifacts/logs/README.md`
- 验收标准
  - 评审者可一键得到错误与 stopReason 摘要
- Worktree 指南
  - 分支名：chore/logs-jq-templates-1

