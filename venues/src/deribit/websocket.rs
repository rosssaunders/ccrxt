//! Deribit WebSocket types and structures
//!
//! This module provides WebSocket communication types for the Deribit API,
//! which uses JSON-RPC 2.0 protocol for all WebSocket communication.

use serde::{Deserialize, Serialize};
use websockets::VenueMessage;

/// JSON-RPC 2.0 request structure for Deribit WebSocket API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest<T> {
    /// JSON-RPC version, always "2.0"
    pub jsonrpc: String,
    /// Method name to call
    pub method: String,
    /// Request parameters
    pub params: T,
    /// Request ID for matching responses
    pub id: u64,
}

impl<T> JsonRpcRequest<T> {
    /// Create a new JSON-RPC request
    pub fn new(method: String, params: T, id: u64) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            method,
            params,
            id,
        }
    }
}

/// JSON-RPC 2.0 response structure for Deribit WebSocket API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse<T> {
    /// JSON-RPC version, always "2.0"
    pub jsonrpc: String,
    /// Response result (present on success)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<T>,
    /// Error details (present on failure)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
    /// Request ID for matching requests
    pub id: u64,
}

/// JSON-RPC error structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    /// Error code
    pub code: i32,
    /// Error message
    pub message: String,
    /// Additional error data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// Generic Deribit WebSocket message that can be either a request or response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeribitMessage {
    /// JSON-RPC request
    Request(JsonRpcRequest<serde_json::Value>),
    /// JSON-RPC response
    Response(JsonRpcResponse<serde_json::Value>),
}

impl VenueMessage for DeribitMessage {}

/// Parameters for the set_heartbeat method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetHeartbeatParams {
    /// The heartbeat interval in seconds, but not less than 10
    pub interval: u32,
}

/// Response for the set_heartbeat method (just "ok" on success)
pub type SetHeartbeatResult = String;

/// Typed request for set_heartbeat
pub type SetHeartbeatRequest = JsonRpcRequest<SetHeartbeatParams>;

/// Typed response for set_heartbeat
pub type SetHeartbeatResponse = JsonRpcResponse<SetHeartbeatResult>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_rpc_request_serialization() {
        let params = SetHeartbeatParams { interval: 30 };
        let request = JsonRpcRequest::new("public/set_heartbeat".to_string(), params, 1);
        
        let json = serde_json::to_string(&request).expect("Should serialize");
        let expected = r#"{"jsonrpc":"2.0","method":"public/set_heartbeat","params":{"interval":30},"id":1}"#;
        
        assert_eq!(json, expected);
    }

    #[test]
    fn test_json_rpc_response_success_serialization() {
        let response = JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: Some("ok".to_string()),
            error: None,
            id: 1,
        };
        
        let json = serde_json::to_string(&response).expect("Should serialize");
        let expected = r#"{"jsonrpc":"2.0","result":"ok","id":1}"#;
        
        assert_eq!(json, expected);
    }

    #[test]
    fn test_json_rpc_response_error_serialization() {
        let error = JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        };
        
        let response: JsonRpcResponse<String> = JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(error),
            id: 1,
        };
        
        let json = serde_json::to_string(&response).expect("Should serialize");
        let expected = r#"{"jsonrpc":"2.0","error":{"code":-32602,"message":"Invalid params"},"id":1}"#;
        
        assert_eq!(json, expected);
    }

    #[test]
    fn test_set_heartbeat_params_validation() {
        let params = SetHeartbeatParams { interval: 30 };
        assert_eq!(params.interval, 30);
        
        // Test serialization
        let json = serde_json::to_string(&params).expect("Should serialize");
        let expected = r#"{"interval":30}"#;
        assert_eq!(json, expected);
        
        // Test deserialization
        let deserialized: SetHeartbeatParams = serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(deserialized.interval, 30);
    }

    #[test]
    fn test_deribit_message_request_deserialization() {
        let json = r#"{"jsonrpc":"2.0","method":"public/set_heartbeat","params":{"interval":30},"id":1}"#;
        let message: DeribitMessage = serde_json::from_str(json).expect("Should deserialize");
        
        match message {
            DeribitMessage::Request(req) => {
                assert_eq!(req.jsonrpc, "2.0");
                assert_eq!(req.method, "public/set_heartbeat");
                assert_eq!(req.id, 1);
            }
            _ => panic!("Expected request message"),
        }
    }

    #[test]
    fn test_deribit_message_response_deserialization() {
        let json = r#"{"jsonrpc":"2.0","result":"ok","id":1}"#;
        let message: DeribitMessage = serde_json::from_str(json).expect("Should deserialize");
        
        match message {
            DeribitMessage::Response(resp) => {
                assert_eq!(resp.jsonrpc, "2.0");
                assert_eq!(resp.id, 1);
                assert_eq!(resp.result, Some(serde_json::Value::String("ok".to_string())));
                assert!(resp.error.is_none());
            }
            _ => panic!("Expected response message"),
        }
    }
}