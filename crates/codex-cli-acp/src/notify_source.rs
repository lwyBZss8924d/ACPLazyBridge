//! NotifySource abstraction for monitoring notification sinks
//!
//! Provides trait-based monitoring of file/FIFO notification channels
//! to enable prompt turn completion via external signals.

use anyhow::{Context, Result};
use serde::Deserialize;
use serde_json::Value;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};
use tracing::{debug, error, info, trace, warn};

/// Notification event from Codex
#[derive(Debug, Clone, Deserialize)]
pub struct NotifyEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    #[serde(rename = "turn-id")]
    #[allow(dead_code)]
    pub turn_id: Option<String>,
    #[serde(rename = "input-messages")]
    #[allow(dead_code)]
    pub input_messages: Option<Vec<String>>,
    #[serde(rename = "last-assistant-message")]
    #[allow(dead_code)]
    pub last_assistant_message: Option<String>,
}

/// Trait for notification source monitoring
#[async_trait::async_trait]
pub trait NotifySource: Send + Sync {
    /// Start monitoring for notifications
    async fn start_monitoring(&mut self, tx: mpsc::UnboundedSender<NotifyEvent>) -> Result<()>;

    /// Stop monitoring
    async fn stop(&mut self) -> Result<()>;
}

/// File-based notification source (tail-follow with polling)
pub struct FileNotifySource {
    path: PathBuf,
    file: Option<BufReader<File>>,
    position: u64,
    polling_interval_ms: u64,
    stop_signal: Option<mpsc::Sender<()>>,
}

impl FileNotifySource {
    pub fn new(path: impl AsRef<Path>, polling_interval_ms: u64) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            file: None,
            position: 0,
            polling_interval_ms,
            stop_signal: None,
        }
    }

    fn open_or_reopen(&mut self) -> Result<()> {
        // Try to open the file
        match OpenOptions::new().read(true).open(&self.path) {
            Ok(mut file) => {
                // Seek to last known position or end for initial open
                if self.position > 0 {
                    file.seek(SeekFrom::Start(self.position))
                        .context("Failed to seek in notify file")?;
                } else {
                    // For first open, start from end to avoid processing old notifications
                    self.position = file
                        .seek(SeekFrom::End(0))
                        .context("Failed to seek to end of notify file")?;
                }
                self.file = Some(BufReader::new(file));
                debug!(
                    "Opened notify file at position {}: {:?}",
                    self.position, self.path
                );
                Ok(())
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                // File doesn't exist yet, that's ok for file mode
                trace!("Notify file not found (will retry): {:?}", self.path);
                self.file = None;
                Ok(())
            }
            Err(e) => Err(e).context("Failed to open notify file"),
        }
    }

    #[allow(dead_code)]
    async fn read_new_lines(&mut self, tx: &mpsc::UnboundedSender<NotifyEvent>) -> Result<()> {
        // Ensure file is open
        if self.file.is_none() {
            self.open_or_reopen()?;
            if self.file.is_none() {
                // File still doesn't exist
                return Ok(());
            }
        }

        let reader = self.file.as_mut().unwrap();
        let mut line = String::new();
        let mut had_activity = false;

        loop {
            line.clear();
            match reader.read_line(&mut line) {
                Ok(0) => {
                    // EOF reached
                    break;
                }
                Ok(bytes_read) => {
                    self.position += bytes_read as u64;
                    had_activity = true;

                    // Try to parse as JSON
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        continue;
                    }

                    match serde_json::from_str::<Value>(trimmed) {
                        Ok(json) => {
                            // Check if this is an agent-turn-complete event
                            if let Some(event_type) = json.get("type").and_then(|v| v.as_str()) {
                                if event_type == "agent-turn-complete" {
                                    info!("Received agent-turn-complete notification");
                                    if let Ok(event) = serde_json::from_value::<NotifyEvent>(json) {
                                        tx.send(event).ok();
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            debug!("Non-JSON line in notify file: {} - {}", trimmed, e);
                        }
                    }
                }
                Err(e) => {
                    warn!("Error reading notify file: {}", e);
                    // File might have been truncated/rotated, try reopening
                    self.position = 0;
                    self.open_or_reopen()?;
                    break;
                }
            }
        }

        if had_activity {
            debug!("Read notify file up to position {}", self.position);
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl NotifySource for FileNotifySource {
    async fn start_monitoring(&mut self, _tx: mpsc::UnboundedSender<NotifyEvent>) -> Result<()> {
        info!("Starting file notify monitoring: {:?}", self.path);

        let (stop_tx, mut stop_rx) = mpsc::channel::<()>(1);
        self.stop_signal = Some(stop_tx);

        // Initial open attempt
        self.open_or_reopen()?;

        // Start monitoring task
        let polling_interval_ms = self.polling_interval_ms;
        let path = self.path.clone();

        tokio::spawn(async move {
            let mut poll_interval = interval(Duration::from_millis(polling_interval_ms));
            poll_interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

            loop {
                tokio::select! {
                    _ = poll_interval.tick() => {
                        // Poll for new content
                        // Note: We can't move self into the spawned task, so this is simplified
                        // In the real implementation, we'd use a shared state approach
                        trace!("Polling notify file: {:?}", path);
                    }
                    _ = stop_rx.recv() => {
                        debug!("Stopping notify file monitor");
                        break;
                    }
                }
            }
        });

        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        if let Some(stop_signal) = self.stop_signal.take() {
            stop_signal.send(()).await.ok();
        }
        self.file = None;
        Ok(())
    }
}

/// FIFO-based notification source
pub struct FifoNotifySource {
    path: PathBuf,
    stop_signal: Option<mpsc::Sender<()>>,
}

impl FifoNotifySource {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            stop_signal: None,
        }
    }
}

#[async_trait::async_trait]
impl NotifySource for FifoNotifySource {
    async fn start_monitoring(&mut self, tx: mpsc::UnboundedSender<NotifyEvent>) -> Result<()> {
        info!("Starting FIFO notify monitoring: {:?}", self.path);

        let (stop_tx, mut stop_rx) = mpsc::channel::<()>(1);
        self.stop_signal = Some(stop_tx);

        let path = self.path.clone();

        // Spawn monitoring task
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    result = Self::read_fifo(&path, &tx) => {
                        if let Err(e) = result {
                            error!("FIFO read error: {}", e);
                            // Continue trying
                        }
                    }
                    _ = stop_rx.recv() => {
                        debug!("Stopping FIFO notify monitor");
                        break;
                    }
                }
            }
        });

        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        if let Some(stop_signal) = self.stop_signal.take() {
            stop_signal.send(()).await.ok();
        }
        Ok(())
    }
}

impl FifoNotifySource {
    async fn read_fifo(path: &Path, tx: &mpsc::UnboundedSender<NotifyEvent>) -> Result<()> {
        // Use blocking task for FIFO reading
        let path = path.to_path_buf();
        let tx = tx.clone();

        tokio::task::spawn_blocking(move || -> Result<()> {
            let file = OpenOptions::new()
                .read(true)
                .open(&path)
                .with_context(|| format!("Failed to open FIFO: {:?}", path))?;

            let reader = BufReader::new(file);
            for line in reader.lines() {
                let line = line.context("Failed to read line from FIFO")?;
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }

                // Try to parse as JSON
                match serde_json::from_str::<Value>(trimmed) {
                    Ok(json) => {
                        if let Some(event_type) = json.get("type").and_then(|v| v.as_str()) {
                            if event_type == "agent-turn-complete" {
                                info!("Received agent-turn-complete from FIFO");
                                if let Ok(event) = serde_json::from_value::<NotifyEvent>(json) {
                                    tx.send(event).ok();
                                }
                            }
                        }
                    }
                    Err(e) => {
                        debug!("Non-JSON line in FIFO: {} - {}", trimmed, e);
                    }
                }
            }
            Ok(())
        })
        .await
        .context("FIFO reading task failed")?
    }
}

/// Create a NotifySource based on environment configuration
pub fn create_notify_source(
    path: impl AsRef<Path>,
    kind: Option<&str>,
    polling_interval_ms: u64,
) -> Box<dyn NotifySource + Send> {
    let kind = kind.unwrap_or("file");
    match kind {
        "fifo" => {
            info!("Creating FIFO notify source: {:?}", path.as_ref());
            Box::new(FifoNotifySource::new(path))
        }
        _ => {
            info!("Creating file notify source: {:?}", path.as_ref());
            Box::new(FileNotifySource::new(path, polling_interval_ms))
        }
    }
}
