//! BitMart private WebSocket client
//!
//! This module provides a WebSocket client for BitMart private account data streams.

use secrecy::{ExposeSecret, SecretString};

use crate::bitmart::public::websocket::{Operation, WsError, WsMessage};

/// BitMart WebSocket URL for private channels
pub const BITMART_WS_PRIVATE_URL: &str = "wss://ws-manager-compress.bitmart.com/user?protocol=1.1";

/// WebSocket client for BitMart private channels
pub struct WsClient {
    url: String,
    api_key: SecretString,
    api_secret: SecretString,
    memo: SecretString,
}

impl WsClient {
    /// Creates a new WebSocket client for private channels
    ///
    /// # Arguments
    /// * `api_key` - BitMart API key
    /// * `api_secret` - BitMart API secret
    /// * `memo` - BitMart memo/passphrase
    pub fn new(
        api_key: impl Into<SecretString>,
        api_secret: impl Into<SecretString>,
        memo: impl Into<SecretString>,
    ) -> Self {
        Self {
            url: BITMART_WS_PRIVATE_URL.to_string(),
            api_key: api_key.into(),
            api_secret: api_secret.into(),
            memo: memo.into(),
        }
    }

    /// Creates a new WebSocket client with a custom URL
    ///
    /// # Arguments
    /// * `url` - Custom WebSocket URL
    /// * `api_key` - BitMart API key
    /// * `api_secret` - BitMart API secret
    /// * `memo` - BitMart memo/passphrase
    pub fn with_url(
        url: String,
        api_key: impl Into<SecretString>,
        api_secret: impl Into<SecretString>,
        memo: impl Into<SecretString>,
    ) -> Self {
        Self {
            url,
            api_key: api_key.into(),
            api_secret: api_secret.into(),
            memo: memo.into(),
        }
    }

    /// Gets the WebSocket URL
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Create a login message for authentication
    ///
    /// # Arguments
    /// * `timestamp` - Current Unix timestamp in milliseconds
    ///
    /// # Returns
    /// WebSocket login message
    pub fn login_message(&self, timestamp: u64) -> Result<WsMessage, WsError> {
        // Create signature for authentication
        let signature = self.create_signature(timestamp)?;
        
        Ok(WsMessage {
            op: Operation::Login,
            args: vec![
                self.api_key.expose_secret().to_string(),
                timestamp.to_string(),
                signature,
            ],
        })
    }

    /// Create signature for authentication
    fn create_signature(&self, timestamp: u64) -> Result<String, WsError> {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;
        
        type HmacSha256 = Hmac<Sha256>;
        
        // Create the message to sign: timestamp + "#" + memo + "#" + "bitmart.WebSocket"
        let message = format!("{}#{}#bitmart.WebSocket", timestamp, self.memo.expose_secret());
        
        // Create HMAC
        let mut mac = HmacSha256::new_from_slice(self.api_secret.expose_secret().as_bytes())
            .map_err(|e| WsError::Connection(format!("Invalid secret key: {}", e)))?;
        
        mac.update(message.as_bytes());
        let result = mac.finalize();
        
        // Convert to hex string
        Ok(hex::encode(result.into_bytes()))
    }
}

/// WebSocket channel types for private data
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrivateChannel {
    /// Order updates
    Order,
    /// Asset updates
    Asset,
    /// Position updates (for futures)
    Position,
}

impl PrivateChannel {
    /// Convert channel to string format for topic construction
    pub fn as_str(&self) -> &'static str {
        match self {
            PrivateChannel::Order => "spot/user/order",
            PrivateChannel::Asset => "spot/user/asset",
            PrivateChannel::Position => "futures/user/position",
        }
    }

    /// Create a topic string for subscription
    pub fn topic(&self) -> String {
        self.as_str().to_string()
    }
}

impl WsClient {
    /// Subscribe to order updates
    ///
    /// # Returns
    /// WebSocket message to send for subscription
    pub fn subscribe_orders(&self) -> WsMessage {
        let topic = PrivateChannel::Order.topic();
        WsMessage::subscribe(vec![topic])
    }

    /// Subscribe to asset updates
    ///
    /// # Returns
    /// WebSocket message to send for subscription
    pub fn subscribe_assets(&self) -> WsMessage {
        let topic = PrivateChannel::Asset.topic();
        WsMessage::subscribe(vec![topic])
    }

    /// Subscribe to multiple private channels
    ///
    /// # Arguments
    /// * `channels` - Vector of private channels to subscribe to
    ///
    /// # Returns
    /// WebSocket message to send for subscription
    pub fn subscribe_channels(&self, channels: Vec<PrivateChannel>) -> WsMessage {
        let topics: Vec<String> = channels.iter().map(|ch| ch.topic()).collect();
        WsMessage::subscribe(topics)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_private_ws_client_creation() {
        let client = WsClient::new("api_key", "api_secret", "memo");
        assert_eq!(client.url(), BITMART_WS_PRIVATE_URL);
    }

    #[test]
    fn test_private_channel_topics() {
        assert_eq!(PrivateChannel::Order.topic(), "spot/user/order");
        assert_eq!(PrivateChannel::Asset.topic(), "spot/user/asset");
        assert_eq!(PrivateChannel::Position.topic(), "futures/user/position");
    }

    #[test]
    fn test_signature_creation() {
        let client = WsClient::new("test_key", "test_secret", "test_memo");
        let timestamp = 1640995200000;
        
        // This test validates that the signature creation doesn't panic
        // Actual signature verification would require known test vectors
        let result = client.create_signature(timestamp);
        assert!(result.is_ok());
        
        let signature = result.unwrap();
        assert!(!signature.is_empty());
        assert_eq!(signature.len(), 64); // SHA256 hex should be 64 chars
    }

    #[test]
    fn test_login_message_creation() {
        let client = WsClient::new("test_key", "test_secret", "test_memo");
        let timestamp = 1640995200000;
        
        let login_msg = client.login_message(timestamp);
        assert!(login_msg.is_ok());
        
        let msg = login_msg.unwrap();
        assert_eq!(msg.op, Operation::Login);
        assert_eq!(msg.args.len(), 3);
        assert_eq!(msg.args[0], "test_key");
        assert_eq!(msg.args[1], "1640995200000");
        // args[2] is the signature - just check it's not empty
        assert!(!msg.args[2].is_empty());
    }

    #[test]
    fn test_subscription_messages() {
        let client = WsClient::new("api_key", "api_secret", "memo");
        
        let order_msg = client.subscribe_orders();
        assert_eq!(order_msg.args, vec!["spot/user/order"]);
        
        let asset_msg = client.subscribe_assets();
        assert_eq!(asset_msg.args, vec!["spot/user/asset"]);
        
        let multi_msg = client.subscribe_channels(vec![
            PrivateChannel::Order,
            PrivateChannel::Asset,
        ]);
        assert_eq!(multi_msg.args.len(), 2);
        assert!(multi_msg.args.contains(&"spot/user/order".to_string()));
        assert!(multi_msg.args.contains(&"spot/user/asset".to_string()));
    }
}
