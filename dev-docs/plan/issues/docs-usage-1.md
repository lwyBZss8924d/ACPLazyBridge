# docs/usage-1 — Usage manual & Zed settings examples

## 背景 / 需求
- 为新手提供从零到一的使用手册：安装、配置、运行、日志查看；
- 提供 Zed `~/.config/zed/settings.json` 片段示例（Codex/Claude/Gemini 占位），说明权限模式/网络开关；
- 与当前实现（codex-cli-acp）对齐，避免文档与实际行为不一致。

参考（需求/架构/计划）
- dev-docs/requirements/acp-lazybridge-requirements.md
- dev-docs/design/acp-lazybridge-architecture.md
- dev-docs/plan/acp-lazybridge-project-plan.md
- dev-docs/plan/m1-technical-implementation-plan.md
- dev-docs/plan/issues/m1-issue-list.md（docs-usage-1）

## 技术方案
- 在 `dev-docs/usage/` 新增文档：
  1) Quickstart：安装 rust/cargo、构建、运行 `codex-cli-acp`；
  2) Zed 配置：在 settings.json 添加 agent_servers 条目（指向 `target/release/codex-cli-acp`）；
  3) 环境变量：ACPLB_IDLE_TIMEOUT_MS / ACPLB_POLLING_INTERVAL_MS / CODEX_CMD /（未来）ANTHROPIC_API_KEY / GEMINI_API_KEY；
  4) 日志查看与 JSONL 回放；
  5) 常见问题（PATH、权限、长输出等）；
- 在 `examples/zed/settings.json` 给出完整可用例子（用 `{{SECRET}}` 占位）。

## local_refs 引用
- (local_refs/zed-acp-examples) agent_servers: 
  - @/Users/arthur/dev-space/ACPLazyBridge/local_refs/zed-acp-examples/agent_servers/src/acp.rs
  - @/Users/arthur/dev-space/ACPLazyBridge/local_refs/zed-acp-examples/agent_servers/src/agent_servers.rs
  - @/Users/arthur/dev-space/ACPLazyBridge/local_refs/zed-acp-examples/agent_servers/src/claude.rs
  - @/Users/arthur/dev-space/ACPLazyBridge/local_refs/zed-acp-examples/agent_servers/src/gemini.rs
- Codex CLI 配置：dev-docs/coding-agents-cli-config/CodexCLI/CodexCLI-Config.md
- Claude Code CLI 配置：dev-docs/coding-agents-cli-config/ClaudeCode/ClaudeCode-Config.md

## 对应的 dev-docs/review 条目
- REQ: REQ-LAZY-0006（promptCapabilities 与使用手册交付）
- ARC: 交付流程与可运维性
- ZED: agent_servers 配置与使用指南

## 任务拆分
1) 创建 `dev-docs/usage/README.md` 与 `examples/zed/settings.json`
2) 扩充 CLAUDE.md / CONTRIBUTING.md 的相关链接与索引
3) 验证脚本化：在 WARP.md 增加一条从构建到回放的命令链路

## 验收标准（DoD）
- 新手按文档能在本地跑通 smoke（initialize/new/prompt），并用 jq 查看日志摘要；
- settings.json 示例对齐当前行为（仅 codex-cli-acp 生效，Claude/Gemini 保留占位）；
- 文档链接无 404，CI 通过。

## Worktree-first
- 分支：docs/usage-1
- 初始化：git worktree add ../acplb-worktrees/docs-usage-1 docs/usage-1
- 合并：PR 方式；traceability.csv 更新

## 提交说明模板
- Commit 标题
  - docs(usage-1): usage manual and Zed settings examples for ACPLazyBridge
- PR 描述要点
  - 快速开始、Zed 配置片段、环境变量与回放、常见问题、未来 Claude/Gemini 路线

