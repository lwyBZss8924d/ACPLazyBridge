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
    stdin: ChildStdin,
    stdout: ChildStdout,
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

        let mut child = cmd.spawn()
            .with_context(|| format!("Failed to spawn process: {}", path))?;

        let stdin = child.stdin.take()
            .context("Failed to take stdin from child process")?;
        let stdout = child.stdout.take()
            .context("Failed to take stdout from child process")?;
        let stderr = child.stderr.take()
            .context("Failed to take stderr from child process")?;

        trace!("Spawned process with PID: {:?}", child.id());

        Ok(Self {
            child,
            stdin,
            stdout,
            stderr: Some(stderr),
            stderr_task: None,
        })
    }

    /// Start monitoring stderr and logging output.
    /// 
    /// Note: This takes ownership of stderr, so it can only be called once.
    pub fn monitor_stderr(&mut self) -> Result<()> {
        // Take stderr from self (can only be done once)
        let stderr = self.stderr.take()
            .context("stderr already taken or not available")?;
        
        let task = tokio::spawn(async move {
            let mut reader = BufReader::new(stderr);
            let mut line = String::new();
            
            while let Ok(bytes) = reader.read_line(&mut line).await {
                if bytes == 0 {
                    break;
                }
                let trimmed = line.trim_end();
                if !trimmed.is_empty() {
                    warn!("Process stderr: {}", trimmed);
                }
                line.clear();
            }
        });
        self.stderr_task = Some(task);
        Ok(())
    }

    /// Get mutable reference to stdin for writing.
    pub fn stdin(&mut self) -> &mut ChildStdin {
        &mut self.stdin
    }

    /// Get mutable reference to stdout for reading.
    pub fn stdout(&mut self) -> &mut ChildStdout {
        &mut self.stdout
    }

    /// Check if the process is still running.
    pub fn is_running(&mut self) -> bool {
        self.child.try_wait()
            .map(|status| status.is_none())
            .unwrap_or(false)
    }

    /// Kill the child process.
    pub async fn kill(&mut self) -> Result<()> {
        self.child.kill().await
            .context("Failed to kill child process")?;
        Ok(())
    }

    /// Wait for the process to exit and return its status.
    pub async fn wait(&mut self) -> Result<std::process::ExitStatus> {
        let status = self.child.wait().await
            .context("Failed to wait for child process")?;
        
        if let Some(task) = self.stderr_task.take() {
            task.abort();
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
    
    while let Some(line) = lines.next_line().await
        .context("Failed to read line from stream")? {
        
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

/// Write a JSON line to an async writer.
/// 
/// Appends a newline and flushes the writer to ensure the message is sent immediately.
pub async fn write_line<W>(writer: &mut W, json_str: &str) -> Result<()>
where
    W: AsyncWrite + Unpin,
{
    trace!("Writing line: {}", json_str);
    
    writer.write_all(json_str.as_bytes()).await
        .context("Failed to write JSON to stream")?;
    writer.write_all(b"\n").await
        .context("Failed to write newline to stream")?;
    writer.flush().await
        .context("Failed to flush stream")?;
    
    Ok(())
}

/// Start a task that reads lines from a reader and sends them to a channel.
pub fn spawn_reader_task<R>(
    reader: R,
    sender: UnboundedSender<String>,
) -> JoinHandle<Result<()>>
where
    R: AsyncRead + Unpin + Send + 'static,
{
    tokio::spawn(async move {
        read_lines(reader, |line| {
            let sender = sender.clone();
            async move {
                sender.unbounded_send(line)
                    .map_err(|e| anyhow::anyhow!("Failed to send message to queue: {}", e))
            }
        }).await
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use futures::StreamExt;

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
        }).await.unwrap();
        
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
        }).await.unwrap();
        
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
}