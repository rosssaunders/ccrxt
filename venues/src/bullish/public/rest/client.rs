//! Bullish Public REST API client

use crate::bullish::{EndpointType, RateLimiter, RestResult};
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::borrow::Cow;

/// Public REST client for Bullish exchange
///
/// This client handles all public API endpoints that don't require authentication.
/// It provides automatic rate limiting and error handling.
pub struct RestClient {
    /// The underlying HTTP client used for making requests
    pub(crate) client: Client,
    /// The base URL for the API
    pub(crate) base_url: Cow<'static, str>,
    /// Rate limiter for API requests
    pub(crate) rate_limiter: RateLimiter,
}

impl RestClient {
    /// Creates a new public RestClient
    ///
    /// # Arguments
    /// * `base_url` - The base URL for the API
    /// * `client` - The HTTP client to use
    /// * `rate_limiter` - Rate limiter for requests
    ///
    /// # Returns
    /// A new RestClient instance
    pub fn new(base_url: impl Into<Cow<'static, str>>, client: Client, rate_limiter: RateLimiter) -> Self {
        Self {
            client,
            base_url: base_url.into(),
            rate_limiter,
        }
    }

    /// Send a public request to the Bullish API
    ///
    /// This method handles rate limiting and error handling for public endpoints.
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path
    /// * `endpoint_type` - The endpoint type for rate limiting
    ///
    /// # Returns
    /// The deserialized response or an error
    pub async fn send_request<T>(&self, endpoint: &str, endpoint_type: EndpointType) -> RestResult<T>
    where
        T: DeserializeOwned,
    {
        // Check rate limits
        self.rate_limiter
            .check_limits(endpoint_type)
            .await
            .map_err(|e| crate::bullish::Errors::RateLimitError(e.to_string()))?;

        let url = format!("{}{}", self.base_url, endpoint);

        let response = self.client.get(&url).send().await?;

        self.rate_limiter.increment_request(endpoint_type).await;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(crate::bullish::Errors::Error(format!(
                "Request failed: {}",
                error_text
            )));
        }

        let result: T = response.json().await?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_client_creation() {
        let client = Client::new();
        let rate_limiter = RateLimiter::new();

        let rest_client = RestClient::new("https://api.exchange.bullish.com", client, rate_limiter);

        assert_eq!(rest_client.base_url, "https://api.exchange.bullish.com");
    }

    #[tokio::test]
    async fn test_rate_limiting_integration() {
        let client = Client::new();
        let rate_limiter = RateLimiter::new();

        let _rest_client = RestClient::new("https://api.exchange.bullish.com", client, rate_limiter);

        // Test that rate limiter integration works
        // This is a basic structure test since we can't make real API calls in unit tests
    }
}
