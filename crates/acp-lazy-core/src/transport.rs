//! Transport layer for ACP communication, based on Zed's RPC patterns.
//!
//! This module provides line-based JSONL communication with proper async handling,
//! following patterns from local_refs/agent-client-protocol/rust/rpc.rs

use anyhow::{Context, Result};
use futures::channel::mpsc::{self, UnboundedReceiver, UnboundedSender};
use serde_json::Value;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStderr, ChildStdin, ChildStdout, Command};
use tokio::task::JoinHandle;
use tracing::{debug, error, trace, warn};

/// Manages a child process with stdio communication channels.
pub struct ProcessTransport {
    child: Child,
    stdin: Option<ChildStdin>,
    stdout: Option<ChildStdout>,
    stderr: Option<ChildStderr>,
    stderr_task: Option<JoinHandle<()>>,
}

impl ProcessTransport {
    /// Spawn a new process with piped stdio.
    ///
    /// # Arguments
    /// * `path` - Path to the executable
    /// * `args` - Command line arguments
    /// * `env` - Optional environment variables
    /// * `cwd` - Optional working directory
    pub async fn spawn(
        path: &str,
        args: &[String],
        env: Option<Vec<(String, String)>>,
        cwd: Option<&str>,
    ) -> Result<Self> {
        let mut cmd = Command::new(path);
        cmd.args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true);

        if let Some(env_vars) = env {
            for (key, value) in env_vars {
                cmd.env(key, value);
            }
        }

        if let Some(dir) = cwd {
            cmd.current_dir(dir);
        }

        let mut child = cmd
            .spawn()
            .with_context(|| format!("Failed to spawn process: {}", path))?;

        let stdin = child
            .stdin
            .take()
            .context("Failed to take stdin from child process")?;
        let stdout = child
            .stdout
            .take()
            .context("Failed to take stdout from child process")?;
        let stderr = child
            .stderr
            .take()
            .context("Failed to take stderr from child process")?;

        trace!("Spawned process with PID: {:?}", child.id());

        Ok(Self {
            child,
            stdin: Some(stdin),
            stdout: Some(stdout),
            stderr: Some(stderr),
            stderr_task: None,
        })
    }

    /// Start monitoring stderr and logging output.
    ///
    /// Note: This takes ownership of stderr, so it can only be called once.
    /// Log level is determined by content: error patterns trigger warn/error,
    /// normal output goes to debug to reduce noise.
    pub fn monitor_stderr(&mut self) -> Result<()> {
        // Take stderr from self (can only be done once)
        let stderr = self
            .stderr
            .take()
            .context("stderr already taken or not available")?;

        let task = tokio::spawn(async move {
            let mut reader = BufReader::new(stderr);
            let mut line = String::new();

            // Patterns that indicate errors or important warnings
            let error_patterns = ["error", "fatal", "panic", "fail", "exception"];
            let warn_patterns = ["warn", "warning", "deprecated"];

            while let Ok(bytes) = reader.read_line(&mut line).await {
                if bytes == 0 {
                    // EOF reached, exit gracefully
                    debug!("stderr monitor: EOF reached");
                    break;
                }
                let trimmed = line.trim_end();
                if !trimmed.is_empty() {
                    let lower = trimmed.to_lowercase();

                    // Determine severity based on content
                    if error_patterns.iter().any(|p| lower.contains(p)) {
                        error!("Process stderr: {}", trimmed);
                    } else if warn_patterns.iter().any(|p| lower.contains(p)) {
                        warn!("Process stderr: {}", trimmed);
                    } else {
                        debug!("Process stderr: {}", trimmed);
                    }
                }
                line.clear();
            }
            debug!("stderr monitor: Task ending normally");
        });
        self.stderr_task = Some(task);
        Ok(())
    }

    /// Get mutable reference to stdin for writing.
    pub fn stdin(&mut self) -> &mut ChildStdin {
        self.stdin.as_mut().expect("stdin already taken")
    }

    /// Get mutable reference to stdout for reading.
    pub fn stdout(&mut self) -> Option<&mut ChildStdout> {
        self.stdout.as_mut()
    }
    
    /// Take ownership of stdout (can only be called once).
    /// Returns None if already taken or not available.
    pub fn take_stdout(&mut self) -> Option<ChildStdout> {
        self.stdout.take()
    }
    
    /// Check if stdout is still available
    pub fn has_stdout(&self) -> bool {
        self.stdout.is_some()
    }

    /// Check if the process is still running.
    pub fn is_running(&mut self) -> bool {
        self.child
            .try_wait()
            .map(|status| status.is_none())
            .unwrap_or(false)
    }

    /// Kill the child process.
    pub async fn kill(&mut self) -> Result<()> {
        self.child
            .kill()
            .await
            .context("Failed to kill child process")?;
        Ok(())
    }

    /// Wait for the process to exit and return its status.
    pub async fn wait(&mut self) -> Result<std::process::ExitStatus> {
        let status = self
            .child
            .wait()
            .await
            .context("Failed to wait for child process")?;

        // Join the stderr task to ensure it completes gracefully
        if let Some(task) = self.stderr_task.take() {
            // Give the task a chance to finish naturally (up to 100ms)
            // This ensures we capture any final stderr output
            let _ = tokio::time::timeout(std::time::Duration::from_millis(100), task).await;
        }

        Ok(status)
    }
}

/// Message queue for handling incoming JSON lines.
pub struct MessageQueue {
    incoming_tx: UnboundedSender<String>,
    incoming_rx: Option<UnboundedReceiver<String>>,
}

impl MessageQueue {
    /// Create a new message queue.
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded();
        Self {
            incoming_tx: tx,
            incoming_rx: Some(rx),
        }
    }

    /// Get the sender for enqueuing messages.
    pub fn sender(&self) -> UnboundedSender<String> {
        self.incoming_tx.clone()
    }

    /// Take the receiver (can only be called once).
    pub fn take_receiver(&mut self) -> Option<UnboundedReceiver<String>> {
        self.incoming_rx.take()
    }
}

impl Default for MessageQueue {
    fn default() -> Self {
        Self::new()
    }
}

/// Read JSON lines from an async reader and send them to a handler.
///
/// This function reads lines from the input, skips empty lines and whitespace,
/// and passes valid lines to the provided handler. Errors in individual lines
/// are logged but don't stop processing.
pub async fn read_lines<R, F, Fut>(reader: R, mut handler: F) -> Result<()>
where
    R: AsyncRead + Unpin,
    F: FnMut(String) -> Fut,
    Fut: std::future::Future<Output = Result<()>>,
{
    let mut lines = BufReader::new(reader).lines();

    while let Some(line) = lines
        .next_line()
        .await
        .context("Failed to read line from stream")?
    {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            trace!("Skipping empty line");
            continue;
        }

        trace!("Read line: {}", trimmed);

        // Validate it's valid JSON before passing to handler
        match serde_json::from_str::<Value>(trimmed) {
            Ok(_) => {
                if let Err(e) = handler(trimmed.to_string()).await {
                    error!("Handler error for line: {}", e);
                    // Continue processing other lines
                }
            }
            Err(e) => {
                error!("Invalid JSON line (skipping): {} - Error: {}", trimmed, e);
                // Continue processing other lines
            }
        }
    }

    debug!("Finished reading lines from stream");
    Ok(())
}

/// Read JSON values from an async reader and send parsed values to a handler.
///
/// This function is more efficient than `read_lines` when the handler needs
/// parsed JSON values, as it avoids double-parsing. Lines that aren't valid
/// JSON are logged and skipped.
pub async fn read_values<R, F, Fut>(reader: R, mut handler: F) -> Result<()>
where
    R: AsyncRead + Unpin,
    F: FnMut(Value) -> Fut,
    Fut: std::future::Future<Output = Result<()>>,
{
    let mut lines = BufReader::new(reader).lines();

    while let Some(line) = lines
        .next_line()
        .await
        .context("Failed to read line from stream")?
    {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            trace!("Skipping empty line");
            continue;
        }

        trace!("Read line: {}", trimmed);

        // Parse JSON once and pass the Value to the handler
        match serde_json::from_str::<Value>(trimmed) {
            Ok(value) => {
                if let Err(e) = handler(value).await {
                    error!("Handler error for parsed value: {}", e);
                    // Continue processing other lines
                }
            }
            Err(e) => {
                error!("Invalid JSON line (skipping): {} - Error: {}", trimmed, e);
                // Continue processing other lines
            }
        }
    }

    debug!("Finished reading values from stream");
    Ok(())
}

/// Write a JSON line to an async writer.
///
/// Appends a newline and flushes the writer to ensure the message is sent immediately.
pub async fn write_line<W>(writer: &mut W, json_str: &str) -> Result<()>
where
    W: AsyncWrite + Unpin,
{
    trace!("Writing line: {}", json_str);

    writer
        .write_all(json_str.as_bytes())
        .await
        .context("Failed to write JSON to stream")?;
    writer
        .write_all(b"\n")
        .await
        .context("Failed to write newline to stream")?;
    writer.flush().await.context("Failed to flush stream")?;

    Ok(())
}

/// Start a task that reads lines from a reader and sends them to a channel.
pub fn spawn_reader_task<R>(reader: R, sender: UnboundedSender<String>) -> JoinHandle<Result<()>>
where
    R: AsyncRead + Unpin + Send + 'static,
{
    tokio::spawn(async move {
        read_lines(reader, |line| {
            let sender = sender.clone();
            async move {
                sender
                    .unbounded_send(line)
                    .map_err(|e| anyhow::anyhow!("Failed to send message to queue: {}", e))
            }
        })
        .await
    })
}

/// Start a task that reads JSON values from a reader and sends them to a channel.
///
/// This is more efficient than `spawn_reader_task` when values need to be parsed,
/// as it avoids double-parsing.
pub fn spawn_value_reader_task<R>(
    reader: R,
    sender: UnboundedSender<Value>,
) -> JoinHandle<Result<()>>
where
    R: AsyncRead + Unpin + Send + 'static,
{
    tokio::spawn(async move {
        read_values(reader, |value| {
            let sender = sender.clone();
            async move {
                sender
                    .unbounded_send(value)
                    .map_err(|e| anyhow::anyhow!("Failed to send value to queue: {}", e))
            }
        })
        .await
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;
    use std::sync::{Arc, Mutex};

    #[tokio::test]
    async fn test_write_line() {
        let mut buffer = Vec::new();
        let json = r#"{"test": "value"}"#;

        write_line(&mut buffer, json).await.unwrap();

        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "{\"test\": \"value\"}\n");
    }

    #[tokio::test]
    async fn test_read_lines_skip_empty() {
        let input = "  \n{\"valid\": 1}\n\n  \n{\"valid\": 2}\n";
        let cursor = std::io::Cursor::new(input);

        let received = Arc::new(Mutex::new(Vec::new()));
        let received_clone = received.clone();

        read_lines(cursor, move |line| {
            let received = received_clone.clone();
            async move {
                received.lock().unwrap().push(line);
                Ok(())
            }
        })
        .await
        .unwrap();

        let results = received.lock().unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0], r#"{"valid": 1}"#);
        assert_eq!(results[1], r#"{"valid": 2}"#);
    }

    #[tokio::test]
    async fn test_read_lines_skip_invalid_json() {
        let input = "{\"valid\": 1}\ninvalid json\n{\"valid\": 2}\n";
        let cursor = std::io::Cursor::new(input);

        let received = Arc::new(Mutex::new(Vec::new()));
        let received_clone = received.clone();

        read_lines(cursor, move |line| {
            let received = received_clone.clone();
            async move {
                received.lock().unwrap().push(line);
                Ok(())
            }
        })
        .await
        .unwrap();

        let results = received.lock().unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0], r#"{"valid": 1}"#);
        assert_eq!(results[1], r#"{"valid": 2}"#);
    }

    #[tokio::test]
    async fn test_message_queue() {
        let mut queue = MessageQueue::new();
        let sender = queue.sender();
        let mut receiver = queue.take_receiver().unwrap();

        sender.unbounded_send("message1".to_string()).unwrap();
        sender.unbounded_send("message2".to_string()).unwrap();

        assert_eq!(receiver.next().await.unwrap(), "message1");
        assert_eq!(receiver.next().await.unwrap(), "message2");
    }

    #[tokio::test]
    async fn test_stderr_capture_on_exit() {
        // This test simulates a process that writes to stderr right before exiting
        // to ensure we don't lose tail output

        // Use echo command to write to stderr and exit
        let mut transport = ProcessTransport::spawn(
            "sh",
            &[
                "-c".to_string(),
                "echo 'early stderr' >&2; sleep 0.01; echo 'final stderr' >&2".to_string(),
            ],
            None,
            None,
        )
        .await
        .unwrap();

        // Start monitoring stderr
        transport.monitor_stderr().unwrap();

        // Wait for process to complete - should capture all stderr
        let status = transport.wait().await.unwrap();
        assert!(status.success());

        // Give a moment for logs to be processed
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        // Note: In a real test we'd capture the logs, but for now
        // this ensures the code paths are exercised without panicking
    }

    #[tokio::test]
    async fn test_stderr_logging_severity() {
        // Test that error patterns trigger appropriate log levels
        // This test verifies the code paths work without panicking

        // Simulate stderr with various severity patterns
        let mut transport = ProcessTransport::spawn(
            "sh",
            &["-c".to_string(), 
             "echo 'normal output' >&2; echo 'WARNING: deprecated' >&2; echo 'ERROR: failed' >&2".to_string()],
            None,
            None
        ).await.unwrap();

        // Start monitoring - will use smart severity detection
        transport.monitor_stderr().unwrap();

        // Wait for process to complete
        let status = transport.wait().await.unwrap();
        assert!(status.success());

        // Give a moment for logs to be processed
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        // In real usage, 'normal output' would be debug level,
        // 'WARNING: deprecated' would be warn level,
        // 'ERROR: failed' would be error level
    }

    #[tokio::test]
    async fn test_read_values_parses_once() {
        let input = r#"{"id": 1, "value": "first"}
{"id": 2, "value": "second"}
invalid json
{"id": 3, "value": "third"}"#;
        let cursor = std::io::Cursor::new(input);

        let received = Arc::new(Mutex::new(Vec::new()));
        let received_clone = received.clone();

        read_values(cursor, move |value| {
            let received = received_clone.clone();
            async move {
                // Verify we received a parsed Value, not a string
                if let Some(id) = value.get("id").and_then(|v| v.as_i64()) {
                    received.lock().unwrap().push(id);
                }
                Ok(())
            }
        })
        .await
        .unwrap();

        let results = received.lock().unwrap();
        assert_eq!(results.len(), 3);
        assert_eq!(results[0], 1);
        assert_eq!(results[1], 2);
        assert_eq!(results[2], 3);
    }

    #[tokio::test]
    async fn test_spawn_value_reader_task() {
        let input = r#"{"type": "message", "content": "hello"}
{"type": "message", "content": "world"}"#;
        let cursor = std::io::Cursor::new(input);

        let (tx, mut rx) = mpsc::unbounded::<Value>();
        let task = spawn_value_reader_task(cursor, tx);

        task.await.unwrap().unwrap();

        // Verify we received parsed Values
        let val1 = rx.next().await.unwrap();
        assert_eq!(val1["type"], "message");
        assert_eq!(val1["content"], "hello");

        let val2 = rx.next().await.unwrap();
        assert_eq!(val2["type"], "message");
        assert_eq!(val2["content"], "world");
    }
}
