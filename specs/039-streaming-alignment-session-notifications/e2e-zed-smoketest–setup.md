# E2E Zed Smoke Test – Setup Notes (2025-09-27)

## Latest Status

✅ **WORKING**: Streaming notifications are now successfully reaching Zed IDE!

- ACPLazyBridge wraps SessionNotifications in proper JSON-RPC format (`{"jsonrpc": "2.0", "method": "session/update", "params": {...}}`)
- The docker wrapper launches the `codex-cli-sandbox-t033` image with embedded API key and forwards stdin/stdout to `codex proto`
- Codex CLI (v0.42.0) in Docker container successfully processes prompts and returns streaming responses
- Notifications flow correctly: Zed → ACPLazyBridge → Docker → Codex CLI → ACPLazyBridge → Zed UI

```text
Zed IDE (ACP Client Custom Agents "agent_servers" | "ACPLazyBridge")
    ↓ ↑
ACPLazyBridge (acp-lazy-core | codex-cli-acp)
    ↓ ↑
Docker Container (codex-cli-sandbox Codex CLI binary)
    ↓ ↑
Codex CLI (non-interactive Codex CLI CMD - isolated test environment)
```

## Zed Configuration

Add the following to your Zed settings.json under `agent_servers`:

```json
{
  "agent_servers": {
    "ACPLazyBridge": {
      "command": "${WORKTREE}/target/release/codex-cli-acp",
      "args": ["--acp"],
      "env": {
        "RUST_LOG": "codex_cli_acp=debug,acp_lazy_core=debug",
        "CODEX_CMD": "${WORKTREE}/scripts/codex-docker-wrapper.sh",
        "ACPLB_IDLE_TIMEOUT_MS": "60000",
        "ACPLB_NOTIFY_INJECT": "auto"
      }
    }
  }
}
```

**Note**: Replace `${WORKTREE}` with your actual ACPLazyBridge worktree path.

## Docker Container Setup

### Building the T033 Test Container

The T033 container includes:

- Codex CLI v0.42.0 (npm package) _always use latest version from `codex/codex-cli`_
- Embedded API key for testing
- Pre-configured `config.toml` with test settings

Build process:

```bash
# Navigate to Codex CLI directory
cd ${CODEX_CLI_DIR}

# Build T033 container using the dedicated build script
./scripts/build_t033_container.sh
```

The build script will:

1. Check for `.env` file with API key
2. Build npm package if needed (or use existing dist/codex.tgz)
3. Create Docker image `codex-cli-sandbox-t033:latest`

### Container Configuration

The container uses a pre-configured `config.toml` (codex-cli/config.toml) with:

- Model: `gpt-5-codex`
- Approval policy: `never` (for automated testing)
- Sandbox mode: Full access within container
- Trust level: Configured for test workspace

```toml
# Test configuration for Codex CLI sandbox container
# This is for T033 testing only - DO NOT USE IN PRODUCTION
# OPENAI_API_KEY test APIKEY only for Codex CLI running in sandbox testing environment

model = "gpt-5-codex"
model_provider = "openai"
model_reasoning_effort = "minimal"
model_verbosity = "medium"
preferred_auth_method = "apikey"
model_reasoning_summary = "auto"
show_raw_agent_reasoning = true

# OpenAI Test APIKEY Environment
[model_providers.openai]
name = "OpenAI"
base_url = "https://api.openai.com/v1"
env_key = "OPENAI_API_KEY"
wire_api = "responses"

[profiles.full_auto]
approval_policy = "never"
sandbox_mode = "danger-full-access"
experimental_use_exec_command_tool = true

[sandbox_workspace_write]
# These settings are used if you switch back to workspace-write
network_access = true
exclude_tmpdir_env_var = false
# Use for test workspace any path here
writable_roots = [
  "~/xx/xx"
]

# Shell environment policy for container
[shell_environment_policy]
experimental_use_profile = true
inherit = "all"
include_only = []

[tools]
# Enable web search tool (alias: web_search_request)
web_search = true

# Callback ACPLB notify
notify = ["notify-send", "Codex"]

# Project trust entries
[projects."/Users/arthur/dev-space"]
trust_level = "trusted"

# Project trust entries
[projects."/Users/arthur/dev-space/claude-code-acp/"]
trust_level = "trusted"

```

### Running the Test

1. **Build ACPLazyBridge release binary**:

   ```bash
   cd ${ACPLB_WORKTREE}
   cargo build --release -p codex-cli-acp
   ```

2. **Start Zed with updated configuration** (see Zed Configuration above)

3. **Test in Zed**:

   - Open Assistant Panel
   - Select "ACPLazyBridge" as the agent
   - Send a test prompt
   - Observe streaming responses in the UI

4. **Monitor logs**:

   ```bash
   # Docker container logs
   docker logs codex-t033-test

   # Zed ACP logs
   # In Zed: Command Palette → "dev: open acp logs"
   ```

## E2E Test Script

Run the complete end-to-end test suite:

```bash
cd ${ACPLB_WORKTREE}
./scripts/run-e2e-test.sh
```

This will:

- Build Docker container
- Build ACPLazyBridge binary
- Run T033c smoke tests
- Validate protocol compliance
- Generate evidence in `_artifacts/tests/T033/`

## Verification

Success indicators:

- ✅ Zed UI shows streaming responses
- ✅ No "received message with neither id nor method" errors in Zed logs
- ✅ JSON-RPC notifications have proper format
- ✅ E2E tests capture 4+ SessionNotifications

## Troubleshooting

If notifications aren't appearing:

1. Restart Zed to reload configuration
2. Verify `ACPLB_IDLE_TIMEOUT_MS` is in milliseconds (e.g., 60000)
3. Ensure Docker container has valid API key
4. Check logs for JSON-RPC format errors
