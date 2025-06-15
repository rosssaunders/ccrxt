//! Integration tests for Deribit private WebSocket client
//! 
//! These tests verify that the client compiles correctly and that the endpoint methods
//! are accessible with proper type signatures.

use crate::deribit::private::PrivateWebSocketClient;
use rest::secrets::ExposableSecret;
use websockets::{WebSocketConnection, VenueMessage};

/// Test secret implementation for integration tests
#[derive(Debug)]
struct TestSecret {
    secret: String,
}

impl TestSecret {
    fn new(secret: String) -> Self {
        Self { secret }
    }
}

impl ExposableSecret for TestSecret {
    fn expose_secret(&self) -> String {
        self.secret.clone()
    }
}

#[tokio::test]
async fn test_deribit_private_endpoints_compile() {
    // Test that the Deribit private WebSocket client compiles and is accessible
    let api_key = Box::new(TestSecret::new("test_key".to_string())) as Box<dyn ExposableSecret>;
    let api_secret = Box::new(TestSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;

    let client = PrivateWebSocketClient::new(api_key, api_secret, None);

    // Test that methods exist by verifying we can get function references to them
    // This proves they compile and are accessible without needing to call them
    let _ = PrivateWebSocketClient::new;
    let _ = PrivateWebSocketClient::unsubscribe;

    // Verify that the client implements the WebSocketConnection trait
    let _: Box<dyn WebSocketConnection<_>> = Box::new(client);

    println!("All Deribit private WebSocket methods are accessible and properly typed");
}

#[test]
fn test_deribit_message_trait_implementation() {
    use crate::deribit::private::DeribitMessage;
    
    let message = DeribitMessage {
        jsonrpc: "2.0".to_string(),
        id: Some(1),
        method: Some("private/unsubscribe".to_string()),
        params: None,
        result: None,
        error: None,
    };

    // Test that DeribitMessage implements VenueMessage trait
    fn test_venue_message<T: VenueMessage>(_: T) {}
    test_venue_message(message);
}

#[test]
fn test_unsubscribe_request_structure() {
    use crate::deribit::private::{UnsubscribeParams, UnsubscribeResult};
    use serde_json;

    let params = UnsubscribeParams {
        channels: vec![
            "user.orders.BTC-PERPETUAL.raw".to_string(),
            "user.trades.any.BTC.raw".to_string(),
        ],
    };

    // Test serialization
    let serialized = serde_json::to_value(&params).expect("Should serialize UnsubscribeParams");
    assert!(serialized.is_object());
    assert!(serialized.get("channels").is_some());

    // Test that UnsubscribeResult is the correct type
    let result: UnsubscribeResult = vec!["remaining_channel".to_string()];
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], "remaining_channel");
}

#[test]
fn test_client_creation_variants() {
    let api_key = Box::new(TestSecret::new("test_key".to_string())) as Box<dyn ExposableSecret>;
    let api_secret = Box::new(TestSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;

    // Test with default URL
    let client1 = PrivateWebSocketClient::new(
        api_key, 
        api_secret, 
        None
    );
    assert!(!client1.is_connected());

    // Test with custom URL
    let api_key2 = Box::new(TestSecret::new("test_key2".to_string())) as Box<dyn ExposableSecret>;
    let api_secret2 = Box::new(TestSecret::new("test_secret2".to_string())) as Box<dyn ExposableSecret>;
    let custom_url = "wss://testnet.deribit.com/ws/api/v2".to_string();
    let client2 = PrivateWebSocketClient::new(
        api_key2, 
        api_secret2, 
        Some(custom_url)
    );
    assert!(!client2.is_connected());
}

#[test]
fn test_json_rpc_error_handling() {
    use crate::deribit::private::DeribitError;
    use serde_json;

    let error = DeribitError {
        code: 10001,
        message: "invalid_request".to_string(),
        data: Some(serde_json::json!({
            "reason": "Channel not found",
            "channel": "invalid.channel.name"
        })),
    };

    // Test serialization/deserialization
    let serialized = serde_json::to_string(&error).expect("Should serialize DeribitError");
    let deserialized: DeribitError = serde_json::from_str(&serialized).expect("Should deserialize DeribitError");

    assert_eq!(deserialized.code, 10001);
    assert_eq!(deserialized.message, "invalid_request");
    assert!(deserialized.data.is_some());

    if let Some(data) = deserialized.data {
        assert_eq!(data.get("reason").unwrap(), "Channel not found");
        assert_eq!(data.get("channel").unwrap(), "invalid.channel.name");
    }
}