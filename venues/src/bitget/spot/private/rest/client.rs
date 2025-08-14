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

use std::{borrow::Cow, sync::Arc};

use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
use chrono::Utc;
use hmac::{Hmac, Mac};
use rest::{
    HttpClient,
    http_client::{Method as HttpMethod, RequestBuilder},
    secrets::ExposableSecret,
};
use serde::Serialize;
use sha2::Sha256;

use super::credentials::Credentials;
use crate::bitget::spot::{Errors, RestResult, rate_limit::RateLimiter};

/// A client for interacting with the Bitget private REST API
///
/// This client handles encrypted API keys and secrets for enhanced security.
/// The API key, secret, and passphrase are stored in encrypted form and only decrypted when needed.
#[non_exhaustive]
pub struct RestClient {
    /// The underlying HTTP client used for making requests.
    pub(crate) http_client: Arc<dyn HttpClient>,
    /// The rate limiter for this client.
    pub(crate) rate_limiter: RateLimiter,
    /// The credentials for authenticating with Bitget API.
    pub(crate) credentials: Credentials,
    /// The base URL for the API.
    pub(crate) base_url: Cow<'static, str>,
}

impl RestClient {
    /// Creates a new RestClient with encrypted API credentials
    ///
    /// # Arguments
    /// * `credentials` - The credentials for authentication
    /// * `base_url` - The base URL for the API
    /// * `rate_limiter` - The rate limiter instance
    /// * `http_client` - The HTTP client to use
    ///
    /// # Returns
    /// A new RestClient instance
    pub fn new(
        credentials: Credentials,
        base_url: impl Into<Cow<'static, str>>,
        rate_limiter: RateLimiter,
        http_client: Arc<dyn HttpClient>,
    ) -> Self {
        Self {
            http_client,
            rate_limiter,
            credentials,
            base_url: base_url.into(),
        }
    }

    /// Generates the signature for Bitget API authentication
    ///
    /// The signature is created by:
    /// 1. Creating the string: timestamp + method + requestPath + queryString + body
    /// 2. HMAC SHA256 with the API secret
    /// 3. Base64 encoding the result
    fn generate_signature(
        &self,
        timestamp: i64,
        method: &str,
        request_path: &str,
        query_string: Option<&str>,
        body: Option<&str>,
    ) -> Result<String, Errors> {
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
            Hmac::<Sha256>::new_from_slice(self.credentials.api_secret.expose_secret().as_bytes())
                .map_err(|e| Errors::Error(format!("Failed to create HMAC: {e}")))?;

        mac.update(sign_string.as_bytes());
        let result = mac.finalize();
        let signature = BASE64.encode(result.into_bytes());

        Ok(signature)
    }

    /// Sends a signed request to the Bitget API
    ///
    /// This method automatically handles timestamp generation, parameter serialization, and request signing for private endpoints.
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path (e.g., "/api/v2/spot/account/assets")
    /// * `method` - The HTTP method to use  
    /// * `query_params` - Optional query parameters to serialize
    /// * `body_params` - Optional body parameters to serialize as JSON
    /// * `endpoint_limit_per_second` - The rate limit for this specific endpoint
    /// * `is_order` - Whether this is an order-related endpoint
    /// * `order_limit_per_second` - Order-specific rate limit if applicable
    ///
    /// # Returns
    /// A result containing the parsed response data and metadata, or an error
    pub(super) async fn send_signed_request<T, Q, B>(
        &self,
        endpoint: &str,
        method: HttpMethod,
        query_params: Option<&Q>,
        body_params: Option<&B>,
        endpoint_limit_per_second: u32,
        is_order: bool,
        order_limit_per_second: Option<u32>,
    ) -> RestResult<T>
    where
        T: serde::de::DeserializeOwned,
        Q: Serialize + ?Sized,
        B: Serialize + ?Sized,
    {
        // Check rate limits before making the request
        self.rate_limiter
            .check_limits(endpoint_limit_per_second, is_order, order_limit_per_second)
            .await?;

        // Serialize query parameters if provided
        let query_string = if let Some(params) = query_params {
            let serialized = serde_urlencoded::to_string(params)
                .map_err(|e| Errors::Error(format!("Failed to serialize query parameters: {e}")))?;
            if serialized.is_empty() {
                None
            } else {
                Some(serialized)
            }
        } else {
            None
        };

        // Serialize body parameters if provided
        let body_string = if let Some(params) = body_params {
            let serialized = serde_json::to_string(params)
                .map_err(|e| Errors::Error(format!("Failed to serialize body parameters: {e}")))?;
            Some(serialized)
        } else {
            None
        };

        // Generate timestamp
        let timestamp = Utc::now().timestamp_millis();

        // Generate signature
        let method_str = match method {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Head => "HEAD",
            HttpMethod::Options => "OPTIONS",
        };
        let signature = self.generate_signature(
            timestamp,
            method_str,
            endpoint,
            query_string.as_deref(),
            body_string.as_deref(),
        )?;

        // Build URL
        let url = match &query_string {
            Some(q) if !q.is_empty() => format!("{}{}?{}", self.base_url, endpoint, q),
            _ => format!("{}{}", self.base_url, endpoint),
        };

        // Build request
        let mut builder = RequestBuilder::new(method, url)
            .header("ACCESS-KEY", self.credentials.api_key.expose_secret())
            .header("ACCESS-SIGN", &signature)
            .header("ACCESS-TIMESTAMP", timestamp.to_string())
            .header(
                "ACCESS-PASSPHRASE",
                self.credentials.api_passphrase.expose_secret(),
            )
            .header("locale", "en-US");

        // Add body if provided
        if let Some(body_content) = body_string {
            builder = builder
                .header("Content-Type", "application/json")
                .body(body_content.into_bytes());
        }

        // Execute request
        let start_time = std::time::Instant::now();
        let request = builder.build();
        let response = self
            .http_client
            .execute(request)
            .await
            .map_err(|e| Errors::HttpError(format!("HTTP request failed: {e}")))?;
        let _request_duration = start_time.elapsed();

        // Update rate limiter counters
        self.rate_limiter.increment_request().await;
        if is_order {
            self.rate_limiter.increment_order().await;
        }

        // Handle HTTP status codes
        let status = response.status;
        let response_text = response
            .text()
            .map_err(|e| Errors::HttpError(format!("Failed to read response: {e}")))?;

        // Parse response
        if status == 200 || status == 201 {
            // Try to parse as successful response
            match serde_json::from_str::<BitgetResponse<T>>(&response_text) {
                Ok(bitget_response) => {
                    if bitget_response.code == "00000" {
                        Ok(crate::bitget::spot::RestResponse {
                            data: bitget_response.data,
                            headers: crate::bitget::spot::ResponseHeaders::default(),
                        })
                    } else {
                        // API returned an error in successful HTTP response
                        let error_response = crate::bitget::spot::ErrorResponse {
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
            match status {
                429 => {
                    Err(Errors::ApiError(
                        crate::bitget::spot::errors::ApiError::RateLimitExceeded {
                            msg: format!("Rate limit exceeded: {response_text}"),
                            retry_after: None, // Bitget doesn't provide Retry-After header
                        },
                    ))
                }
                403 => Err(Errors::ApiError(
                    crate::bitget::spot::errors::ApiError::Forbidden {
                        msg: format!("Forbidden: {response_text}"),
                    },
                )),
                408 => Err(Errors::ApiError(
                    crate::bitget::spot::errors::ApiError::RequestTimeout {
                        msg: format!("Request timeout: {response_text}"),
                    },
                )),
                500..=599 => Err(Errors::ApiError(
                    crate::bitget::spot::errors::ApiError::InternalServerError {
                        msg: format!("Server error: {response_text}"),
                    },
                )),
                _ => {
                    // Try to parse as error response
                    match serde_json::from_str::<crate::bitget::spot::ErrorResponse>(&response_text)
                    {
                        Ok(error_response) => Err(Errors::ApiError(error_response.into())),
                        Err(_) => Err(Errors::Error(format!(
                            "HTTP {} error: {}",
                            status, response_text
                        ))),
                    }
                }
            }
        }
    }

    /// Convenience method for GET requests with query parameters only
    pub(super) async fn send_get_signed_request<T, Q>(
        &self,
        endpoint: &str,
        params: Q,
        endpoint_limit_per_second: u32,
        is_order: bool,
        order_limit_per_second: Option<u32>,
    ) -> RestResult<T>
    where
        T: serde::de::DeserializeOwned,
        Q: Serialize,
    {
        self.send_signed_request(
            endpoint,
            HttpMethod::Get,
            Some(&params),
            None::<&()>,
            endpoint_limit_per_second,
            is_order,
            order_limit_per_second,
        )
        .await
    }

    /// Convenience method for POST requests with body parameters only
    pub(super) async fn send_post_signed_request<T, B>(
        &self,
        endpoint: &str,
        params: B,
        endpoint_limit_per_second: u32,
        is_order: bool,
        order_limit_per_second: Option<u32>,
    ) -> RestResult<T>
    where
        T: serde::de::DeserializeOwned,
        B: Serialize,
    {
        self.send_signed_request(
            endpoint,
            HttpMethod::Post,
            None::<&()>,
            Some(&params),
            endpoint_limit_per_second,
            is_order,
            order_limit_per_second,
        )
        .await
    }

    /// Convenience method for GET requests with no parameters
    pub(super) async fn send_get_signed_request_no_params<T>(
        &self,
        endpoint: &str,
        endpoint_limit_per_second: u32,
        is_order: bool,
        order_limit_per_second: Option<u32>,
    ) -> RestResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        self.send_signed_request(
            endpoint,
            HttpMethod::Get,
            None::<&()>,
            None::<&()>,
            endpoint_limit_per_second,
            is_order,
            order_limit_per_second,
        )
        .await
    }
}

/// Represents a successful response from the Bitget API
#[derive(Debug, serde::Deserialize)]
struct BitgetResponse<T> {
    code: String,
    msg: String,
    #[serde(rename = "requestTime")]
    _request_time: i64,
    data: T,
}
