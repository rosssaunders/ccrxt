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
use reqwest::StatusCode;
use rest::secrets::ExposableSecret;
use serde::Deserialize;
use sha2::Sha256;
use std::time::Instant;

use super::api_errors::BinanceCoinMAPIError;
use super::rate_limit::BinanceCoinMRateLimiter;
use super::types::{
    BinanceCoinMError, BinanceCoinMResponse, BinanceCoinMResult, BinanceHeaders, ErrorResponse,
};

/// Represents a successful or error response from the Binance API.
/// This enum is used to handle both successful responses and error responses
/// in a unified way, allowing for easier error handling and response parsing.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ApiResponse<T> {
    Ok(T),
    Err(ErrorResponse),
}

/// A client for interacting with the Binance Coin-M Futures private REST API
///
/// This client handles encrypted API keys and secrets for enhanced security.
/// The API key and secret are stored in encrypted form and only decrypted when needed.
pub struct BinanceCoinMPrivateRest {
    pub(crate) client: Client,
    pub(crate) rate_limiter: BinanceCoinMRateLimiter,
    pub(crate) encrypted_api_key: Box<dyn ExposableSecret>,
    pub(crate) encrypted_api_secret: Box<dyn ExposableSecret>,
    pub(crate) base_url: String,
}

impl BinanceCoinMPrivateRest {
    /// Creates a new BinanceCoinMPrivateRest client with encrypted API credentials
    ///
    /// # Arguments
    /// * `encrypted_api_key` - The encrypted API key
    /// * `encrypted_api_secret` - The encrypted API secret
    /// * `base_url` - The base URL for the API
    /// * `encryption_key` - The key used for decrypting the API credentials
    ///
    /// # Returns
    /// A new BinanceCoinMPrivateRest client instance
    pub fn new(
        encrypted_api_key: Box<dyn ExposableSecret>,
        encrypted_api_secret: Box<dyn ExposableSecret>,
        base_url: String,
    ) -> Self {
        Self {
            client: Client::new(),
            rate_limiter: BinanceCoinMRateLimiter::default(),
            encrypted_api_key,
            encrypted_api_secret,
            base_url,
        }
    }

    /// Signs a request using the decrypted API secret
    pub fn sign_request(&self, query_string: &str) -> Result<String, BinanceCoinMError> {
        let api_secret = self.encrypted_api_secret.expose_secret();
        let mut mac = Hmac::<Sha256>::new_from_slice(api_secret.as_bytes())
            .map_err(|e| BinanceCoinMError::Error(format!("SigningFailed: {}", e)))?;
        mac.update(query_string.as_bytes());
        Ok(hex::encode(mac.finalize().into_bytes()))
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
    pub async fn send_request<T>(
        &self,
        endpoint: &str,
        method: reqwest::Method,
        query_string: Option<&str>,
        body: Option<&str>,
        weight: u32,
        is_order: bool,
    ) -> BinanceCoinMResult<T>
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
        let rate_limit_start = Instant::now();

        let url = match query_string {
            Some(qs) if method == reqwest::Method::GET => format!("{}{}?{}", self.base_url, endpoint, qs),
            _ => format!("{}{}", self.base_url, endpoint),
        };

        let api_key = self.encrypted_api_key.expose_secret();
        let mut request = self.client.request(method.clone(), &url);
        request = request.header("X-MBX-APIKEY", api_key);

        if method != reqwest::Method::GET {
            if let Some(body_str) = body {
                request = request.header("Content-Type", "application/x-www-form-urlencoded");
                request = request.body(body_str.to_owned());
            }
        }

        // Log the request
        //println!("Request: {}", url);

        let request_start = Instant::now();
        let response = request.send().await.map_err(BinanceCoinMError::HttpError)?;
        let rate_limit_duration = rate_limit_start.elapsed();
        let request_duration = request_start.elapsed();

        let headers = BinanceHeaders {
            used_weight_1m: response
                .headers()
                .get("X-MBX-USED-WEIGHT-1M")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse().ok()),
            order_count_1m: response
                .headers()
                .get("X-MBX-ORDER-COUNT-1M")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse().ok()),
            order_count_1d: response
                .headers()
                .get("X-MBX-ORDER-COUNT-1D")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse().ok()),
            order_count_1s: response
                .headers()
                .get("X-MBX-ORDER-COUNT-1S")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse().ok()),
        };
        // Update rate limiter from headers
        self.rate_limiter.update_from_headers(&headers).await;

        match response.status() {
            StatusCode::OK => {
                let text = response
                    .text()
                    .await
                    .map_err(BinanceCoinMError::HttpError)?;

                // Log the response
                //println!("Response: {}", text);

                let data: T = serde_json::from_str(&text).map_err(|e| {
                    BinanceCoinMError::Error(format!("JSON decode error: {} | body: {}", e, text))
                })?;
                // Check if the response is actually an error disguised as success
                if let Ok(api_response) = serde_json::from_str::<ApiResponse<T>>(&text) {
                    match api_response {
                        ApiResponse::Err(err) => {
                            return Err(BinanceCoinMError::ApiError(BinanceCoinMAPIError::from(
                                err,
                            )))
                        }
                        ApiResponse::Ok(_) => {} // Continue with normal flow
                    }
                }
                Ok(BinanceCoinMResponse {
                    data,
                    rate_limit_duration,
                    request_duration,
                    headers,
                })
            }
            _status => {
                let text = response
                    .text()
                    .await
                    .map_err(BinanceCoinMError::HttpError)?;
                let err: ErrorResponse = serde_json::from_str(&text).map_err(|e| {
                    BinanceCoinMError::Error(format!("JSON decode error: {} | body: {}", e, text))
                })?;
                Err(BinanceCoinMError::ApiError(BinanceCoinMAPIError::from(err)))
            }
        }
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
    pub async fn send_signed_request<T, R>(
        &self,
        endpoint: &str,
        method: reqwest::Method,
        request: R,
        weight: u32,
        is_order: bool,
    ) -> BinanceCoinMResult<T>
    where
        T: serde::de::DeserializeOwned,
        R: super::types::PrivateRequest,
    {
        // Serialize the request to a string (custom serializer handles batchOrders as JSON string if needed)
        let serialized = serde_urlencoded::to_string(&request)
            .map_err(|e| BinanceCoinMError::Error(format!("Failed to encode params: {}\nBacktrace:\n{}", e, std::backtrace::Backtrace::capture())))?;
        let signature = self.sign_request(&serialized)?;
        let signed = format!("{}&signature={}", serialized, signature);

        if method == reqwest::Method::GET {
            self.send_request(endpoint, method, Some(&signed), None, weight, is_order).await
        } else {
            self.send_request(endpoint, method, None, Some(&signed), weight, is_order).await
        }
    }
}
