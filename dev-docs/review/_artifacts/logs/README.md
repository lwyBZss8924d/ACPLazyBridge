# 证据留存与日志规范

目标

- 将回放/真实运行的 stdout/stderr 持久化，作为合规评审与回归验证的证据。

约定

- 日志目录：dev-docs/review/_artifacts/logs/
- 命名规范：run_YYYYMMDD_HHMMSS.log（回放或一次手测的完整输出）
- 工具：tee + jq（见 ../jq/filters.md）

常用命令

- 运行并留存：
  cat dev-docs/review/_artifacts/tests/handshake.jsonl \
    | target/debug/codex-cli-acp \
    | tee dev-docs/review/_artifacts/logs/run_$(date +%Y%m%d_%H%M%S).log

- 过滤错误与停止原因：
  cat dev-docs/review/_artifacts/logs/<logfile>.log \
    | jq -c 'select(.error)'
  cat dev-docs/review/_artifacts/logs/<logfile>.log \
    | jq -c 'select(.result.stopReason!=null) | .result.stopReason'

- 过滤流式与 turn-complete：
  cat dev-docs/review/_artifacts/logs/<logfile>.log \
    | jq -c 'select(.method=="session/update" and .params.type=="agent_message_chunk")'
  cat dev-docs/review/_artifacts/logs/<logfile>.log \
    | jq -c 'select(.method=="session/update" and .params.type=="agent-turn-complete")'

注意

- 生产/敏感信息请勿写入日志；若需排查，使用脱敏后的片段。
- YOLO 模式必须在日志中有显式风险提示（仅在隔离环境使用）。
