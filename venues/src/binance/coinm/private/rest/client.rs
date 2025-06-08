//! Binance Coin-M Futures API request handling module.
//!
//! This module provides functionality for making HTTP requests to the Binance Coin-M Futures API.
//! It handles authentication, rate limiting headers, error responses, and request/response timing.
//!
//! ## Binance Exchange Behavior
//!
//! The Binance API has specific behaviors that this module handles:
//!
//! - **Dual Error Format**: Binance can return errors in two ways:
//!   1. HTTP error status codes with error JSON in the body
//!   2. HTTP 200 OK with error details in the response JSON (disguised errors)
//!
//! - **Rate Limiting Headers**: Binance includes rate limiting information in response headers:
//!   - `X-MBX-USED-WEIGHT-1M`: API weight used in the last minute
//!   - `X-MBX-ORDER-COUNT-1M`: Orders placed in the last minute  
//!   - `X-MBX-ORDER-COUNT-1D`: Orders placed in the last day
//!   - `X-MBX-ORDER-COUNT-1S`: Orders placed in the last second
//!
//! - **Authentication**: Requires API key in `X-MBX-APIKEY` header for authenticated endpoints
//!
//! - **Timestamp Requirements**: Signed requests must include a timestamp parameter and signature
//!   based on the current UTC timestamp in milliseconds
//!
//! - **Request Signing**: For private endpoints, query parameters (including timestamp) must be
//!   signed using HMAC-SHA256 with the API secret
use hex;
use hmac::{Hmac, Mac};
use reqwest::Client;
use rest::secrets::ExposableSecret;
use serde::de::DeserializeOwned;
use sha2::Sha256;

use crate::binance::coinm::{RestResult, RestResponse, Errors, RateLimiter, execute_request};
use crate::binance::coinm::private::rest::PrivateRequest;

/// Represents a successful or error response from the Binance API.
/// This enum is used to handle both successful responses and error responses
/// in a unified way, allowing for easier error handling and response parsing.
// #[derive(Debug, Deserialize)]
// #[serde(untagged)]
// enum ApiResponse<T> {
//     Ok(T),
//     Err(ErrorResponse),
// }

/// Signs a request using the decrypted API secret
/// Signs a query string using the decrypted API secret and returns the signature as a hex string.
///
/// # Arguments
/// * `query_string` - The query string to sign
///
/// # Returns
/// A result containing the signature as a hex string or a Hmac error if signing fails.
fn sign_request(
    api_secret: &dyn ExposableSecret,
    query_string: &str,
) -> Result<String, Errors> {
    let api_secret = api_secret.expose_secret();
    let mut mac = Hmac::<Sha256>::new_from_slice(api_secret.as_bytes())
        .map_err(|_| Errors::InvalidApiKey())?;
    mac.update(query_string.as_bytes());
    Ok(hex::encode(mac.finalize().into_bytes()))
}

/// A client for interacting with the Binance Coin-M Futures private REST API
///
/// This client handles encrypted API keys and secrets for enhanced security.
/// The API key and secret are stored in encrypted form and only decrypted when needed.
pub struct RestClient {
    pub(crate) client: Client,
    pub(crate) rate_limiter: RateLimiter,
    pub(crate) api_key: Box<dyn ExposableSecret>,
    pub(crate) api_secret: Box<dyn ExposableSecret>,
    pub(crate) base_url: String,
}

impl RestClient {
    /// Creates a new RestClient with encrypted API credentials
    ///
    /// # Arguments
    /// * `encrypted_api_key` - The encrypted API key
    /// * `encrypted_api_secret` - The encrypted API secret
    /// * `base_url` - The base URL for the API
    /// * `encryption_key` - The key used for decrypting the API credentials
    ///
    /// # Returns
    /// A new RestClient instance
    pub fn new(
        api_key: Box<dyn ExposableSecret>,
        api_secret: Box<dyn ExposableSecret>,
        base_url: String,
        rate_limiter: RateLimiter,
        client: Client,
    ) -> Self {
        Self {
            client: client,
            rate_limiter: rate_limiter,
            api_key,
            api_secret,
            base_url,
        }
    }

    /// Sends a request to the Binance API
    ///
    /// This method encapsulates all the logic for making authenticated requests to the Binance API,
    /// including rate limiting, error handling, and response parsing.
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path (e.g., "/fapi/v1/order")
    /// * `method` - The HTTP method to use
    /// * `query_string` - Optional query string parameters (for GET or for URL params)
    /// * `body` - Optional body (for POST/PUT/DELETE with x-www-form-urlencoded)
    /// * `weight` - The request weight for this endpoint
    /// * `is_order` - Whether this is an order-related endpoint
    ///
    /// # Returns
    /// A result containing the parsed response data and metadata, or an error
    pub(super) async fn send_request<T>(
        &self,
        endpoint: &str,
        method: reqwest::Method,
        query_string: Option<&str>,
        body: Option<&str>,
        weight: u32,
        is_order: bool,
    ) -> RestResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        // Check rate limits before sending
        self.rate_limiter.check_limits(weight, is_order).await?;
        
        // Increment raw request counter
        self.rate_limiter.increment_raw_request().await;
        if is_order {
            self.rate_limiter.increment_order().await;
        }

        let url = match query_string {
            Some(qs) if method == reqwest::Method::GET => format!("{}{}?{}", self.base_url, endpoint, qs),
            _ => format!("{}{}", self.base_url, endpoint),
        };
        let mut headers = vec![];
        // Only add API key header for private endpoints
        if !self.api_key.expose_secret().is_empty() {
            headers.push(("X-MBX-APIKEY", self.api_key.expose_secret()));
        }
        
        // Add Content-Type header for form-encoded body
        if body.is_some() {
            headers.push(("Content-Type", "application/x-www-form-urlencoded".to_string()));
        }

        let response = execute_request(&self.client, &url, method, Some(headers), body).await.map_err(Errors::from)?;

        let headers = response.headers.clone();
        
        let request_duration = response.duration;

        let rest_response = RestResponse {
            data: response.data,
            request_duration: request_duration,
            headers: headers,
        };

        // Update rate limiter from headers
        //self.rate_limiter.update_from_headers(&headers).await;

        Ok(rest_response)
    }

    /// Sends a signed request to the Binance API
    ///
    /// This method automatically handles timestamp generation and request signing for private endpoints.
    /// It appends the current timestamp and generates the required signature.
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path (e.g., "/fapi/v1/order")
    /// * `method` - The HTTP method to use
    /// * `request` - The request parameters implementing PrivateRequest
    /// * `weight` - The request weight for this endpoint
    /// * `is_order` - Whether this is an order-related endpoint
    ///
    /// # Returns
    /// A result containing the parsed response data and metadata, or an error
    pub(super) async fn send_signed_request<T, R>(
        &self,
        endpoint: &str,
        method: reqwest::Method,
        request: R,
        weight: u32,
        is_order: bool,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
        R: PrivateRequest,
    {
        let serialized = serde_urlencoded::to_string(&request)
            .map_err(|e| Errors::Error(format!("Failed to encode params: {}\nBacktrace:\n{}", e, std::backtrace::Backtrace::capture())))?;
        
        print!("Sending request to Binance API: {} {} with params: {}\n", method, endpoint, serialized);
        
        let signature = sign_request(self.api_secret.as_ref(), &serialized)?;
        let signed = format!("{}&signature={}", serialized, signature);
        if method == reqwest::Method::GET {
            self.send_request(endpoint, method, Some(&signed), None, weight, is_order).await
        } else {
            self.send_request(endpoint, method, None, Some(&signed), weight, is_order).await
        }
    }
}
