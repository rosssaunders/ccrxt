//! Deribit trading platform implementation
//! 
//! This module provides rate limiting, private REST API client, and utilities for the Deribit API.
//! Deribit uses a credit-based rate limiting system with different tiers based
//! on trading volume and JSON-RPC 2.0 protocol for API communication.
//!
//! # Example Usage
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

pub mod errors;
pub mod rate_limit;

mod private {
    pub mod rest;
    pub mod verify_block_trade;
    pub use self::rest::RestClient as PrivateRestClient;
    pub use self::verify_block_trade::{
        VerifyBlockTradeRequest, VerifyBlockTradeResponse, VerifyBlockTradeResult,
        Trade, TradeRole, TradeDirection,
    };
}

// Re-export public modules
pub use errors::{Errors, ErrorResponse, JsonRpcError};
pub use rate_limit::*;

// Export private client and types
pub use private::PrivateRestClient;
pub use private::{
    VerifyBlockTradeRequest, VerifyBlockTradeResponse, VerifyBlockTradeResult,
    Trade, TradeRole, TradeDirection,
};

/// Type alias for results returned by Deribit API operations
pub type RestResult<T> = Result<T, Errors>;