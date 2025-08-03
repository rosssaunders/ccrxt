use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::{
    RestResult,
    enums::{AutoCloseType, OrderSide, OrderStatus, OrderType, TimeInForce},
};

const FORCE_ORDERS_ENDPOINT: &str = "/fapi/v1/forceOrders";

/// Request parameters for getting user's force orders.
///
/// Parameters for retrieving force orders (liquidation and ADL orders).
/// Request parameters for the User's Force Orders endpoint.
///
/// Parameters for retrieving force orders (liquidation and ADL orders).
///
/// See [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Users-Force-Orders
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetForceOrdersRequest {
    /// Trading symbol (e.g., "BTCUSDT"). Optional.
    /// If omitted, returns for all symbols.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Cow<'static, str>>,

    /// Auto close type filter. Optional.
    /// "LIQUIDATION" for liquidation orders, "ADL" for ADL orders.
    /// If not sent, orders with both types will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_close_type: Option<AutoCloseType>,

    /// Start time for filtering orders (milliseconds since epoch). Optional.
    /// If not sent, data within 7 days before endTime can be queried.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time for filtering orders (milliseconds since epoch). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Maximum number of results to return (default 50, max 100). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// The value cannot be greater than 60000. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp in milliseconds since epoch.
    pub timestamp: u64,
}

/// Response containing user's force orders.
pub type GetForceOrdersResponse = Vec<ForceOrder>;

/// Individual force order record.
///
/// Represents a single force order (liquidation or ADL order) from the API.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForceOrder {
    /// Trading symbol.
    pub symbol: Cow<'static, str>,

    /// Unique order ID assigned by Binance.
    pub order_id: u64,

    /// Order status.
    pub status: OrderStatus,

    /// Client order ID.
    pub client_order_id: Cow<'static, str>,

    /// Order price.
    pub price: Cow<'static, str>,

    /// Average execution price.
    pub avg_price: Cow<'static, str>,

    /// Original order quantity.
    pub orig_qty: Cow<'static, str>,

    /// Executed quantity.
    pub executed_qty: Cow<'static, str>,

    /// Cumulative quote asset amount.
    pub cum_quote: Cow<'static, str>,

    /// Time in force for the order.
    pub time_in_force: TimeInForce,

    /// Order type.
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Reduce only flag.
    pub reduce_only: bool,

    /// Close position flag.
    pub close_position: bool,

    /// Order side (buy/sell).
    pub side: OrderSide,

    /// Position side.
    pub position_side: Cow<'static, str>,

    /// Stop price.
    pub stop_price: Cow<'static, str>,

    /// Working type.
    pub working_type: Cow<'static, str>,

    /// Original order type.
    pub orig_type: Cow<'static, str>,

    /// Order timestamp (milliseconds since epoch).
    pub time: u64,

    /// Update timestamp (milliseconds since epoch).
    pub update_time: u64,

    /// Auto close type (LIQUIDATION or ADL).
    pub auto_close_type: AutoCloseType,
}

impl UsdmClient {
    /// User's Force Orders (USER_DATA)
    ///
    /// Query user's Force Orders (liquidation and ADL orders).
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Users-Force-Orders
    ///
    /// Rate limit: 20 with symbol, 50 without symbol
    ///
    /// # Arguments
    /// * `params` - The request parameters for force orders
    ///
    /// # Returns
    /// Returns a list of force orders for the account.
    pub async fn get_force_orders(
        &self,
        params: GetForceOrdersRequest,
    ) -> RestResult<GetForceOrdersResponse> {
        let weight = if params.symbol.is_some() { 20 } else { 50 };
        self.send_get_signed_request(FORCE_ORDERS_ENDPOINT, params, weight, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_force_orders_request_serialization_minimal() {
        let request = GetForceOrdersRequest {
            timestamp: 1234567890,
            ..Default::default()
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "timestamp=1234567890");
    }

    #[test]
    fn test_get_force_orders_request_serialization_with_symbol() {
        let request = GetForceOrdersRequest {
            symbol: Some("BTCUSDT".into()),
            timestamp: 1234567890,
            ..Default::default()
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("timestamp=1234567890"));
    }

    #[test]
    fn test_get_force_orders_request_serialization_full() {
        let request = GetForceOrdersRequest {
            symbol: Some("BTCUSDT".into()),
            auto_close_type: Some(AutoCloseType::Liquidation),
            start_time: Some(1625097600000),
            end_time: Some(1625184000000),
            limit: Some(100),
            recv_window: Some(60000),
            timestamp: 1234567890,
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("autoCloseType=LIQUIDATION"));
        assert!(serialized.contains("startTime=1625097600000"));
        assert!(serialized.contains("endTime=1625184000000"));
        assert!(serialized.contains("limit=100"));
        assert!(serialized.contains("recvWindow=60000"));
        assert!(serialized.contains("timestamp=1234567890"));
    }

    #[test]
    fn test_force_order_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "orderId": 6071832819,
            "status": "FILLED",
            "clientOrderId": "autoclose-1596107620040000020",
            "price": "10871.09",
            "avgPrice": "10913.21000",
            "origQty": "0.001",
            "executedQty": "0.001",
            "cumQuote": "10.91321",
            "timeInForce": "IOC",
            "type": "LIMIT",
            "reduceOnly": false,
            "closePosition": false,
            "side": "SELL",
            "positionSide": "BOTH",
            "stopPrice": "0",
            "workingType": "CONTRACT_PRICE",
            "origType": "LIMIT",
            "time": 1596107620044,
            "updateTime": 1596107620087,
            "autoCloseType": "LIQUIDATION"
        }"#;

        let order: ForceOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.symbol, "BTCUSDT");
        assert_eq!(order.order_id, 6071832819);
        assert_eq!(order.status, OrderStatus::Filled);
        assert_eq!(order.client_order_id, "autoclose-1596107620040000020");
        assert_eq!(order.price, "10871.09");
        assert_eq!(order.avg_price, "10913.21000");
        assert_eq!(order.orig_qty, "0.001");
        assert_eq!(order.executed_qty, "0.001");
        assert_eq!(order.cum_quote, "10.91321");
        assert_eq!(order.time_in_force, TimeInForce::IOC);
        assert_eq!(order.order_type, OrderType::Limit);
        assert!(!order.reduce_only);
        assert!(!order.close_position);
        assert_eq!(order.side, OrderSide::Sell);
        assert_eq!(order.position_side, "BOTH");
        assert_eq!(order.stop_price, "0");
        assert_eq!(order.working_type, "CONTRACT_PRICE");
        assert_eq!(order.orig_type, "LIMIT");
        assert_eq!(order.time, 1596107620044);
        assert_eq!(order.update_time, 1596107620087);
        assert_eq!(order.auto_close_type, AutoCloseType::Liquidation);
    }

    #[test]
    fn test_force_orders_response_deserialization() {
        let json = r#"[{
            "symbol": "BTCUSDT",
            "orderId": 6071832819,
            "status": "FILLED",
            "clientOrderId": "autoclose-1596107620040000020",
            "price": "10871.09",
            "avgPrice": "10913.21000",
            "origQty": "0.001",
            "executedQty": "0.001",
            "cumQuote": "10.91321",
            "timeInForce": "IOC",
            "type": "LIMIT",
            "reduceOnly": false,
            "closePosition": false,
            "side": "SELL",
            "positionSide": "BOTH",
            "stopPrice": "0",
            "workingType": "CONTRACT_PRICE",
            "origType": "LIMIT",
            "time": 1596107620044,
            "updateTime": 1596107620087,
            "autoCloseType": "LIQUIDATION"
        }]"#;

        let response: GetForceOrdersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);
        let order = &response[0];
        assert_eq!(order.symbol, "BTCUSDT");
        assert_eq!(order.order_id, 6071832819);
        assert_eq!(order.status, OrderStatus::Filled);
        assert_eq!(order.client_order_id, "autoclose-1596107620040000020");
        assert_eq!(order.auto_close_type, AutoCloseType::Liquidation);
    }

    #[test]
    fn test_force_orders_response_deserialization_empty() {
        let json = "[]";
        let response: GetForceOrdersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 0);
    }
}
