//! Deribit trading platform implementation
//! 
//! This module provides rate limiting, REST API clients, and other utilities for the Deribit API.
//! Deribit uses a credit-based rate limiting system with different tiers based
//! on trading volume and supports JSON-RPC 2.0 protocol.
//!
//! # Example Usage
//!
//! ```rust
//! use venues::deribit::{RateLimiter, AccountTier, EndpointType, PublicRestClient};
//! use reqwest::Client;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a rate limiter for a Tier 3 account (1-25M USD trading volume)
//!     let rate_limiter = RateLimiter::new(AccountTier::Tier3);
//!     
//!     // Create a public REST client
//!     let client = Client::new();
//!     let rest_client = PublicRestClient::new("https://www.deribit.com", client, rate_limiter);
//!     
//!     // Get current server time
//!     let current_time = rest_client.get_time().await?;
//!     println!("Current server time: {} ms", current_time);
//!     
//!     Ok(())
//! }
//! ```

pub mod enums;
pub mod errors;
pub mod public;
pub mod rate_limit;

pub use enums::{JsonRpcRequest, JsonRpcResponse};
pub use errors::{ApiError, Errors, JsonRpcError};
pub use public::{RestClient as PublicRestClient, RestResult};
pub use rate_limit::*;