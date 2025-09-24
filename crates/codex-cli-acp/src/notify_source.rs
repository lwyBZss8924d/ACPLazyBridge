//! NotifySource abstraction for monitoring notification sinks
//!
//! Provides trait-based monitoring of file/FIFO notification channels
//! to enable prompt turn completion via external signals.

use anyhow::{Context, Result};
use serde::Deserialize;
use serde_json::Value;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use tokio::sync::mpsc;
use tokio::time::{Duration, MissedTickBehavior};
use tracing::{debug, error, info, trace};

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
    polling_interval_ms: u64,
    stop_signal: Option<mpsc::Sender<()>>,
}

impl FileNotifySource {
    pub fn new(path: impl AsRef<Path>, polling_interval_ms: u64) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            polling_interval_ms,
            stop_signal: None,
        }
    }
}

#[async_trait::async_trait]
impl NotifySource for FileNotifySource {
    async fn start_monitoring(&mut self, tx: mpsc::UnboundedSender<NotifyEvent>) -> Result<()> {
        info!("Starting file notify monitoring: {:?}", self.path);

        let (stop_tx, mut stop_rx) = mpsc::channel::<()>(1);
        self.stop_signal = Some(stop_tx);

        let path = self.path.clone();
        let interval = Duration::from_millis(self.polling_interval_ms);

        tokio::spawn(async move {
            let mut position: u64 = 0;
            let mut ticker = tokio::time::interval(interval);
            ticker.set_missed_tick_behavior(MissedTickBehavior::Skip);

            loop {
                tokio::select! {
                    _ = stop_rx.recv() => {
                        debug!("Stopping file notify monitor");
                        break;
                    }
                    _ = ticker.tick() => {
                        if let Err(err) = scan_notify_file(&path, &tx, &mut position) {
                            trace!("Notify file scan error: {}", err);
                        }
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

fn scan_notify_file(
    path: &Path,
    tx: &mpsc::UnboundedSender<NotifyEvent>,
    position: &mut u64,
) -> Result<()> {
    if !path.exists() {
        return Ok(());
    }

    let mut file = OpenOptions::new().read(true).open(path)?;
    if *position > 0 {
        file.seek(SeekFrom::Start(*position))?;
    }

    let mut reader = BufReader::new(file);
    let mut line = String::new();

    loop {
        line.clear();
        let bytes = reader.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        *position += bytes as u64;

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        match serde_json::from_str::<Value>(trimmed) {
            Ok(json) => {
                if let Some(event_type) = json.get("type").and_then(|v| v.as_str()) {
                    if event_type == "agent-turn-complete" {
                        if let Ok(event) = serde_json::from_value::<NotifyEvent>(json) {
                            tx.send(event).ok();
                        }
                    }
                }
            }
            Err(err) => {
                debug!("Skipping invalid notify line {}: {}", trimmed, err);
            }
        }
    }

    Ok(())
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
                    _ = stop_rx.recv() => {
                        debug!("Stopping FIFO notify monitor");
                        break;
                    }
                    result = tokio::task::spawn_blocking({
                        let path = path.clone();
                        let tx = tx.clone();
                        move || read_fifo_blocking(&path, &tx)
                    }) => {
                        if let Err(join_err) = result {
                            error!("FIFO task join error: {}", join_err);
                            break;
                        } else if let Err(err) = result.unwrap() {
                            error!("FIFO read error: {}", err);
                        }
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

fn read_fifo_blocking(path: &Path, tx: &mpsc::UnboundedSender<NotifyEvent>) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .open(path)
        .with_context(|| format!("Failed to open FIFO: {:?}", path))?;

    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.context("Failed to read line from FIFO")?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

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
