//! Binance Spot Public API REST client
//!
//! This module provides a client for making requests to the Binance Spot public API endpoints.
//! Public endpoints do not require authentication and are used for market data and exchange information.

use std::borrow::Cow;

use reqwest::Client;

use crate::binance::spot::{
    RateLimiter, RestResult,
    rest::common::{build_url, send_rest_request},
};

/// A client for interacting with the Binance Spot public REST API
///
/// This client handles public endpoints that do not require authentication.
/// It includes rate limiting and error handling for all public API calls.
#[derive(Clone)]
pub struct RestClient {
    /// The base URL for the API
    pub base_url: Cow<'static, str>,
    /// The underlying HTTP client used for making requests
    pub client: Client,
    /// The rate limiter for this client
    pub rate_limiter: RateLimiter,
}

impl RestClient {
    /// Creates a new public RestClient
    ///
    /// # Arguments
    /// * `base_url` - The base URL for the API (typically "https://api.binance.com")
    /// * `client` - The HTTP client to use for requests
    /// * `rate_limiter` - The rate limiter instance
    ///
    /// # Returns
    /// A new RestClient instance for public API access
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

    /// Sends a request to the Binance Spot public API
    ///
    /// This method handles all the logic for making public requests to the Binance API,
    /// including rate limiting, error handling, and response parsing.
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path (e.g., "/api/v3/ping")
    /// * `method` - The HTTP method to use
    /// * `query_string` - Optional query string parameters
    /// * `weight` - The request weight for this endpoint
    ///
    /// # Returns
    /// A result containing the parsed response data and metadata, or an error
    pub(super) async fn send_request<T>(
        &self,
        endpoint: &str,
        method: reqwest::Method,
        query_string: Option<&str>,
        weight: u32,
    ) -> RestResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = build_url(&self.base_url, endpoint, query_string)?;

        let rest_response = send_rest_request(
            &self.client,
            &url,
            method,
            vec![], // No authentication headers for public endpoints
            None,   // No body for public endpoints
            &self.rate_limiter,
            weight,
            false, // Public endpoints are not order endpoints
        )
        .await?;

        Ok(crate::binance::spot::RestResponse {
            data: rest_response.data,
            headers: rest_response.headers,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_client_creation() {
        let client = Client::new();
        let rate_limiter = RateLimiter::new();

        let rest_client = RestClient::new("https://api.binance.com", client, rate_limiter);

        assert_eq!(rest_client.base_url, "https://api.binance.com");
    }
}
