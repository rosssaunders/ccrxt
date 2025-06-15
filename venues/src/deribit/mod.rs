//! Deribit trading platform implementation
//! 
//! This module provides rate limiting, WebSocket connectivity, and other utilities for the Deribit API.
//! Deribit uses a credit-based rate limiting system with different tiers based
//! on trading volume.
//!
//! # Rate Limiting Example
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
//! # WebSocket Example
//!
//! ```rust
//! use venues::deribit::websocket::{DeribitWebSocketClient, WebSocketConnection};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut client = DeribitWebSocketClient::new();
//!     
//!     // Connect to Deribit WebSocket API
//!     client.connect().await?;
//!     
//!     // Subscribe to public channels
//!     let channels = vec!["book.BTC-PERPETUAL.100ms".to_string()];
//!     let subscribed = client.public_subscribe(channels).await?;
//!     println!("Subscribed to: {:?}", subscribed);
//!     
//!     // Disconnect when done
//!     client.disconnect().await?;
//!     
//!     Ok(())
//! }
//! ```

pub mod rate_limit;
pub mod websocket;

pub use rate_limit::*;
pub use websocket::*;