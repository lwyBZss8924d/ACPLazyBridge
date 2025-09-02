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
- Worktree 指南
  - 分支名：docs/usage-1

