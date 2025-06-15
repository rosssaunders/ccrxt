//! Deribit trading platform implementation
//! 
//! This module provides JSON-RPC clients, rate limiting, and other utilities for the Deribit API.
//! Deribit uses a credit-based rate limiting system with different tiers based
//! on trading volume.
//!
//! # Example Usage
//!
//! ```rust
//! use venues::deribit::{RateLimiter, AccountTier, EndpointType};
//! use venues::deribit::public::JsonRpcClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a rate limiter for a Tier 3 account (1-25M USD trading volume)
//!     let limiter = RateLimiter::new(AccountTier::Tier3);
//!     
//!     // Create a public client for JSON-RPC requests
//!     let client = reqwest::Client::new();
//!     let public_client = JsonRpcClient::new("https://www.deribit.com/api/v2", client, limiter);
//!     
//!     // Get platform status
//!     let status = public_client.get_status().await?;
//!     println!("Platform locked: {}", status.locked);
//!     println!("Locked indices: {:?}", status.locked_indices);
//!     
//!     Ok(())
//! }
//! ```

pub mod errors;
pub mod jsonrpc;
pub mod public;
pub mod rate_limit;

// Re-export commonly used types
pub use errors::{DeribitError, DeribitResult};
pub use jsonrpc::{JsonRpcRequest, JsonRpcResponse, JsonRpcError};
pub use rate_limit::*;