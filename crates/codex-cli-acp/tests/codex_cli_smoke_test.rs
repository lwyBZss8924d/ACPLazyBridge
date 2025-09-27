//! Optional Codex CLI smoke test.
//!
//! This test exercises the `codex` binary in non-interactive mode when the
//! sandbox container exposes it via environment variables. It intentionally
//! skips when the binary is unavailable so that regular CI runs remain
//! hermetic.

use std::env;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

const BIN_ENV: &str = "ACPLB_CODEX_SMOKE_BIN";
const CONFIG_ENV: &str = "ACPLB_CODEX_SMOKE_CONFIG";

#[test]
fn codex_cli_exec_smoke() {
    let bin = match env::var(BIN_ENV) {
        Ok(path) => path,
        Err(_) => return, // skip when the sandbox binary is not provided
    };

    // Optional configuration file path can be supplied by the sandbox.
    let config_arg = env::var(CONFIG_ENV).ok();

    // ast-grep-ignore: rust-no-unwrap
    let version_status = Command::new(&bin)
        .arg("--version")
        .status()
        .expect("failed to invoke codex --version");
    assert!(
        version_status.success(),
        "codex --version exited with {:?}",
        version_status
    );

    let mut exec_cmd = Command::new(&bin);
    if let Some(config) = config_arg {
        exec_cmd.arg("--config").arg(config);
    }
    // ast-grep-ignore: rust-no-unwrap
    let exec_status = exec_cmd
        .arg("exec")
        .arg("--help")
        .status()
        .expect("failed to invoke codex exec --help");
    assert!(
        exec_status.success(),
        "codex exec --help exited with {:?}",
        exec_status
    );
}

/// T033c: Test that SessionNotifications are written to stdout when using real Codex CLI
///
/// This test verifies the fix for the streaming notification issue where
/// SessionNotifications weren't reaching Zed. It runs the actual ACPLazyBridge
/// adapter with Docker-wrapped Codex CLI and confirms that notifications appear
/// in the output stream.
#[test]
#[ignore] // Run with: cargo test --ignored t033c_streaming_notifications
fn t033c_streaming_notifications() {
    // Check if we have the necessary environment for the test
    let codex_cmd =
        env::var("CODEX_CMD").unwrap_or_else(|_| "./scripts/codex-docker-wrapper.sh".to_string());

    // Skip if running in CI without Docker setup
    // The Docker container now has embedded API key, so we only check for CI
    if env::var("CI").is_ok() && env::var("SKIP_DOCKER_TESTS").is_ok() {
        eprintln!("Skipping T033c: Docker tests disabled in CI");
        return;
    }

    // Build path to our adapter binary using cargo manifest dir
    // ast-grep-ignore: rust-no-unwrap
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let adapter_bin =
        std::path::Path::new(&manifest_dir).join("../../target/release/codex-cli-acp");

    if !adapter_bin.exists() {
        eprintln!(
            "Skipping T033c: Release binary not found. Run: cargo build --release -p codex-cli-acp"
        );
        return;
    }

    // Start the adapter with Codex CLI
    // Set higher idle timeout for the test to ensure we capture all notifications
    // ast-grep-ignore: rust-no-unwrap
    let mut child = Command::new(&adapter_bin)
        .env("CODEX_CMD", &codex_cmd)
        .env("RUST_LOG", "warn")
        .env("ACPLB_IDLE_TIMEOUT_MS", "15000") // 15 seconds for test
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn codex-cli-acp");

    // ast-grep-ignore: rust-no-unwrap
    let mut stdin = child.stdin.take().expect("Failed to get stdin");
    // ast-grep-ignore: rust-no-unwrap
    let stdout = child.stdout.take().expect("Failed to get stdout");
    let mut reader = BufReader::new(stdout);

    // Send initialization
    // ast-grep-ignore: rust-no-unwrap
    writeln!(
        stdin,
        r#"{{"jsonrpc":"2.0","id":1,"method":"initialize","params":{{"protocolVersion":1}}}}"#
    )
    .expect("Failed to write initialize");

    // Create session
    // ast-grep-ignore: rust-no-unwrap
    writeln!(stdin, r#"{{"jsonrpc":"2.0","id":2,"method":"session/new","params":{{"cwd":"/tmp","mcpServers":[]}}}}"#)
        .expect("Failed to write session/new");

    let mut session_id = None;
    let mut responses = Vec::new();

    // Read initial responses to get session ID
    let mut line_buffer = String::new();
    for _ in 0..2 {
        line_buffer.clear();
        // ast-grep-ignore: rust-no-unwrap
        reader
            .read_line(&mut line_buffer)
            .expect("Failed to read line");
        let line = line_buffer.trim().to_string();
        responses.push(line.clone());

        // Extract session ID from session/new response
        if line.contains(r#""id":2"#) && line.contains("sessionId") {
            if let Some(start) = line.find(r#""sessionId":""#) {
                let start = start + 13;
                if let Some(end) = line[start..].find('"') {
                    session_id = Some(line[start..start + end].to_string());
                }
            }
        }
    }

    // ast-grep-ignore: rust-no-unwrap
    let session_id = session_id.expect("Failed to get session ID");
    eprintln!("T033c: Created session: {}", session_id);

    // Send prompt with correct session ID
    let prompt_request = format!(
        r#"{{"jsonrpc":"2.0","id":3,"method":"session/prompt","params":{{"sessionId":"{}","prompt":[{{"type":"text","text":"Say hello"}}]}}}}"#,
        session_id
    );
    // ast-grep-ignore: rust-no-unwrap
    writeln!(stdin, "{}", prompt_request).expect("Failed to write prompt");

    // Collect all responses including notifications
    let mut has_notifications = false;
    let mut notification_count = 0;
    let mut prompt_response_received = false;

    // Read with timeout
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(10);

    loop {
        if start.elapsed() > timeout {
            eprintln!("T033c: Timeout reached");
            break;
        }

        line_buffer.clear();
        match reader.read_line(&mut line_buffer) {
            Ok(0) => break, // EOF
            Ok(_) => {
                let line = line_buffer.trim().to_string();
                if line.is_empty() {
                    continue;
                }
                responses.push(line.clone());

                // Check for SessionNotification (has "update" field)
                if line.contains(r#""update":"#) {
                    has_notifications = true;
                    notification_count += 1;
                    eprintln!("T033c: Found SessionNotification #{}", notification_count);
                }

                // Check for prompt response
                if line.contains(r#""id":3"#) && line.contains("stopReason") {
                    prompt_response_received = true;
                    eprintln!("T033c: Received prompt response");
                    break;
                }
            }
            Err(e) => {
                eprintln!("T033c: Read error: {}", e);
                break;
            }
        }
    }

    // Save evidence
    // ast-grep-ignore: rust-no-unwrap
    let evidence_dir = env::current_dir()
        .unwrap()
        .join("_artifacts/039-streaming-alignment/tests");
    std::fs::create_dir_all(&evidence_dir).ok();

    let evidence_file = evidence_dir.join(format!(
        "T033c_output_{}.jsonl",
        chrono::Utc::now().format("%Y%m%d_%H%M%S")
    ));

    let evidence_content = responses.join("\n");
    std::fs::write(&evidence_file, &evidence_content).ok();
    eprintln!("T033c: Evidence saved to {:?}", evidence_file);

    // Clean up
    drop(stdin);
    let _ = child.kill();
    let _ = child.wait();

    // Assertions
    assert!(prompt_response_received, "Did not receive prompt response");
    assert!(
        has_notifications,
        "No SessionNotifications found in output. Expected streaming updates but got only: {}",
        responses.last().unwrap_or(&"<empty>".to_string())
    );
    assert!(
        notification_count > 0,
        "Expected at least one SessionNotification, got {}",
        notification_count
    );

    eprintln!(
        "T033c: SUCCESS - Found {} SessionNotifications",
        notification_count
    );
}
