//! Shared Binance REST client logic for all Binance venues.
//!
//! This module provides a unified API client that works across all Binance venues
//! (Spot, USDM, COINM, Options, Portfolio) with venue-specific configuration.

use hex;
use hmac::{Hmac, Mac};
use secrets::ExposableSecret;
use sha2::Sha256;

pub mod client;
pub mod credentials;
pub mod errors;
pub mod rate_limiter;
pub mod rate_limiter_trait;

pub mod venue_trait;

// Re-export commonly used items
pub use client::{
    PrivateBinanceClient, PublicBinanceClient, RateLimitInfo, ResponseHeaders, RestResponse,
};
pub use credentials::Credentials;
pub use errors::{ApiError, ErrorResponse, Errors};
pub use rate_limiter::{RateLimiter, UsageStats};
pub use rate_limiter_trait::BinanceRateLimiter;
pub use venue_trait::{RateLimits, VenueConfig};

/// Signs a query string using the decrypted API secret and returns the signature as a hex string.
pub fn sign_request(
    api_secret: &dyn ExposableSecret,
    query_string: &str,
) -> Result<String, errors::Errors> {
    let api_secret = api_secret.expose_secret();
    let mut mac = Hmac::<Sha256>::new_from_slice(api_secret.as_bytes())
        .map_err(|_| errors::Errors::InvalidApiKey)?;
    mac.update(query_string.as_bytes());
    Ok(hex::encode(mac.finalize().into_bytes()))
}
