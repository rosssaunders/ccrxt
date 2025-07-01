//! BitMart Private API request handling module.
//!
//! This module provides functionality for making HTTP requests to the BitMart private API.
//! It handles authentication, rate limiting headers, error responses, and request/response timing.
//!
//! ## BitMart Exchange Behavior
//!
//! The BitMart API has specific behaviors that this module handles:
//!
//! - **Error Format**: BitMart returns errors in HTTP 200 OK responses with error details in the JSON
//! - **Authentication**: Uses X-BM-KEY, X-BM-SIGN (HMAC SHA256), and X-BM-TIMESTAMP headers
//! - **Rate Limiting Headers**: BitMart includes rate limiting information in response headers:
//!   - `X-BM-RateLimit-Remaining`: Number of requests used in current window
//!   - `X-BM-RateLimit-Limit`: Max number of requests in current window  
//!   - `X-BM-RateLimit-Reset`: Current time window in seconds

use std::borrow::Cow;

use base64::{Engine as _, engine::general_purpose};
use hmac::{Hmac, Mac};
use reqwest::{Client, Method};
use rest::secrets::ExposableSecret;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use sha2::Sha256;

use crate::bitmart::rate_limit::{EndpointType, RateLimiter};
use crate::bitmart::{Errors, RestResult};

/// BitMart private REST client
pub struct RestClient {
    /// The encrypted API key
    api_key: Box<dyn ExposableSecret>,
    /// The encrypted API secret for signing requests
    api_secret: Box<dyn ExposableSecret>,
    /// The base URL for the BitMart private REST API
    base_url: Cow<'static, str>,
    /// HTTP client for making requests
    client: Client,
    /// Rate limiter for managing API limits
    rate_limiter: RateLimiter,
}

/// Response wrapper for BitMart API responses
#[derive(Debug, Deserialize)]
struct BitMartResponse<T> {
    code: i32,
    message: String,
    #[serde(default)]
    trace: String,
    data: Option<T>,
}

impl RestClient {
    /// Creates a new RestClient with encrypted API credentials
    ///
    /// # Arguments
    /// * `api_key` - The encrypted API key
    /// * `api_secret` - The encrypted API secret
    /// * `base_url` - The base URL for the API
    /// * `client` - The HTTP client to use for requests
    /// * `rate_limiter` - The rate limiter for managing API limits
    ///
    /// # Returns
    /// A new RestClient instance
    pub fn new(
        api_key: Box<dyn ExposableSecret>,
        api_secret: Box<dyn ExposableSecret>,
        base_url: impl Into<Cow<'static, str>>,
        client: Client,
        rate_limiter: RateLimiter,
    ) -> Self {
        Self {
            api_key,
            api_secret,
            base_url: base_url.into(),
            client,
            rate_limiter,
        }
    }

    /// Sign a request for BitMart private endpoints
    ///
    /// Creates the signature according to BitMart's signing algorithm:
    /// 1. Create message string: timestamp + method + requestPath + body
    /// 2. Sign with HMAC SHA256 using the API secret
    /// 3. Encode as Base64
    ///
    /// # Arguments
    /// * `timestamp` - The timestamp string (milliseconds)
    /// * `method` - The HTTP method (uppercase)
    /// * `request_path` - The request path including query parameters
    /// * `body` - The request body (empty string for GET requests)
    ///
    /// # Returns
    /// A result containing the signature as a base64 string or an error
    pub fn sign_request(
        &self,
        timestamp: &str,
        method: &str,
        request_path: &str,
        body: &str,
    ) -> Result<String, Errors> {
        // Create the message string: timestamp + method + requestPath + body
        let message = format!("{timestamp}{method}{request_path}{body}");

        // Sign with HMAC SHA256
        let api_secret = self.api_secret.expose_secret();
        let mut mac = Hmac::<Sha256>::new_from_slice(api_secret.as_bytes())
            .map_err(|_| Errors::InvalidApiKey())?;
        mac.update(message.as_bytes());

        // Encode as Base64
        Ok(general_purpose::STANDARD.encode(mac.finalize().into_bytes()))
    }

    /// Send a request to a private endpoint
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path
    /// * `method` - The HTTP method to use
    /// * `request` - The request object (parameters/body), must implement Serialize
    /// * `endpoint_type` - The endpoint type for rate limiting
    ///
    /// # Returns
    /// A result containing the parsed response data or an error
    pub async fn send_request<R, T>(
        &self,
        endpoint: &str,
        method: Method,
        request: Option<&R>,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        R: serde::Serialize + ?Sized,
        T: DeserializeOwned,
    {
        // Check rate limits before making the request
        self.rate_limiter
            .check_limits(endpoint_type.clone())
            .await?;

        // Serialize request to query params or JSON body
        let (url, body_str, request_path) = match method {
            Method::GET | Method::DELETE => {
                let query = if let Some(req) = request {
                    let s = serde_urlencoded::to_string(req).unwrap_or_default();
                    if s.is_empty() { None } else { Some(s) }
                } else {
                    None
                };
                let url = if let Some(params) = &query {
                    format!("{}{}?{}", self.base_url, endpoint, params)
                } else {
                    format!("{}{}", self.base_url, endpoint)
                };
                let request_path = if let Some(params) = &query {
                    format!("{endpoint}?{params}")
                } else {
                    endpoint.to_string()
                };
                (url, String::new(), request_path)
            }
            _ => {
                let body = if let Some(req) = request {
                    serde_json::to_string(req).unwrap_or_default()
                } else {
                    String::new()
                };
                let url = format!("{}{}", self.base_url, endpoint);
                (url, body, endpoint.to_string())
            }
        };

        // Create timestamp
        let timestamp = chrono::Utc::now().timestamp_millis().to_string();

        // Create signature
        let signature = self.sign_request(&timestamp, method.as_str(), &request_path, &body_str)?;

        // Build request
        let mut request_builder = self
            .client
            .request(method.clone(), &url)
            .header("X-BM-KEY", self.api_key.expose_secret())
            .header("X-BM-SIGN", signature)
            .header("X-BM-TIMESTAMP", timestamp)
            .header("Content-Type", "application/json");

        // Add body if present and not GET/DELETE
        if !(body_str.is_empty() || method == Method::GET || method == Method::DELETE) {
            request_builder = request_builder.body(body_str.clone());
        }

        // Send request
        let response = request_builder.send().await?;

        // Record the request for rate limiting
        self.rate_limiter.increment_request(endpoint_type).await;

        // Parse response
        let response_text = response.text().await?;
        let bitmart_response: BitMartResponse<T> =
            serde_json::from_str(&response_text).map_err(|e| {
                Errors::Error(format!(
                    "Failed to parse response: {e} - Response: {response_text}"
                ))
            })?;

        // Check for API errors
        if bitmart_response.code != 1000 {
            let error_response = crate::bitmart::errors::ErrorResponse {
                code: bitmart_response.code,
                message: bitmart_response.message,
                trace: bitmart_response.trace,
            };
            return Err(Errors::ApiError(error_response.into()));
        }

        // Return data or error if no data
        bitmart_response
            .data
            .ok_or_else(|| Errors::Error("API response missing data field".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use rest::secrets::ExposableSecret;

    use super::*;
    use crate::bitmart::rate_limit::RateLimiter;

    #[derive(Clone)]
    struct TestSecret {
        value: String,
    }

    impl TestSecret {
        fn new(value: String) -> Self {
            Self { value }
        }
    }

    impl ExposableSecret for TestSecret {
        fn expose_secret(&self) -> String {
            self.value.clone()
        }
    }

    #[test]
    fn test_private_client_creation() {
        let api_key = Box::new(TestSecret::new("test_key".to_string())) as Box<dyn ExposableSecret>;
        let api_secret =
            Box::new(TestSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;
        let client = Client::new();
        let rate_limiter = RateLimiter::new();

        let rest_client = RestClient::new(
            api_key,
            api_secret,
            "https://api-cloud.bitmart.com",
            client,
            rate_limiter,
        );

        assert_eq!(rest_client.base_url, "https://api-cloud.bitmart.com");
    }

    #[test]
    fn test_signature_generation() {
        let api_key = Box::new(TestSecret::new("test_key".to_string())) as Box<dyn ExposableSecret>;
        let api_secret =
            Box::new(TestSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;
        let client = Client::new();
        let rate_limiter = RateLimiter::new();

        let rest_client = RestClient::new(
            api_key,
            api_secret,
            "https://api-cloud.bitmart.com",
            client,
            rate_limiter,
        );

        let timestamp = "1609459200000";
        let method = "GET";
        let request_path = "/account/v1/wallet";
        let body = "";

        let signature = rest_client
            .sign_request(timestamp, method, request_path, body)
            .unwrap();

        // Verify the signature is a valid base64 string
        assert!(general_purpose::STANDARD.decode(&signature).is_ok());
    }
}
