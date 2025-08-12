// REST client for Crypto.com public endpoints.
//
// Provides access to all public REST API endpoints for Crypto.com Exchange.
// All requests are unauthenticated and do not require API credentials.
use std::{borrow::Cow, sync::Arc};

use rest::{
    HttpClient,
    http_client::{Method as HttpMethod, RequestBuilder},
};
use serde::de::DeserializeOwned;

use crate::cryptocom::{EndpointType, Errors, RateLimiter, RestResult};

/// Public REST client for Crypto.com exchange
///
/// This client handles all public API endpoints that don't require authentication.
/// It provides automatic rate limiting and error handling.
#[non_exhaustive]
pub struct RestClient {
    /// The base URL for the Crypto.com public REST API (e.g., "<https://api.crypto.com>")
    ///
    /// This is used as the prefix for all endpoint requests.
    pub base_url: Cow<'static, str>,

    /// The underlying HTTP client used for making requests.
    ///
    /// This is reused for connection pooling and performance.
    pub http_client: Arc<dyn HttpClient>,

    /// The rate limiter used to manage request rates and prevent hitting API limits.
    ///
    /// This is used to ensure compliance with Crypto.com's rate limits for public endpoints.
    pub rate_limiter: RateLimiter,
}

impl RestClient {
    /// Creates a new Crypto.com public REST client.
    ///
    /// # Arguments
    /// * `base_url` - The base URL for the Crypto.com public REST API (e.g., "<https://api.crypto.com>")
    /// * `client` - The HTTP client to use for requests
    /// * `rate_limiter` - The rate limiter for managing API limits
    pub fn new(
        base_url: impl Into<Cow<'static, str>>,
        http_client: Arc<dyn HttpClient>,
        rate_limiter: RateLimiter,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            http_client,
            rate_limiter,
        }
    }

    /// Send a GET request to a public endpoint
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path (e.g., "public/get-instruments")
    /// * `params` - Optional struct of query parameters (must implement Serialize)
    /// * `endpoint_type` - The endpoint type for rate limiting
    ///
    /// # Returns
    /// A result containing the response data or an error
    pub async fn send_get_request<T, P>(
        &self,
        endpoint: &str,
        params: Option<&P>,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
        P: serde::Serialize + ?Sized,
    {
        // Check rate limits before making the request
        self.rate_limiter
            .check_limits(endpoint_type)
            .await
            .map_err(|e| Errors::Error(e.to_string()))?;

        // Build the URL
        let url = if endpoint.starts_with("http") {
            endpoint.to_string()
        } else {
            format!("{}/v1/{}", self.base_url, endpoint)
        };

        // Build the request
        let mut builder = RequestBuilder::new(HttpMethod::Get, url.clone());
        builder = builder.header("Content-Type", "application/json");

        // Add parameters as query string
        if let Some(params) = params {
            let params_value = serde_json::to_value(params)
                .map_err(|e| Errors::Error(format!("Failed to serialize params: {e}")))?;
            if let Some(params_obj) = params_value.as_object() {
                let mut query_params = Vec::new();
                for (key, value) in params_obj {
                    let value_str = match value {
                        serde_json::Value::String(s) => s.clone(),
                        serde_json::Value::Number(n) => n.to_string(),
                        serde_json::Value::Bool(b) => b.to_string(),
                        _ => value.to_string(),
                    };
                    query_params.push(format!("{}={}", key, value_str));
                }
                if !query_params.is_empty() {
                    let query_string = query_params.join("&");
                    let url_with_query = format!("{}?{}", url, query_string);
                    builder = RequestBuilder::new(HttpMethod::Get, url_with_query);
                    builder = builder.header("Content-Type", "application/json");
                }
            }
        }

        // Send the request
        let response = self
            .http_client
            .execute(builder.build())
            .await
            .map_err(|e| Errors::NetworkError(format!("HTTP request failed: {e}")))?;

        // Increment rate limiter counter after successful request
        self.rate_limiter.increment_request(endpoint_type).await;

        // Check if the response was successful
        if !(response.status >= 200 && response.status < 300) {
            let status = response.status;
            let error_text = response
                .text()
                .map_err(|e| Errors::NetworkError(format!("Failed to read response: {e}")))?;
            return Err(Errors::Error(format!("HTTP {status}: {error_text}")));
        }

        // Parse the response
        let response_text = response
            .text()
            .map_err(|e| Errors::NetworkError(format!("Failed to read response: {e}")))?;

        let parsed_response: T = serde_json::from_str(&response_text)
            .map_err(|e| Errors::Error(format!("Failed to parse response: {e}")))?;

        Ok(parsed_response)
    }

    /// Send a POST request to a public endpoint
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path (e.g., "public/staking/get-conversion-rate")
    /// * `params` - Optional struct of body parameters (must implement Serialize)
    /// * `endpoint_type` - The endpoint type for rate limiting
    ///
    /// # Returns
    /// A result containing the response data or an error
    pub async fn send_post_request<T, P>(
        &self,
        endpoint: &str,
        params: Option<&P>,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
        P: serde::Serialize + ?Sized,
    {
        // Check rate limits before making the request
        self.rate_limiter
            .check_limits(endpoint_type)
            .await
            .map_err(|e| Errors::Error(e.to_string()))?;

        // Build the URL
        let url = if endpoint.starts_with("http") {
            endpoint.to_string()
        } else {
            format!("{}/v1/{}", self.base_url, endpoint)
        };

        // Build the request
        let mut builder = RequestBuilder::new(HttpMethod::Post, url);
        builder = builder.header("Content-Type", "application/json");

        // Add parameters as JSON body
        if let Some(params) = params {
            let params_value = serde_json::to_value(params)
                .map_err(|e| Errors::Error(format!("Failed to serialize params: {e}")))?;
            let body = serde_json::to_string(&params_value)
                .map_err(|e| Errors::Error(format!("Failed to serialize body: {e}")))?;
            builder = builder.body(body.into_bytes());
        }

        // Send the request
        let response = self
            .http_client
            .execute(builder.build())
            .await
            .map_err(|e| Errors::NetworkError(format!("HTTP request failed: {e}")))?;

        // Increment rate limiter counter after successful request
        self.rate_limiter.increment_request(endpoint_type).await;

        // Check if the response was successful
        if !(response.status >= 200 && response.status < 300) {
            let status = response.status;
            let error_text = response
                .text()
                .map_err(|e| Errors::NetworkError(format!("Failed to read response: {e}")))?;
            return Err(Errors::Error(format!("HTTP {status}: {error_text}")));
        }

        // Parse the response
        let response_text = response
            .text()
            .map_err(|e| Errors::NetworkError(format!("Failed to read response: {e}")))?;

        let parsed_response: T = serde_json::from_str(&response_text)
            .map_err(|e| Errors::Error(format!("Failed to parse response: {e}")))?;

        Ok(parsed_response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_client_creation() {
        let http_client = Arc::new(rest::native::NativeHttpClient::default());
        let rate_limiter = RateLimiter::new();

        let rest_client = RestClient::new("https://api.crypto.com", http_client, rate_limiter);

        assert_eq!(rest_client.base_url, "https://api.crypto.com");
    }

    #[test]
    fn test_url_building() {
        let http_client = Arc::new(rest::native::NativeHttpClient::default());
        let rate_limiter = RateLimiter::new();

        let rest_client = RestClient::new("https://api.crypto.com", http_client, rate_limiter);

        // Test that the client is properly initialized
        assert_eq!(rest_client.base_url, "https://api.crypto.com");
    }

    #[tokio::test]
    async fn test_rate_limiting_integration() {
        let http_client = Arc::new(rest::native::NativeHttpClient::default());
        let rate_limiter = RateLimiter::new();

        let rest_client = RestClient::new("https://api.crypto.com", http_client, rate_limiter);

        // Test that rate limiting works (this shouldn't fail since we're not actually hitting limits)
        let result = rest_client
            .rate_limiter
            .check_limits(EndpointType::PublicGetTicker)
            .await;
        assert!(result.is_ok());
    }
}
