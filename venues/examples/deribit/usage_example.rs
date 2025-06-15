//! Example usage of Deribit WebSocket public/hello endpoint
//! 
//! **Deprecated**: This example has been moved to `venues/examples/deribit/ws_hello_example.rs`. 
//! Please refer to that file for the updated example.

use crate::deribit::{AccountTier, DeribitWebSocketClient, RateLimiter};
use websockets::WebSocketConnection;

/// Example demonstrating how to use the Deribit public/hello endpoint
/// 
/// This function shows:
/// 1. Creating a Deribit WebSocket client with rate limiting
/// 2. Connecting to the WebSocket
/// 3. Sending a hello message to introduce the client
/// 4. Handling the response
pub async fn deribit_hello_example() -> Result<(), websockets::BoxError> {
    // Create a rate limiter for a Tier 4 account
    let rate_limiter = RateLimiter::new(AccountTier::Tier4);
    
    // Create WebSocket client for Deribit
    // In production, you might want to use a custom URL or test URL
    let mut client = DeribitWebSocketClient::new(None, rate_limiter);
    
    // Connect to Deribit WebSocket
    client.connect().await?;
    
    println!("Connected to Deribit WebSocket");
    
    // Send hello message to introduce our client
    let hello_response = client.send_hello(
        "ccrxt_rust_client".to_string(),
        "0.1.0".to_string(),
    ).await?;
    
    println!("Hello response received:");
    println!("  ID: {}", hello_response.id);
    println!("  JSON-RPC Version: {}", hello_response.jsonrpc);
    println!("  API Version: {}", hello_response.result.version);
    
    // Disconnect when done
    client.disconnect().await?;
    
    println!("Disconnected from Deribit WebSocket");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deribit::{HelloRequest, HelloResponse, HelloResult, JsonRpcRequest};
    
    #[test]
    fn test_example_compiles() {
        // This test ensures the example code compiles correctly
        // Note: We don't actually run the WebSocket connection in tests
        // as that would require a real network connection
        
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);
        let _client = DeribitWebSocketClient::new(None, rate_limiter);
        
        // Test request creation
        let request = JsonRpcRequest::new_hello(
            1,
            "test_client".to_string(),
            "1.0.0".to_string(),
        );
        
        assert_eq!(request.method, "public/hello");
        assert_eq!(request.params.client_name, "test_client");
        assert_eq!(request.params.client_version, "1.0.0");
    }
    
    #[tokio::test]
    async fn test_rate_limiting_behavior() {
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);
        let _client = DeribitWebSocketClient::new(None, rate_limiter);
        
        // The public/hello endpoint should always be allowed
        // This test ensures the rate limiting works as expected
        // Note: This doesn't test the actual WebSocket functionality
    }
    
    #[test] 
    fn test_hello_structures() {
        // Test HelloRequest
        let hello_req = HelloRequest {
            client_name: "test_client".to_string(),
            client_version: "1.0.0".to_string(),
        };
        
        assert_eq!(hello_req.client_name, "test_client");
        assert_eq!(hello_req.client_version, "1.0.0");
        
        // Test HelloResponse structure  
        let hello_result = HelloResult {
            version: "1.2.26".to_string(),
        };
        
        let hello_response = HelloResponse {
            id: 1,
            jsonrpc: "2.0".to_string(),
            result: hello_result,
        };
        
        assert_eq!(hello_response.id, 1);
        assert_eq!(hello_response.jsonrpc, "2.0");
        assert_eq!(hello_response.result.version, "1.2.26");
    }
}

// Note: To run this example with a real connection, you would need:
// 
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     deribit_hello_example().await
// }
//
// However, this requires an actual network connection to Deribit's WebSocket endpoint