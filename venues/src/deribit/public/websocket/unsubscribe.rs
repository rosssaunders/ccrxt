//! Unsubscribe from specific channels on Deribit WebSocket API.
//!
//! This file defines the request and response payloads for the `public/unsubscribe` RPC call.
use crate::deribit::public::websocket::client::{DeribitWebSocketError, PrivateWebSocketClient};

use serde::{Deserialize, Serialize};

/// Parameters for the unsubscribe request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsubscribeRequest {
    /// A list of channels to unsubscribe from.
    pub channels: Vec<String>,
}

/// Response for the `public/unsubscribe` method.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsubscribeResponse {
    /// Result - array of unsubscribed channels.
    pub result: Vec<String>,
}

impl PrivateWebSocketClient {
    /// Unsubscribe from specific channels
    pub async fn unsubscribe(&mut self, request: UnsubscribeRequest) -> Result<UnsubscribeResponse, DeribitWebSocketError> {
        self.send_serializable(&request).await?;
        // Wait for the response
        let response_str = self.receive_response().await?;
        let response: UnsubscribeResponse = serde_json::from_str(&response_str)?;
        Ok(response)
    }
}
