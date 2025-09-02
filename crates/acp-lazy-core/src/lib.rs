//! acp-lazy-core: shared utilities for building ACP agent servers in Rust.

pub mod permissions {
    #[derive(Debug, Clone, Copy)]
    pub enum AcpPermissionMode {
        Default,
        AcceptEdits,
        BypassPermissions,
        Plan,
    }

    #[derive(Debug, Clone)]
    pub struct CodexTurnOverrides {
        pub approval_policy: &'static str,       // "never" | "on-request" | ...
        pub sandbox_mode: &'static str,          // "read-only" | "workspace-write" | "danger-full-access"
        pub network_access: bool,
    }

    impl Default for CodexTurnOverrides {
        fn default() -> Self {
            Self { approval_policy: "never", sandbox_mode: "read-only", network_access: false }
        }
    }

    /// Map ACP modes to non-interactive Codex overrides to avoid UI approvals.
    pub fn map_acp_to_codex(mode: AcpPermissionMode) -> CodexTurnOverrides {
        match mode {
            AcpPermissionMode::Default => CodexTurnOverrides::default(),
            AcpPermissionMode::Plan => CodexTurnOverrides::default(),
            AcpPermissionMode::AcceptEdits => CodexTurnOverrides {
                approval_policy: "never",
                sandbox_mode: "workspace-write",
                network_access: false,
            },
            AcpPermissionMode::BypassPermissions => CodexTurnOverrides {
                approval_policy: "never",
                sandbox_mode: "workspace-write",
                network_access: true,
            },
        }
    }
}

pub mod transport {
    use anyhow::{Context, Result};
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    use tokio::process::{Child, Command};

    /// Spawn a child process with piped stdio.
    pub async fn spawn_with_stdio(path: &str, args: &[String]) -> Result<Child> {
        let mut cmd = Command::new(path);
        cmd.args(args)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());
        let child = cmd.spawn().context("failed to spawn child")?;
        Ok(child)
    }

    /// Read JSON lines from a reader and pass them to a handler (queue-friendly).
    pub async fn read_lines<R, F, Fut>(reader: R, mut handle: F) -> Result<()>
    where
        R: tokio::io::AsyncRead + Unpin,
        F: FnMut(String) -> Fut,
        Fut: std::future::Future<Output = Result<()>>,
    {
        let mut lines = BufReader::new(reader).lines();
        while let Some(line) = lines.next_line().await? {
            let line = line.trim().to_string();
            if line.is_empty() {
                continue;
            }
            handle(line).await?;
        }
        Ok(())
    }

    /// Write a JSON line to a writer (appends \n).
    pub async fn write_line<W: tokio::io::AsyncWrite + Unpin>(writer: &mut W, s: &str) -> Result<()> {
        writer.write_all(s.as_bytes()).await?;
        writer.write_all(b"\n").await?;
        writer.flush().await?;
        Ok(())
    }
}

pub mod logging {
    pub fn init() {
        use tracing_subscriber::prelude::*;
        let fmt_layer = tracing_subscriber::fmt::layer().with_target(false);
        let filter = tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));
        tracing_subscriber::registry().with(filter).with(fmt_layer).init();
    }
}

