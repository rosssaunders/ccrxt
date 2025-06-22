//! Bitget Private API request handling module.
//!
//! This module provides functionality for making HTTP requests to the Bitget Private API.
//! It handles authentication, rate limiting, error responses, and request/response timing.
//!
//! ## Bitget Authentication
//!
//! The Bitget API requires the following headers for authentication:
//! - `ACCESS-KEY`: API key
//! - `ACCESS-SIGN`: Base64 encoded HMAC SHA256 signature
//! - `ACCESS-TIMESTAMP`: Unix timestamp in milliseconds
//! - `ACCESS-PASSPHRASE`: API key passphrase
//! - `Content-Type`: application/json for POST requests
//! - `locale`: Language setting (optional)
//!
//! ## Rate Limiting
//!
//! Bitget has different rate limits:
//! - Overall IP limit: 6000 requests per minute
//! - Endpoint-specific limits: varies (3-20 requests per second)
//! - UID-based limits for private endpoints

use std::borrow::Cow;

use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
use chrono::Utc;
use hmac::{Hmac, Mac};
use reqwest::Client;
use rest::secrets::ExposableSecret;
use sha2::Sha256;

use crate::bitget::rate_limit::RateLimiter;
use crate::bitget::{Errors, RestResult};

/// A client for interacting with the Bitget private REST API
///
/// This client handles encrypted API keys and secrets for enhanced security.
/// The API key, secret, and passphrase are stored in encrypted form and only decrypted when needed.
#[non_exhaustive]
pub struct RestClient {
    /// The underlying HTTP client used for making requests.
    pub(crate) client: Client,
    /// The rate limiter for this client.
    pub(crate) rate_limiter: RateLimiter,
    /// The encrypted API key.
    pub(crate) api_key: Box<dyn ExposableSecret>,
    /// The encrypted API secret.
    pub(crate) api_secret: Box<dyn ExposableSecret>,
    /// The encrypted API passphrase.
    pub(crate) api_passphrase: Box<dyn ExposableSecret>,
    /// The base URL for the API.
    pub(crate) base_url: Cow<'static, str>,
}

impl RestClient {
    /// Creates a new RestClient with encrypted API credentials
    ///
    /// # Arguments
    /// * `api_key` - The encrypted API key
    /// * `api_secret` - The encrypted API secret  
    /// * `api_passphrase` - The encrypted API passphrase
    /// * `base_url` - The base URL for the API
    /// * `rate_limiter` - The rate limiter instance
    /// * `client` - The HTTP client to use
    ///
    /// # Returns
    /// A new RestClient instance
    pub fn new(
        api_key: Box<dyn ExposableSecret>,
        api_secret: Box<dyn ExposableSecret>,
        api_passphrase: Box<dyn ExposableSecret>,
        base_url: impl Into<Cow<'static, str>>,
        rate_limiter: RateLimiter,
        client: Client,
    ) -> Self {
        Self {
            client,
            rate_limiter,
            api_key,
            api_secret,
            api_passphrase,
            base_url: base_url.into(),
        }
    }

    /// Generates the signature for Bitget API authentication
    ///
    /// The signature is created by:
    /// 1. Creating the string: timestamp + method + requestPath + queryString + body
    /// 2. HMAC SHA256 with the API secret
    /// 3. Base64 encoding the result
    fn generate_signature(&self, timestamp: i64, method: &str, request_path: &str, query_string: Option<&str>, body: Option<&str>) -> Result<String, Errors> {
        let query_part = match query_string {
            Some(q) if !q.is_empty() => format!("?{q}"),
            _ => String::new(),
        };

        let body_part = body.unwrap_or("");

        let sign_string = format!(
            "{}{}{}{}{}",
            timestamp,
            method.to_uppercase(),
            request_path,
            query_part,
            body_part
        );

        let mut mac =
            Hmac::<Sha256>::new_from_slice(self.api_secret.expose_secret().as_bytes()).map_err(|e| Errors::Error(format!("Failed to create HMAC: {e}")))?;

        mac.update(sign_string.as_bytes());
        let result = mac.finalize();
        let signature = BASE64.encode(result.into_bytes());

        Ok(signature)
    }

    /// Sends a signed request to the Bitget API
    ///
    /// This method automatically handles timestamp generation and request signing for private endpoints.
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path (e.g., "/api/v2/spot/account/assets")
    /// * `method` - The HTTP method to use  
    /// * `query_string` - Optional query string parameters
    /// * `body` - Optional JSON body for POST requests
    /// * `endpoint_limit_per_second` - The rate limit for this specific endpoint
    /// * `is_order` - Whether this is an order-related endpoint
    /// * `order_limit_per_second` - Order-specific rate limit if applicable
    ///
    /// # Returns
    /// A result containing the parsed response data and metadata, or an error
    pub(super) async fn send_signed_request<T>(
        &self,
        endpoint: &str,
        method: reqwest::Method,
        query_string: Option<&str>,
        body: Option<&str>,
        endpoint_limit_per_second: u32,
        is_order: bool,
        order_limit_per_second: Option<u32>,
    ) -> RestResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        // Check rate limits before making the request
        self.rate_limiter
            .check_limits(endpoint_limit_per_second, is_order, order_limit_per_second)
            .await?;

        // Generate timestamp
        let timestamp = Utc::now().timestamp_millis();

        // Generate signature
        let signature = self.generate_signature(timestamp, method.as_str(), endpoint, query_string, body)?;

        // Build URL
        let url = match query_string {
            Some(q) if !q.is_empty() => format!("{}{}?{}", self.base_url, endpoint, q),
            _ => format!("{}{}", self.base_url, endpoint),
        };

        // Build request
        let mut request_builder = self
            .client
            .request(method, &url)
            .header("ACCESS-KEY", self.api_key.expose_secret())
            .header("ACCESS-SIGN", signature)
            .header("ACCESS-TIMESTAMP", timestamp.to_string())
            .header("ACCESS-PASSPHRASE", self.api_passphrase.expose_secret())
            .header("locale", "en-US");

        // Add body if provided
        if let Some(body_content) = body {
            request_builder = request_builder
                .header("Content-Type", "application/json")
                .body(body_content.to_owned());
        }

        // Execute request
        let start_time = std::time::Instant::now();
        let response = request_builder.send().await.map_err(Errors::HttpError)?;
        let _request_duration = start_time.elapsed();

        // Update rate limiter counters
        self.rate_limiter.increment_request().await;
        if is_order {
            self.rate_limiter.increment_order().await;
        }

        // Handle HTTP status codes
        let status = response.status();
        let response_text = response.text().await.map_err(Errors::HttpError)?;

        // Parse response
        if status.is_success() {
            // Try to parse as successful response
            match serde_json::from_str::<BitgetResponse<T>>(&response_text) {
                Ok(bitget_response) => {
                    if bitget_response.code == "00000" {
                        Ok(crate::bitget::RestResponse {
                            data: bitget_response.data,
                            headers: crate::bitget::ResponseHeaders::default(),
                        })
                    } else {
                        // API returned an error in successful HTTP response
                        let error_response = crate::bitget::ErrorResponse {
                            code: bitget_response.code,
                            msg: bitget_response.msg,
                        };
                        Err(Errors::ApiError(error_response.into()))
                    }
                }
                Err(_) => {
                    // Failed to parse as success, treat as error
                    Err(Errors::Error(format!(
                        "Failed to parse response: {response_text}"
                    )))
                }
            }
        } else {
            // Handle HTTP error status codes
            match status.as_u16() {
                429 => {
                    Err(Errors::ApiError(
                        crate::bitget::errors::ApiError::RateLimitExceeded {
                            msg: format!("Rate limit exceeded: {response_text}"),
                            retry_after: None, // Bitget doesn't provide Retry-After header
                        },
                    ))
                }
                403 => Err(Errors::ApiError(
                    crate::bitget::errors::ApiError::Forbidden {
                        msg: format!("Forbidden: {response_text}"),
                    },
                )),
                408 => Err(Errors::ApiError(
                    crate::bitget::errors::ApiError::RequestTimeout {
                        msg: format!("Request timeout: {response_text}"),
                    },
                )),
                500..=599 => Err(Errors::ApiError(
                    crate::bitget::errors::ApiError::InternalServerError {
                        msg: format!("Server error: {response_text}"),
                    },
                )),
                _ => {
                    // Try to parse as error response
                    match serde_json::from_str::<crate::bitget::ErrorResponse>(&response_text) {
                        Ok(error_response) => Err(Errors::ApiError(error_response.into())),
                        Err(_) => Err(Errors::Error(format!(
                            "HTTP {} error: {}",
                            status.as_u16(),
                            response_text
                        ))),
                    }
                }
            }
        }
    }
}

/// Represents a successful response from the Bitget API
#[derive(Debug, serde::Deserialize)]
struct BitgetResponse<T> {
    code: String,
    msg: String,
    #[serde(rename = "requestTime")]
    request_time: i64,
    data: T,
}
