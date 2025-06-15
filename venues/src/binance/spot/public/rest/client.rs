// REST client for Binance Spot public endpoints.
//
// Provides access to all public REST API endpoints for Binance Spot.
// All requests are unauthenticated and do not require API credentials.
use reqwest::Client;
use std::borrow::Cow;

use crate::binance::spot::{RateLimiter, RestResult};

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct RestClient {
    /// The base URL for the Binance Spot public REST API (e.g., "<https://api.binance.com>").
    ///
    /// This is used as the prefix for all endpoint requests.
    pub base_url: Cow<'static, str>,

    /// The underlying HTTP client used for making requests.
    ///
    /// This is reused for connection pooling and performance.
    pub client: Client,

    /// The rate limiter used to manage request rates and prevent hitting API limits.
    ///
    /// This is used to ensure compliance with Binance's rate limits for public endpoints.
    pub rate_limiter: RateLimiter,
}

impl RestClient {
    /// Creates a new Binance Spot public REST client.
    ///
    /// # Arguments
    /// * `base_url` - The base URL for the Binance Spot public REST API (e.g., "<https://api.binance.com>").
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

    /// Send a request with form body as &[(&str, &str)]
    pub async fn send_request<T>(
        &self,
        endpoint: &str,
        method: reqwest::Method,
        query_string: Option<&str>,
        body: Option<&[(&str, &str)]>,
        weight: u32,
    ) -> RestResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url =
            crate::binance::spot::rest::common::build_url(&self.base_url, endpoint, query_string)?;
        let headers = vec![];
        let body_data = match body {
            Some(b) => Some(serde_urlencoded::to_string(b).map_err(|e| {
                crate::binance::spot::Errors::Error(format!("URL encoding error: {}", e))
            })?),
            None => None,
        };
        let rest_response = crate::binance::spot::rest::common::send_rest_request(
            &self.client,
            &url,
            method,
            headers,
            body_data.as_deref(),
            &self.rate_limiter,
            weight,
            false,
        )
        .await?;
        Ok(crate::binance::spot::RestResponse {
            data: rest_response.data,
            headers: rest_response.headers,
        })
    }

    /// Test connectivity to the Rest API.
    ///
    /// Weight: 1
    pub async fn ping(&self) -> RestResult<serde_json::Value> {
        self.send_request("/api/v3/ping", reqwest::Method::GET, None, None, 1)
            .await
    }

    /// Test connectivity to the Rest API and get the current server time.
    ///
    /// Weight: 1
    pub async fn time(&self) -> RestResult<serde_json::Value> {
        self.send_request("/api/v3/time", reqwest::Method::GET, None, None, 1)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_public_client_creation() {
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new();

        let rest_client = RestClient::new("https://api.binance.com", client, rate_limiter);

        assert_eq!(rest_client.base_url, "https://api.binance.com");
    }

    #[tokio::test]
    async fn test_url_building() {
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new();

        let rest_client = RestClient::new("https://api.binance.com", client, rate_limiter);

        // Test that the client is properly initialized
        assert_eq!(rest_client.base_url, "https://api.binance.com");
    }

    #[tokio::test]
    async fn test_client_has_all_public_methods() {
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new();
        let rest_client = RestClient::new("https://api.binance.com", client, rate_limiter);

        // This test just ensures all our methods are accessible
        // We're not calling them to avoid network requests in tests
        let _ = &rest_client.ping();
        let _ = &rest_client.time();
        let _ = &rest_client.exchange_info();
        let _ = &rest_client.depth("BTCUSDT", Some(5));
        let _ = &rest_client.trades("BTCUSDT", Some(5));
        let _ = &rest_client.ticker_24hr(Some("BTCUSDT"), None, None);
        let _ = &rest_client.ticker_price(Some("BTCUSDT"), None);
        let _ = &rest_client.klines("BTCUSDT", "1h", None, None, None, None);
    }
}