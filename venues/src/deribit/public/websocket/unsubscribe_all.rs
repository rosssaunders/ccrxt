//! Unsubscribe from all channels on Deribit WebSocket API.
//!
//! This file defines the request and response payloads for the `public/unsubscribe_all` RPC call.
use serde::{Deserialize, Serialize};

use crate::deribit::public::websocket::client::{DeribitWebSocketError, PrivateWebSocketClient};

/// Request for the `public/unsubscribe_all` method (no parameters).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnsubscribeAllRequest;

/// Response for the `public/unsubscribe_all` method.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsubscribeAllResponse {
    /// Result - "ok" on success.
    pub result: String,
}

impl PrivateWebSocketClient {
    /// Unsubscribes from all channels for this client instance.
    pub async fn unsubscribe_all(&mut self) -> Result<UnsubscribeAllResponse, DeribitWebSocketError> {
        let req = UnsubscribeAllRequest;
        self.send_and_receive::<UnsubscribeAllRequest, UnsubscribeAllResponse>(&req)
            .await
    }
}
