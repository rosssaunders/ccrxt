//! Unsubscribe from all channels on Deribit WebSocket API.
//!
//! This file defines the request and response payloads for the `public/unsubscribe_all` RPC call.
use crate::deribit::public::websocket::client::{DeribitWebSocketClient, DeribitWebSocketError};
use futures::SinkExt;

use serde::{Deserialize, Serialize};

/// Request for the `public/unsubscribe_all` method (no parameters).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsubscribeAllRequest {
    /// Always "2.0".
    pub jsonrpc: String,
    /// Request ID.
    pub id: u64,
    /// Always "public/unsubscribe_all".
    pub method: String,
}

impl UnsubscribeAllRequest {
    /// Create a new unsubscribe_all request.
    pub fn new(id: u64) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            method: "public/unsubscribe_all".to_string(),
        }
    }
}

/// Response for the `public/unsubscribe_all` method.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsubscribeAllResponse {
    /// Always "2.0".
    pub jsonrpc: String,
    /// Request ID from the original request.
    pub id: u64,
    /// Result - "ok" on success.
    pub result: String,
}

impl DeribitWebSocketClient {
    /// Unsubscribes from all channels for this client instance.
    pub async fn unsubscribe_all(&mut self) -> Result<UnsubscribeAllResponse, DeribitWebSocketError> {
        if !self.is_connected() {
            return Err(DeribitWebSocketError::Connection(
                "Not connected".to_string(),
            ));
        }
        let req_id = self.next_request_id();
        let req = UnsubscribeAllRequest::new(req_id);
        let msg = serde_json::to_string(&req)?;
        if let Some(ws) = &mut self.websocket {
            ws.send(tokio_tungstenite::tungstenite::Message::Text(msg.into()))
                .await
                .map_err(|e| DeribitWebSocketError::Connection(e.to_string()))?;
            // Wait for the response
            let response_str = self.receive_response().await?;
            let response: UnsubscribeAllResponse = serde_json::from_str(&response_str)?;
            Ok(response)
        } else {
            Err(DeribitWebSocketError::Connection(
                "WebSocket not connected".to_string(),
            ))
        }
    }
}
