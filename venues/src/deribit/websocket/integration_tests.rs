#[cfg(test)]
mod integration_tests {
    use crate::deribit::websocket::{
        WebSocketClient, WebSocketConfig, DeribitMessage, DisableHeartbeatRequest
    };
    use websockets::WebSocketConnection;
    use tokio_test;

    #[tokio::test]
    async fn test_websocket_client_creation_and_config() {
        // Test default client creation
        let client = WebSocketClient::new();
        assert!(!client.is_connected());
        
        // Test custom config
        let config = WebSocketConfig {
            url: "wss://test.deribit.com/ws/api/v2".to_string(),
            request_timeout_ms: 10000,
            auto_reconnect: false,
            max_reconnect_attempts: 3,
            reconnect_delay_ms: 2000,
        };
        
        let client = WebSocketClient::with_config(config.clone());
        assert!(!client.is_connected());
        // Note: Can't test private config fields directly, but creation works
    }

    #[tokio::test]
    async fn test_disable_heartbeat_request_generation() {
        let _client = WebSocketClient::new();
        
        // Test request creation (using public interface)
        let request = DisableHeartbeatRequest::new(123);
        assert_eq!(request.jsonrpc, "2.0");
        assert_eq!(request.id, 123);
        assert_eq!(request.method, "public/disable_heartbeat");
        
        // Test serialization
        let json = serde_json::to_string(&request).expect("Failed to serialize request");
        let expected = r#"{"jsonrpc":"2.0","id":123,"method":"public/disable_heartbeat"}"#;
        assert_eq!(json, expected);
    }

    #[tokio::test]
    async fn test_websocket_trait_implementation() {
        let mut client = WebSocketClient::new();
        
        // Test WebSocketConnection trait methods exist
        assert!(!client.is_connected());
        
        // Test disconnect without connection (should not panic)
        assert!(client.disconnect().await.is_ok());
        
        // Note: We can't test actual connection without a running WebSocket server
        // These tests verify the interface is correctly implemented
    }

    #[test]
    fn test_disable_heartbeat_not_connected_error() {
        tokio_test::block_on(async {
            let client = WebSocketClient::new();
            
            // Should return error when not connected
            let result = client.disable_heartbeat().await;
            assert!(result.is_err());
            
            if let Err(error) = result {
                match error {
                    crate::deribit::websocket::WebSocketError::ConnectionError(msg) => {
                        assert_eq!(msg, "Not connected");
                    }
                    _ => panic!("Expected ConnectionError"),
                }
            }
        });
    }

    #[test]
    fn test_message_stream_interface() {
        tokio_test::block_on(async {
            let mut client = WebSocketClient::new();
            
            // Test that message_stream can be called (interface test)
            let _stream = client.message_stream();
            // Note: Stream won't produce messages without a connection, 
            // but this verifies the interface works
        });
    }

    #[test]
    fn test_comprehensive_error_scenarios() {
        use crate::deribit::websocket::WebSocketError;
        
        // Test different error types
        let conn_error = WebSocketError::ConnectionError("Connection failed".to_string());
        assert_eq!(format!("{}", conn_error), "WebSocket connection error: Connection failed");
        
        let timeout_error = WebSocketError::Timeout;
        assert_eq!(format!("{}", timeout_error), "Request timeout");
        
        let invalid_response = WebSocketError::InvalidResponse("Bad response".to_string());
        assert_eq!(format!("{}", invalid_response), "Invalid response received: Bad response");
        
        let protocol_error = WebSocketError::ProtocolError("Protocol violation".to_string());
        assert_eq!(format!("{}", protocol_error), "WebSocket protocol error: Protocol violation");
        
        let rate_limit_error = WebSocketError::RateLimit("Too many requests".to_string());
        assert_eq!(format!("{}", rate_limit_error), "Rate limit exceeded: Too many requests");
    }

    #[test]
    fn test_json_rpc_response_parsing() {
        // Test successful response
        let success_json = r#"{"jsonrpc":"2.0","id":123,"result":"ok"}"#;
        let response: crate::deribit::websocket::messages::JsonRpcResponse = 
            serde_json::from_str(success_json).expect("Failed to parse success response");
        
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, 123);
        assert!(response.result.is_some());
        assert!(response.error.is_none());
        
        if let Some(result) = response.result {
            assert_eq!(result.as_str().unwrap(), "ok");
        }
        
        // Test error response
        let error_json = r#"{"jsonrpc":"2.0","id":456,"error":{"code":-32601,"message":"Method not found"}}"#;
        let response: crate::deribit::websocket::messages::JsonRpcResponse = 
            serde_json::from_str(error_json).expect("Failed to parse error response");
        
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, 456);
        assert!(response.result.is_none());
        assert!(response.error.is_some());
        
        if let Some(error) = response.error {
            assert_eq!(error.code, -32601);
            assert_eq!(error.message, "Method not found");
        }
    }

    #[test]
    fn test_deribit_message_enum_comprehensive() {
        // Test that we can parse both request and response messages
        let messages = vec![
            (r#"{"jsonrpc":"2.0","id":1,"method":"public/test"}"#, "request"),
            (r#"{"jsonrpc":"2.0","id":2,"result":"ok"}"#, "response"),
            (r#"{"jsonrpc":"2.0","id":3,"error":{"code":-1,"message":"Error"}}"#, "response"),
            (r#"{"jsonrpc":"2.0","id":4,"method":"private/heartbeat","params":{}}"#, "request"),
        ];
        
        for (json, expected_type) in messages {
            let message: DeribitMessage = serde_json::from_str(json)
                .expect(&format!("Failed to parse: {}", json));
            
            match (message, expected_type) {
                (DeribitMessage::JsonRpcRequest(_), "request") => {
                    // Expected
                }
                (DeribitMessage::JsonRpcResponse(_), "response") => {
                    // Expected  
                }
                (actual, expected) => {
                    panic!("Expected {} but got {:?} for JSON: {}", expected, actual, json);
                }
            }
        }
    }
}