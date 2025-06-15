//! Deribit trading platform implementation
//! 
//! This module provides rate limiting and API client functionality for the Deribit API.
//! Deribit uses a credit-based rate limiting system with different tiers based
//! on trading volume, and implements a JSON-RPC 2.0 protocol.
//!
//! # Example Usage
//!
//! ```rust
//! use venues::deribit::{RateLimiter, AccountTier, EndpointType, PublicRestClient, Currency, GetCombosRequest};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a rate limiter for a Tier 3 account (1-25M USD trading volume)
//!     let limiter = RateLimiter::new(AccountTier::Tier3);
//!     
//!     // Create a public REST client
//!     let client = PublicRestClient::new(
//!         "https://www.deribit.com/api/v2",
//!         reqwest::Client::new(),
//!         limiter,
//!     );
//!     
//!     // Get combos for BTC
//!     let request = GetCombosRequest {
//!         currency: Currency::Btc,
//!     };
//!     let combos = client.get_combos(request).await?;
//!     
//!     println!("Found {} combos", combos.result.len());
//!     
//!     Ok(())
//! }
//! ```

pub mod enums;
pub mod errors;
pub mod examples;
pub mod rate_limit;

pub mod public {
    pub mod rest;
    pub use self::rest::RestClient;
}

pub use enums::*;
pub use errors::{Errors, JsonRpcError, RestResult};
pub use public::RestClient as PublicRestClient;
pub use rate_limit::*;

// Re-export the get_combos types for convenience
pub use public::rest::get_combos::{GetCombosRequest, GetCombosResponse, Combo, ComboLeg};