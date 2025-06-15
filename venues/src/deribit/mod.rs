//! Deribit trading platform implementation
//! 
//! This module provides rate limiting and API client implementation for the Deribit API.
//! Deribit uses a credit-based rate limiting system with different tiers based
//! on trading volume.
//!
//! # Example Usage
//!
//! ```rust
//! use venues::deribit::{PublicRestClient, RateLimiter, AccountTier, EndpointType};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a public client
//!     let client = PublicRestClient::new_default("https://api.deribit.com", AccountTier::Tier3);
//!     
//!     // Generate a fork token
//!     let response = client.fork_token(
//!         "refresh_token_here".to_string(),
//!         "my_session".to_string(),
//!         1
//!     ).await?;
//!     
//!     println!("Access token: {}", response.result.access_token);
//!     
//!     Ok(())
//! }
//! ```

pub mod errors;
pub mod rate_limit;
pub mod public;
mod integration_tests;

pub use errors::*;
pub use rate_limit::*;
pub use public::RestClient as PublicRestClient;