//! Deribit trading platform implementation
//! 
//! This module provides rate limiting and REST API access for the Deribit API.
//! Deribit uses a credit-based rate limiting system with different tiers based
//! on trading volume.
//!
//! # Example Usage
//!
//! ```rust
//! use venues::deribit::{RateLimiter, AccountTier, EndpointType, PublicRestClient, GetComboDetailsRequest};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a rate limiter for a Tier 3 account (1-25M USD trading volume)
//!     let limiter = RateLimiter::new(AccountTier::Tier3);
//!     
//!     // Create a public REST client
//!     let client = reqwest::Client::new();
//!     let rest_client = PublicRestClient::new("https://deribit.com", client, limiter);
//!     
//!     // Get combo details
//!     let request = GetComboDetailsRequest {
//!         combo_id: "COMBO-123456".to_string(),
//!     };
//!     let response = rest_client.get_combo_details(request).await?;
//!     println!("Combo details: {:?}", response.result);
//!     
//!     Ok(())
//! }
//! ```

pub mod enums;
mod errors;
mod examples;
mod integration_tests;
pub mod rate_limit;

pub mod public {
    mod rest;
    pub use self::rest::RestClient;
    pub use self::rest::{GetComboDetailsRequest, GetComboDetailsResponse};
}

pub use enums::*;
pub use errors::{ApiError, ErrorResponse, Errors};
pub use public::RestClient as PublicRestClient;
pub use rate_limit::{AccountTier, EndpointType, RateLimitError, RateLimiter};
pub use public::{GetComboDetailsRequest, GetComboDetailsResponse};

/// Type alias for results returned by Deribit API operations
pub type RestResult<T> = Result<T, Errors>;