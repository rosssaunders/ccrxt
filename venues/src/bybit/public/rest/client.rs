// REST client for ByBit V5 public endpoints.
//
// Provides access to all public REST API endpoints for ByBit Exchange.
// These endpoints do not require authentication.

use std::{borrow::Cow, sync::Arc};

use rest::{
    HttpClient,
    http_client::{Method as HttpMethod, RequestBuilder},
};
use serde::de::DeserializeOwned;

use crate::bybit::{EndpointType, Errors, RateLimiter, RestResult};

/// Public REST client for ByBit V5 exchange
///
/// This client handles all public API endpoints that don't require authentication.
/// It provides automatic rate limiting and error handling.
pub struct RestClient {
    /// The base URL for the ByBit V5 public REST API (e.g., "https://api.bybit.com")
    ///
    /// This is used as the prefix for all endpoint requests.
    pub base_url: Cow<'static, str>,

    /// The underlying HTTP client used for making requests.
    ///
    /// This is reused for connection pooling and performance.
    pub http_client: Arc<dyn HttpClient>,

    /// The rate limiter used to manage request rates and prevent hitting API limits.
    ///
    /// This is used to ensure compliance with ByBit's rate limits for public endpoints.
    pub rate_limiter: RateLimiter,
}

impl RestClient {
    /// Creates a new RestClient
    ///
    /// # Arguments
    /// * `base_url` - The base URL for the API
    /// * `rate_limiter` - The rate limiter instance
    /// * `http_client` - The HTTP client instance
    ///
    /// # Returns
    /// A new RestClient instance
    pub fn new(
        base_url: impl Into<Cow<'static, str>>,
        rate_limiter: RateLimiter,
        http_client: Arc<dyn HttpClient>,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            rate_limiter,
            http_client,
        }
    }

    /// Sends a public request to the ByBit V5 API
    ///
    /// This method handles public endpoints that don't require authentication.
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path (e.g., "/v5/market/time")
    /// * `query_params` - Optional query parameters as a serializable struct
    /// * `endpoint_type` - The endpoint type for rate limiting
    ///
    /// # Returns
    /// A result containing the parsed response data, or an error
    pub(super) async fn send_public_request<T, Q>(
        &self,
        endpoint: &str,
        query_params: Option<Q>,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
        Q: serde::Serialize,
    {
        // Check rate limits before making request
        self.rate_limiter
            .check_limits(endpoint_type.clone())
            .await?;

        // Build the URL with query parameters
        let mut url = format!("{}{}", self.base_url, endpoint);
        if let Some(params) = query_params {
            let query_string = serde_urlencoded::to_string(&params)?;
            if !query_string.is_empty() {
                url.push('?');
                url.push_str(&query_string);
            }
        }

        // Build request
        let request = RequestBuilder::new(HttpMethod::Get, url).build();

        // Send the request
        let response = self.http_client.execute(request).await
            .map_err(|e| Errors::NetworkError(format!("HTTP request failed: {e}")))?;

        // Record the request for rate limiting
        self.rate_limiter.increment_request(endpoint_type).await;

        // Check for HTTP errors
        let status = response.status;
        if status != 200 && status != 201 {
            let error_text = response
                .text()
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(Errors::ApiError(format!("HTTP {status}: {error_text}")));
        }

        // Parse the response
        let response_text = response.text()
            .map_err(|e| Errors::NetworkError(format!("Failed to read response: {e}")))?;
        let parsed_response: T = serde_json::from_str(&response_text)?;

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

        let rest_client = RestClient::new("https://api.bybit.com", rate_limiter, http_client);

        assert_eq!(rest_client.base_url, "https://api.bybit.com");
    }
}
