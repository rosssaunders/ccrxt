// Shared REST client logic for Binance Options public and private clients.
// Handles URL construction, header assembly, request execution, and rate limiter update.

use crate::binance::options::{ErrorResponse, Errors, RateLimiter, ResponseHeaders};
use reqwest::{Client, Method};
use std::time::Duration;
use url::Url;

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

/// Response struct for Options REST requests, includes data, headers, and request duration.
pub struct RestResponse<T> {
    pub data: T,
    pub headers: ResponseHeaders,
    pub request_duration: Duration,
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

/// Internal helper to execute HTTP requests and parse responses for Options API.
async fn execute_request<T>(
    client: &Client,
    url: &str,
    method: Method,
    headers: Option<Vec<(&str, String)>>,
    body: Option<&str>,
) -> Result<ParsedResponse<T>, Errors>
where
    T: serde::de::DeserializeOwned,
{
    use crate::binance::options::ApiError;
    use reqwest::StatusCode;
    use tracing::debug;

    let start = std::time::Instant::now();
    let mut request = client.request(method, url);

    // Add headers if provided
    if let Some(headers) = headers {
        for (key, value) in headers {
            request = request.header(key, value);
        }
    }

    // Add body if provided
    if let Some(body_str) = body {
        request = request.body(body_str.to_string());
        request = request.header("Content-Type", "application/x-www-form-urlencoded");
    }

    let response = request.send().await.map_err(Errors::HttpError)?;
    let duration = start.elapsed();
    let headers = ResponseHeaders::from_reqwest_headers(response.headers());
    let status = response.status();
    let response_text = response.text().await.map_err(Errors::HttpError)?;

    debug!(
        "Options API response: status={}, body={}",
        status, response_text
    );

    // Handle non-success status codes
    match status {
        | StatusCode::OK => {
            // Parse the JSON response
            let data: T = serde_json::from_str(&response_text)
                .map_err(|e| Errors::Error(format!("JSON parsing error: {}", e)))?;
            Ok(ParsedResponse {
                data,
                headers,
                duration,
            })
        },
        | StatusCode::TOO_MANY_REQUESTS => {
            let error_msg = extract_msg(&response_text).await;
            Err(Errors::ApiError(ApiError::TooManyRequests {
                msg: error_msg,
            }))
        },
        | StatusCode::IM_A_TEAPOT => {
            let error_msg = extract_msg(&response_text).await;
            Err(Errors::ApiError(ApiError::IpAutoBanned { msg: error_msg }))
        },
        | StatusCode::UNAUTHORIZED => {
            let error_msg = extract_msg(&response_text).await;
            Err(Errors::ApiError(ApiError::Unauthorized { msg: error_msg }))
        },
        | _ => {
            // Try to parse as ErrorResponse first
            match serde_json::from_str::<ErrorResponse>(&response_text) {
                | Ok(error_response) => Err(Errors::ApiError(error_response.into())),
                | Err(_) => Err(Errors::Error(format!("HTTP {}: {}", status, response_text))),
            }
        },
    }
}

/// Represents a parsed HTTP response, including the deserialized data and response headers.
struct ParsedResponse<T> {
    /// The deserialized response body.
    pub data: T,
    /// The parsed response headers.
    pub headers: ResponseHeaders,
    /// The duration of the HTTP request.
    pub duration: Duration,
}

// Helper to extract error message from JSON or fallback to raw text
async fn extract_msg(text: &str) -> String {
    serde_json::from_str::<ErrorResponse>(text)
        .map(|e| e.msg)
        .unwrap_or_else(|_| text.to_owned())
}
