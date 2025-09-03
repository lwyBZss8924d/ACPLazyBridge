//! Parameter validation for ACP protocol compliance

use anyhow::Result;
use std::path::Path;

/// Error kinds for proper JSON-RPC error code mapping
#[derive(Debug, Clone)]
pub enum RpcErrorKind {
    InvalidParams,
    Internal,
    MethodNotFound,
}

/// Custom error type that carries RPC error classification
#[derive(Debug)]
pub struct RpcError {
    pub kind: RpcErrorKind,
    pub message: String,
}

impl RpcError {
    pub fn invalid_params(msg: impl Into<String>) -> Self {
        Self {
            kind: RpcErrorKind::InvalidParams,
            message: msg.into(),
        }
    }

    pub fn internal(msg: impl Into<String>) -> Self {
        Self {
            kind: RpcErrorKind::Internal,
            message: msg.into(),
        }
    }
}

impl std::fmt::Display for RpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for RpcError {}

/// Validates that a path is absolute as required by ACP protocol
pub fn validate_absolute_path(path: &str) -> Result<(), RpcError> {
    let p = Path::new(path);
    if !p.is_absolute() {
        return Err(RpcError::invalid_params(format!(
            "Path must be absolute, got: {}",
            path
        )));
    }
    Ok(())
}

/// Validates that a line number is 1-based as required by ACP protocol
pub fn validate_line_number(line: Option<u32>) -> Result<(), RpcError> {
    if let Some(n) = line {
        if n < 1 {
            return Err(RpcError::invalid_params(format!(
                "Line numbers must be 1-based (minimum 1), got: {}", 
                n
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_absolute_path() {
        // Valid absolute paths
        assert!(validate_absolute_path("/home/user/file.txt").is_ok());
        assert!(validate_absolute_path("/").is_ok());
        assert!(validate_absolute_path("/usr/bin/app").is_ok());
        
        // Invalid relative paths
        assert!(validate_absolute_path("file.txt").is_err());
        assert!(validate_absolute_path("./file.txt").is_err());
        assert!(validate_absolute_path("../file.txt").is_err());
        assert!(validate_absolute_path("home/user/file.txt").is_err());
    }

    #[test]
    fn test_validate_line_number() {
        // Valid line numbers
        assert!(validate_line_number(Some(1)).is_ok());
        assert!(validate_line_number(Some(100)).is_ok());
        assert!(validate_line_number(Some(u32::MAX)).is_ok());
        assert!(validate_line_number(None).is_ok());
        
        // Invalid line numbers (0-based)
        assert!(validate_line_number(Some(0)).is_err());
    }
}