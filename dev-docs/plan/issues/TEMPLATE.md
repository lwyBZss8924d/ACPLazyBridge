# ISSUE 模板（提交到仓库的任务建议使用本模板）

标题：<模块>-<序号> — <一句话目标>

## 背景 / 需求
- 说明要解决的问题与范围（引用 REQ/ARC/SPEC）

## 技术方案
- 设计与实现要点（接口/数据结构/并发/错误处理/日志）

## local_refs 引用
- (local_refs/agent-client-protocol)
- (local_refs/codex)
- (local_refs/zed-acp-examples)

## 对应的 dev-docs/review 条目
- SPEC: ...
- REQ: ...
- ARC: ...
- CODEX: ...
- ZED: ...

## 验收标准
- 测试用例：_artifacts/tests/<file>.jsonl
- 日志证据：_artifacts/logs/<run_yyyymmdd_hhmmss>.log + jq 过滤脚本

## Worktree-first
- 分支：feature/<模块>-<序号>
- 初始化：git worktree add ../<模块>-<序号> feature/<模块>-<序号>
- 合并：以 PR 方式合入主仓；提交前确保 traceability.csv 更新到 Verified/Partial，无孤儿条目


