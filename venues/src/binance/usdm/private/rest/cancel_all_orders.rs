//! Cancel all open orders and countdown cancel all orders on Binance USDM REST API.

use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use thiserror::Error;

use crate::binance::usdm::private::rest::client::RestClient;
use crate::binance::usdm::signing::sign_query;
use chrono::Utc;
use reqwest::Method;

/// Error type for USDM cancel all orders endpoints.
#[derive(Debug, Error, Clone, Deserialize)]
#[serde(tag = "code", content = "msg")]
pub enum CancelAllOrdersError {
    /// Invalid API key or signature.
    #[error("Invalid API key or signature: {0}")]
    InvalidKey(String),
    /// Symbol not found.
    #[error("Symbol not found: {0}")]
    SymbolNotFound(String),
    /// Rate limit exceeded.
    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),
    /// Any other error.
    #[error("Other error: {0}")]
    Other(String),
}

/// Error response from Binance REST API.
#[derive(Debug, Clone, Deserialize)]
pub struct CancelAllOrdersErrorResponse {
    /// Error code returned by Binance.
    pub code: i64,
    /// Error message returned by Binance.
    pub msg: String,
}

impl From<CancelAllOrdersErrorResponse> for CancelAllOrdersError {
    fn from(e: CancelAllOrdersErrorResponse) -> Self {
        match e.code {
            -2015 | -2014 => CancelAllOrdersError::InvalidKey(e.msg),
            -1121 => CancelAllOrdersError::SymbolNotFound(e.msg),
            -1003 => CancelAllOrdersError::RateLimit(e.msg),
            _ => CancelAllOrdersError::Other(e.msg),
        }
    }
}

/// Result type for cancel all orders operations.
pub type CancelAllOrdersResult<T> = Result<T, CancelAllOrdersError>;

/// Request to cancel all open orders for a symbol.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOpenOrdersRequest {
    /// API key (securely stored)
    #[serde(skip)]
    pub api_key: SecretString,
    /// API secret (securely stored)
    #[serde(skip)]
    pub api_secret: SecretString,
    /// Symbol to cancel all orders for
    pub symbol: Cow<'static, str>,
}

/// Request to set up countdown cancel all orders.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CountdownCancelAllRequest {
    /// API key (securely stored)
    #[serde(skip)]
    pub api_key: SecretString,
    /// API secret (securely stored)
    #[serde(skip)]
    pub api_secret: SecretString,
    /// Symbol to set countdown cancel for
    pub symbol: Cow<'static, str>,
    /// Countdown time in milliseconds (0 to disable)
    pub countdown_time: u64,
}

/// Response for cancel all open orders.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOpenOrdersResponse {
    /// Response code (200 for success)
    pub code: u16,
    /// Response message
    pub msg: Cow<'static, str>,
}

/// Response for countdown cancel all orders.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CountdownCancelAllResponse {
    /// Symbol
    pub symbol: Cow<'static, str>,
    /// Countdown time in milliseconds
    pub countdown_time: u64,
}

impl RestClient {
    /// Cancel all open orders for a symbol (DELETE /fapi/v1/allOpenOrders)
    /// [Binance API docs](https://binance-docs.github.io/apidocs/futures/en/#cancel-all-open-orders-trade)
    pub async fn cancel_all_open_orders(
        &self,
        params: CancelAllOpenOrdersRequest,
    ) -> CancelAllOrdersResult<CancelAllOpenOrdersResponse> {
        use crate::binance::usdm::request::execute_request;
        use tracing::debug;

        // 1. Prepare endpoint and method
        let endpoint = "/fapi/v1/allOpenOrders";
        let method = Method::DELETE;
        let url = format!("{}{}", self.base_url, endpoint);

        // 2. Add timestamp and recvWindow
        let timestamp = Utc::now().timestamp_millis();
        let recv_window = 5000u64;

        // 3. Serialize params to query string
        let mut query_pairs = serde_urlencoded::to_string(&params)
            .map_err(|e| CancelAllOrdersError::Other(format!("Failed to serialize params: {e}")))?;
        if !query_pairs.is_empty() {
            query_pairs.push('&');
        }
        query_pairs.push_str(&format!("timestamp={timestamp}&recvWindow={recv_window}"));

        // 4. Sign the query string
        let signature = sign_query(&query_pairs, &params.api_secret);
        query_pairs.push_str(&format!("&signature={signature}"));

        // 5. Set headers
        let headers = vec![
            ("X-MBX-APIKEY", params.api_key.expose_secret().to_string()),
        ];

        // 6. Rate limiting
        self.rate_limiter
            .acquire_order()
            .await
            .map_err(|e| CancelAllOrdersError::Other(format!("Rate limiting error: {e}")))?;

        debug!(endpoint = endpoint, "Sending cancel all open orders request");

        // 7. Execute request
        let resp = execute_request::<CancelAllOpenOrdersResponse>(
            &self.client,
            &url,
            method,
            Some(headers),
            Some(&query_pairs),
        )
        .await
        .map_err(|e| match e {
            crate::binance::usdm::Errors::ApiError(api_err) => {
                CancelAllOrdersError::Other(format!("API error: {api_err}"))
            }
            crate::binance::usdm::Errors::HttpError(http_err) => {
                CancelAllOrdersError::Other(format!("HTTP error: {http_err}"))
            }
            crate::binance::usdm::Errors::Error(msg) => CancelAllOrdersError::Other(msg),
            crate::binance::usdm::Errors::InvalidApiKey() => {
                CancelAllOrdersError::InvalidKey("Invalid API key or signature".to_string())
            }
        })?;

        Ok(resp.data)
    }

    /// Set countdown cancel all orders (POST /fapi/v1/countdownCancelAll)
    /// [Binance API docs](https://binance-docs.github.io/apidocs/futures/en/#auto-cancel-all-open-orders-trade)
    pub async fn countdown_cancel_all(
        &self,
        params: CountdownCancelAllRequest,
    ) -> CancelAllOrdersResult<CountdownCancelAllResponse> {
        use crate::binance::usdm::request::execute_request;
        use tracing::debug;

        // 1. Prepare endpoint and method
        let endpoint = "/fapi/v1/countdownCancelAll";
        let method = Method::POST;
        let url = format!("{}{}", self.base_url, endpoint);

        // 2. Add timestamp and recvWindow
        let timestamp = Utc::now().timestamp_millis();
        let recv_window = 5000u64;

        // 3. Serialize params to query string
        let mut query_pairs = serde_urlencoded::to_string(&params)
            .map_err(|e| CancelAllOrdersError::Other(format!("Failed to serialize params: {e}")))?;
        if !query_pairs.is_empty() {
            query_pairs.push('&');
        }
        query_pairs.push_str(&format!("timestamp={timestamp}&recvWindow={recv_window}"));

        // 4. Sign the query string
        let signature = sign_query(&query_pairs, &params.api_secret);
        query_pairs.push_str(&format!("&signature={signature}"));

        // 5. Set headers
        let headers = vec![
            ("X-MBX-APIKEY", params.api_key.expose_secret().to_string()),
            (
                "Content-Type",
                "application/x-www-form-urlencoded".to_string(),
            ),
        ];

        // 6. Rate limiting
        self.rate_limiter
            .acquire_order()
            .await
            .map_err(|e| CancelAllOrdersError::Other(format!("Rate limiting error: {e}")))?;

        debug!(endpoint = endpoint, "Sending countdown cancel all request");

        // 7. Execute request
        let resp = execute_request::<CountdownCancelAllResponse>(
            &self.client,
            &url,
            method,
            Some(headers),
            Some(&query_pairs),
        )
        .await
        .map_err(|e| match e {
            crate::binance::usdm::Errors::ApiError(api_err) => {
                CancelAllOrdersError::Other(format!("API error: {api_err}"))
            }
            crate::binance::usdm::Errors::HttpError(http_err) => {
                CancelAllOrdersError::Other(format!("HTTP error: {http_err}"))
            }
            crate::binance::usdm::Errors::Error(msg) => CancelAllOrdersError::Other(msg),
            crate::binance::usdm::Errors::InvalidApiKey() => {
                CancelAllOrdersError::InvalidKey("Invalid API key or signature".to_string())
            }
        })?;

        Ok(resp.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_countdown_cancel_all_response_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "countdownTime": 120000
        }"#;

        let response: CountdownCancelAllResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTCUSDT");
        assert_eq!(response.countdown_time, 120000);
    }
}
