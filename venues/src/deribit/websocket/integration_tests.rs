//! Integration tests for Deribit WebSocket API
//!
//! These tests verify the complete functionality of the public/unsubscribe endpoint

#[cfg(test)]
mod tests {
    use crate::deribit::websocket::{
        DeribitWebSocketClient, UnsubscribeRequest, UnsubscribeResponse,
        JsonRpcRequest, JsonRpcResponse, DeribitMessage,
    };
    use serde_json::json;
    use websockets::WebSocketConnection;

    #[test]
    fn test_complete_unsubscribe_workflow() {
        // Test the complete JSON-RPC workflow for public/unsubscribe
        
        // Create a request
        let channels = vec![
            "deribit_price_index.btc_usd".to_string(),
            "trades.BTC-PERPETUAL.raw".to_string(),
        ];
        let unsubscribe_request = UnsubscribeRequest::new(channels.clone());
        
        // Convert to JSON-RPC request
        let id = 42;
        let json_rpc_request = JsonRpcRequest::new(
            id,
            "public/unsubscribe",
            Some(unsubscribe_request.to_params()),
        );
        
        // Verify request structure
        assert_eq!(json_rpc_request.jsonrpc, "2.0");
        assert_eq!(json_rpc_request.id, id);
        assert_eq!(json_rpc_request.method, "public/unsubscribe");
        
        let params = json_rpc_request.params.unwrap();
        let expected_params = json!({
            "channels": [
                "deribit_price_index.btc_usd",
                "trades.BTC-PERPETUAL.raw"
            ]
        });
        assert_eq!(params, expected_params);
        
        // Simulate a response (remaining subscriptions after unsubscribing)
        let remaining_channels = vec!["orderbook.BTC-PERPETUAL.raw".to_string()];
        let response_value = json!(remaining_channels);
        
        let json_rpc_response = JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: response_value.clone(),
        };
        
        // Parse the response
        let unsubscribe_response = UnsubscribeResponse::try_from(response_value)
            .expect("Failed to parse unsubscribe response");
        
        assert_eq!(unsubscribe_response.channels(), &remaining_channels);
        assert!(unsubscribe_response.has_remaining_subscriptions());
        assert_eq!(unsubscribe_response.remaining_subscription_count(), 1);
    }

    #[test]
    fn test_unsubscribe_from_all_channels() {
        // Test unsubscribing from all channels (empty response)
        
        let unsubscribe_request = UnsubscribeRequest::new(vec![
            "all.channels".to_string(),
        ]);
        
        // Simulate response with no remaining subscriptions
        let response_value = json!([]);
        let unsubscribe_response = UnsubscribeResponse::try_from(response_value)
            .expect("Failed to parse empty response");
        
        assert!(!unsubscribe_response.has_remaining_subscriptions());
        assert_eq!(unsubscribe_response.remaining_subscription_count(), 0);
    }

    #[test]
    fn test_json_rpc_message_serialization_roundtrip() {
        // Test that we can serialize and deserialize complete JSON-RPC messages
        
        let request = JsonRpcRequest::new(
            1,
            "public/unsubscribe",
            Some(json!({"channels": ["test.channel"]})),
        );
        
        let message = DeribitMessage::Request(request);
        
        // Serialize to JSON
        let json_str = serde_json::to_string(&message)
            .expect("Failed to serialize message");
        
        // Deserialize back
        let deserialized: DeribitMessage = serde_json::from_str(&json_str)
            .expect("Failed to deserialize message");
        
        // Verify the message matches
        match (message, deserialized) {
            (DeribitMessage::Request(orig), DeribitMessage::Request(deser)) => {
                assert_eq!(orig.jsonrpc, deser.jsonrpc);
                assert_eq!(orig.id, deser.id);
                assert_eq!(orig.method, deser.method);
                assert_eq!(orig.params, deser.params);
            }
            _ => panic!("Message types don't match"),
        }
    }

    #[test]
    fn test_deribit_channel_name_validation() {
        // Test with various Deribit channel name patterns
        
        let valid_channels = vec![
            "deribit_price_index.btc_usd".to_string(),
            "trades.BTC-PERPETUAL.raw".to_string(),
            "book.BTC-PERPETUAL.raw".to_string(),
            "ticker.BTC-PERPETUAL".to_string(),
            "announcements".to_string(),
        ];
        
        let request = UnsubscribeRequest::new(valid_channels.clone());
        
        // Verify all channels are preserved
        assert_eq!(request.channels, valid_channels);
        
        // Test JSON serialization preserves channel names exactly
        let params = request.to_params();
        let channels_from_params = params.get("channels")
            .expect("Channels field missing")
            .as_array()
            .expect("Channels should be array");
        
        for (i, channel) in valid_channels.iter().enumerate() {
            let param_channel = channels_from_params[i]
                .as_str()
                .expect("Channel should be string");
            assert_eq!(param_channel, channel);
        }
    }

    #[test]
    fn test_client_usage_pattern() {
        // Test the expected client usage pattern
        
        let client = DeribitWebSocketClient::new_default();
        
        // Verify client is created with correct settings
        assert_eq!(client.next_request_id(), 1);
        assert_eq!(client.next_request_id(), 2);
        assert!(!client.is_connected());
        
        // Test single channel unsubscribe request creation
        let request = UnsubscribeRequest::single_channel("test.channel");
        assert_eq!(request.channels.len(), 1);
        assert_eq!(request.channels[0], "test.channel");
        
        // Test multiple channel request creation
        let mut multi_request = UnsubscribeRequest::new(vec!["channel1".to_string()]);
        multi_request.add_channel("channel2");
        multi_request.add_channel("channel3");
        
        assert_eq!(multi_request.channels.len(), 3);
        assert_eq!(multi_request.channels, vec!["channel1", "channel2", "channel3"]);
    }

    #[test]
    fn test_error_response_handling() {
        // Test JSON-RPC error response structure
        
        use crate::deribit::websocket::{JsonRpcErrorResponse, JsonRpcError};
        
        let error_response = JsonRpcErrorResponse {
            jsonrpc: "2.0".to_string(),
            id: Some(1),
            error: JsonRpcError {
                code: -32602,
                message: "Invalid params".to_string(),
                data: Some(json!({"details": "Channel not found"})),
            },
        };
        
        let error_message = DeribitMessage::Error(error_response);
        
        // Test error message display
        let display_str = format!("{}", error_message);
        assert!(display_str.contains("Error: Invalid params"));
        
        // Test serialization of error message
        let json_str = serde_json::to_string(&error_message)
            .expect("Failed to serialize error message");
        
        let deserialized: DeribitMessage = serde_json::from_str(&json_str)
            .expect("Failed to deserialize error message");
        
        match deserialized {
            DeribitMessage::Error(err) => {
                assert_eq!(err.error.code, -32602);
                assert_eq!(err.error.message, "Invalid params");
            }
            _ => panic!("Expected error message"),
        }
    }

    #[test]
    fn test_api_documentation_compliance() {
        // Test that our implementation matches the API documentation exactly
        
        // From documentation: 
        // Parameter: channels (required, array) - A list of channels to unsubscribe from
        let request = UnsubscribeRequest::new(vec![
            "deribit_price_index.btc_usd".to_string(),
        ]);
        
        let params = request.to_params();
        
        // Verify parameter structure matches documentation
        assert!(params.is_object());
        assert!(params.get("channels").is_some());
        assert!(params.get("channels").unwrap().is_array());
        
        // From documentation:
        // Response: result (array of string) - A list of subscribed channels
        let response_data = json!(["remaining.channel1", "remaining.channel2"]);
        let response = UnsubscribeResponse::try_from(response_data)
            .expect("Failed to parse response");
        
        // Verify response is array of strings
        let channels = response.channels();
        assert_eq!(channels.len(), 2);
        assert_eq!(channels[0], "remaining.channel1");
        assert_eq!(channels[1], "remaining.channel2");
        
        // Test JSON-RPC 2.0 compliance
        let json_rpc_request = JsonRpcRequest::new(
            1,
            "public/unsubscribe",
            Some(params),
        );
        
        assert_eq!(json_rpc_request.jsonrpc, "2.0");
        assert_eq!(json_rpc_request.method, "public/unsubscribe");
        assert!(json_rpc_request.params.is_some());
    }
}