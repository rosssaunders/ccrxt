use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::{
    RestResult,
    enums::{OrderSide, OrderStatus, OrderType, PositionSide, TimeInForce, WorkingType},
};

/// Endpoint path for getting all open orders.
const OPEN_ORDERS_ENDPOINT: &str = "/fapi/v1/openOrders";

/// Request parameters for the Current All Open Orders endpoint.
///
/// Retrieves all open orders for a given symbol, or all symbols if none is specified, on Binance USDM Futures.
///
/// All fields are optional unless specified by the API.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetOpenOrdersRequest {
    /// Trading symbol (e.g., "BTCUSDT"). Optional.
    /// Must match a valid symbol listed on Binance USDM Futures.
    /// If omitted, returns open orders for all symbols.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Cow<'static, str>>,

    /// The number of milliseconds the request is valid for. Optional.
    /// Used to specify how long the request remains valid after timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp (milliseconds since epoch). Required.
    /// Must be the current server time in milliseconds.
    pub timestamp: u64,
}

/// Represents a single open order returned by the Current All Open Orders endpoint.
///
/// All fields are deserialized from the Binance USDM Futures API response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenOrder {
    /// Trading symbol for the order.
    pub symbol: Cow<'static, str>,

    /// Unique order ID assigned by Binance.
    #[serde(rename = "orderId")]
    pub order_id: u64,

    /// Client-supplied order ID.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: Cow<'static, str>,

    /// Price at which the order is placed.
    pub price: Cow<'static, str>,

    /// Original quantity of the order.
    #[serde(rename = "origQty")]
    pub orig_qty: Cow<'static, str>,

    /// Executed quantity so far.
    #[serde(rename = "executedQty")]
    pub executed_qty: Cow<'static, str>,

    /// Cumulative quote asset transacted for the order.
    #[serde(rename = "cumQuote")]
    pub cum_quote: Option<Cow<'static, str>>,

    /// Average price for the order.
    #[serde(rename = "avgPrice")]
    pub avg_price: Option<Cow<'static, str>>,

    /// Status of the order (see OrderStatus enum).
    pub status: OrderStatus,

    /// Time in force policy for the order (see TimeInForce enum).
    pub time_in_force: TimeInForce,

    /// Type of the order (see OrderType enum).
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Side of the order (buy/sell).
    pub side: OrderSide,

    /// Position side (LONG/SHORT/BOTH).
    pub position_side: PositionSide,

    /// If true, the order is reduce-only.
    pub reduce_only: Option<bool>,

    /// Stop price for stop orders.
    pub stop_price: Option<Cow<'static, str>>,

    /// Working type (CONTRACT_PRICE/MARK_PRICE).
    pub working_type: WorkingType,

    /// If true, the conditional order trigger is protected.
    pub price_protect: Option<bool>,

    /// Price match mode (e.g., NONE).
    pub price_match: Option<Cow<'static, str>>,

    /// Self-trade prevention mode (e.g., NONE).
    pub self_trade_prevention_mode: Option<Cow<'static, str>>,

    /// If true, this is a close-all order.
    pub close_position: Option<bool>,

    /// Order time (milliseconds since epoch).
    pub time: Option<u64>,

    /// Update time (milliseconds since epoch).
    pub update_time: Option<u64>,

    /// Activation price (for trailing stop market orders).
    pub activate_price: Option<Cow<'static, str>>,

    /// Callback rate (for trailing stop market orders).
    pub price_rate: Option<Cow<'static, str>>,

    /// Original order type (for trailing stop market orders).
    pub orig_type: Option<OrderType>,

    /// Good-till-date (for GTD orders).
    pub good_till_date: Option<u64>,
}

impl UsdmClient {
    /// Current All Open Orders
    ///
    /// Get all open orders for a symbol, or all symbols if none is specified, on Binance USDM Futures.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Current-All-Open-Orders)
    ///
    /// Rate limit: 1 for single symbol, 40 if symbol omitted
    ///
    /// # Arguments
    /// * `params` - The request parameters for open orders
    ///
    /// # Returns
    /// A vector of open orders for the account
    pub async fn get_open_orders(
        &self,
        params: GetOpenOrdersRequest,
    ) -> RestResult<Vec<OpenOrder>> {
        // Determine rate limit based on whether a symbol is specified
        let rate_limit = if params.symbol.is_some() { 1 } else { 40 };
        self.send_get_signed_request(OPEN_ORDERS_ENDPOINT, params, rate_limit, true)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_open_orders_request_serialization_with_symbol() {
        let request = GetOpenOrdersRequest {
            symbol: Some(Cow::Borrowed("BTCUSDT")),
            recv_window: None,
            timestamp: 1234567890,
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=BTCUSDT&timestamp=1234567890");
        assert!(!serialized.contains("api_key"));
        assert!(!serialized.contains("api_secret"));
    }

    #[test]
    fn test_get_open_orders_request_serialization_all_symbols() {
        let request = GetOpenOrdersRequest {
            symbol: None,
            recv_window: None,
            timestamp: 1234567890,
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "timestamp=1234567890");
    }

    #[test]
    fn test_open_order_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "orderId": 1234567890,
            "clientOrderId": "my_order_123",
            "price": "45380.10",
            "origQty": "0.100",
            "executedQty": "0.050",
            "status": "PARTIALLY_FILLED",
            "timeInForce": "GTC",
            "type": "LIMIT",
            "side": "BUY",
            "positionSide": "LONG",
            "workingType": "CONTRACT_PRICE"
        }"#;
        let order: OpenOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.symbol, "BTCUSDT");
        assert_eq!(order.order_id, 1234567890);
        assert_eq!(order.client_order_id, "my_order_123");
        assert_eq!(order.price, "45380.10");
        assert_eq!(order.orig_qty, "0.100");
        assert_eq!(order.executed_qty, "0.050");
        assert!(matches!(order.status, OrderStatus::PartiallyFilled));
        assert!(matches!(order.time_in_force, TimeInForce::GTC));
        assert!(matches!(order.order_type, OrderType::Limit));
        assert!(matches!(order.side, OrderSide::Buy));
        assert!(matches!(order.position_side, PositionSide::Long));
        assert!(matches!(order.working_type, WorkingType::ContractPrice));
    }

    #[test]
    fn test_open_order_stop_order() {
        let json = r#"{
            "symbol": "ETHUSDT",
            "orderId": 9876543210,
            "clientOrderId": "stop_order_456",
            "price": "3000.00",
            "origQty": "1.000",
            "executedQty": "0.000",
            "status": "NEW",
            "timeInForce": "GTC",
            "type": "STOP",
            "side": "SELL",
            "positionSide": "SHORT",
            "workingType": "MARK_PRICE"
        }"#;
        let order: OpenOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.symbol, "ETHUSDT");
        assert!(matches!(order.status, OrderStatus::New));
        assert!(matches!(order.order_type, OrderType::Stop));
        assert_eq!(order.executed_qty, "0.000");
    }

    #[test]
    fn test_open_orders_array_deserialization() {
        let json = r#"[
            {
                "symbol": "BTCUSDT",
                "orderId": 1234567890,
                "clientOrderId": "order1",
                "price": "45380.10",
                "origQty": "0.100",
                "executedQty": "0.000",
                "status": "NEW",
                "timeInForce": "GTC",
                "type": "LIMIT",
                "side": "BUY",
                "positionSide": "LONG",
                "workingType": "CONTRACT_PRICE"
            },
            {
                "symbol": "ETHUSDT",
                "orderId": 1234567891,
                "clientOrderId": "order2",
                "price": "3070.50",
                "origQty": "0.500",
                "executedQty": "0.100",
                "status": "PARTIALLY_FILLED",
                "timeInForce": "IOC",
                "type": "LIMIT",
                "side": "SELL",
                "positionSide": "SHORT",
                "workingType": "MARK_PRICE"
            }
        ]"#;
        let orders: Vec<OpenOrder> = serde_json::from_str(json).unwrap();
        assert_eq!(orders.len(), 2);
        assert_eq!(orders[0].symbol, "BTCUSDT");
        assert_eq!(orders[0].order_id, 1234567890);
        assert!(matches!(orders[0].status, OrderStatus::New));
        assert_eq!(orders[1].symbol, "ETHUSDT");
        assert_eq!(orders[1].order_id, 1234567891);
        assert!(matches!(orders[1].status, OrderStatus::PartiallyFilled));
    }

    #[test]
    fn test_open_orders_empty_response() {
        let json = r#"[]"#;
        let orders: Vec<OpenOrder> = serde_json::from_str(json).unwrap();
        assert_eq!(orders.len(), 0);
    }

    #[test]
    fn test_open_order_take_profit_market() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "orderId": 3333333333,
            "clientOrderId": "tp_order",
            "price": "0",
            "origQty": "0.200",
            "executedQty": "0.000",
            "status": "NEW",
            "timeInForce": "GTC",
            "type": "TAKE_PROFIT_MARKET",
            "side": "SELL",
            "positionSide": "LONG",
            "workingType": "MARK_PRICE"
        }"#;
        let order: OpenOrder = serde_json::from_str(json).unwrap();
        assert!(matches!(order.order_type, OrderType::TakeProfitMarket));
        assert_eq!(order.price, "0"); // TP market orders don't have price
    }

    #[test]
    fn test_open_order_both_position_side() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "orderId": 4444444444,
            "clientOrderId": "both_position",
            "price": "45000.00",
            "origQty": "0.100",
            "executedQty": "0.000",
            "status": "NEW",
            "timeInForce": "GTC",
            "type": "LIMIT",
            "side": "BUY",
            "positionSide": "BOTH",
            "workingType": "CONTRACT_PRICE"
        }"#;
        let order: OpenOrder = serde_json::from_str(json).unwrap();
        assert!(matches!(order.position_side, PositionSide::Both));
    }
}
