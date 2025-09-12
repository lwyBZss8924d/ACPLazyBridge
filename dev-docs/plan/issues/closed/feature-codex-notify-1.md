---

Issue status: "open"
Issue number: [#16]
Issue title: Notify sink integration + idle fallback + dedup
Issue URL: [Notify sink integration + idle fallback + dedup](https://github.com/lwyBZss8924d/ACPLazyBridge/issues/16)
Issue type: "Engineering task"
Issue owner: "lwyBZss8924d", "claude-code", "warp-agent", "claude[bot]"
Task Worktree directory: acplb-worktrees/codex-notify-1
Task Feature branch: feature/codex-notify-1
Linked plan issue file: dev-docs/plan/issues/open/feature-codex-notify-1.md
Implementation commit: 7471368 feat(codex-notify-1): integrate notify sink with idle fallback and dedup

---

# feature/codex-notify-1 — Notify sink integration + idle fallback + dedup

## Background / Requirement

- 在原有 Codex proto 流式过程中(⚠️ 需要再次确认)，若缺少 `task_complete`（或等价结束信号），ACP 侧仍需可靠结束一次 turn，避免卡死。
- 将 Codex 的通知（notify sink，如文件/FIFO/管道或未来 CLI 事件）纳入统一的流式更新通道，配合去重与空闲超时，保证 session/update 输出稳定。
- 满足 ACP v1 规范的 session/update 结构，避免重复最终块；可配置 idle 超时。

**References/Requirements/Architecture/Plan**:

- @dev-docs/requirements/acp-lazybridge-requirements.md
- @dev-docs/design/acp-lazybridge-architecture.md
- @dev-docs/plan/acp-lazybridge-project-plan.md
- @dev-docs/plan/m1-technical-implementation-plan.md
- @dev-docs/plan/issues/m1-issue-list.md（codex-notify-1）

## Technical Solution

⚠️ ACPLazyBridge related interface design & implementation must strictly follow ACP specifications & check Codex CLI parameters!
**ACP-DocsAndSourceCodeReference**: [ACP-DocsAndSourceCodeReference.md](ACP-DocsAndSourceCodeReference.md)

### 决策记录（2025-09-04 / codex-notify-1）

- Notify 转发器注入策略（与 Codex CLI 行为对齐）
  - 结论：当检测到 ACPLB_NOTIFY_PATH 已设置，且用户没有显式自定义 Codex 的 notify 程序时，适配器自动通过 `-c notify=...` 注入一个“转发器”程序，将 Codex 传入的单参 JSON 写入 ACPLB_NOTIFY_PATH。提供覆盖开关与自定义命令：
    - ACPLB_NOTIFY_INJECT=auto（默认）/never/force
    - ACPLB_NOTIFY_CMD（可选，自定义 notify 程序命令数组；存在时优先使用，不再注入内置转发器）
  - 依据：Codex 的 notify 是“执行一个外部程序并传入 JSON 字符串作为唯一实参”，并非内建文件管道；为最小侵入地集成“统一流通道”，在不破坏用户已有配置的前提下自动注入最可靠。
  - 与 claude-code-acp 最佳实践一致：不引入非规范化的 session/update 类型，仅将通知用于“尽快结束 turn”。

- Notify sink 类型范围（M1）
  - 结论：M1 仅支持 file/FIFO 两类路径（ACPLB_NOTIFY_KIND=file|fifo，默认 file）。Unix domain socket 放入 M2（记录在 Roadmap），并在文档中说明。
  - 依据：macOS/Linux 下 file/FIFO 均可低成本实现；socket 需要额外兼容处理与重连策略，推迟到后续里程碑。

- Idle 超时默认值
  - 结论：保持 1200ms（与现有实现和文档一致），轮询 100ms；如需要更长抖动容忍，由使用方通过 ACPLB_IDLE_TIMEOUT_MS 覆盖即可。
  - 备注：实现层面仅用于兜底，不影响“task_complete/notify”两种立即结束路径。

### Codex notify 转发器（实现说明）

- 设计：内置极小转发器（建议作为 codex-cli-acp 的辅助二进制，例如 `acplb-notify-forwarder`），从进程实参 `argv[1]` 读取 JSON 字符串，按行追加写入 ACPLB_NOTIFY_PATH（换行终止），错误写入 stderr 并以非零码退出。
- 自动注入：
  - 触发条件：存在 ACPLB_NOTIFY_PATH 且 ACPLB_NOTIFY_INJECT!=never，且未提供 ACPLB_NOTIFY_CMD。
  - 动作：为 Codex 进程追加 `-c notify=["acplb-notify-forwarder"]`（或平台等价形式）；转发器通过环境变量读取 ACPLB_NOTIFY_PATH/ACPLB_NOTIFY_KIND。
- 用户自定义：若设置了 ACPLB_NOTIFY_CMD（数组），则完全按用户命令传给 Codex 的 `-c notify=...`，适配器不再注入内置转发器。
- 失败容错：转发失败仅影响“尽快结束”路径，不影响 stdout 主流；适配器会在 idle 窗口后正常兜底结束。

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
  - ACPLB_NOTIFY_PATH（可选；启用通知监听与“尽快结束”路径）
  - ACPLB_NOTIFY_INJECT（默认 auto；可选 auto|never|force）。当设置 ACPLB_NOTIFY_PATH 时：auto 表示在未显式配置 Codex notify 的情况下自动注入内置转发器；never 禁止注入；force 强制覆盖 Codex notify。
  - ACPLB_NOTIFY_CMD（可选；字符串数组）。若提供则直接作为 Codex 的 `-c notify=...` 值，覆盖内置转发器。
  - ACPLB_NOTIFY_KIND（默认 file；可选 file|fifo）。声明转发/监听的落地形式（M1 支持 file/FIFO；socket 延后）。
- 错误处理与日志：
  - 监听任务容错（IO 错误不崩溃，只记录 error，并按 idle 流程结束）；
  - 关键阶段 structured logging（session 开始/结束、notify 收到、idle 触发、去重命中）。

## Refs

- **ACP Rust libraries**: `cargo install agent-client-protocol` (`agent-client-protocol = "0.1.1"`) https://docs.rs/agent-client-protocol/0.1.1
- **ACP JSON Schema**: @/Users/arthur/dev-space/agent-client-protocol/schema/schema.json
- **ACP Example Agent**: @/Users/arthur/dev-space/agent-client-protocol/rust/example_agent.rs
- **ACP Example Client**: @/Users/arthur/dev-space/agent-client-protocol/rust/example_client.rs

**Zed ACP Examples**:

- @/Users/arthur/dev-space/ACPLazyBridge/local_refs/zed-acp-examples/agent_servers/src/acp.rs

**Agent Client adapter for ACP IDE Client Best Practice "claude-code-acp"**:

- (/Users/arthur/dev-space/claude-code-acp)

**codex cli source code & docs reference**:

- (/Users/arthur/dev-space/codex)
- (/Users/arthur/dev-space/codex/codex-cli)
- (/Users/arthur/dev-space/codex/codex-rs)
- @codex-cli/README.md

- @codex-rs/core/src/codex.rs

```rust
631    pub fn notify_approval(&self, sub_id: &str, decision: ReviewDecision) {
632           let entry = {
907    /// notification issues do not interfere with the main workflow.
908    fn maybe_notify(&self, notification: UserNotification) {
909        let Some(notify_command) = &self.notify else {    
```

- @codex-rs/mcp-server/src/outgoing_message.rs

```rust
59           rx_approve
60    }
61
62    pub(crate) async fn notify_client_response(&self, id: RequestId, result: Result) {
63        let entry = {
64            let mut request_id_to_callback = self.request_id_to_callback.lock().await;
65            request_id_to_callback.remove_entry(&id)
```

- @codex-rs/core/src/mcp_tool_call.rs

```rust
73     ResponseInputItem::McpToolCallOutput { call_id, result }
74 }
75
76 async fn notify_mcp_tool_call_event(sess: &Session, sub_id: &str, event: EventMsg) {
77     sess.send_event(Event {
78        id: sub_id.to_string(),
79        msg: event,
```

- @codex-rs/login/src/server.rs

```rust
72
73 #[derive(Clone, Debug)]
74 pub struct ShutdownHandle {
75    shutdown_notify: Arc<tokio::sync::Notify>,
76 }
77
78 impl ShutdownHandle {
```

- @codex-rs/core/src/config.rs

```rust
423
424    /// Optional external command to spawn for end-user notifications.
425    #[serde(default)]
426    pub notify: Option<Vec<String>>,
427
428    /// System instructions.
429    pub instructions: Option<String>,
```

- @codex-rs/tui/src/streaming/controller.rs

```rust
170            if self.finishing_after_drain {
171                // Reset and notify
172                self.state.clear();
185    /// Apply a full final answer: replace queued content with only the remaining tail,
186    /// then finalize immediately and notify completion.
187    pub(crate) fn apply_final_answer(&mut self, message: &str, sink: &impl HistorySink) -> bool {
```

- @codex-rs/mcp-server/src/message_processor.rs

```rust
147   pub(crate) async fn process_response(&mut self, response: JSONRPCResponse) {
148        tracing::info!("<- response: {:?}", response);
149        let JSONRPCResponse { id, result, .. } = response;
150        self.outgoing.notify_client_response(id, result).await
151    }
152
153    /// Handle a fire-and-forget JSON-RPC notification.
```

- @codex-rs/mcp-server/src/codex_message_processor.rs

```rust
234                        // Update in-memory auth cache now that login completed.
235                        auth_manager.reload();
236
237                        // Notify clients with the actual current auth mode.
238                        let current_auth_method = auth_manager.auth().map(|a| a.mode);
239                        let payload = AuthStatusChangeNotification {
240                            auth_method: current_auth_method,
```

> (/Users/arthur/dev-space/codex/docs)

- /Users/arthur/dev-space/codex/docs/advanced.md
- /Users/arthur/dev-space/codex/docs/authentication.md
- /Users/arthur/dev-space/codex/docs/config.md
- /Users/arthur/dev-space/codex/docs/contributing.md
- /Users/arthur/dev-space/codex/docs/CLA.md
- /Users/arthur/dev-space/codex/docs/experimental.md
- /Users/arthur/dev-space/codex/docs/faq.md
- /Users/arthur/dev-space/codex/docs/getting-started.md
- /Users/arthur/dev-space/codex/docs/install.md
- /Users/arthur/dev-space/codex/docs/platform-sandboxing.md
- /Users/arthur/dev-space/codex/docs/prompts.md
- /Users/arthur/dev-space/codex/docs/release_management.md
- /Users/arthur/dev-space/codex/docs/sandbox.md
- /Users/arthur/dev-space/codex/docs/zdr.md

## Corresponding dev-docs/review items

- SPEC: SPEC-ACP-STREAM-0003（session/update 流式、结束行为）
- REQ: REQ-LAZY-0004（idle fallback）
- ARC: ARC-LAZY-0001（传输/并发/资源管控）
- CODEX: CODEX-CLI-0001/0002/0003（权限/网络/执行参数）
- ZED: ZED-REF-0001（参考实现行为）

## Task decomposition

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

## Acceptance Criteria (DoD)

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

## Commit template

- Commit 标题
  - feat(codex-notify-1): integrate notify sink with idle fallback and dedup
- PR 描述要点
  - 背景与目标；
  - 配置项说明（ACPLB_IDLE_TIMEOUT_MS/ACPLB_POLLING_INTERVAL_MS/ACPLB_NOTIFY_PATH）；
  - 关键实现（去重/结束条件/容错）；
  - 测试与证据（JSONL 场景、日志与 jq）；

======

codex-rs [notify documentation](../docs/config.md#notify) "415 ## notify"

## notify

Specify a program that will be executed to get notified about events generated by Codex. Note that the program will receive the notification argument as a string of JSON, e.g.:

```json
{
  "type": "agent-turn-complete",
  "turn-id": "12345",
  "input-messages": ["Rename `foo` to `bar` and update the callsites."],
  "last-assistant-message": "Rename complete and verified `cargo build` succeeds."
}
```

The `"type"` property will always be set. Currently, `"agent-turn-complete"` is the only notification type that is supported.

As an example, here is a Python script that parses the JSON and decides whether to show a desktop push notification using [terminal-notifier](https://github.com/julienXX/terminal-notifier) on macOS:

```python
#!/usr/bin/env python3

import json
import subprocess
import sys


def main() -> int:
    if len(sys.argv) != 2:
        print("Usage: notify.py <NOTIFICATION_JSON>")
        return 1

    try:
        notification = json.loads(sys.argv[1])
    except json.JSONDecodeError:
        return 1

    match notification_type := notification.get("type"):
        case "agent-turn-complete":
            assistant_message = notification.get("last-assistant-message")
            if assistant_message:
                title = f"Codex: {assistant_message}"
            else:
                title = "Codex: Turn Complete!"
            input_messages = notification.get("input_messages", [])
            message = " ".join(input_messages)
            title += message
        case _:
            print(f"not sending a push notification for: {notification_type}")
            return 0

    subprocess.check_output(
        [
            "terminal-notifier",
            "-title",
            title,
            "-message",
            message,
            "-group",
            "codex",
            "-ignoreDnD",
            "-activate",
            "com.googlecode.iterm2",
        ]
    )

    return 0


if __name__ == "__main__":
    sys.exit(main())
```

To have Codex use this script for notifications, you would configure it via `notify` in `~/.codex/config.toml` using the appropriate path to `notify.py` on your computer:

```toml
notify = ["python3", "/Users/mbolin/.codex/notify.py"]
```

======

codex-cli (codex-cli/README.md) "228 344 384"

---

## CLI reference

| Command                              | Purpose                             | Example                              |
| ------------------------------------ | ----------------------------------- | ------------------------------------ |
| `codex`                              | Interactive REPL                    | `codex`                              |
| `codex "..."`                        | Initial prompt for interactive REPL | `codex "fix lint errors"`            |
| `codex -q "..."`                     | Non-interactive "quiet mode"        | `codex -q --json "explain utils.ts"` |
| `codex completion <bash\|zsh\|fish>` | Print shell completion script       | `codex completion bash`              |

Key flags: `--model/-m`, `--approval-mode/-a`, `--quiet/-q`, and `--notify`.

---

## Configuration guide

Codex configuration files can be placed in the `~/.codex/` directory, supporting both YAML and JSON formats.

### Basic configuration parameters

| Parameter           | Type    | Default    | Description                      | Available Options                                                                              |
| ------------------- | ------- | ---------- | -------------------------------- | ---------------------------------------------------------------------------------------------- |
| `model`             | string  | `o4-mini`  | AI model to use                  | Any model name supporting OpenAI API                                                           |
| `approvalMode`      | string  | `suggest`  | AI assistant's permission mode   | `suggest` (suggestions only)<br>`auto-edit` (automatic edits)<br>`full-auto` (fully automatic) |
| `fullAutoErrorMode` | string  | `ask-user` | Error handling in full-auto mode | `ask-user` (prompt for user input)<br>`ignore-and-continue` (ignore and proceed)               |
| `notify`            | boolean | `true`     | Enable desktop notifications     | `true`/`false`                                                                                     |

### Configuration examples

(1) YAML format (save as `~/.codex/config.yaml`):

```yaml
model: o4-mini
approvalMode: suggest
fullAutoErrorMode: ask-user
notify: true
```

(2) JSON format (save as `~/.codex/config.json`):

```json
{
  "model": "o4-mini",
  "approvalMode": "suggest",
  "fullAutoErrorMode": "ask-user",
  "notify": true
}
```

======

## ACP(`agent-client-protocol`)refs

https://agentclientprotocol.com/

> open source repo: [agent-client-protocol](https://github.com/zed-industries/agent-client-protocol)

- **ACP meta**: @/Users/arthur/dev-space/agent-client-protocol/schema/meta.json
- **ACP JSON Schema**: @/Users/arthur/dev-space/agent-client-protocol/schema/schema.json

### ACP - local docs

> (/Users/arthur/dev-space/agent-client-protocol/docs/protocol)

- @/Users/arthur/dev-space/agent-client-protocol/docs/overview/architecture.mdx
- @/Users/arthur/dev-space/agent-client-protocol/docs/overview/introduction.mdx
- @/Users/arthur/dev-space/agent-client-protocol/docs/protocol/overview.mdx
- @/Users/arthur/dev-space/agent-client-protocol/docs/protocol/initialization.mdx
- @/Users/arthur/dev-space/agent-client-protocol/docs/protocol/schema.mdx
- @/Users/arthur/dev-space/agent-client-protocol/docs/protocol/session-setup.mdx
- @/Users/arthur/dev-space/agent-client-protocol/docs/protocol/prompt-turn.mdx
- @/Users/arthur/dev-space/agent-client-protocol/docs/protocol/agent-plan.mdx
- @/Users/arthur/dev-space/agent-client-protocol/docs/protocol/content.mdx
- @/Users/arthur/dev-space/agent-client-protocol/docs/protocol/tool-calls.mdx
- @/Users/arthur/dev-space/agent-client-protocol/docs/protocol/file-system.mdx

### ACP Rust libraries

description: "Rust library for the Agent Client Protocol"

```bash
cargo install agent-client-protocol
```

- (``agent-client-protocol = "0.1.1"``)
- [DOCS.RS agent_client_protocol](https://docs.rs/agent-client-protocol/0.1.1)

#### ACP Rust libraries - local open source code

> (/Users/arthur/dev-space/agent-client-protocol/rust)

The ACP(agent-client-protocol) Rust crate provides implementations of both sides of the Agent Client Protocol that you can use to build your own agent server or client.

To get started, add the crate as a dependency to your project's `Cargo.toml`:

```bash
cargo add agent-client-protocol
```

Depending on what kind of tool you're building, you'll need to implement either the [Agent](https://docs.rs/agent-client-protocol/latest/agent_client_protocol/trait.Agent.html) trait or the
[Client](https://docs.rs/agent-client-protocol/latest/agent_client_protocol/trait.Client.html) trait to define the interaction with the ACP counterpart.

The [agent](https://github.com/zed-industries/agent-client-protocol/blob/main/rust/example_agent.rs) and
[client](https://github.com/zed-industries/agent-client-protocol/blob/main/rust/example_client.rs) example binaries provide runnable examples of how to do this, which you can use as a starting point.

You can read the full documentation for the `agent-client-protocol` crate on [docs.rs](https://docs.rs/agent-client-protocol/latest/agent_client_protocol/).

**ACP Users**

The `agent-client-protocol` crate powers the integration with external agents in the [Zed](https://zed.dev) editor.

======
