use anyhow::Result;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    acp_lazy_core::logging::init();
    info!("codex-cli-acp starting (skeleton)");

    // NOTE: This binary is intentionally left as a skeleton.
    // See examples/minimal_acp_echo.rs for a minimal ACP echo server example.
    // Real implementation should follow the plan in:
    //   dev-docs/plan/m1-technical-implementation-plan.md

    warn!("codex-cli-acp skeleton: no-op process exiting");
    Ok(())
}

