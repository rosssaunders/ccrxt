//! BitMart Public API request handling module.
//!
//! This module provides functionality for making HTTP requests to the BitMart public API.
//! It handles rate limiting, error responses, and request/response timing.
//!
//! ## BitMart Exchange Behavior
//!
//! The BitMart API has specific behaviors that this module handles:
//!
//! - **Error Format**: BitMart returns errors in HTTP 200 OK responses with error details in the JSON
//! - **No Authentication**: Public endpoints don't require authentication headers
//! - **Rate Limiting**: BitMart public endpoints have specific rate limits per IP
//! - **Base URL**: Uses https://api-cloud.bitmart.com for public endpoints

use std::borrow::Cow;

use reqwest::Client;
use serde::{Deserialize, de::DeserializeOwned};

use crate::bitmart::{
    Errors, RestResult,
    rate_limit::{EndpointType, RateLimiter},
};

/// BitMart public REST client
pub struct RestClient {
    /// The base URL for the BitMart public REST API
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
    /// Create a new BitMart public REST client
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL for the BitMart public API (e.g., "https://api-cloud.bitmart.com")
    /// * `client` - HTTP client for making requests
    /// * `rate_limiter` - Rate limiter for managing API limits
    pub fn new(
        base_url: impl Into<Cow<'static, str>>,
        client: Client,
        rate_limiter: RateLimiter,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            client,
            rate_limiter,
        }
    }

    /// Send a GET request to the BitMart public API
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The API endpoint (e.g., "/spot/v1/currencies")
    /// * `request` - Optional request parameters (will be serialized as query parameters)
    /// * `endpoint_type` - The type of endpoint for rate limiting
    ///
    /// # Returns
    ///
    /// The parsed response data
    pub async fn send_get_request<R, T>(
        &self,
        endpoint: &str,
        request: Option<&R>,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        R: serde::Serialize + ?Sized,
        T: DeserializeOwned,
    {
        // Check rate limits
        self.rate_limiter
            .check_limits(endpoint_type.clone())
            .await?;

        // Build URL
        let url = format!("{}{}", self.base_url, endpoint);

        // Build URL with query parameters
        let final_url = if let Some(req) = request {
            let query_params = serde_urlencoded::to_string(req).map_err(|e| {
                Errors::Error(format!("Failed to serialize query parameters: {e}"))
            })?;

            if query_params.is_empty() {
                url
            } else {
                format!("{url}?{query_params}")
            }
        } else {
            url
        };

        // Build request
        let request_builder = self.client.get(&final_url);

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
            let error_response = crate::bitmart::ErrorResponse {
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
    use super::*;

    #[test]
    fn test_public_client_creation() {
        let client = Client::new();
        let rate_limiter = RateLimiter::new();

        let rest_client = RestClient::new("https://api-cloud.bitmart.com", client, rate_limiter);

        assert_eq!(rest_client.base_url, "https://api-cloud.bitmart.com");
    }
}
