//! Query current open order on Binance USDM REST API.

use std::borrow::Cow;

use chrono::Utc;
use reqwest::Method;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::binance::usdm::{enums::*, private::rest::client::RestClient, signing::sign_query};

/// Error type for USDM open order query endpoints.
#[derive(Debug, Error, Clone, Deserialize)]
#[serde(tag = "code", content = "msg")]
pub enum OpenOrderError {
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
pub struct OpenOrderErrorResponse {
    /// Error code returned by Binance.
    pub code: i64,
    /// Error message returned by Binance.
    pub msg: String,
}

impl From<OpenOrderErrorResponse> for OpenOrderError {
    fn from(e: OpenOrderErrorResponse) -> Self {
        match e.code {
            -2015 | -2014 => OpenOrderError::InvalidKey(e.msg),
            -2013 => OpenOrderError::OrderNotFound(e.msg),
            -1003 => OpenOrderError::RateLimit(e.msg),
            _ => OpenOrderError::Other(e.msg),
        }
    }
}

/// Result type for open order operations.
pub type OpenOrderResult<T> = Result<T, OpenOrderError>;

/// Request to query a current open order.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentOpenOrderRequest {
    /// API key (securely stored)
    #[serde(skip)]
    pub api_key: SecretString,
    /// API secret (securely stored)
    #[serde(skip)]
    pub api_secret: SecretString,
    /// Symbol
    pub symbol: Cow<'static, str>,
    /// Order ID to query (either this or origClientOrderId must be provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,
    /// Original client order ID (either this or orderId must be provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id: Option<Cow<'static, str>>,
}

/// Response for a current open order query.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentOpenOrderResponse {
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
    /// Reduce only flag
    pub reduce_only: bool,
    /// Close position flag
    pub close_position: bool,
    /// Stop price
    pub stop_price: Option<Cow<'static, str>>,
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
    /// Order time
    pub time: u64,
    /// Update time
    pub update_time: u64,
    /// Activation price (for trailing stop orders)
    pub activate_price: Option<Cow<'static, str>>,
    /// Price rate (for trailing stop orders)
    pub price_rate: Option<Cow<'static, str>>,
}

impl RestClient {
    /// Query current open order (GET /fapi/v1/openOrder)
    /// [Binance API docs](https://binance-docs.github.io/apidocs/futures/en/#query-current-open-order-user_data)
    pub async fn get_current_open_order(
        &self,
        params: CurrentOpenOrderRequest,
    ) -> OpenOrderResult<CurrentOpenOrderResponse> {
        use tracing::debug;

        use crate::binance::usdm::request::execute_request;

        // 1. Prepare endpoint and method
        let endpoint = "/fapi/v1/openOrder";
        let method = Method::GET;
        let url = format!("{}{}", self.base_url, endpoint);

        // 2. Add timestamp and recvWindow
        let timestamp = Utc::now().timestamp_millis();
        let recv_window = 5000u64;

        // 3. Serialize params to query string
        let mut query_pairs = serde_urlencoded::to_string(&params)
            .map_err(|e| OpenOrderError::Other(format!("Failed to serialize params: {e}")))?;
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
            .map_err(|e| OpenOrderError::Other(format!("Rate limiting error: {e}")))?;

        debug!(
            endpoint = endpoint,
            "Sending current open order query request"
        );

        // 7. Execute request
        let resp = execute_request::<CurrentOpenOrderResponse>(
            &self.client,
            &url,
            method,
            Some(headers),
            Some(&query_pairs),
        )
        .await
        .map_err(|e| match e {
            crate::binance::usdm::Errors::ApiError(api_err) => {
                OpenOrderError::Other(format!("API error: {api_err}"))
            }
            crate::binance::usdm::Errors::HttpError(http_err) => {
                OpenOrderError::Other(format!("HTTP error: {http_err}"))
            }
            crate::binance::usdm::Errors::Error(msg) => OpenOrderError::Other(msg),
            crate::binance::usdm::Errors::InvalidApiKey() => {
                OpenOrderError::InvalidKey("Invalid API key or signature".to_string())
            }
        })?;

        Ok(resp.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_open_order_response_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "orderId": 12345,
            "clientOrderId": "myOrder1",
            "price": "50000.0",
            "origQty": "1.0",
            "executedQty": "0.5",
            "cumQuote": "25000.0",
            "status": "PARTIALLY_FILLED",
            "timeInForce": "GTC",
            "type": "LIMIT",
            "side": "BUY",
            "positionSide": "BOTH",
            "reduceOnly": false,
            "closePosition": false,
            "stopPrice": null,
            "workingType": "CONTRACT_PRICE",
            "priceProtect": false,
            "origType": "LIMIT",
            "priceMatch": "NONE",
            "selfTradePreventionMode": "NONE",
            "goodTillDate": 0,
            "time": 1641038400000,
            "updateTime": 1641038400000,
            "activatePrice": null,
            "priceRate": null
        }"#;

        let response: CurrentOpenOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTCUSDT");
        assert_eq!(response.order_id, 12345);
        assert_eq!(response.status, OrderStatus::PartiallyFilled);
    }
}
