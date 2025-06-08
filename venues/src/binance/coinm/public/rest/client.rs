// REST client for Binance Coin-M public endpoints.
//
// Provides access to all public REST API endpoints for Binance Coin-M Futures.
// All requests are unauthenticated and do not require API credentials.
//
// See: https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Exchange-Information

use std::time::Duration;
use reqwest::Client;

use crate::binance::coinm::{execute_request, Errors, RateLimiter, RestResult, RestResponse};

#[derive(Debug, Clone)]
pub struct RestClient {
    /// The base URL for the Binance Coin-M public REST API (e.g., "https://dapi.binance.com").
    ///
    /// This is used as the prefix for all endpoint requests.
    base_url: String,

    /// The underlying HTTP client used for making requests.
    ///
    /// This is reused for connection pooling and performance.
    client: Client,

    /// The rate limiter used to manage request rates and prevent hitting API limits.
    ///
    /// This is used to ensure compliance with Binance's rate limits for public endpoints.
    rate_limiter: RateLimiter,
}

impl RestClient {
    /// Creates a new Binance Coin-M public REST client.
    ///
    /// # Arguments
    /// * `base_url` - The base URL for the Binance Coin-M public REST API (e.g., "https://dapi.binance.com").
    pub fn new<S: Into<String>>(base_url: S, client: Client, rate_limiter: RateLimiter) -> Self {
        Self {
            base_url: base_url.into(),
            client: client,
            rate_limiter: rate_limiter,
        }
    }

    pub async fn send_request<T>(
        &self,
        endpoint: &str,
        method: reqwest::Method,
        query_string: Option<&str>,
        body: Option<&str>,
        weight: u32
    ) -> RestResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        // Check rate limits before sending
        //self.rate_limiter.check_limits(weight, is_order).await?;

        // Increment raw request counter
        // self.rate_limiter.increment_raw_request().await;
        // if is_order {
        //     self.rate_limiter.increment_order().await;
        // }
        // let rate_limit_start = Instant::now();

        let url = match query_string {
            Some(qs) if method == reqwest::Method::GET => format!("{}{}?{}", self.base_url, endpoint, qs),
            _ => format!("{}{}", self.base_url, endpoint),
        };
        let mut headers = vec![];

        let rest_response = execute_request(&self.client, &url, method, Some(headers.clone()), body)
            .await
            .map_err(Errors::from)?;

        // At this point, rest_response is the parsed response from execute_request.
        // If execute_request returns a type that is not already a RestResponse<T>, convert it here.
        // Assuming execute_request returns a type that can be converted into RestResponse<T>:

        // Manually build the RestResponse object from the raw response.
        // Assume rest_response is a tuple or struct with (body, headers, status) or similar.
        // If execute_request returns the raw response body as bytes or string, parse here.
        let headers = rest_response.headers.clone();
        
        let rest_response = RestResponse {
            data: rest_response.data,
            request_duration: rest_response.duration,
            headers: headers,
        };

        // Optionally update rate limiter from headers if needed
        // self.rate_limiter.update_from_headers(&rest_response.headers).await;

        Ok(rest_response)

        // Update rate limiter from headers
        // self.rate_limiter.update_from_headers(&headers).await;
    }
}
