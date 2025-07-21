use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::UsdmClient;
use crate::binance::usdm::RestResult;
use crate::binance::usdm::enums::{AutoCloseType, OrderSide, OrderStatus, OrderType, TimeInForce};

const FORCE_ORDERS_ENDPOINT: &str = "/fapi/v1/forceOrders";

/// Request parameters for getting user's force orders.
///
/// Parameters for retrieving force orders (liquidation and ADL orders).
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetForceOrdersRequest {
    /// Trading symbol (e.g., "BTCUSDT"). Optional.
    /// If omitted, returns for all symbols.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Cow<'static, str>>,

    /// Start time for filtering orders (milliseconds since epoch). Optional.
    /// If not sent, data within 7 days before endTime can be queried.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time for filtering orders (milliseconds since epoch). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Auto close type filter. Optional.
    /// "LIQUIDATION" for liquidation orders, "ADL" for ADL orders.
    /// If not sent, orders with both types will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_close_type: Option<AutoCloseType>,

    /// Maximum number of results to return (default 50, max 100). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
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

    /// Order side (buy/sell).
    pub side: OrderSide,

    /// Order type.
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Time in force for the order.
    pub time_in_force: TimeInForce,

    /// Original order quantity.
    pub orig_qty: Cow<'static, str>,

    /// Order price.
    pub price: Cow<'static, str>,

    /// Average execution price.
    pub avg_price: Cow<'static, str>,

    /// Current order status.
    pub status: OrderStatus,

    /// Order timestamp (milliseconds since epoch).
    pub time: u64,

    /// Auto close type (LIQUIDATION or ADL).
    pub auto_close_type: AutoCloseType,
}

impl UsdmClient {
    /// User's Force Orders (USER_DATA)
    ///
    /// Get user's force orders (liquidation and ADL orders).
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Users-Force-Orders
    ///
    /// Rate limit: 20 if symbol is sent, 50 if symbol is not sent
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
        // Weight depends on whether symbol is provided
        let weight = if params.symbol.is_some() { 20 } else { 50 };

        self.send_signed_request(FORCE_ORDERS_ENDPOINT, Method::GET, params, weight, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_force_orders_request_serialization_minimal() {
        let request = GetForceOrdersRequest::default();
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_get_force_orders_request_serialization_with_symbol() {
        let request = GetForceOrdersRequest {
            symbol: Some("BTCUSDT".into()),
            ..Default::default()
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=BTCUSDT");
    }

    #[test]
    fn test_get_force_orders_request_serialization_full() {
        let request = GetForceOrdersRequest {
            symbol: Some("BTCUSDT".into()),
            start_time: Some(1625097600000),
            end_time: Some(1625184000000),
            auto_close_type: Some(AutoCloseType::Liquidation),
            limit: Some(100),
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("startTime=1625097600000"));
        assert!(serialized.contains("endTime=1625184000000"));
        assert!(serialized.contains("autoCloseType=LIQUIDATION"));
        assert!(serialized.contains("limit=100"));
    }

    #[test]
    fn test_force_order_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "orderId": 6071832819,
            "side": "SELL",
            "type": "LIMIT",
            "timeInForce": "IOC",
            "origQty": "0.001",
            "price": "10871.09",
            "avgPrice": "10913.21000",
            "status": "FILLED",
            "time": 1596107620044,
            "autoCloseType": "LIQUIDATION"
        }"#;

        let order: ForceOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.symbol, "BTCUSDT");
        assert_eq!(order.order_id, 6071832819);
        assert_eq!(order.side, OrderSide::Sell);
        assert_eq!(order.order_type, OrderType::Limit);
        assert_eq!(order.time_in_force, TimeInForce::IOC);
        assert_eq!(order.status, OrderStatus::Filled);
        assert_eq!(order.auto_close_type, AutoCloseType::Liquidation);
    }

    #[test]
    fn test_force_orders_response_deserialization() {
        let json = r#"[{
            "symbol": "BTCUSDT",
            "orderId": 6071832819,
            "side": "SELL",
            "type": "LIMIT",
            "timeInForce": "IOC",
            "origQty": "0.001",
            "price": "10871.09",
            "avgPrice": "10913.21000",
            "status": "FILLED",
            "time": 1596107620044,
            "autoCloseType": "LIQUIDATION"
        }]"#;

        let response: GetForceOrdersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);
        assert_eq!(response[0].symbol, "BTCUSDT");
        assert_eq!(response[0].order_id, 6071832819);
        assert_eq!(response[0].auto_close_type, AutoCloseType::Liquidation);
    }

    #[test]
    fn test_force_orders_response_deserialization_empty() {
        let json = "[]";
        let response: GetForceOrdersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 0);
    }
}
