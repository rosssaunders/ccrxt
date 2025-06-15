//! Deribit trading platform implementation
//! 
//! This module provides rate limiting, WebSocket connectivity, and other utilities 
//! for the Deribit API. Deribit uses a credit-based rate limiting system with 
//! different tiers based on trading volume.
//!
//! # Example Usage
//!
//! ```rust
//! use venues::deribit::{RateLimiter, AccountTier, EndpointType};
//! use venues::deribit::websocket::WebSocketClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a rate limiter for a Tier 3 account (1-25M USD trading volume)
//!     let limiter = RateLimiter::new(AccountTier::Tier3);
//!     
//!     // Check if we can make a non-matching engine request (consumes 500 credits)
//!     limiter.check_limits(EndpointType::NonMatchingEngine).await?;
//!     
//!     // Record the request after making it
//!     limiter.record_request(EndpointType::NonMatchingEngine).await;
//!     
//!     // Create and connect WebSocket client
//!     let mut ws_client = WebSocketClient::new();
//!     ws_client.connect().await?;
//!     
//!     // Disable heartbeat messages
//!     let response = ws_client.disable_heartbeat().await?;
//!     println!("Heartbeat disabled: {}", response.is_success());
//!     
//!     // Disconnect
//!     ws_client.disconnect().await?;
//!     
//!     Ok(())
//! }
//! ```

pub mod rate_limit;
pub mod websocket;

pub use rate_limit::*;
pub use websocket::*;