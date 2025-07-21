use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::UsdmClient;
use crate::binance::usdm::RestResult;
use crate::binance::usdm::enums::{
    OrderSide, OrderStatus, OrderType, PositionSide, TimeInForce, WorkingType,
};

/// Endpoint path for the All Orders API.
const ALL_ORDERS_ENDPOINT: &str = "/fapi/v1/allOrders";

/// Request parameters for the All Orders endpoint.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAllOrdersRequest {
    /// Trading symbol (e.g., "BTCUSDT"). Required.
    /// Must be a valid symbol listed on Binance USDM Futures.
    pub symbol: Cow<'static, str>,

    /// Filter by order ID. Returns orders >= this ID. Optional.
    /// If set, only orders with orderId >= this value are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,

    /// Filter by start time (milliseconds since epoch). Optional.
    /// Only orders created after this time are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// Filter by end time (milliseconds since epoch). Optional.
    /// Only orders created before this time are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Maximum number of orders to return. Default 500, max 1000. Optional.
    /// If not set, the API default is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Represents a single order returned by the All Orders endpoint.
///
/// All fields use enums for fixed sets of values, matching the Binance API.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllOrder {
    /// Trading symbol for the order.
    pub symbol: Cow<'static, str>,

    /// Unique order ID assigned by Binance.
    pub order_id: u64,

    /// Client order ID provided by the user.
    pub client_order_id: Cow<'static, str>,

    /// Price at which the order was placed.
    pub price: Cow<'static, str>,

    /// Original quantity of the order.
    pub orig_qty: Cow<'static, str>,

    /// Executed quantity of the order.
    pub executed_qty: Cow<'static, str>,

    /// Status of the order. See [`OrderStatus`] enum.
    pub status: OrderStatus,

    /// Time in force for the order. See [`TimeInForce`] enum.
    pub time_in_force: TimeInForce,

    /// Type of the order. See [`OrderType`] enum.
    pub order_type: OrderType,

    /// Side of the order (buy/sell). See [`OrderSide`] enum.
    pub side: OrderSide,

    /// Position side for the order. See [`PositionSide`] enum.
    pub position_side: PositionSide,

    /// Working type for the order. See [`WorkingType`] enum.
    pub working_type: WorkingType,
}

impl UsdmClient {
    /// All Orders (USER_DATA)
    ///
    /// Get all account orders; active, canceled, or filled.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/All-Orders
    ///
    /// Rate limit: 5
    ///
    /// # Arguments
    /// * `params` - The request parameters for All Orders
    ///
    /// # Returns
    /// Returns a list of all orders for the account.
    pub async fn get_all_orders(&self, params: GetAllOrdersRequest) -> RestResult<Vec<AllOrder>> {
        self.send_signed_request(ALL_ORDERS_ENDPOINT, Method::GET, params, 5, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_get_all_orders_request_serialization() {
        let req = GetAllOrdersRequest {
            symbol: Cow::Borrowed("BTCUSDT"),
            order_id: Some(123456),
            start_time: Some(1625097600000),
            end_time: Some(1625098600000),
            limit: Some(100),
        };
        let serialized = serde_urlencoded::to_string(&req).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("orderId=123456"));
        assert!(serialized.contains("startTime=1625097600000"));
        assert!(serialized.contains("endTime=1625098600000"));
        assert!(serialized.contains("limit=100"));
    }

    #[test]
    fn test_all_order_response_deserialization() {
        let json = r#"{
            \"symbol\": \"BTCUSDT\",
            \"orderId\": 123456,
            \"clientOrderId\": \"myOrder\",
            \"price\": \"50000.00\",
            \"origQty\": \"0.10000000\",
            \"executedQty\": \"0.05000000\",
            \"status\": \"NEW\",
            \"timeInForce\": \"GTC\",
            \"type\": \"LIMIT\",
            \"side\": \"BUY\",
            \"positionSide\": \"BOTH\",
            \"workingType\": \"CONTRACT_PRICE\"
        }"#;
        let order: AllOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.symbol, "BTCUSDT");
        assert_eq!(order.order_id, 123456);
        assert_eq!(order.client_order_id, "myOrder");
        assert_eq!(order.price, "50000.00");
        assert_eq!(order.orig_qty, "0.10000000");
        assert_eq!(order.executed_qty, "0.05000000");
        assert_eq!(order.status, OrderStatus::New);
        assert_eq!(order.time_in_force, TimeInForce::GTC);
        assert_eq!(order.order_type, OrderType::Limit);
        assert_eq!(order.side, OrderSide::Buy);
        assert_eq!(order.position_side, PositionSide::Both);
        assert_eq!(order.working_type, WorkingType::ContractPrice);
    }
}
