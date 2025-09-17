# SDD Rules - Tools MCP

## MCP Servers Configuration

```bash
# MCP Servers Authorization Token
export GITHUB_TOKEN=$(security find-generic-password -s github)
export JINA_API_KEY=$(security find-generic-password -s jina)
export ANTHROPIC_API_KEY=$(security find-generic-password -s anthropic)
```

### "github-mcp"

- [MCP] <https://github.com/github/github-mcp-server>
**Purpose**: Repository operations
**install**: remote HTTP server
**Agents**: All
**Config**:

```json
{
  "mcpServers": {
    "github": {
      "url": "https://api.githubcopilot.com/mcp/",
      "headers": {
        "Authorization": "Bearer YOUR_GITHUB_PAT"
      }
    }
  }
}
```

**For Claude Code CLI:**

```bash
claude mcp add --transport http github https://api.githubcopilot.com/mcp -H "Authorization: Bearer YOUR_GITHUB_PAT"
```

### "jina-mcp"

- [MCP] <https://github.com/jina-ai/MCP>
**Purpose**: Web research and extraction
**install**: remote HTTP server
**Agents**: All
**Config**:

```json
{
  "mcpServers": {
    "jina-mcp-server": {
      "url": "https://mcp.jina.ai/mcp",
      "headers": {
        "Authorization": "Bearer ${JINA_API_KEY}"
      }
    }
  }
}
```

**For Claude Code CLI:**

```bash
claude mcp add --transport sse jina-mcp https://mcp.jina.ai/sse \
  --header "X-API-Key: Bearer ${JINA_API_KEY}"
```

### "context7"

- [MCP] <https://github.com/upstash/context7>
**Purpose**: Library documentation
**install**: remote HTTP server
**Agents**: All
**Config**:

```json
{
  "mcpServers": {
    "context7": {
      "url": "https://mcp.context7.com/mcp",
      "headers": {
        "CONTEXT7_API_KEY": "{CONTEXT7_API_KEY}"
      }
    }
  }
}
```

**For Claude Code CLI:**

```bash
claude mcp add --transport http context7 https://mcp.context7.com/mcp --header "CONTEXT7_API_KEY: YOUR_API_KEY"
```

### "deepwiki"

- [MCP] <https://mcp.deepwiki.com>
**Purpose**: any Github Repository research
**install**: remote HTTP server
**Agents**: All
**Config**:

```json
{
  "mcpServers": {
    "deepwiki": {
      "serverUrl": "https://mcp.deepwiki.com/mcp"
    }
  }
}
```

**For Claude Code CLI:**

```bash
claude mcp add -s user -t http deepwiki https://mcp.deepwiki.com/mcp
```

### "serena"

- [MCP] <https://github.com/oraios/serena>
**Purpose**: Semantic code analysis
**install**: local stdio macp server

```bash
uvx --from git+https://github.com/oraios/serena serena start-mcp-server
```

**Agents**: All
**Config**:

```json
{
    "mcpServers": {
        "serena": {
            "command": "/abs/path/to/uvx",
            "args": ["--from", "git+https://github.com/oraios/serena", "serena", "start-mcp-server"]
        }
    }
}
```

**For Claude Code CLI:**

```bash
claude mcp add serena -- uvx --from git+https://github.com/oraios/serena serena start-mcp-server --context ide-assistant --project $(pwd)
```

---

```yaml
constitution:
    version: "1.0.1"
    last_checked: "2025-09-17T04:32:00Z"
rules:
    name: "tools-mcp-list"
    category: "tools-mcp"
    version: "1.0.1"
document:
    type: "sdd-rule"
    path: "sdd-rules/rules/tools-mcp/sdd-rules-tools-mcp.md"
    last_updated: "2025-09-17T08:26:00Z"
    related:
        - "sdd-rules/AGENTS.md"
```
