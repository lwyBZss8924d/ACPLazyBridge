//! Minimal notify forwarder for Codex CLI integration
//!
//! This binary is invoked by Codex as a notify program with a single JSON argument.
//! It writes the JSON (newline-terminated) to the path specified in ACPLB_NOTIFY_PATH.

use std::env;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::process;

fn main() {
    if let Err(e) = run() {
        eprintln!("acplb-notify-forwarder error: {}", e);
        process::exit(1);
    }
}

fn run() -> io::Result<()> {
    // Get JSON argument from argv[1]
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Expected exactly 1 argument (JSON), got {}", args.len() - 1),
        ));
    }
    let json_str = &args[1];

    // Get notify path from environment
    let notify_path = env::var("ACPLB_NOTIFY_PATH").map_err(|e| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("ACPLB_NOTIFY_PATH not set: {}", e),
        )
    })?;

    // Get notify kind (default: file)
    let notify_kind = env::var("ACPLB_NOTIFY_KIND").unwrap_or_else(|_| "file".to_string());

    // Open file with appropriate options based on kind
    let mut file = match notify_kind.as_str() {
        "fifo" => {
            // For FIFO, open in append mode without creating
            OpenOptions::new().append(true).open(&notify_path)?
        }
        _ => {
            // For regular file, create if needed and append
            OpenOptions::new()
                .create(true)
                .append(true)
                .mode(0o644)
                .open(&notify_path)?
        }
    };

    // Write JSON with newline
    writeln!(file, "{}", json_str)?;
    file.flush()?;

    // Log success to stderr for debugging (optional)
    if env::var("ACPLB_DEBUG").is_ok() {
        eprintln!(
            "acplb-notify-forwarder: wrote to {} ({})",
            notify_path, notify_kind
        );
    }

    Ok(())
}
