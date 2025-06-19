//! Unsubscribe from specific channels on Deribit WebSocket API.
//!
//! This file defines the request and response payloads for the `public/unsubscribe` RPC call.
use crate::deribit::public::websocket::client::{DeribitWebSocketClient, DeribitWebSocketError};

use futures::SinkExt;
use serde::{Deserialize, Serialize};

/// Parameters for the unsubscribe request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsubscribeParams {
    /// A list of channels to unsubscribe from.
    pub channels: Vec<String>,
}

/// Request for the `public/unsubscribe` method.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsubscribeRequest {
    /// Always "2.0".
    pub jsonrpc: String,
    /// Request ID.
    pub id: u64,
    /// Always "public/unsubscribe".
    pub method: String,
    /// Parameters containing channels to unsubscribe from.
    pub params: UnsubscribeParams,
}

impl UnsubscribeRequest {
    /// Create a new unsubscribe request.
    pub fn new(id: u64, channels: Vec<String>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            method: "public/unsubscribe".to_string(),
            params: UnsubscribeParams { channels },
        }
    }
}

/// Response for the `public/unsubscribe` method.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsubscribeResponse {
    /// Always "2.0".
    pub jsonrpc: String,
    /// Request ID from the original request.
    pub id: u64,
    /// Result - array of unsubscribed channels.
    pub result: Vec<String>,
}

impl DeribitWebSocketClient {
    /// Send an unsubscribe request with specific channels and wait for the response
    pub async fn unsubscribe(&mut self, channels: Vec<String>) -> Result<UnsubscribeResponse, DeribitWebSocketError> {
        if !self.is_connected() {
            return Err(DeribitWebSocketError::Connection(
                "Not connected".to_string(),
            ));
        }
        let req_id = self.next_request_id();
        let req = UnsubscribeRequest::new(req_id, channels);
        let msg = serde_json::to_string(&req)?;
        if let Some(ws) = &mut self.websocket {
            ws.send(tokio_tungstenite::tungstenite::Message::Text(msg.into()))
                .await
                .map_err(|e| DeribitWebSocketError::Connection(e.to_string()))?;
            // Wait for the response
            let response_str = self.receive_response().await?;
            let response: UnsubscribeResponse = serde_json::from_str(&response_str)?;
            Ok(response)
        } else {
            Err(DeribitWebSocketError::Connection(
                "WebSocket not connected".to_string(),
            ))
        }
    }
}
