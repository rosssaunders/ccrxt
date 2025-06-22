//! Binance Portfolio Margin API request handling module.
//!
//! This module provides functionality for making HTTP requests to the Binance Portfolio Margin API.
//! It handles authentication, rate limiting headers, error responses, and request/response timing.
//!
//! ## Binance Exchange Behavior
//!
//! The Binance Portfolio Margin API has specific behaviors that this module handles:
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
use std::borrow::Cow;

use hex;
use hmac::{Hmac, Mac};
use reqwest::Client;
use rest::secrets::ExposableSecret;
use sha2::Sha256;

use crate::binance::portfolio::{Errors, RateLimiter, RestResult};

/// Signs a request using the decrypted API secret
/// Signs a query string using the decrypted API secret and returns the signature as a hex string.
///
/// # Arguments
/// * `query_string` - The query string to sign
///
/// # Returns
/// A result containing the signature as a hex string or a Hmac error if signing fails.
#[allow(dead_code)]
fn sign_request(api_secret: &dyn ExposableSecret, query_string: &str) -> Result<String, Errors> {
    let api_secret = api_secret.expose_secret();
    let mut mac = Hmac::<Sha256>::new_from_slice(api_secret.as_bytes()).map_err(|_| Errors::InvalidApiKey())?;
    mac.update(query_string.as_bytes());
    Ok(hex::encode(mac.finalize().into_bytes()))
}

/// A client for interacting with the Binance Portfolio Margin private REST API
///
/// This client handles encrypted API keys and secrets for enhanced security.
/// The API key and secret are stored in encrypted form and only decrypted when needed.
#[non_exhaustive]
#[allow(dead_code)]
pub struct RestClient {
    /// The underlying HTTP client used for making requests.
    pub(crate) client: Client,
    /// The rate limiter for this client.
    pub(crate) rate_limiter: RateLimiter,
    /// The encrypted API key.
    pub(crate) api_key: Box<dyn ExposableSecret>,
    /// The encrypted API secret.
    pub(crate) api_secret: Box<dyn ExposableSecret>,
    /// The base URL for the API.
    pub(crate) base_url: Cow<'static, str>,
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
        base_url: impl Into<Cow<'static, str>>,
        rate_limiter: RateLimiter,
        client: Client,
    ) -> Self {
        Self {
            client,
            rate_limiter,
            api_key,
            api_secret,
            base_url: base_url.into(),
        }
    }

    /// Sends a request to the Binance Portfolio Margin API
    ///
    /// This method encapsulates all the logic for making authenticated requests to the Binance API,
    /// including rate limiting, error handling, and response parsing.
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path (e.g., "/papi/v1/account")
    /// * `method` - The HTTP method to use
    /// * `query_string` - Optional query string parameters (for GET or for URL params)
    /// * `body` - Optional body (for POST/PUT/DELETE with x-www-form-urlencoded)
    /// * `weight` - The request weight for this endpoint
    /// * `is_order` - Whether this is an order-related endpoint
    ///
    /// # Returns
    /// A result containing the parsed response data and metadata, or an error
    #[allow(dead_code)]
    pub(super) async fn send_request<T>(
        &self,
        endpoint: &str,
        method: reqwest::Method,
        query_string: Option<&str>,
        body: Option<&[(&str, &str)]>,
        weight: u32,
        is_order: bool,
    ) -> RestResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = crate::binance::portfolio::rest::common::build_url(&self.base_url, endpoint, query_string)?;
        let mut headers = vec![];
        if !self.api_key.expose_secret().is_empty() {
            headers.push(("X-MBX-APIKEY", self.api_key.expose_secret()));
        }
        let body_data = body
            .map(|b| serde_urlencoded::to_string(b).map_err(|e| Errors::Error(format!("Failed to serialize body: {e}"))))
            .transpose()?;
        if body_data.is_some() {
            headers.push((
                "Content-Type",
                "application/x-www-form-urlencoded".to_string(),
            ));
        }
        let rest_response = crate::binance::portfolio::rest::common::send_rest_request(
            &self.client,
            &url,
            method,
            headers,
            body_data.as_deref(),
            &self.rate_limiter,
            weight,
            is_order,
        )
        .await?;
        Ok(crate::binance::portfolio::RestResponse {
            data: rest_response.data,
            request_duration: rest_response.request_duration,
            headers: rest_response.headers,
        })
    }

    /// Sends a signed request to the Binance Portfolio Margin API
    ///
    /// This method automatically handles timestamp generation and request signing for private endpoints.
    /// It appends the current timestamp and generates the required signature.
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path (e.g., "/papi/v1/account")
    /// * `method` - The HTTP method to use
    /// * `request` - The request parameters implementing PrivateRequest
    /// * `weight` - The request weight for this endpoint
    /// * `is_order` - Whether this is an order-related endpoint
    ///
    /// # Returns
    /// A result containing the parsed response data and metadata, or an error
    #[allow(dead_code)]
    pub(super) async fn send_signed_request<T, R>(&self, endpoint: &str, method: reqwest::Method, request: R, weight: u32, is_order: bool) -> RestResult<T>
    where
        T: serde::de::DeserializeOwned,
        R: serde::Serialize,
    {
        let serialized = serde_urlencoded::to_string(&request).map_err(|e| {
            crate::binance::portfolio::Errors::Error(format!(
                "Failed to encode params: {}\nBacktrace:\n{}",
                e,
                std::backtrace::Backtrace::capture()
            ))
        })?;
        let signature = sign_request(self.api_secret.as_ref(), &serialized)?;
        let signed = format!("{serialized}&signature={signature}");
        if method == reqwest::Method::GET {
            self.send_request(endpoint, method, Some(&signed), None, weight, is_order)
                .await
        } else {
            self.send_request(
                endpoint,
                method,
                None,
                Some(&[("signed", signed.as_str())]),
                weight,
                is_order,
            )
            .await
        }
    }
}
