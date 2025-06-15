//! Usage examples for Deribit WebSocket public/unsubscribe endpoint
//!
//! This module demonstrates how to use the public/unsubscribe endpoint
//! in various scenarios.

#[cfg(test)]
pub mod usage_examples {
    use crate::deribit::websocket::{
        DeribitWebSocketClient, UnsubscribeRequest, UnsubscribeResponse, JsonRpcRequest,
    };
    use serde_json::json;
    use websockets::WebSocketConnection;

    /// Example: Basic unsubscribe from multiple channels
    #[tokio::test]
    async fn example_basic_unsubscribe() {
        // Create a WebSocket client
        let client = DeribitWebSocketClient::new_default();
        
        // Create an unsubscribe request for multiple channels
        let request = UnsubscribeRequest::new(vec![
            "deribit_price_index.btc_usd".to_string(),
            "trades.BTC-PERPETUAL.raw".to_string(),
            "book.BTC-PERPETUAL.100ms".to_string(),
        ]);
        
        // In a real application, you would:
        // 1. Connect to the WebSocket
        // client.connect().await.unwrap();
        
        // 2. Send the unsubscribe request
        // let request_id = client.public_unsubscribe(&request).await.unwrap();
        
        // 3. Wait for and handle the response
        // Note: Since we can't make actual network connections in tests,
        // we'll demonstrate the request/response structure instead
        
        // Verify the request structure
        assert_eq!(request.channels.len(), 3);
        assert!(request.channels.contains(&"deribit_price_index.btc_usd".to_string()));
        
        println!("✓ Created unsubscribe request for {} channels", request.channels.len());
    }

    /// Example: Unsubscribe from a single channel using convenience method
    #[tokio::test]
    async fn example_single_channel_unsubscribe() {
        let client = DeribitWebSocketClient::new_default();
        
        // Create request for a single channel using convenience method
        let request = UnsubscribeRequest::single_channel("ticker.BTC-PERPETUAL");
        
        // In a real application:
        // client.connect().await.unwrap();
        // let request_id = client.public_unsubscribe_single("ticker.BTC-PERPETUAL").await.unwrap();
        
        assert_eq!(request.channels.len(), 1);
        assert_eq!(request.channels[0], "ticker.BTC-PERPETUAL");
        
        println!("✓ Created single-channel unsubscribe request");
    }

    /// Example: Building an unsubscribe request incrementally
    #[test]
    fn example_incremental_request_building() {
        // Start with one channel
        let mut request = UnsubscribeRequest::single_channel("announcements");
        
        // Add more channels as needed
        request.add_channel("deribit_price_index.btc_usd");
        request.add_channel("deribit_price_index.eth_usd");
        
        // Convert to JSON-RPC parameters
        let params = request.to_params();
        let expected = json!({
            "channels": [
                "announcements",
                "deribit_price_index.btc_usd", 
                "deribit_price_index.eth_usd"
            ]
        });
        
        assert_eq!(params, expected);
        println!("✓ Built unsubscribe request incrementally");
    }

    /// Example: Handling different types of Deribit channels
    #[test]
    fn example_deribit_channel_types() {
        // Deribit supports various channel types
        let channels = vec![
            // Price index channels
            "deribit_price_index.btc_usd".to_string(),
            "deribit_price_index.eth_usd".to_string(),
            
            // Trading channels
            "trades.BTC-PERPETUAL.raw".to_string(),
            "trades.ETH-PERPETUAL.raw".to_string(),
            
            // Order book channels
            "book.BTC-PERPETUAL.raw".to_string(),
            "book.BTC-PERPETUAL.100ms".to_string(),
            
            // Ticker channels
            "ticker.BTC-PERPETUAL".to_string(),
            
            // General announcements
            "announcements".to_string(),
        ];
        
        let request = UnsubscribeRequest::new(channels.clone());
        
        // Verify all channel types are supported
        assert_eq!(request.channels, channels);
        
        // Verify JSON serialization preserves channel names exactly
        let params = request.to_params();
        let serialized_channels = params["channels"].as_array().unwrap();
        
        for (i, channel) in channels.iter().enumerate() {
            assert_eq!(serialized_channels[i].as_str().unwrap(), channel);
        }
        
        println!("✓ Demonstrated various Deribit channel types");
    }

    /// Example: Creating a complete JSON-RPC request
    #[test]
    fn example_complete_json_rpc_request() {
        let request = UnsubscribeRequest::new(vec![
            "trades.BTC-PERPETUAL.raw".to_string(),
        ]);
        
        // Create a complete JSON-RPC 2.0 request
        let json_rpc_request = JsonRpcRequest::new(
            42, // Request ID
            "public/unsubscribe",
            Some(request.to_params()),
        );
        
        // Serialize to JSON string (what would be sent over WebSocket)
        let json_string = serde_json::to_string(&json_rpc_request).unwrap();
        
        // Verify the JSON structure
        let parsed: serde_json::Value = serde_json::from_str(&json_string).unwrap();
        assert_eq!(parsed["jsonrpc"], "2.0");
        assert_eq!(parsed["id"], 42);
        assert_eq!(parsed["method"], "public/unsubscribe");
        assert!(parsed["params"]["channels"].is_array());
        
        println!("✓ Created complete JSON-RPC request");
        println!("Request JSON: {}", json_string);
    }

    /// Example: Parsing unsubscribe response
    #[test]
    fn example_response_parsing() {
        // Simulate a response from Deribit (remaining subscriptions)
        let response_json = json!([
            "ticker.BTC-PERPETUAL",
            "book.ETH-PERPETUAL.raw"
        ]);
        
        // Parse the response
        let response = UnsubscribeResponse::try_from(response_json).unwrap();
        
        // Check remaining subscriptions
        println!("Remaining subscriptions: {}", response.remaining_subscription_count());
        for channel in response.channels() {
            println!("  - {}", channel);
        }
        
        assert_eq!(response.remaining_subscription_count(), 2);
        assert!(response.has_remaining_subscriptions());
        
        println!("✓ Parsed unsubscribe response successfully");
    }

    /// Example: Error handling scenarios
    #[test]
    fn example_error_scenarios() {
        use crate::deribit::websocket::UnsubscribeError;
        
        // Example errors that might occur
        let errors = vec![
            UnsubscribeError::InvalidChannel {
                channel: "invalid.channel.format".to_string(),
            },
            UnsubscribeError::NotSubscribed {
                channel: "trades.BTC-PERPETUAL.raw".to_string(),
            },
            UnsubscribeError::JsonRpcError {
                code: -32602,
                message: "Invalid params".to_string(),
            },
            UnsubscribeError::WebSocketError("Connection lost".to_string()),
        ];
        
        // Demonstrate error message formatting
        for error in errors {
            println!("Error: {}", error);
        }
        
        println!("✓ Demonstrated error handling");
    }

    /// Example: Real-world usage pattern
    #[test]
    fn example_real_world_pattern() {
        // This demonstrates how you might use the unsubscribe endpoint
        // in a real trading application
        
        // 1. Create client
        let client = DeribitWebSocketClient::new_default();
        assert!(!client.is_connected());
        
        // 2. Prepare channels to unsubscribe from
        let mut unsubscribe_request = UnsubscribeRequest::new(vec![]);
        
        // Add channels you no longer need
        unsubscribe_request.add_channel("trades.BTC-PERPETUAL.raw");
        unsubscribe_request.add_channel("book.BTC-PERPETUAL.100ms");
        
        // 3. Create the JSON-RPC request
        let request_id = 123;
        let json_rpc_request = JsonRpcRequest::new(
            request_id,
            "public/unsubscribe", 
            Some(unsubscribe_request.to_params()),
        );
        
        // 4. In a real application, you would:
        // - Connect to WebSocket: client.connect().await?
        // - Send the request: client.send_request(...).await?
        // - Wait for response with matching ID
        // - Parse response and update your subscription state
        
        // Verify request structure
        assert_eq!(json_rpc_request.id, request_id);
        assert_eq!(json_rpc_request.method, "public/unsubscribe");
        
        println!("✓ Demonstrated real-world usage pattern");
    }
}