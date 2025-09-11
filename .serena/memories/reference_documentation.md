# Reference Documentation Hub

## Overview
A centralized reference hub has been established under `dev-docs/references/` to serve as the single source of truth for all third-party dependencies and protocols used in ACPLazyBridge.

## Structure
```
dev-docs/references/
├── acp.md                    # Agent Client Protocol official resources
├── zed_ide.md                # Zed IDE integration documentation
├── acp_adapters/
│   └── claude_code_acp.md    # Claude Code ACP adapter references
└── cli_agents/
    ├── claude_code.md         # Claude Code CLI documentation
    ├── codex.md              # OpenAI Codex CLI resources
    └── gemini.md             # Google Gemini CLI documentation
```

## Key References

### Agent Client Protocol (ACP)
- **Official Website**: https://agentclientprotocol.com/
- **GitHub Repository**: https://github.com/zed-industries/agent-client-protocol
- **Schema Files**: meta.json and schema.json in the official repo
- **Local Reference**: dev-docs/references/acp.md

### CLI Agents
- **Claude Code**: https://github.com/anthropics/claude-code
  - Local ref: dev-docs/references/cli_agents/claude_code.md
- **Codex**: https://github.com/openai/codex
  - Local ref: dev-docs/references/cli_agents/codex.md
- **Gemini CLI**: https://github.com/google-gemini/gemini-cli
  - Local ref: dev-docs/references/cli_agents/gemini.md

### ACP Adapters
- **Claude Code ACP**: https://github.com/zed-industries/claude-code-acp
  - Local ref: dev-docs/references/acp_adapters/claude_code_acp.md

### Zed IDE Integration
- **GitHub**: https://github.com/zed-industries/zed
- **Docs**: https://zed.dev/docs
- **Local ref**: dev-docs/references/zed_ide.md

## Usage Guidelines
1. Always reference the official documentation first
2. All URLs in reference docs must be public and permanent
3. Update references when upstream changes occur
4. Cross-reference with local_refs/ for vendored copies

## Maintenance
See `dev-docs/engineering/workflow.md` for guidelines on maintaining and updating this reference hub.