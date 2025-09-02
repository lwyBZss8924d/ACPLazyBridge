//! JSON-RPC 2.0 protocol types for ACP communication.
//!
//! Based on patterns from local_refs/agent-client-protocol and the JSON-RPC 2.0 spec.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

/// JSON-RPC 2.0 version string.
pub const JSONRPC_VERSION: &str = "2.0";

/// Standard JSON-RPC 2.0 error codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    /// Parse error: Invalid JSON was received by the server.
    ParseError = -32700,
    /// Invalid Request: The JSON sent is not a valid Request object.
    InvalidRequest = -32600,
    /// Method not found: The method does not exist or is not available.
    MethodNotFound = -32601,
    /// Invalid params: Invalid method parameter(s).
    InvalidParams = -32602,
    /// Internal error: Internal JSON-RPC error.
    InternalError = -32603,
}

impl ErrorCode {
    pub fn as_i32(self) -> i32 {
        self as i32
    }

    pub fn message(&self) -> &'static str {
        match self {
            Self::ParseError => "Parse error",
            Self::InvalidRequest => "Invalid Request",
            Self::MethodNotFound => "Method not found",
            Self::InvalidParams => "Invalid params",
            Self::InternalError => "Internal error",
        }
    }
}

/// JSON-RPC 2.0 Error object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    /// Error code
    pub code: i32,
    /// Error message
    pub message: String,
    /// Optional additional data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl Error {
    /// Create a new error with the given code and message.
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code: code.as_i32(),
            message: message.into(),
            data: None,
        }
    }

    /// Create a standard error from an error code.
    pub fn from_code(code: ErrorCode) -> Self {
        Self::new(code, code.message())
    }

    /// Add additional data to the error.
    pub fn with_data(mut self, data: impl Serialize) -> Self {
        self.data = Some(serde_json::to_value(data).unwrap_or(Value::Null));
        self
    }

    /// Create a parse error.
    pub fn parse_error() -> Self {
        Self::from_code(ErrorCode::ParseError)
    }

    /// Create an invalid request error.
    pub fn invalid_request() -> Self {
        Self::from_code(ErrorCode::InvalidRequest)
    }

    /// Create a method not found error.
    pub fn method_not_found(method: &str) -> Self {
        Self::new(ErrorCode::MethodNotFound, format!("Method not found: {}", method))
    }

    /// Create an invalid params error.
    pub fn invalid_params(details: impl Into<String>) -> Self {
        Self::new(ErrorCode::InvalidParams, "Invalid params")
            .with_data(details.into())
    }

    /// Create an internal error.
    pub fn internal_error(details: impl Into<String>) -> Self {
        Self::new(ErrorCode::InternalError, "Internal error")
            .with_data(details.into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.message, self.code)
    }
}

impl std::error::Error for Error {}

/// A JSON-RPC 2.0 Request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    /// JSON-RPC version (always "2.0")
    pub jsonrpc: String,
    /// Request ID (number or string)
    pub id: RequestId,
    /// Method name
    pub method: String,
    /// Method parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

impl Request {
    /// Create a new request with the given ID, method, and params.
    pub fn new(id: impl Into<RequestId>, method: impl Into<String>, params: Option<Value>) -> Self {
        Self {
            jsonrpc: JSONRPC_VERSION.to_string(),
            id: id.into(),
            method: method.into(),
            params,
        }
    }
}

/// A JSON-RPC 2.0 Notification (request without ID).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    /// JSON-RPC version (always "2.0")
    pub jsonrpc: String,
    /// Method name
    pub method: String,
    /// Method parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

impl Notification {
    /// Create a new notification with the given method and params.
    pub fn new(method: impl Into<String>, params: Option<Value>) -> Self {
        Self {
            jsonrpc: JSONRPC_VERSION.to_string(),
            method: method.into(),
            params,
        }
    }
}

/// A JSON-RPC 2.0 Response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    /// JSON-RPC version (always "2.0")
    pub jsonrpc: String,
    /// Request ID this is responding to
    pub id: RequestId,
    /// Result (if successful)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    /// Error (if failed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}

impl Response {
    /// Create a successful response.
    pub fn success(id: RequestId, result: Value) -> Self {
        Self {
            jsonrpc: JSONRPC_VERSION.to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }

    /// Create an error response.
    pub fn error(id: RequestId, error: Error) -> Self {
        Self {
            jsonrpc: JSONRPC_VERSION.to_string(),
            id,
            result: None,
            error: Some(error),
        }
    }
}

/// Request ID can be a number, string, or null.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestId {
    Number(i64),
    String(String),
    Null,
}

impl From<i64> for RequestId {
    fn from(n: i64) -> Self {
        Self::Number(n)
    }
}

impl From<i32> for RequestId {
    fn from(n: i32) -> Self {
        Self::Number(n as i64)
    }
}

impl From<String> for RequestId {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<&str> for RequestId {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}

impl fmt::Display for RequestId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{}", n),
            Self::String(s) => write!(f, "{}", s),
            Self::Null => write!(f, "null"),
        }
    }
}

/// Incoming message that could be a request, response, or notification.
#[derive(Debug, Clone, Deserialize)]
pub struct IncomingMessage {
    /// JSON-RPC version
    pub jsonrpc: Option<String>,
    /// Request/Response ID
    pub id: Option<RequestId>,
    /// Method name (for requests/notifications)
    pub method: Option<String>,
    /// Parameters (for requests/notifications)
    pub params: Option<Value>,
    /// Result (for responses)
    pub result: Option<Value>,
    /// Error (for error responses)
    pub error: Option<Error>,
}

impl IncomingMessage {
    /// Determine the type of message and validate it.
    pub fn classify(&self) -> Result<MessageType, Error> {
        // Check JSON-RPC version
        if self.jsonrpc.as_deref() != Some(JSONRPC_VERSION) {
            return Err(Error::invalid_request().with_data("Invalid or missing jsonrpc version"));
        }

        match (&self.id, &self.method, &self.result, &self.error) {
            // Request: has ID and method
            (Some(id), Some(method), None, None) => {
                Ok(MessageType::Request(Request {
                    jsonrpc: JSONRPC_VERSION.to_string(),
                    id: id.clone(),
                    method: method.clone(),
                    params: self.params.clone(),
                }))
            }
            // Notification: has method but no ID
            (None, Some(method), None, None) => {
                Ok(MessageType::Notification(Notification {
                    jsonrpc: JSONRPC_VERSION.to_string(),
                    method: method.clone(),
                    params: self.params.clone(),
                }))
            }
            // Success Response: has ID and result
            (Some(id), None, Some(result), None) => {
                Ok(MessageType::Response(Response {
                    jsonrpc: JSONRPC_VERSION.to_string(),
                    id: id.clone(),
                    result: Some(result.clone()),
                    error: None,
                }))
            }
            // Error Response: has ID and error
            (Some(id), None, None, Some(error)) => {
                Ok(MessageType::Response(Response {
                    jsonrpc: JSONRPC_VERSION.to_string(),
                    id: id.clone(),
                    result: None,
                    error: Some(error.clone()),
                }))
            }
            // Invalid combination
            _ => Err(Error::invalid_request().with_data("Invalid message structure")),
        }
    }
}

/// Classified message type.
#[derive(Debug, Clone)]
pub enum MessageType {
    Request(Request),
    Notification(Notification),
    Response(Response),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_error_codes() {
        assert_eq!(ErrorCode::ParseError.as_i32(), -32700);
        assert_eq!(ErrorCode::InvalidRequest.as_i32(), -32600);
        assert_eq!(ErrorCode::MethodNotFound.as_i32(), -32601);
        assert_eq!(ErrorCode::InvalidParams.as_i32(), -32602);
        assert_eq!(ErrorCode::InternalError.as_i32(), -32603);
    }

    #[test]
    fn test_request_serialization() {
        let request = Request::new(1, "test_method", Some(json!({"key": "value"})));
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains(r#""jsonrpc":"2.0""#));
        assert!(json.contains(r#""id":1"#));
        assert!(json.contains(r#""method":"test_method""#));
    }

    #[test]
    fn test_notification_serialization() {
        let notification = Notification::new("test_notification", None);
        let json = serde_json::to_string(&notification).unwrap();
        assert!(json.contains(r#""jsonrpc":"2.0""#));
        assert!(!json.contains(r#""id""#));
        assert!(json.contains(r#""method":"test_notification""#));
    }

    #[test]
    fn test_response_success() {
        let response = Response::success(RequestId::Number(1), json!({"result": "ok"}));
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains(r#""result":"#));
        assert!(!json.contains(r#""error":"#));
    }

    #[test]
    fn test_response_error() {
        let error = Error::method_not_found("unknown");
        let response = Response::error(RequestId::Number(1), error);
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains(r#""error":"#));
        assert!(!json.contains(r#""result":"#));
    }

    #[test]
    fn test_message_classification() {
        // Test request
        let msg = IncomingMessage {
            jsonrpc: Some(JSONRPC_VERSION.to_string()),
            id: Some(RequestId::Number(1)),
            method: Some("test".to_string()),
            params: None,
            result: None,
            error: None,
        };
        match msg.classify().unwrap() {
            MessageType::Request(req) => assert_eq!(req.method, "test"),
            _ => panic!("Expected request"),
        }

        // Test notification
        let msg = IncomingMessage {
            jsonrpc: Some(JSONRPC_VERSION.to_string()),
            id: None,
            method: Some("notify".to_string()),
            params: None,
            result: None,
            error: None,
        };
        match msg.classify().unwrap() {
            MessageType::Notification(notif) => assert_eq!(notif.method, "notify"),
            _ => panic!("Expected notification"),
        }
    }
}