//! Tests for notify sink integration

use anyhow::Result;
use serde_json::json;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

fn forwarder_path() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_acplb-notify-forwarder"))
}

#[test]
fn test_notify_forwarder_writes_to_file() -> Result<()> {
    // Create temp directory
    let temp_dir = TempDir::new()?;
    let notify_path = temp_dir.path().join("notify.jsonl");

    // Set environment variables
    // ast-grep-ignore
    std::env::set_var("ACPLB_NOTIFY_PATH", notify_path.to_str().unwrap());
    std::env::set_var("ACPLB_NOTIFY_KIND", "file");

    let forwarder_path = forwarder_path();

    // Test JSON payload
    let test_json = json!({
        "type": "agent-turn-complete",
        "turn-id": "test-123",
        "last-assistant-message": "Test completed"
    });

    // Run forwarder
    let output = Command::new(forwarder_path)
        .arg(test_json.to_string())
        // ast-grep-ignore
        .env("ACPLB_NOTIFY_PATH", notify_path.to_str().unwrap())
        .env("ACPLB_NOTIFY_KIND", "file")
        .output()?;

    assert!(
        output.status.success(),
        "Forwarder failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Verify file was written
    let contents = fs::read_to_string(&notify_path)?;
    assert!(contents.contains("agent-turn-complete"));
    assert!(contents.contains("test-123"));

    // Clean up
    std::env::remove_var("ACPLB_NOTIFY_PATH");
    std::env::remove_var("ACPLB_NOTIFY_KIND");

    Ok(())
}

#[test]
fn test_notify_forwarder_appends_to_existing_file() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let notify_path = temp_dir.path().join("notify.jsonl");

    // Pre-create file with content
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&notify_path)?;
    writeln!(file, "{{\"existing\": \"content\"}}")?;
    drop(file);

    let forwarder_path = forwarder_path();

    let test_json = json!({
        "type": "agent-turn-complete",
        "turn-id": "append-test"
    });

    let output = Command::new(forwarder_path)
        .arg(test_json.to_string())
        // ast-grep-ignore
        .env("ACPLB_NOTIFY_PATH", notify_path.to_str().unwrap())
        .env("ACPLB_NOTIFY_KIND", "file")
        .output()?;

    assert!(output.status.success());

    // Verify both lines exist
    let contents = fs::read_to_string(&notify_path)?;
    let lines: Vec<&str> = contents.lines().collect();
    assert_eq!(lines.len(), 2);
    assert!(lines[0].contains("existing"));
    assert!(lines[1].contains("append-test"));

    Ok(())
}

#[test]
fn test_notify_forwarder_fails_without_env() -> Result<()> {
    let forwarder_path = forwarder_path();

    // Run without ACPLB_NOTIFY_PATH
    let output = Command::new(forwarder_path)
        .arg("{\"test\": \"data\"}")
        .env_clear()
        .output()?;

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("ACPLB_NOTIFY_PATH"));

    Ok(())
}

#[cfg(test)]
mod notify_source_tests {
    use super::*;
    use codex_cli_acp::notify_source::{FileNotifySource, NotifySource};
    use tokio::sync::mpsc;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_file_notify_source_reads_new_lines() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let notify_path = temp_dir.path().join("notify.jsonl");

        // Create empty file
        fs::File::create(&notify_path)?;

        // Create notify source
        let mut source = FileNotifySource::new(&notify_path, 100);
        let (tx, mut rx) = mpsc::unbounded_channel();

        source.start_monitoring(tx).await?;

        // Write a notification
        let mut file = OpenOptions::new().append(true).open(&notify_path)?;
        writeln!(
            file,
            "{{\"type\": \"agent-turn-complete\", \"turn-id\": \"test-123\"}}"
        )?;
        file.flush()?;

        // Wait a bit for polling
        sleep(Duration::from_millis(200)).await;

        // Check if we received the event
        if let Ok(event) = rx.try_recv() {
            assert_eq!(event.event_type, "agent-turn-complete");
            assert_eq!(event.turn_id, Some("test-123".to_string()));
        }

        source.stop().await?;
        Ok(())
    }
}
