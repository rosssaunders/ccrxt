// Shared REST client logic for Binance Spot public and private clients.
// Handles URL construction, header assembly, request execution, and rate limiter update.

use reqwest::{Client, Method};
use url::Url;

use crate::binance::spot::{Errors, RateLimiter, ResponseHeaders, execute_request};

/// Helper to build a URL with optional query parameters using `url::Url`.
pub(crate) fn build_url(
    base_url: &str,
    endpoint: &str,
    query: Option<&str>,
) -> Result<String, Errors> {
    let mut url =
        Url::parse(base_url).map_err(|e| Errors::Error(format!("Invalid base_url: {e}")))?;
    url.set_path(endpoint);
    if let Some(qs) = query {
        url.set_query(Some(qs));
    }
    Ok(url.to_string())
}

/// Shared logic for sending a REST request and updating the rate limiter.
pub(crate) async fn send_rest_request<T>(
    client: &Client,
    url: &str,
    method: Method,
    headers: Vec<(&str, String)>,
    body: Option<&str>,
    rate_limiter: &RateLimiter,
    weight: u32,
    is_order: bool,
) -> Result<RestResponse<T>, Errors>
where
    T: serde::de::DeserializeOwned,
{
    // Check rate limits before sending
    rate_limiter.check_limits(weight, is_order).await?;
    if is_order {
        rate_limiter.increment_order().await;
    }
    rate_limiter.increment_raw_request().await;

    let response = execute_request(client, url, method, Some(headers), body).await?;
    rate_limiter.update_from_headers(&response.headers).await;
    Ok(RestResponse {
        data: response.data,
        headers: response.headers,
        request_duration: response.duration,
    })
}

/// Response structure used by the shared send_rest_request function
pub(crate) struct RestResponse<T> {
    pub data: T,
    pub headers: ResponseHeaders,
    #[allow(dead_code)]
    pub request_duration: std::time::Duration,
}
