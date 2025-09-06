//! Bullish Public REST API client (canonical)
//!
//! This top-level client replaces the former nested `public::rest::client` implementation.
//! It uses the shared `HttpClient` abstraction for WASM compatibility and centralizes
//! rate limiting + error handling.

use std::{borrow::Cow, sync::Arc};

use rest::{HttpClient, Method as HttpMethod, RequestBuilder};
use serde::de::DeserializeOwned;

use crate::bullish::{EndpointType, RateLimiter, RestResult};

/// Public REST client for Bullish exchange
///
/// Handles all public API endpoints that don't require authentication.
/// Applies rate limiting and structured error mapping.
pub struct RestClient {
    /// Abstract HTTP client (reqwest-free API surface)
    pub(crate) http_client: Arc<dyn HttpClient>,
    /// Base URL for API
    pub(crate) base_url: Cow<'static, str>,
    /// Rate limiter
    pub(crate) rate_limiter: RateLimiter,
}

impl RestClient {
    /// Create a new public REST client
    pub fn new(
        base_url: impl Into<Cow<'static, str>>,
        http_client: Arc<dyn HttpClient>,
        rate_limiter: RateLimiter,
    ) -> Self {
        Self {
            http_client,
            base_url: base_url.into(),
            rate_limiter,
        }
    }

    /// Perform a GET request against a public endpoint
    pub async fn send_get_request<T>(
        &self,
        endpoint: &str,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
    {
        self.rate_limiter
            .check_limits(endpoint_type)
            .await
            .map_err(|e| crate::bullish::Errors::RateLimitError(e.to_string()))?;

        let url = format!("{}{}", self.base_url, endpoint);
        let request = RequestBuilder::new(HttpMethod::Get, url.clone())
            .header("Content-Type", "application/json")
            .build();
        let response = self
            .http_client
            .execute(request)
            .await
            .map_err(crate::bullish::Errors::from)?;

        self.rate_limiter.increment_request(endpoint_type).await;

        if !response.is_success() {
            return Err(Self::error_from_response(url, response));
        }

        let result: T = response.json().map_err(crate::bullish::Errors::from)?;
        Ok(result)
    }

    /// Perform a POST request against a public endpoint
    pub async fn send_post_request<T, B>(
        &self,
        endpoint: &str,
        body: Option<&B>,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
        B: serde::Serialize,
    {
        self.rate_limiter
            .check_limits(endpoint_type)
            .await
            .map_err(|e| crate::bullish::Errors::RateLimitError(e.to_string()))?;

        let url = format!("{}{}", self.base_url, endpoint);
        let mut builder = RequestBuilder::new(HttpMethod::Post, url.clone())
            .header("Content-Type", "application/json");
        if let Some(b) = body {
            let serialized = serde_json::to_vec(b).map_err(|e| {
                crate::bullish::Errors::Error(format!("Failed to serialize body: {e}"))
            })?;
            builder = builder
                .header("Content-Length", serialized.len().to_string())
                .body(serialized);
        }
        let request = builder.build();
        let response = self
            .http_client
            .execute(request)
            .await
            .map_err(crate::bullish::Errors::from)?;

        self.rate_limiter.increment_request(endpoint_type).await;

        if !response.is_success() {
            return Err(Self::error_from_response(url, response));
        }

        let result: T = response.json().map_err(crate::bullish::Errors::from)?;
        Ok(result)
    }

    fn error_from_response(url: String, response: rest::Response) -> crate::bullish::Errors {
        let status = response.status;
        let error_text = response.text().unwrap_or_default();
        let detailed = if let Ok(err_resp) =
            serde_json::from_str::<crate::bullish::ErrorResponse>(&error_text)
        {
            format!(
                "HTTP {} from {} - {}: {}{}",
                status,
                url,
                err_resp.error.code,
                err_resp.error.message,
                err_resp
                    .error
                    .details
                    .as_ref()
                    .map(|d| format!(" (details: {})", d))
                    .unwrap_or_default()
            )
        } else if error_text.trim().is_empty() {
            format!("HTTP {} from {} (empty body)", status, url)
        } else {
            format!("HTTP {} from {} - body: {}", status, url, error_text)
        };
        crate::bullish::Errors::Error(format!("Request failed: {detailed}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_client_creation() {
        let client = Arc::new(rest::native::NativeHttpClient::default());
        let rate_limiter = RateLimiter::new();
        let rest_client = RestClient::new("https://api.exchange.bullish.com", client, rate_limiter);
        assert_eq!(rest_client.base_url, "https://api.exchange.bullish.com");
    }
}
