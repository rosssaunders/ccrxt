// REST client for OKX public endpoints.
//
// Provides access to all public REST API endpoints for OKX Exchange.
// All requests are unauthenticated and do not require API credentials.
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::borrow::Cow;

use crate::okx::{EndpointType, Errors, RateLimiter, RestResult};

/// Public REST client for OKX exchange
///
/// This client handles all public API endpoints that don't require authentication.
/// It provides automatic rate limiting and error handling.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct RestClient {
    /// The base URL for the OKX public REST API (e.g., "https://www.okx.com")
    ///
    /// This is used as the prefix for all endpoint requests.
    pub base_url: Cow<'static, str>,

    /// The underlying HTTP client used for making requests.
    ///
    /// This is reused for connection pooling and performance.
    pub client: Client,

    /// The rate limiter used to manage request rates and prevent hitting API limits.
    ///
    /// This is used to ensure compliance with OKX's rate limits for public endpoints.
    pub rate_limiter: RateLimiter,
}

impl RestClient {
    /// Creates a new OKX public REST client.
    ///
    /// # Arguments
    /// * `base_url` - The base URL for the OKX public REST API (e.g., "https://www.okx.com")
    /// * `client` - The HTTP client to use for requests
    /// * `rate_limiter` - The rate limiter for managing API limits
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

    /// Send a request to a public endpoint
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path (e.g., "api/v5/public/instruments")
    /// * `method` - The HTTP method to use
    /// * `params` - Optional struct of query/body parameters (must implement Serialize)
    /// * `endpoint_type` - The endpoint type for rate limiting
    ///
    /// # Returns
    /// A result containing the response data or an error
    pub async fn send_request<T, P>(
        &self,
        endpoint: &str,
        method: reqwest::Method,
        params: Option<&P>,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
        P: serde::Serialize + ?Sized,
    {
        // Check rate limits before making the request
        self.rate_limiter
            .check_limits(endpoint_type.clone())
            .await
            .map_err(|e| Errors::Error(e.to_string()))?;

        // Build the URL
        let url = if endpoint.starts_with("http") {
            endpoint.to_string()
        } else {
            format!("{}/{}", self.base_url, endpoint)
        };

        // Build the request
        let mut request_builder = self.client.request(method.clone(), &url);

        // Add parameters based on method
        if let Some(params) = params {
            let params_value = serde_json::to_value(params)
                .map_err(|e| Errors::Error(format!("Failed to serialize params: {}", e)))?;
            if method == reqwest::Method::GET {
                // For GET requests, add parameters as query string
                if let Some(params_obj) = params_value.as_object() {
                    for (key, value) in params_obj {
                        let value_str = match value {
                            serde_json::Value::String(s) => s.clone(),
                            serde_json::Value::Number(n) => n.to_string(),
                            serde_json::Value::Bool(b) => b.to_string(),
                            _ => value.to_string(),
                        };
                        request_builder = request_builder.query(&[(key, value_str)]);
                    }
                }
            } else {
                // For POST requests, add parameters as JSON body
                request_builder = request_builder.json(&params_value);
            }
        }

        // Add required headers
        request_builder = request_builder.header("Content-Type", "application/json");

        // Send the request
        let response = request_builder.send().await.map_err(Errors::HttpError)?;

        // Increment rate limiter counter after successful request
        self.rate_limiter.increment_request(endpoint_type).await;

        // Check if the response was successful
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.map_err(Errors::HttpError)?;
            return Err(Errors::Error(format!("HTTP {}: {}", status, error_text)));
        }

        // Parse the response
        let response_text = response.text().await.map_err(Errors::HttpError)?;

        let parsed_response: T = serde_json::from_str(&response_text)
            .map_err(|e| Errors::Error(format!("Failed to parse response: {}", e)))?;

        Ok(parsed_response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_client_creation() {
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new();

        let rest_client = RestClient::new("https://www.okx.com", client, rate_limiter);

        assert_eq!(rest_client.base_url, "https://www.okx.com");
    }

    #[test]
    fn test_url_building() {
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new();

        let rest_client = RestClient::new("https://www.okx.com", client, rate_limiter);

        // Test that the client is properly initialized
        assert_eq!(rest_client.base_url, "https://www.okx.com");
    }

    #[tokio::test]
    async fn test_rate_limiting_integration() {
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new();

        let rest_client = RestClient::new("https://www.okx.com", client, rate_limiter);

        // Test that rate limiting works (this shouldn't fail since we're not actually hitting limits)
        let result = rest_client
            .rate_limiter
            .check_limits(EndpointType::PublicMarketData)
            .await;
        assert!(result.is_ok());
    }
}
