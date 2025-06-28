//! Shared Binance REST client logic for all Binance venues.
//!
//! This module provides a unified API client that works across all Binance venues
//! (Spot, USDM, COINM, Options, Portfolio) with venue-specific configuration.

use hex;
use hmac::{Hmac, Mac};
use serde::Serialize;
use sha2::Sha256;

use rest::secrets::ExposableSecret;

pub mod client;
pub mod errors;
pub mod rate_limiter;
pub mod request;
pub mod venue_trait;

// Re-export commonly used items
pub use client::{BinanceClient, RateLimitInfo, ResponseHeaders, RestResponse};
pub use errors::{ApiError, ErrorResponse, Errors};
pub use rate_limiter::{RateLimiter, UsageStats};
pub use venue_trait::{RateLimits, VenueConfig};

/// Signs a query string using the decrypted API secret and returns the signature as a hex string.
pub fn sign_request(
    api_secret: &dyn ExposableSecret,
    query_string: &str,
) -> Result<String, String> {
    let api_secret = api_secret.expose_secret();
    let mut mac = Hmac::<Sha256>::new_from_slice(api_secret.as_bytes())
        .map_err(|_| "Invalid API key/secret".to_string())?;
    mac.update(query_string.as_bytes());
    Ok(hex::encode(mac.finalize().into_bytes()))
}

/// Canonical send_signed_request for all Binance venues.
///
/// - Serializes the request struct.
/// - Signs the request.
/// - Calls the venue's send_request implementation.
///
/// # Arguments
/// * `client` - The RestClient instance
/// * `endpoint` - The API endpoint path
/// * `method` - The HTTP method
/// * `request` - The request struct (will be serialized)
/// * `weight` - The request weight
/// * `is_order` - Whether this is an order endpoint
///
/// # Returns
/// The full RestResponse with data, headers, and metadata.
pub async fn send_signed_request<T, R, C>(
    client: &C,
    endpoint: &str,
    method: reqwest::Method,
    request: R,
    weight: u32,
    is_order: bool,
) -> Result<C::RestResponse<T>, C::Error>
where
    T: serde::de::DeserializeOwned + Send + 'static,
    R: Serialize,
    C: BinanceRestClient,
{
    let serialized = serde_urlencoded::to_string(&request).map_err(C::from_serialize)?;
    let signature = sign_request(client.api_secret(), &serialized).map_err(C::from_signature)?;
    let signed = format!("{serialized}&signature={signature}");

    if method == reqwest::Method::GET {
        client
            .send_request(endpoint, method, Some(&signed), None, weight, is_order)
            .await
    } else {
        client
            .send_request(endpoint, method, None, Some(&signed), weight, is_order)
            .await
    }
}

/// Simplified trait for Binance REST clients to share signing logic.
///
/// This trait is for backward compatibility with existing implementations.
/// New code should use the `BinanceClient<V>` directly.
pub trait BinanceRestClient {
    type Error;
    type RestResponse<T>;

    /// Get the API secret for signing requests.
    fn api_secret(&self) -> &dyn ExposableSecret;

    /// Send an HTTP request with optional query string or body.
    async fn send_request<T>(
        &self,
        endpoint: &str,
        method: reqwest::Method,
        query_string: Option<&str>,
        body: Option<&str>,
        weight: u32,
        is_order: bool,
    ) -> Result<Self::RestResponse<T>, Self::Error>
    where
        T: serde::de::DeserializeOwned + Send + 'static;

    /// Extract the data from the RestResponse.
    fn extract_data<T>(response: Self::RestResponse<T>) -> T;

    /// Convert serialization errors to the venue's error type.
    fn from_serialize(e: serde_urlencoded::ser::Error) -> Self::Error;

    /// Convert signature errors to the venue's error type.
    fn from_signature(e: String) -> Self::Error;
}
