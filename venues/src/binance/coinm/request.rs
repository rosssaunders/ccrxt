use reqwest::{Client, StatusCode};
use std::time::Instant;
use super::super::{
    api_errors::{BinanceCoinMError, BinanceCoinMResult},
    types::{BinanceResponse, BinanceHeaders},
};
use chrono::Utc;

pub async fn send_request<T, F, R>(
    client: &Client,
    base_url: &str,
    endpoint: &str,
    method: reqwest::Method,
    query_string: Option<&str>,
    api_key: Option<&str>,
    rate_limit_check: F,
) -> BinanceCoinMResult<BinanceResponse<T>>
where
    T: serde::de::DeserializeOwned,
    F: FnOnce() -> R,
    R: std::future::Future<Output = BinanceCoinMResult<()>>,
{
    let rate_limit_start = Instant::now();
    rate_limit_check().await?;

    let url = match query_string {
        Some(qs) => format!("{}{}?{}", base_url, endpoint, qs),
        None => format!("{}{}", base_url, endpoint),
    };

    let mut request = client.request(method, &url);
    if let Some(key) = api_key {
        request = request.header("X-MBX-APIKEY", key);
    }

    let request_start = Instant::now();
    let response = request.send().await?;

    let headers = BinanceHeaders {
        used_weight_1m: response.headers()
            .get("X-MBX-USED-WEIGHT-1M")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse().ok()),
        order_count_1m: response.headers()
            .get("X-MBX-ORDER-COUNT-1M")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse().ok()),
        order_count_1d: response.headers()
            .get("X-MBX-ORDER-COUNT-1D")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse().ok()),
        order_count_1s: response.headers()
            .get("X-MBX-ORDER-COUNT-1S")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse().ok()),
    };

    match response.status() {
        StatusCode::OK => {
            let data: T = response.json().await?;
            Ok(BinanceResponse {
                data,
                rate_limit_duration: rate_limit_start.elapsed(),
                request_duration: request_start.elapsed(),
                headers,
            })
        }
        StatusCode::BAD_REQUEST => {
            let error: super::super::api_errors::BinanceErrorResponse = response.json().await?;
            Err(BinanceCoinMError::from(error))
        }
        StatusCode::UNAUTHORIZED => Err(BinanceCoinMError::RejectedMbxKey(-2015)),
        StatusCode::TOO_MANY_REQUESTS => Err(BinanceCoinMError::TooManyRequests(-1003, "Too many requests".to_string())),
        StatusCode::INTERNAL_SERVER_ERROR => Err(BinanceCoinMError::Disconnected(-1001)),
        _ => Err(BinanceCoinMError::Unknown(-1000)),
    }
}

/// Appends a timestamp and signature to a query string.
///
/// # Arguments
/// * `query_str` - The base query string (without timestamp or signature)
/// * `sign_fn` - A closure that takes the query string (with timestamp) and returns the signature
///
/// # Returns
/// The full query string with timestamp and signature appended.
pub fn append_timestamp_and_signature<F, E>(
    mut query_str: String,
    sign_fn: F,
) -> Result<String, E>
where
    F: Fn(&str) -> Result<String, E>,
{
    let timestamp = Utc::now().timestamp_millis();
    if !query_str.is_empty() {
        query_str.push('&');
    }
    query_str.push_str(&format!("timestamp={}", timestamp));
    let signature = sign_fn(&query_str)?;
    query_str.push_str(&format!("&signature={}", signature));
    Ok(query_str)
} 