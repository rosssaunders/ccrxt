use std::time::Duration;
use tracing::debug;

use reqwest::StatusCode;

use crate::binance::option::{ResponseHeaders, Errors, ApiError};
use crate::binance::option::errors::ErrorResponse;

// Helper to extract error message from JSON or fallback to raw text
async fn extract_msg(text: &str) -> String {
    serde_json::from_str::<ErrorResponse>(text)
        .map(|e| e.msg)
        .unwrap_or_else(|_| text.to_owned())
}

// https://developers.binance.com/docs/derivatives/option/common-definition#http-return-codes
/* 
HTTP Return Codes
HTTP 4XX return codes are used for malformed requests; the issue is on the sender's side.
HTTP 403 return code is used when the WAF Limit (Web Application Firewall) has been violated.
HTTP 429 return code is used when breaking a request rate limit.
HTTP 418 return code is used when an IP has been auto-banned for continuing to send requests after receiving 429 codes.
HTTP 5XX return codes are used for internal errors; the issue is on Binance's side. It is important to NOT treat this as a failure operation; the execution status is UNKNOWN and could have been a success.
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

/// Internal helper to execute HTTP requests and parse responses for Options API.
/// This function encapsulates the shared HTTP logic (building, sending, and parsing the response to idiomatic Rust types).
pub(crate) async fn execute_request<T>(
    response: reqwest::Response,
    request_duration: Duration,
) -> Result<ParsedResponse<T>, Errors>
where
    T: serde::de::DeserializeOwned,
{
    let headers = ResponseHeaders::default(); // TODO: Parse actual headers when implementing full client
    
    let status = response.status();
    let text = response.text().await.map_err(Errors::HttpError)?;

    debug!("Response status: {}", status);
    debug!("Response body: {}", text);

    match status {
        StatusCode::OK => {
            let data = serde_json::from_str::<T>(&text)
                .map_err(|e| Errors::Error(format!("Failed to parse response: {}", e)))?;
            
            Ok(ParsedResponse {
                data,
                headers,
                duration: request_duration,
            })
        }
        StatusCode::BAD_REQUEST => {
            let error = serde_json::from_str::<ErrorResponse>(&text)
                .map_err(|_| Errors::Error(format!("Bad Request: {}", text)))?;
            Err(Errors::ApiError(ApiError::from(error)))
        }
        StatusCode::UNAUTHORIZED => {
            Err(Errors::InvalidApiKey())
        }
        StatusCode::TOO_MANY_REQUESTS => {
            let error = serde_json::from_str::<ErrorResponse>(&text)
                .unwrap_or_else(|_| ErrorResponse {
                    code: -1003,
                    msg: "Rate limit exceeded".to_string(),
                });
            Err(Errors::ApiError(ApiError::RateLimitExceeded(error)))
        }
        _ => {
            let msg = extract_msg(&text).await;
            Err(Errors::Error(format!("HTTP {}: {}", status, msg)))
        }
    }
}