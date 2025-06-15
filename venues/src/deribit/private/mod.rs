//! Deribit private WebSocket implementation
//! 
//! This module provides private WebSocket functionality for the Deribit exchange,
//! implementing the JSON-RPC 2.0 protocol for private channel operations.
//! 
//! # Features
//! 
//! - JSON-RPC 2.0 protocol support
//! - Private channel unsubscription
//! - Automatic authentication using client credentials
//! - Full integration with the common WebSocket trait
//! 
//! # Example
//! 
//! ```rust
//! use venues::deribit::private::PrivateWebSocketClient;
//! use rest::secrets::ExposableSecret;
//! use websockets::WebSocketConnection;
//! 
//! # struct ExampleSecret { secret: String }
//! # impl ExampleSecret { fn new(s: String) -> Self { Self { secret: s } } }
//! # impl ExposableSecret for ExampleSecret { fn expose_secret(&self) -> String { self.secret.clone() } }
//! # 
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let api_key = Box::new(ExampleSecret::new("your_api_key".to_string()));
//! let api_secret = Box::new(ExampleSecret::new("your_api_secret".to_string()));
//! 
//! let mut client = PrivateWebSocketClient::new(api_key, api_secret, None);
//! 
//! // Connect and authenticate
//! client.connect().await?;
//! 
//! // Unsubscribe from channels
//! let channels = vec!["user.orders.BTC-PERPETUAL.raw".to_string()];
//! let remaining = client.unsubscribe(channels).await?;
//! 
//! println!("Remaining channels: {:?}", remaining);
//! # Ok(())
//! # }
//! ```

mod websocket;

#[cfg(test)]
mod integration_tests;

pub use websocket::*;