//! JSONL playback utility for manual testing
//!
//! Usage: cargo run --bin playback < test.jsonl

use anyhow::{Context, Result};
use serde_json::Value;
use std::io::{self, BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    // Parse arguments
    let args: Vec<String> = std::env::args().collect();
    let test_file = args.get(1);

    // Build the binary first
    eprintln!("Building codex-cli-acp...");
    Command::new("cargo")
        .args(["build", "--bin", "codex-cli-acp"])
        .output()
        .context("Failed to build codex-cli-acp")?;

    // Spawn the ACP server
    eprintln!("Starting ACP server...");
    let mut child = Command::new("cargo")
        .args(["run", "--bin", "codex-cli-acp"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .env("RUST_LOG", "info")
        .spawn()
        .context("Failed to spawn codex-cli-acp")?;

    let mut stdin_writer = child.stdin.take().expect("Failed to get stdin");
    let stdout = child.stdout.take().expect("Failed to get stdout");
    let stderr = child.stderr.take().expect("Failed to get stderr");

    // Channel for collecting responses
    let (tx, rx) = mpsc::channel();

    // Thread to read stdout
    let _stdout_thread = thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines().map_while(|l| l.ok()) {
            if !line.trim().is_empty() {
                println!("<<< {}", line);
                if let Ok(json) = serde_json::from_str::<Value>(&line) {
                    tx.send(json).ok();
                }
            }
        }
    });

    // Thread to log stderr
    thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines().map_while(|l| l.ok()) {
            eprintln!("STDERR: {}", line);
        }
    });

    // Read from stdin or file
    let stdin = io::stdin();
    let reader: Box<dyn BufRead> = if let Some(filename) = test_file {
        eprintln!("Reading from file: {}", filename);
        Box::new(BufReader::new(std::fs::File::open(filename)?))
    } else {
        eprintln!("Reading from stdin (paste JSONL and press Ctrl+D when done):");
        Box::new(stdin.lock())
    };

    // Process each line
    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Parse and validate JSON
        let request: Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to parse JSON: {}", e);
                eprintln!("Line: {}", line);
                continue;
            }
        };

        // Display the request
        println!(">>> {}", request);

        // Send request
        writeln!(stdin_writer, "{}", request)?;
        stdin_writer.flush()?;

        // Check if this is a notification (no id field)
        let is_notification = request.get("id").is_none();

        if !is_notification {
            // Wait for a matching response by id
            let req_id = request.get("id").cloned();
            let deadline = std::time::Instant::now() + Duration::from_secs(5);
            loop {
                let remain = deadline.saturating_duration_since(std::time::Instant::now());
                if remain.is_zero() {
                    eprintln!("⚠️  Timeout waiting for response (id={:?})", req_id);
                    break;
                }
                match rx.recv_timeout(remain) {
                    Ok(resp) => {
                        if let (Some(rid), Some(pid)) = (resp.get("id"), req_id.as_ref()) {
                            if rid == pid {
                                // matched; stdout thread already printed
                                break;
                            }
                        }
                        // Non-matching message (e.g., async notification): ignore and continue waiting
                    }
                    Err(_) => {
                        eprintln!("⚠️  Timeout waiting for response (id={:?})", req_id);
                        break;
                    }
                }
            }
        } else {
            eprintln!("ℹ️  Sent notification (no response expected)");
            thread::sleep(Duration::from_millis(100));
        }

        println!(); // Empty line for readability
    }

    eprintln!("\nPlayback complete. Shutting down...");

    // Clean shutdown
    drop(stdin_writer);
    thread::sleep(Duration::from_millis(500));
    child.kill().ok();
    child.wait().ok();

    Ok(())
}
