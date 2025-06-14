// REST client for Binance Coin-M public endpoints.
//
// Provides access to all public REST API endpoints for Binance Coin-M Futures.
// All requests are unauthenticated and do not require API credentials.
use reqwest::Client;
use std::borrow::Cow;

use crate::binance::coinm::rest::common::{build_url, send_rest_request};
use crate::binance::coinm::{RateLimiter, RestResponse, RestResult};

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct RestClient {
    /// The base URL for the Binance Coin-M public REST API (e.g., "https://dapi.binance.com").
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
    /// Creates a new Binance Coin-M public REST client.
    ///
    /// # Arguments
    /// * `base_url` - The base URL for the Binance Coin-M public REST API (e.g., "https://dapi.binance.com").
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
            crate::binance::coinm::rest::common::build_url(&self.base_url, endpoint, query_string)?;
        let headers = vec![];
        let body_data = body.map(|b| serde_urlencoded::to_string(b).unwrap());
        let rest_response = crate::binance::coinm::rest::common::send_rest_request(
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
        Ok(crate::binance::coinm::RestResponse {
            data: rest_response.data,
            request_duration: rest_response.request_duration,
            headers: rest_response.headers,
        })
    }
}
