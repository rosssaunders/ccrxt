// REST client for Binance Coin-M public endpoints.
//
// Provides access to all public REST API endpoints for Binance Coin-M Futures.
// All requests are unauthenticated and do not require API credentials.
use std::borrow::Cow;

use reqwest::Client;

use crate::binance::coinm::rest::common::{build_url, send_rest_request};
use crate::binance::coinm::{RateLimiter, RestResponse, RestResult};

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct RestClient {
    /// The base URL for the Binance Coin-M public REST API (e.g., "<https://dapi.binance.com>").
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
    /// * `base_url` - The base URL for the Binance Coin-M public REST API (e.g., "<https://dapi.binance.com>").
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
    pub async fn send_request<Req, Resp>(
        &self,
        endpoint: &str,
        method: reqwest::Method,
        params: Option<Req>,
        weight: u32,
    ) -> RestResult<Resp>
    where
        Req: serde::ser::Serialize,
        Resp: serde::de::DeserializeOwned,
    {
        let query_string = serde_urlencoded::to_string(&params).map_err(|e| {
            crate::binance::coinm::Errors::Error(format!("URL encoding error: {e}"))
        })?;

        let url = build_url(&self.base_url, endpoint, Some(&query_string))?;
        let headers = vec![];
        let rest_response: crate::binance::coinm::rest::common::RestResponse<String> =
            send_rest_request(
                &self.client,
                &url,
                method,
                headers,
                None,
                &self.rate_limiter,
                weight,
                false,
            )
            .await?;

        let parsed_data: Resp = serde_json::from_str(&rest_response.data)
            .map_err(|e| crate::binance::coinm::Errors::Error(format!("JSON parse error: {e}")))?;

        Ok(RestResponse {
            data: parsed_data,
            request_duration: rest_response.request_duration,
            headers: rest_response.headers,
        })
    }
}
