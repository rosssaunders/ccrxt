use std::time::Duration;

use reqwest::StatusCode;
use tracing::debug;

use crate::binance::coinm::errors::ErrorResponse;
use crate::binance::coinm::{ApiError, Errors, ResponseHeaders};

// Helper to extract error message from JSON or fallback to raw text
async fn extract_msg(text: &str) -> String {
    serde_json::from_str::<ErrorResponse>(text)
        .map(|e| e.msg)
        .unwrap_or_else(|_| text.to_owned())
}

// https://developers.binance.com/docs/derivatives/coin-margined-futures/general-info#http-return-codes
/*
HTTP Return Codes
HTTP 4XX return codes are used for malformed requests; the issue is on the sender's side.
HTTP 403 return code is used when the WAF Limit (Web Application Firewall) has been violated.
HTTP 409 return code is used when a cancelReplace order partially succeeds. (i.e. if the cancellation of the order fails but the new order placement succeeds.)
HTTP 429 return code is used when breaking a request rate limit.
HTTP 418 return code is used when an IP has been auto-banned for continuing to send requests after receiving 429 codes.
HTTP 5XX return codes are used for internal errors; the issue is on Binance's side. It is important to NOT treat this as a failure operation; the execution status is UNKNOWN and could have been a success.
*/

// https://developers.binance.com/docs/derivatives/coin-margined-futures/general-info#error-codes-and-messages
/*
General Information on Endpoints
For GET endpoints, parameters must be sent as a query string.
For POST, PUT, and DELETE endpoints, the parameters may be sent as a query string or in the request body with content type application/x-www-form-urlencoded. You may mix parameters between both the query string and request body if you wish to do so.
Parameters may be sent in any order.
If a parameter sent in both the query string and request body, the query string parameter will be used.
*/

/// Represents a parsed HTTP response, including the deserialized data and response headers.
pub(crate) struct ParsedResponse<T> {
    /// The deserialized response body.
    pub data: T,

    /// The parsed response headers.
    pub headers: ResponseHeaders,

    /// The duration of the HTTP request.
    pub duration: Duration,
}

/// Internal helper to execute HTTP requests and parse responses for both public and private clients.
/// This function encapsulates the shared HTTP logic (building, sending, and parsing the response to idiomatic Rust types).
pub(crate) async fn execute_request<T>(
    client: &reqwest::Client,
    url: &str,
    method: reqwest::Method,
    headers: Option<Vec<(&str, String)>>,
    body: Option<&str>,
) -> Result<ParsedResponse<T>, Errors>
where
    T: serde::de::DeserializeOwned,
{
    use std::time::Instant;
    let mut request = client.request(method.clone(), url);
    if let Some(hdrs) = headers {
        for (k, v) in hdrs {
            request = request.header(k, v);
        }
    }
    if let Some(b) = body {
        request = request.body(b.to_owned());
    }
    let start = Instant::now();
    let response = request.send().await.map_err(Errors::HttpError)?;
    let duration = start.elapsed();
    let status = response.status();
    let headers = response.headers().clone();
    let text = response.text().await.map_err(Errors::HttpError)?;
    // Parse relevant headers into ResponseHeaders
    let values = headers
        .iter()
        .filter_map(|(name, val)| {
            crate::binance::coinm::RateLimitHeader::parse(name.as_str())
                .and_then(|hdr| val.to_str().ok()?.parse::<u32>().ok().map(|v| (hdr, v)))
        })
        .collect();
    let response_headers = ResponseHeaders { values };

    debug!("HTTP response status = {:?}", status);

    match status {
        StatusCode::OK => {
            // Try to parse as ErrorResponse first
            if let Ok(err) = serde_json::from_str::<ErrorResponse>(&text) {
                // Binance error payloads have a nonzero or negative code
                if err.code != 0 {
                    return Err(Errors::ApiError(ApiError::from(err)));
                }
            }
            // Otherwise, parse as the expected type
            let data: T = serde_json::from_str(&text)
                .map_err(|e| Errors::Error(format!("JSON decode error: {e} | body: {text}")))?;
            Ok(ParsedResponse {
                data,
                headers: response_headers,
                duration,
            })
        }
        StatusCode::TOO_MANY_REQUESTS => {
            // Extract relevant headers for rate limit info
            let used_weight_1m = headers
                .get("x-mbx-used-weight-1m")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse::<u32>().ok());
            let order_count_1m = headers
                .get("x-mbx-order-count-1m")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse::<u32>().ok());
            let retry_after = headers
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse::<u64>().ok());
            let msg = extract_msg(&text).await;
            Err(Errors::ApiError(ApiError::RateLimitExceeded {
                msg,
                used_weight_1m,
                order_count_1m,
                retry_after,
            }))
        }
        StatusCode::FORBIDDEN => {
            // 403 WAF Limit Violation
            let msg = extract_msg(&text).await;
            Err(Errors::ApiError(ApiError::WafLimitViolated { msg }))
        }
        StatusCode::REQUEST_TIMEOUT => {
            // 408 Request Timeout
            let msg = extract_msg(&text).await;
            Err(Errors::ApiError(ApiError::RequestTimeout { msg }))
        }
        StatusCode::IM_A_TEAPOT => {
            // 418 IP Auto-Banned
            let msg = extract_msg(&text).await;
            Err(Errors::ApiError(ApiError::IpAutoBanned { msg }))
        }
        s if s.is_server_error() => {
            // 5XX Internal Server Error, including 503 Service Unavailable
            let msg = extract_msg(&text).await;
            if s == StatusCode::SERVICE_UNAVAILABLE {
                Err(Errors::ApiError(ApiError::ServiceUnavailable { msg }))
            } else {
                Err(Errors::ApiError(ApiError::InternalServerError { msg }))
            }
        }
        _ => {
            // HTTP 4XX return codes are used for for malformed requests; the issue is on the sender's side.
            println!("ERROR: {text:?}");
            let err: ErrorResponse = serde_json::from_str(&text)
                .map_err(|e| Errors::Error(format!("JSON decode error: {e} | body: {text}")))?;
            Err(Errors::ApiError(ApiError::from(err)))
        }
    }
}
