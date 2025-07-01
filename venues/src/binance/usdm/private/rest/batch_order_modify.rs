//! Modify and cancel multiple orders in batches on Binance USDM REST API.

use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use thiserror::Error;

use crate::binance::usdm::enums::*;
use crate::binance::usdm::private::rest::client::RestClient;
use crate::binance::usdm::signing::sign_query;
use chrono::Utc;
use reqwest::Method;

/// Error type for USDM batch order endpoints.
#[derive(Debug, Error, Clone, Deserialize)]
#[serde(tag = "code", content = "msg")]
pub enum BatchOrderError {
    /// Invalid API key or signature.
    #[error("Invalid API key or signature: {0}")]
    InvalidKey(String),
    /// Order not found.
    #[error("Order not found: {0}")]
    OrderNotFound(String),
    /// Insufficient balance.
    #[error("Insufficient balance: {0}")]
    InsufficientBalance(String),
    /// Rate limit exceeded.
    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),
    /// Any other error.
    #[error("Other error: {0}")]
    Other(String),
}

/// Error response from Binance REST API.
#[derive(Debug, Clone, Deserialize)]
pub struct BatchOrderErrorResponse {
    /// Error code returned by Binance.
    pub code: i64,
    /// Error message returned by Binance.
    pub msg: String,
}

impl From<BatchOrderErrorResponse> for BatchOrderError {
    fn from(e: BatchOrderErrorResponse) -> Self {
        match e.code {
            -2015 | -2014 => BatchOrderError::InvalidKey(e.msg),
            -2013 => BatchOrderError::OrderNotFound(e.msg),
            -2010 => BatchOrderError::InsufficientBalance(e.msg),
            -1003 => BatchOrderError::RateLimit(e.msg),
            _ => BatchOrderError::Other(e.msg),
        }
    }
}

/// Result type for batch order operations.
pub type BatchOrderResult<T> = Result<T, BatchOrderError>;

/// Request to modify multiple orders in a batch.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyBatchOrdersRequest {
    /// API key (securely stored)
    #[serde(skip)]
    pub api_key: SecretString,
    /// API secret (securely stored)
    #[serde(skip)]
    pub api_secret: SecretString,
    /// List of order modifications (max 5)
    pub batch_orders: Vec<ModifyBatchOrderItem>,
}

/// Individual order modification in a batch.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyBatchOrderItem {
    /// Order ID to modify (either this or origClientOrderId must be provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,
    /// Original client order ID (either this or orderId must be provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id: Option<Cow<'static, str>>,
    /// Symbol
    pub symbol: Cow<'static, str>,
    /// Order side
    pub side: OrderSide,
    /// Order quantity
    pub quantity: Cow<'static, str>,
    /// Order price
    pub price: Cow<'static, str>,
    /// Price match mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_match: Option<PriceMatch>,
}

/// Request to cancel multiple orders in a batch.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelBatchOrdersRequest {
    /// API key (securely stored)
    #[serde(skip)]
    pub api_key: SecretString,
    /// API secret (securely stored)
    #[serde(skip)]
    pub api_secret: SecretString,
    /// List of order cancellations
    pub order_id_list: Option<Vec<u64>>,
    /// List of client order IDs to cancel
    pub orig_client_order_id_list: Option<Vec<Cow<'static, str>>>,
    /// Symbol
    pub symbol: Cow<'static, str>,
}

/// Response for modified order in batch (can be success or error).
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum ModifyBatchOrderResponse {
    /// Successful modification
    Success(ModifyBatchOrderSuccess),
    /// Error during modification
    Error { code: i64, msg: String },
}

/// Successful order modification response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyBatchOrderSuccess {
    /// Order ID
    pub order_id: u64,
    /// Symbol
    pub symbol: Cow<'static, str>,
    /// Trading pair
    pub pair: Cow<'static, str>,
    /// Order status
    pub status: OrderStatus,
    /// Client order ID
    pub client_order_id: Cow<'static, str>,
    /// Price
    pub price: Cow<'static, str>,
    /// Average price
    pub avg_price: Cow<'static, str>,
    /// Original quantity
    pub orig_qty: Cow<'static, str>,
    /// Executed quantity
    pub executed_qty: Cow<'static, str>,
    /// Cumulative quantity
    pub cum_qty: Cow<'static, str>,
    /// Cumulative base asset quantity
    pub cum_base: Cow<'static, str>,
    /// Time in force
    pub time_in_force: TimeInForce,
    /// Order type
    #[serde(rename = "type")]
    pub order_type: OrderType,
    /// Reduce only flag
    pub reduce_only: bool,
    /// Close position flag
    pub close_position: bool,
    /// Order side
    pub side: OrderSide,
    /// Position side
    pub position_side: PositionSide,
    /// Stop price
    pub stop_price: Cow<'static, str>,
    /// Working type
    pub working_type: WorkingType,
    /// Price protect flag
    pub price_protect: bool,
    /// Original order type
    pub orig_type: OrderType,
    /// Price match mode
    pub price_match: PriceMatch,
    /// Self trade prevention mode
    pub self_trade_prevention_mode: SelfTradePreventionMode,
    /// Good till date
    pub good_till_date: u64,
    /// Update time
    pub update_time: u64,
}

/// Response for cancelled order in batch (can be success or error).
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum CancelBatchOrderResponse {
    /// Successful cancellation
    Success(CancelBatchOrderSuccess),
    /// Error during cancellation
    Error { code: i64, msg: String },
}

/// Successful order cancellation response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelBatchOrderSuccess {
    /// Symbol
    pub symbol: Cow<'static, str>,
    /// Order ID
    pub order_id: u64,
    /// Client order ID
    pub client_order_id: Cow<'static, str>,
    /// Price
    pub price: Cow<'static, str>,
    /// Original quantity
    pub orig_qty: Cow<'static, str>,
    /// Executed quantity
    pub executed_qty: Cow<'static, str>,
    /// Cumulative quote asset transacted quantity
    pub cum_quote: Cow<'static, str>,
    /// Status
    pub status: OrderStatus,
    /// Time in force
    pub time_in_force: TimeInForce,
    /// Order type
    #[serde(rename = "type")]
    pub order_type: OrderType,
    /// Order side
    pub side: OrderSide,
    /// Position side
    pub position_side: PositionSide,
    /// Update time
    pub update_time: u64,
}

impl RestClient {
    /// Modify multiple orders (PUT /fapi/v1/batchOrders)
    /// [Binance API docs](https://binance-docs.github.io/apidocs/futures/en/#modify-multiple-orders-trade)
    pub async fn modify_batch_orders(
        &self,
        params: ModifyBatchOrdersRequest,
    ) -> BatchOrderResult<Vec<ModifyBatchOrderResponse>> {
        use crate::binance::usdm::request::execute_request;
        use tracing::debug;

        // 1. Prepare endpoint and method
        let endpoint = "/fapi/v1/batchOrders";
        let method = Method::PUT;
        let url = format!("{}{}", self.base_url, endpoint);

        // 2. Add timestamp and recvWindow
        let timestamp = Utc::now().timestamp_millis();
        let recv_window = 5000u64;

        // 3. Serialize batch orders to JSON string
        let batch_orders_json = serde_json::to_string(&params.batch_orders).map_err(|e| {
            BatchOrderError::Other(format!("Failed to serialize batch orders: {e}"))
        })?;

        // 4. Create query string
        let mut query_pairs = format!(
            "batchOrders={}&timestamp={}&recvWindow={}",
            urlencoding::encode(&batch_orders_json),
            timestamp,
            recv_window
        );

        // 5. Sign the query string
        let signature = sign_query(&query_pairs, &params.api_secret);
        query_pairs.push_str(&format!("&signature={signature}"));

        // 6. Set headers
        let headers = vec![
            ("X-MBX-APIKEY", params.api_key.expose_secret().to_string()),
            (
                "Content-Type",
                "application/x-www-form-urlencoded".to_string(),
            ),
        ];

        // 7. Rate limiting (batch order endpoint)
        self.rate_limiter
            .acquire_order()
            .await
            .map_err(|e| BatchOrderError::Other(format!("Rate limiting error: {e}")))?;

        debug!(endpoint = endpoint, "Sending modify batch orders request");

        // 8. Execute request
        let resp = execute_request::<Vec<ModifyBatchOrderResponse>>(
            &self.client,
            &url,
            method,
            Some(headers),
            Some(&query_pairs),
        )
        .await
        .map_err(|e| match e {
            crate::binance::usdm::Errors::ApiError(api_err) => {
                BatchOrderError::Other(format!("API error: {api_err}"))
            }
            crate::binance::usdm::Errors::HttpError(http_err) => {
                BatchOrderError::Other(format!("HTTP error: {http_err}"))
            }
            crate::binance::usdm::Errors::Error(msg) => BatchOrderError::Other(msg),
            crate::binance::usdm::Errors::InvalidApiKey() => {
                BatchOrderError::InvalidKey("Invalid API key or signature".to_string())
            }
        })?;

        Ok(resp.data)
    }

    /// Cancel multiple orders (DELETE /fapi/v1/batchOrders)
    /// [Binance API docs](https://binance-docs.github.io/apidocs/futures/en/#cancel-multiple-orders-trade)
    pub async fn cancel_batch_orders(
        &self,
        params: CancelBatchOrdersRequest,
    ) -> BatchOrderResult<Vec<CancelBatchOrderResponse>> {
        use crate::binance::usdm::request::execute_request;
        use tracing::debug;

        // 1. Prepare endpoint and method
        let endpoint = "/fapi/v1/batchOrders";
        let method = Method::DELETE;
        let url = format!("{}{}", self.base_url, endpoint);

        // 2. Add timestamp and recvWindow
        let timestamp = Utc::now().timestamp_millis();
        let recv_window = 5000u64;

        // 3. Serialize params to query string
        let mut query_pairs = serde_urlencoded::to_string(&params)
            .map_err(|e| BatchOrderError::Other(format!("Failed to serialize params: {e}")))?;
        if !query_pairs.is_empty() {
            query_pairs.push('&');
        }
        query_pairs.push_str(&format!("timestamp={timestamp}&recvWindow={recv_window}"));

        // 4. Sign the query string
        let signature = sign_query(&query_pairs, &params.api_secret);
        query_pairs.push_str(&format!("&signature={signature}"));

        // 5. Set headers
        let headers = vec![("X-MBX-APIKEY", params.api_key.expose_secret().to_string())];

        // 6. Rate limiting (batch order endpoint)
        self.rate_limiter
            .acquire_order()
            .await
            .map_err(|e| BatchOrderError::Other(format!("Rate limiting error: {e}")))?;

        debug!(endpoint = endpoint, "Sending cancel batch orders request");

        // 7. Execute request
        let resp = execute_request::<Vec<CancelBatchOrderResponse>>(
            &self.client,
            &url,
            method,
            Some(headers),
            Some(&query_pairs),
        )
        .await
        .map_err(|e| match e {
            crate::binance::usdm::Errors::ApiError(api_err) => {
                BatchOrderError::Other(format!("API error: {api_err}"))
            }
            crate::binance::usdm::Errors::HttpError(http_err) => {
                BatchOrderError::Other(format!("HTTP error: {http_err}"))
            }
            crate::binance::usdm::Errors::Error(msg) => BatchOrderError::Other(msg),
            crate::binance::usdm::Errors::InvalidApiKey() => {
                BatchOrderError::InvalidKey("Invalid API key or signature".to_string())
            }
        })?;

        Ok(resp.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modify_batch_order_item_serialization() {
        let item = ModifyBatchOrderItem {
            order_id: Some(12345),
            orig_client_order_id: None,
            symbol: "BTCUSDT".into(),
            side: OrderSide::Buy,
            quantity: "1.0".into(),
            price: "50000.0".into(),
            price_match: Some(PriceMatch::None),
        };

        let serialized = serde_json::to_string(&item).unwrap();
        assert!(serialized.contains("BTCUSDT"));
        assert!(serialized.contains("BUY"));
    }
}
