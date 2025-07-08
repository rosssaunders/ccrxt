//! Get order amendment history on Binance USDM REST API.

use std::borrow::Cow;

use chrono::Utc;
use reqwest::Method;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::binance::usdm::{enums::*, private::rest::client::RestClient, signing::sign_query};

/// Error type for USDM order amendment endpoints.
#[derive(Debug, Error, Clone, Deserialize)]
#[serde(tag = "code", content = "msg")]
pub enum OrderAmendmentError {
    /// Invalid API key or signature.
    #[error("Invalid API key or signature: {0}")]
    InvalidKey(String),
    /// Order not found.
    #[error("Order not found: {0}")]
    OrderNotFound(String),
    /// Rate limit exceeded.
    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),
    /// Any other error.
    #[error("Other error: {0}")]
    Other(String),
}

/// Error response from Binance REST API.
#[derive(Debug, Clone, Deserialize)]
pub struct OrderAmendmentErrorResponse {
    /// Error code returned by Binance.
    pub code: i64,
    /// Error message returned by Binance.
    pub msg: String,
}

impl From<OrderAmendmentErrorResponse> for OrderAmendmentError {
    fn from(e: OrderAmendmentErrorResponse) -> Self {
        match e.code {
            -2015 | -2014 => OrderAmendmentError::InvalidKey(e.msg),
            -2013 => OrderAmendmentError::OrderNotFound(e.msg),
            -1003 => OrderAmendmentError::RateLimit(e.msg),
            _ => OrderAmendmentError::Other(e.msg),
        }
    }
}

/// Result type for order amendment operations.
pub type OrderAmendmentResult<T> = Result<T, OrderAmendmentError>;

/// Request to get order amendment history.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderAmendmentRequest {
    /// API key (securely stored)
    #[serde(skip)]
    pub api_key: SecretString,
    /// API secret (securely stored)
    #[serde(skip)]
    pub api_secret: SecretString,
    /// Symbol
    pub symbol: Cow<'static, str>,
    /// Order ID to get amendment history for (either this or origClientOrderId must be provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,
    /// Original client order ID (either this or orderId must be provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id: Option<Cow<'static, str>>,
    /// Start time for the query (milliseconds since epoch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    /// End time for the query (milliseconds since epoch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    /// Limit the number of results (default 50, max 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Response containing order amendment history.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderAmendmentResponse {
    /// List of order amendments
    pub amendments: Vec<OrderAmendment>,
    /// Total count of amendments
    pub total: u32,
}

/// Individual order amendment record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderAmendment {
    /// Amendment ID
    pub amendment_id: u64,
    /// Symbol
    pub symbol: Cow<'static, str>,
    /// Order ID
    pub order_id: u64,
    /// Client order ID
    pub client_order_id: Cow<'static, str>,
    /// Amendment timestamp
    pub time: u64,
    /// Amendment type (PRICE, QUANTITY, or BOTH)
    pub amendment_type: AmendmentType,
    /// Original price (before amendment)
    pub orig_price: Cow<'static, str>,
    /// Original quantity (before amendment)
    pub orig_qty: Cow<'static, str>,
    /// New price (after amendment)
    pub price: Cow<'static, str>,
    /// New quantity (after amendment)
    pub quantity: Cow<'static, str>,
    /// Amendment status
    pub status: AmendmentStatus,
    /// Price match mode
    pub price_match: PriceMatch,
}

impl RestClient {
    /// Get order amendment history (GET /fapi/v1/orderAmendment)
    /// [Binance API docs](https://binance-docs.github.io/apidocs/futures/en/#get-order-amendment-history-user_data)
    pub async fn get_order_amendment_history(
        &self,
        params: OrderAmendmentRequest,
    ) -> OrderAmendmentResult<OrderAmendmentResponse> {
        use tracing::debug;

        use crate::binance::usdm::request::execute_request;

        // 1. Prepare endpoint and method
        let endpoint = "/fapi/v1/orderAmendment";
        let method = Method::GET;
        let url = format!("{}{}", self.base_url, endpoint);

        // 2. Add timestamp and recvWindow
        let timestamp = Utc::now().timestamp_millis();
        let recv_window = 5000u64;

        // 3. Serialize params to query string
        let mut query_pairs = serde_urlencoded::to_string(&params)
            .map_err(|e| OrderAmendmentError::Other(format!("Failed to serialize params: {e}")))?;
        if !query_pairs.is_empty() {
            query_pairs.push('&');
        }
        query_pairs.push_str(&format!("timestamp={timestamp}&recvWindow={recv_window}"));

        // 4. Sign the query string
        let signature = sign_query(&query_pairs, &params.api_secret);
        query_pairs.push_str(&format!("&signature={signature}"));

        // 5. Set headers
        let headers = vec![("X-MBX-APIKEY", params.api_key.expose_secret().to_string())];

        // 6. Rate limiting
        self.rate_limiter
            .acquire_request(1)
            .await
            .map_err(|e| OrderAmendmentError::Other(format!("Rate limiting error: {e}")))?;

        debug!(
            endpoint = endpoint,
            "Sending order amendment history request"
        );

        // 7. Execute request
        let resp = execute_request::<OrderAmendmentResponse>(
            &self.client,
            &url,
            method,
            Some(headers),
            Some(&query_pairs),
        )
        .await
        .map_err(|e| match e {
            crate::binance::usdm::Errors::ApiError(api_err) => {
                OrderAmendmentError::Other(format!("API error: {api_err}"))
            }
            crate::binance::usdm::Errors::HttpError(http_err) => {
                OrderAmendmentError::Other(format!("HTTP error: {http_err}"))
            }
            crate::binance::usdm::Errors::Error(msg) => OrderAmendmentError::Other(msg),
            crate::binance::usdm::Errors::InvalidApiKey() => {
                OrderAmendmentError::InvalidKey("Invalid API key or signature".to_string())
            }
        })?;

        Ok(resp.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_amendment_deserialization() {
        let json = r#"{
            "amendments": [
                {
                    "amendmentId": 123456,
                    "symbol": "BTCUSDT",
                    "orderId": 789012,
                    "clientOrderId": "myOrder1",
                    "time": 1641038400000,
                    "amendmentType": "PRICE",
                    "origPrice": "50000.0",
                    "origQty": "1.0",
                    "price": "51000.0",
                    "quantity": "1.0",
                    "status": "SUCCESS",
                    "priceMatch": "NONE"
                }
            ],
            "total": 1
        }"#;

        let response: OrderAmendmentResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.total, 1);
        assert_eq!(response.amendments.len(), 1);
        assert_eq!(response.amendments[0].amendment_id, 123456);
        assert_eq!(response.amendments[0].symbol, "BTCUSDT");
    }
}
