//! Deribit trading platform implementation
//! 
//! This module provides rate limiting and WebSocket functionality for the Deribit API.
//! Deribit uses a credit-based rate limiting system with different tiers based
//! on trading volume and also supports private WebSocket operations using JSON-RPC 2.0.
//!
//! # Example Usage
//!
//! ## Rate Limiting
//! 
//! ```rust
//! use venues::deribit::{RateLimiter, AccountTier, EndpointType};
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
//!     // Check if we can make a matching engine request (limited by tier)
//!     limiter.check_limits(EndpointType::MatchingEngine).await?;
//!     limiter.record_request(EndpointType::MatchingEngine).await;
//!     
//!     // Check rate limit status
//!     let status = limiter.get_status().await;
//!     println!("Available credits: {}", status.available_credits);
//!     println!("Account tier: {:?}", status.account_tier);
//!     
//!     Ok(())
//! }
//! ```
//! 
//! ## Private WebSocket Operations
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
//! client.connect().await?;
//! 
//! // Unsubscribe from private channels
//! let channels = vec!["user.orders.BTC-PERPETUAL.raw".to_string()];
//! let remaining = client.unsubscribe(channels).await?;
//! 
//! println!("Remaining subscribed channels: {:?}", remaining);
//! # Ok(())
//! # }
//! ```

pub mod rate_limit;
pub mod private;

pub use rate_limit::*;