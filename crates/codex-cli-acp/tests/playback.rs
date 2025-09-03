//! JSONL playback test utility for codex-cli-acp
//!
//! This test module allows replaying JSONL test files through the ACP server
//! to verify protocol compliance and response correctness.

use anyhow::{Context, Result};
use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Run a JSONL playback test
fn run_playback_test(test_file: &Path) -> Result<Vec<(Value, Option<Value>)>> {
    // Build the binary first
    Command::new("cargo")
        .args(&["build", "--bin", "codex-cli-acp"])
        .output()
        .context("Failed to build codex-cli-acp")?;

    // Spawn the ACP server
    let mut child = Command::new("cargo")
        .args(&["run", "--bin", "codex-cli-acp"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .env("RUST_LOG", "info")
        .spawn()
        .context("Failed to spawn codex-cli-acp")?;

    let mut stdin = child.stdin.take().expect("Failed to get stdin");
    let stdout = child.stdout.take().expect("Failed to get stdout");
    let stderr = child.stderr.take().expect("Failed to get stderr");

    // Channel for collecting responses
    let (tx, rx) = mpsc::channel();

    // Thread to read stdout
    let stdout_tx = tx.clone();
    let stdout_thread = thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                if !line.trim().is_empty() {
                    if let Ok(json) = serde_json::from_str::<Value>(&line) {
                        stdout_tx.send(json).ok();
                    }
                }
            }
        }
    });

    // Thread to log stderr
    thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            if let Ok(line) = line {
                eprintln!("STDERR: {}", line);
            }
        }
    });

    // Read test file and send requests
    let test_content = std::fs::read_to_string(test_file)
        .with_context(|| format!("Failed to read test file: {:?}", test_file))?;

    let mut results = Vec::new();
    
    for line in test_content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let request: Value = serde_json::from_str(line)
            .with_context(|| format!("Failed to parse request: {}", line))?;

        // Send request
        writeln!(stdin, "{}", request)?;
        stdin.flush()?;

        // Check if this is a notification (no id field)
        let is_notification = request.get("id").is_none();

        // Wait for response (if not a notification)
        let response = if !is_notification {
            // Wait up to 2 seconds for a response
            match rx.recv_timeout(Duration::from_secs(2)) {
                Ok(resp) => Some(resp),
                Err(_) => {
                    eprintln!("Timeout waiting for response to: {}", request);
                    None
                }
            }
        } else {
            // For notifications, give a small delay to process
            thread::sleep(Duration::from_millis(100));
            None
        };

        results.push((request, response));
    }

    // Clean shutdown
    drop(stdin);
    thread::sleep(Duration::from_millis(500));
    child.kill().ok();
    child.wait().ok();

    Ok(results)
}

#[test]
fn test_handshake() {
    let test_file = Path::new("../../dev-docs/review/_artifacts/tests/handshake.jsonl");
    let results = run_playback_test(test_file).expect("Playback failed");
    
    assert!(!results.is_empty(), "No results from playback");
    
    // Check the initialize response
    if let Some((req, Some(resp))) = results.first() {
        assert_eq!(req["method"], "initialize");
        assert_eq!(resp["jsonrpc"], "2.0");
        assert!(resp.get("result").is_some(), "Missing result in response");
        
        let result = &resp["result"];
        assert!(result.get("protocolVersion").is_some());
        assert!(result.get("capabilities").is_some());
        assert_eq!(result["promptCapabilities"]["image"], false);
    }
}

#[test]
fn test_basic_session() {
    let test_file = Path::new("../../dev-docs/review/_artifacts/tests/basic_session.jsonl");
    let results = run_playback_test(test_file).expect("Playback failed");
    
    assert!(results.len() >= 2, "Expected at least 2 interactions");
    
    // Check initialize
    let (_, init_resp) = &results[0];
    assert!(init_resp.is_some());
    assert!(init_resp.as_ref().unwrap().get("result").is_some());
    
    // Check session/new
    let (req, resp) = &results[1];
    assert_eq!(req["method"], "session/new");
    assert!(resp.is_some());
    
    let resp = resp.as_ref().unwrap();
    assert!(resp.get("result").is_some());
    assert!(resp["result"].get("sessionId").is_some());
}

#[test]
fn test_unknown_method() {
    let test_file = Path::new("../../dev-docs/review/_artifacts/tests/unknown_method.jsonl");
    let results = run_playback_test(test_file).expect("Playback failed");
    
    assert!(!results.is_empty());
    
    let (req, resp) = &results[0];
    assert_eq!(req["method"], "unknown/method");
    assert!(resp.is_some());
    
    let resp = resp.as_ref().unwrap();
    assert!(resp.get("error").is_some());
    assert_eq!(resp["error"]["code"], -32601); // Method not found
}

#[test]
fn test_invalid_params() {
    let test_file = Path::new("../../dev-docs/review/_artifacts/tests/invalid_params.jsonl");
    let results = run_playback_test(test_file).expect("Playback failed");
    
    assert!(!results.is_empty());
    
    // Should have an error response
    if let Some((_, Some(resp))) = results.iter().find(|(req, _)| req["method"] == "session/prompt") {
        assert!(resp.get("error").is_some());
        assert_eq!(resp["error"]["code"], -32602); // Invalid params
    }
}

#[test]
fn test_cancel_notification() {
    let test_file = Path::new("../../dev-docs/review/_artifacts/tests/cancel.jsonl");
    let results = run_playback_test(test_file).expect("Playback failed");
    
    // Cancel is a notification, should not get a response
    for (req, resp) in &results {
        if req["method"] == "session/cancel" {
            assert!(resp.is_none(), "Notifications should not have responses");
        }
    }
}