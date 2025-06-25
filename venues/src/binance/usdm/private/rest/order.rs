//! Place, modify, cancel, and query orders on Binance USDM REST API.

use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use thiserror::Error;

use crate::binance::usdm::enums::*;
use crate::binance::usdm::private::rest::client::RestClient;
use crate::binance::usdm::signing::sign_query;
use chrono::Utc;
use reqwest::Method;
use serde_urlencoded;

/// Error type for USDM order endpoints.
#[derive(Debug, Error, Clone, Deserialize)]
#[serde(tag = "code", content = "msg")]
pub enum OrderError {
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
pub struct OrderErrorResponse {
    /// Error code returned by Binance.
    pub code: i64,
    /// Error message returned by Binance.
    pub msg: String,
}

impl From<OrderErrorResponse> for OrderError {
    fn from(e: OrderErrorResponse) -> Self {
        match e.code {
            -2015 | -2014 => OrderError::InvalidKey(e.msg),
            -2013 => OrderError::OrderNotFound(e.msg),
            -2010 => OrderError::InsufficientBalance(e.msg),
            -1003 => OrderError::RateLimit(e.msg),
            _ => OrderError::Other(e.msg),
        }
    }
}

/// Result type for order endpoints.
pub type OrderResult<T> = std::result::Result<T, OrderError>;

/// Request parameters for placing a new order.
#[derive(Debug, Clone, Serialize)]
pub struct NewOrderRequest {
    /// API key (SecretString, securely stored)
    #[serde(skip_serializing)]
    pub api_key: SecretString,

    /// API secret (SecretString, securely stored)
    #[serde(skip_serializing)]
    pub api_secret: SecretString,

    /// Symbol (e.g., "BTCUSDT")
    pub symbol: Cow<'static, str>,

    /// Side (BUY or SELL)
    pub side: OrderSide,

    /// Position side (BOTH, LONG, SHORT)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_side: Option<PositionSide>,

    /// Order type (LIMIT, MARKET, etc.)
    pub order_type: OrderType,

    /// Time in force (GTC, IOC, FOK, GTX)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,

    /// Quantity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,

    /// Price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,

    /// Reduce only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,

    /// New client order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_client_order_id: Option<Cow<'static, str>>,

    /// Stop price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<f64>,

    /// Close position
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_position: Option<bool>,

    /// Activation price (for TRAILING_STOP_MARKET)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_price: Option<f64>,

    /// Callback rate (for TRAILING_STOP_MARKET)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_rate: Option<f64>,

    /// Working type (MARK_PRICE or CONTRACT_PRICE)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_type: Option<WorkingType>,

    /// Price protect
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_protect: Option<bool>,

    /// New order response type (ACK or RESULT)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_order_resp_type: Option<OrderResponseType>,

    /// Self trade prevention mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,
}

/// Response for a new order.
#[derive(Debug, Clone, Deserialize)]
pub struct NewOrderResponse {
    /// Symbol
    pub symbol: Cow<'static, str>,
    /// Order ID
    pub order_id: u64,
    /// Client order ID
    pub client_order_id: Cow<'static, str>,
    /// Transaction time (milliseconds since epoch)
    pub transact_time: u64,
    /// Price (as string for precision)
    pub price: Cow<'static, str>,
    /// Original quantity (as string for precision)
    pub orig_qty: Cow<'static, str>,
    /// Executed quantity (as string for precision)
    pub executed_qty: Cow<'static, str>,
    /// Cumulative quote asset transacted quantity (as string)
    pub cum_quote: Cow<'static, str>,
    /// Status
    pub status: OrderStatus,
    /// Time in force
    pub time_in_force: TimeInForce,
    /// Type
    pub order_type: OrderType,
    /// Side
    pub side: OrderSide,
    /// Position side
    pub position_side: PositionSide,
    /// Working type
    pub working_type: WorkingType,
}

impl RestClient {
    /// Place a new order (POST /fapi/v1/order)
    /// [Binance API docs](https://binance-docs.github.io/apidocs/futures/en/#new-order-trade)
    pub async fn new_order(&self, params: NewOrderRequest) -> OrderResult<NewOrderResponse> {
        use crate::binance::usdm::request::execute_request;
        use tracing::debug;
        // 1. Prepare endpoint and method
        let endpoint = "/fapi/v1/order";
        let method = Method::POST;
        let url = format!("{}{}", self.base_url, endpoint);

        // 2. Add timestamp and recvWindow
        let timestamp = Utc::now().timestamp_millis();
        let recv_window = 5000u64;

        // 3. Serialize params to query string (excluding api_key/api_secret)
        let mut query_pairs = serde_urlencoded::to_string(&params).map_err(|e| OrderError::Other(format!("Failed to serialize params: {e}")))?;
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

        // 6. Rate limiting (order endpoint)
        self.rate_limiter
            .acquire_order()
            .await
            .map_err(|e| OrderError::Other(format!("Rate limiting error: {e}")))?;

        debug!(endpoint = endpoint, "Sending new order request");

        // 7. Execute request
        let resp = execute_request::<NewOrderResponse>(
            &self.client,
            &url,
            method,
            Some(headers),
            Some(&query_pairs),
        )
        .await
        .map_err(|e| match e {
            crate::binance::usdm::Errors::ApiError(api_err) => {
                // Map to our endpoint error type
                OrderError::Other(format!("API error: {api_err}"))
            }
            crate::binance::usdm::Errors::HttpError(http_err) => OrderError::Other(format!("HTTP error: {http_err}")),
            crate::binance::usdm::Errors::Error(msg) => OrderError::Other(msg),
            crate::binance::usdm::Errors::InvalidApiKey() => OrderError::InvalidKey("Invalid API key or signature".to_string()),
        })?;

        Ok(resp.data)
    }
}
