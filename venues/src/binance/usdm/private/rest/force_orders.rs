//! Get user's force orders on Binance USDM REST API.

use std::borrow::Cow;

use chrono::Utc;
use reqwest::Method;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::binance::usdm::{enums::*, private::rest::client::RestClient, signing::sign_query};

/// Error type for USDM force orders endpoints.
#[derive(Debug, Error, Clone, Deserialize)]
#[serde(tag = "code", content = "msg")]
pub enum ForceOrdersError {
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
pub struct ForceOrdersErrorResponse {
    /// Error code returned by Binance.
    pub code: i64,
    /// Error message returned by Binance.
    pub msg: String,
}

impl From<ForceOrdersErrorResponse> for ForceOrdersError {
    fn from(e: ForceOrdersErrorResponse) -> Self {
        match e.code {
            -2015 | -2014 => ForceOrdersError::InvalidKey(e.msg),
            -1121 => ForceOrdersError::SymbolNotFound(e.msg),
            -1003 => ForceOrdersError::RateLimit(e.msg),
            _ => ForceOrdersError::Other(e.msg),
        }
    }
}

/// Result type for force orders operations.
pub type ForceOrdersResult<T> = Result<T, ForceOrdersError>;

/// Request to get user's force orders.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ForceOrdersRequest {
    /// API key (securely stored)
    #[serde(skip)]
    pub api_key: SecretString,
    /// API secret (securely stored)
    #[serde(skip)]
    pub api_secret: SecretString,
    /// Symbol (optional, if omitted returns for all symbols)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Cow<'static, str>>,
    /// Start time for the query (milliseconds since epoch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    /// End time for the query (milliseconds since epoch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    /// Auto close type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_close_type: Option<AutoCloseType>,
    /// Limit the number of results (default 50, max 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Response containing user's force orders.
pub type ForceOrdersResponse = Vec<ForceOrder>;

/// Individual force order record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForceOrder {
    /// Symbol
    pub symbol: Cow<'static, str>,
    /// Order ID
    pub order_id: u64,
    /// Order side
    pub side: OrderSide,
    /// Order type
    #[serde(rename = "type")]
    pub order_type: OrderType,
    /// Time in force
    pub time_in_force: TimeInForce,
    /// Original quantity
    pub orig_qty: Cow<'static, str>,
    /// Price
    pub price: Cow<'static, str>,
    /// Average price
    pub avg_price: Cow<'static, str>,
    /// Order status
    pub status: OrderStatus,
    /// Order time
    pub time: u64,
    /// Auto close type
    pub auto_close_type: AutoCloseType,
}

impl RestClient {
    /// Get user's force orders (GET /fapi/v1/forceOrders)
    /// [Binance API docs](https://binance-docs.github.io/apidocs/futures/en/#user-s-force-orders-user_data)
    pub async fn get_force_orders(
        &self,
        params: ForceOrdersRequest,
    ) -> ForceOrdersResult<ForceOrdersResponse> {
        use tracing::debug;

        use crate::binance::usdm::request::execute_request;

        // 1. Prepare endpoint and method
        let endpoint = "/fapi/v1/forceOrders";
        let method = Method::GET;
        let url = format!("{}{}", self.base_url, endpoint);

        // 2. Add timestamp and recvWindow
        let timestamp = Utc::now().timestamp_millis();
        let recv_window = 5000u64;

        // 3. Serialize params to query string
        let mut query_pairs = serde_urlencoded::to_string(&params)
            .map_err(|e| ForceOrdersError::Other(format!("Failed to serialize params: {e}")))?;
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
            .map_err(|e| ForceOrdersError::Other(format!("Rate limiting error: {e}")))?;

        debug!(endpoint = endpoint, "Sending force orders request");

        // 7. Execute request
        let resp = execute_request::<ForceOrdersResponse>(
            &self.client,
            &url,
            method,
            Some(headers),
            Some(&query_pairs),
        )
        .await
        .map_err(|e| match e {
            crate::binance::usdm::Errors::ApiError(api_err) => {
                ForceOrdersError::Other(format!("API error: {api_err}"))
            }
            crate::binance::usdm::Errors::HttpError(http_err) => {
                ForceOrdersError::Other(format!("HTTP error: {http_err}"))
            }
            crate::binance::usdm::Errors::Error(msg) => ForceOrdersError::Other(msg),
            crate::binance::usdm::Errors::InvalidApiKey() => {
                ForceOrdersError::InvalidKey("Invalid API key or signature".to_string())
            }
        })?;

        Ok(resp.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_force_order_deserialization() {
        let json = r#"[{
            "symbol": "BTCUSDT",
            "orderId": 12345,
            "side": "BUY",
            "type": "MARKET",
            "timeInForce": "IOC",
            "origQty": "1.0",
            "price": "0",
            "avgPrice": "50000.0",
            "status": "FILLED",
            "time": 1641038400000,
            "autoCloseType": "LIQUIDATION"
        }]"#;

        let response: ForceOrdersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);
        assert_eq!(response[0].symbol, "BTCUSDT");
        assert_eq!(response[0].order_id, 12345);
        assert_eq!(response[0].auto_close_type, AutoCloseType::Liquidation);
    }
}
