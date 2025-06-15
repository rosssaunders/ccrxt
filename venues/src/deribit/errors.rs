//! Error types for Deribit API

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Deribit API errors
#[derive(Error, Debug)]
pub enum Errors {
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    
    #[error("Rate limit error: {0}")]
    RateLimit(#[from] crate::deribit::RateLimitError),
    
    #[error("API error: {0}")]
    Error(String),
    
    #[error("JSON-RPC error: code={code}, message={message}")]
    JsonRpcError { code: i32, message: String },
}

/// JSON-RPC error response
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// Type alias for results returned by Deribit API operations
pub type RestResult<T> = Result<T, Errors>;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_jsonrpc_error_serialization() {
        let error = JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        };
        
        let json = serde_json::to_string(&error).unwrap();
        let parsed: JsonRpcError = serde_json::from_str(&json).unwrap();
        
        assert_eq!(parsed.code, -32602);
        assert_eq!(parsed.message, "Invalid params");
        assert!(parsed.data.is_none());
    }
}