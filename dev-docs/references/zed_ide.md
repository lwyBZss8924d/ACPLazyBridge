# Zed IDE - ACP Integration Reference

This document contains links to the source code within the Zed IDE repository that are relevant to its implementation of the Agent Client Protocol (ACP).

## Official Resources

- **Website:** [https://zed.dev/](https://zed.dev/)
- **GitHub Repository:** [https://github.com/zed-industries/zed](https://github.com/zed-industries/zed)

## Core ACP Implementation

These files represent the core logic for handling ACP agents within Zed. The user's original local paths pointed to a vendored copy of these files.

- **ACP Protocol Handling:**
  - `crates/agent/src/acp.rs`
  - **URL:** [https://github.com/zed-industries/zed/blob/main/crates/agent/src/acp.rs](https://github.com/zed-industries/zed/blob/main/crates/agent/src/acp.rs)

- **Agent Server Management:**
  - `crates/agent/src/agent_servers.rs`
  - **URL:** [https://github.com/zed-industries/zed/blob/main/crates/agent/src/agent_servers.rs](https://github.com/zed-industries/zed/blob/main/crates/agent/src/agent_servers.rs)

## Specific Agent Implementations

These files show how specific external CLI tools are integrated as agents.

- **Claude Agent:**
  - `crates/agent/src/agents/claude.rs`
  - **URL:** [https://github.com/zed-industries/zed/blob/main/crates/agent/src/agents/claude.rs](https://github.com/zed-industries/zed/blob/main/crates/agent/src/agents/claude.rs)

- **Gemini Agent:**
  - `crates/agent/src/agents/gemini.rs`
  - **URL:** [https://github.com/zed-industries/zed/blob/main/crates/agent/src/agents/gemini.rs](https://github.com/zed-industries/zed/blob/main/crates/agent/src/agents/gemini.rs)

- **Custom Agent:**
  - `crates/agent/src/agents/custom.rs`
  - **URL:** [https://github.com/zed-industries/zed/blob/main/crates/agent/src/agents/custom.rs](https://github.com/zed-industries/zed/blob/main/crates/agent/src/agents/custom.rs)
