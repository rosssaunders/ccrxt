use std::time::Duration;
use tracing::debug;

use reqwest::StatusCode;

use crate::binance::spot::errors::ErrorResponse;
use crate::binance::spot::{ApiError, Errors, ResponseHeaders};

// Helper to extract error message from JSON or fallback to raw text
async fn extract_msg(text: &str) -> String {
    serde_json::from_str::<ErrorResponse>(text)
        .map(|e| e.msg)
        .unwrap_or_else(|_| text.to_owned())
}

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

    let start = Instant::now();

    let mut request_builder = client.request(method.clone(), url);

    // Add headers
    if let Some(headers) = headers {
        for (key, value) in headers {
            request_builder = request_builder.header(key, value);
        }
    }

    // Add body if provided
    if let Some(body) = body {
        request_builder = request_builder.body(body.to_string());
    }

    debug!("Sending {} request to {}", method, url);

    let response = request_builder
        .send()
        .await
        .map_err(|e| Errors::Error(format!("Failed to send HTTP request: {}", e)))?;

    let duration = start.elapsed();
    let status = response.status();
    let response_headers = ResponseHeaders::from_reqwest_headers(response.headers());

    let response_text = response
        .text()
        .await
        .map_err(|e| Errors::Error(format!("Failed to read response body: {}", e)))?;

    debug!("Response status: {}, body: {}", status, response_text);

    // Handle HTTP error status codes
    if !status.is_success() {
        let error_msg = extract_msg(&response_text).await;
        let error = match status {
            StatusCode::TOO_MANY_REQUESTS => {
                // Extract rate limit headers for detailed error info
                let used_weight_1m = response_headers
                    .values
                    .iter()
                    .find(|(k, _)| k.to_string().contains("used-weight") && k.to_string().contains("1m"))
                    .map(|(_, v)| *v);
                let order_count_1m = response_headers
                    .values
                    .iter()
                    .find(|(k, _)| k.to_string().contains("order-count") && k.to_string().contains("1m"))
                    .map(|(_, v)| *v);
                Errors::ApiError(ApiError::RateLimitExceeded {
                    msg: error_msg,
                    used_weight_1m,
                    order_count_1m,
                    retry_after: None, // Could be extracted from Retry-After header
                })
            }
            StatusCode::IM_A_TEAPOT => Errors::ApiError(ApiError::IpAutoBanned { msg: error_msg }),
            StatusCode::FORBIDDEN => Errors::ApiError(ApiError::WafLimitViolated { msg: error_msg }),
            StatusCode::UNAUTHORIZED => Errors::ApiError(ApiError::Unauthorized { msg: error_msg }),
            StatusCode::BAD_REQUEST => {
                // Try to parse as structured error
                if let Ok(error_response) = serde_json::from_str::<ErrorResponse>(&response_text) {
                    Errors::ApiError(ApiError::from(error_response))
                } else {
                    Errors::Error(format!("Bad Request: {}", error_msg))
                }
            }
            _ => Errors::Error(format!("HTTP {}: {}", status, error_msg)),
        };
        return Err(error);
    }

    // Try to parse the response as the expected type
    let data = serde_json::from_str::<T>(&response_text).map_err(|e| Errors::Error(format!("Failed to parse response: {}", e)))?;

    Ok(ParsedResponse {
        data,
        headers: response_headers,
        duration,
    })
}
