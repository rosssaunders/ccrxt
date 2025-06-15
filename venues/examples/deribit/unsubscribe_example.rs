//! Usage example for Deribit private WebSocket unsubscribe endpoint
//! 
//! This example demonstrates how to use the Deribit private WebSocket client
//! to unsubscribe from channels.

use venues::deribit::private::PrivateWebSocketClient;
use rest::secrets::ExposableSecret;
use websockets::{WebSocketConnection, BoxResult};

/// Example secret implementation for demonstration
#[derive(Debug)]
struct ExampleSecret {
    secret: String,
}

impl ExampleSecret {
    fn new(secret: String) -> Self {
        Self { secret }
    }
}

impl ExposableSecret for ExampleSecret {
    fn expose_secret(&self) -> String {
        self.secret.clone()
    }
}

#[tokio::main]
async fn main() -> BoxResult<()> {
    // Create API credentials (use environment variables in production)
    let api_key = Box::new(ExampleSecret::new("YOUR_API_KEY".to_string())) as Box<dyn ExposableSecret>;
    let api_secret = Box::new(ExampleSecret::new("YOUR_API_SECRET".to_string())) as Box<dyn ExposableSecret>;

    // Create the WebSocket client
    let mut client = PrivateWebSocketClient::new(
        api_key,
        api_secret,
        None, // Use default URL
    );

    // Connect to Deribit WebSocket API
    println!("Connecting to Deribit WebSocket API...");
    client.connect().await?;
    println!("Connected successfully!");

    // Example channels to unsubscribe from
    let channels_to_unsubscribe = vec![
        "user.orders.BTC-PERPETUAL.raw".to_string(),
        "user.trades.BTC-PERPETUAL.raw".to_string(),
    ];

    // Unsubscribe from channels
    println!("Unsubscribing from channels: {:?}", channels_to_unsubscribe);
    match client.unsubscribe(channels_to_unsubscribe).await {
        Ok(remaining_channels) => {
            println!("Successfully unsubscribed!");
            println!("Remaining subscribed channels: {:?}", remaining_channels);
        }
        Err(e) => {
            println!("Failed to unsubscribe: {}", e);
        }
    }

    // Disconnect from the WebSocket
    client.disconnect().await?;
    println!("Disconnected from Deribit WebSocket API");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_secret() {
        let secret = ExampleSecret::new("test_secret".to_string());
        assert_eq!(secret.expose_secret(), "test_secret");
    }

    #[test]
    fn test_client_creation_in_example() {
        let api_key = Box::new(ExampleSecret::new("test_key".to_string())) as Box<dyn ExposableSecret>;
        let api_secret = Box::new(ExampleSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;
        
        let client = PrivateWebSocketClient::new(api_key, api_secret, None);
        assert!(!client.is_connected());
    }
}