# JQ 过滤模板与使用说明

提取错误响应

- 错误摘要（code/message）：
  jq -c 'select(.error) | {id:.id, code:.error.code, message:.error.message}'
- 附带 data 字段（若有）：
  jq -c 'select(.error and .error.data) | {id:.id, code:.error.code, message:.error.message, data:.error.data}'

提取流式更新

- 文本分片：
  jq -c 'select(.method=="session/update" and .params.type=="agent_message_chunk") | .params.content'
- turn 完成通知：
  jq -c 'select(.method=="session/update" and .params.type=="agent-turn-complete") | .params.sessionId'

提取最终停止原因

- 停止原因（end_turn/cancelled 等）：
  jq -c 'select(.result.stopReason!=null) | .result.stopReason'

典型回放与证据留存

- 保存输出日志：
  cmd | tee dev-docs/review/_artifacts/logs/run_$(date +%Y%m%d_%H%M%S).log
- 从日志中提取错误：
  cat dev-docs/review/_artifacts/logs/<logfile>.log | jq -c 'select(.error)'
- 从日志中提取所有 session/update：
  cat dev-docs/review/_artifacts/logs/<logfile>.log | jq -c 'select(.method=="session/update")'
