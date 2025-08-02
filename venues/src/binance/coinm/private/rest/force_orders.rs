use serde::{Deserialize, Serialize};

use crate::binance::{
    coinm::{
        AutoCloseType, OrderSide, OrderStatus, OrderType, PositionSide, RestResult, TimeInForce,
        WorkingType, private::rest::client::RestClient,
    },
};

const FORCE_ORDERS_ENDPOINT: &str = "/dapi/v1/forceOrders";

/// Request parameters for getting user's force orders (GET /dapi/v1/forceOrders).
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetForceOrdersRequest {
    /// Trading symbol (e.g., "BTCUSD_PERP"). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Auto close type. "LIQUIDATION" for liquidation orders, "ADL" for ADL orders. Optional.
    #[serde(rename = "autoCloseType", skip_serializing_if = "Option::is_none")]
    pub auto_close_type: Option<AutoCloseType>,

    /// Start time in milliseconds. Optional.
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time in milliseconds. Optional.
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Number of records to return. Default 50; max 100. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Receive window in milliseconds.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp in milliseconds.
    pub timestamp: u64,
}

/// Individual force order entry.
#[derive(Debug, Clone, Deserialize)]
pub struct ForceOrder {
    /// Order ID.
    #[serde(rename = "orderId")]
    pub order_id: u64,

    /// Trading symbol.
    pub symbol: String,

    /// Trading pair.
    pub pair: String,

    /// Order status.
    pub status: OrderStatus,

    /// Client order ID.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,

    /// Order price.
    pub price: String,

    /// Average price.
    #[serde(rename = "avgPrice")]
    pub avg_price: String,

    /// Original quantity.
    #[serde(rename = "origQty")]
    pub orig_qty: String,

    /// Executed quantity.
    #[serde(rename = "executedQty")]
    pub executed_qty: String,

    /// Cumulative base quantity.
    #[serde(rename = "cumBase")]
    pub cum_base: String,

    /// Time in force.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,

    /// Order type.
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Reduce only flag.
    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,

    /// Close position flag.
    #[serde(rename = "closePosition")]
    pub close_position: bool,

    /// Order side.
    pub side: OrderSide,

    /// Position side.
    #[serde(rename = "positionSide")]
    pub position_side: PositionSide,

    /// Stop price.
    #[serde(rename = "stopPrice")]
    pub stop_price: String,

    /// Working type.
    #[serde(rename = "workingType")]
    pub working_type: WorkingType,

    /// Price protect flag.
    #[serde(rename = "priceProtect")]
    pub price_protect: bool,

    /// Original order type.
    #[serde(rename = "origType")]
    pub orig_type: OrderType,

    /// Order time.
    pub time: u64,

    /// Update time.
    #[serde(rename = "updateTime")]
    pub update_time: u64,
}

/// Response for getting user's force orders (GET /dapi/v1/forceOrders).
pub type GetForceOrdersResponse = Vec<ForceOrder>;

impl RestClient {
    /// Gets user's force orders (USER_DATA) on Binance Coin-M Futures.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Users-Force-Orders
    ///
    /// GET /dapi/v1/forceOrders
    /// Weight: 20 with symbol, 50 without symbol
    /// Requires API key and signature.
    ///
    /// If "autoCloseType" is not sent, orders with both of the types will be returned.
    /// If "startTime" is not sent, data within 200 days before "endTime" can be queried.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`GetForceOrdersRequest`])
    ///
    /// # Returns
    /// A [`GetForceOrdersResponse`] - array of force order entries.
    pub async fn get_force_orders(
        &self,
        params: GetForceOrdersRequest,
    ) -> RestResult<GetForceOrdersResponse> {
        let weight = if params.symbol.is_some() { 20 } else { 50 };
        self.send_get_signed_request(
            FORCE_ORDERS_ENDPOINT,
            params,
            weight,
            false,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_force_orders_request_serialization_minimal() {
        let request = GetForceOrdersRequest {
            symbol: None,
            auto_close_type: None,
            start_time: None,
            end_time: None,
            limit: None,
            recv_window: None,
            timestamp: 1625097600000,
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "timestamp=1625097600000");
    }

    #[test]
    fn test_get_force_orders_request_serialization_with_symbol() {
        let request = GetForceOrdersRequest {
            symbol: Some("BTCUSD_PERP".to_string()),
            auto_close_type: None,
            start_time: None,
            end_time: None,
            limit: None,
            recv_window: None,
            timestamp: 1625097600000,
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSD_PERP"));
        assert!(serialized.contains("timestamp=1625097600000"));
    }

    #[test]
    fn test_get_force_orders_request_serialization_full() {
        let request = GetForceOrdersRequest {
            symbol: Some("BTCUSD_PERP".to_string()),
            auto_close_type: Some(AutoCloseType::Liquidation),
            start_time: Some(1625097600000),
            end_time: Some(1625184000000),
            limit: Some(50),
            recv_window: Some(5000),
            timestamp: 1625184000000,
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSD_PERP"));
        assert!(serialized.contains("autoCloseType=LIQUIDATION"));
        assert!(serialized.contains("startTime=1625097600000"));
        assert!(serialized.contains("endTime=1625184000000"));
        assert!(serialized.contains("limit=50"));
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1625184000000"));
    }

    #[test]
    fn test_force_order_response_deserialization() {
        let json = r#"[
            {
                "orderId": 6071832819,
                "symbol": "BTCUSD_PERP",
                "pair": "BTCUSD",
                "status": "FILLED",
                "clientOrderId": "autoclose-1625097600000",
                "price": "50000",
                "avgPrice": "49950",
                "origQty": "0.1",
                "executedQty": "0.1",
                "cumBase": "0.00001",
                "timeInForce": "IOC",
                "type": "LIMIT",
                "reduceOnly": true,
                "closePosition": false,
                "side": "SELL",
                "positionSide": "BOTH",
                "stopPrice": "0",
                "workingType": "CONTRACT_PRICE",
                "priceProtect": false,
                "origType": "LIMIT",
                "time": 1625097600000,
                "updateTime": 1625097600001
            }
        ]"#;
        let response: GetForceOrdersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);
        let order = &response[0];
        assert_eq!(order.order_id, 6071832819);
        assert_eq!(order.symbol, "BTCUSD_PERP");
        assert_eq!(order.pair, "BTCUSD");
        assert_eq!(order.status, OrderStatus::Filled);
        assert_eq!(order.client_order_id, "autoclose-1625097600000");
        assert_eq!(order.price, "50000");
        assert_eq!(order.avg_price, "49950");
        assert_eq!(order.orig_qty, "0.1");
        assert_eq!(order.executed_qty, "0.1");
        assert_eq!(order.cum_base, "0.00001");
        assert_eq!(order.time_in_force, TimeInForce::IOC);
        assert_eq!(order.order_type, OrderType::Limit);
        assert_eq!(order.reduce_only, true);
        assert_eq!(order.close_position, false);
        assert_eq!(order.side, OrderSide::Sell);
        assert_eq!(order.position_side, PositionSide::Both);
        assert_eq!(order.stop_price, "0");
        assert_eq!(order.working_type, WorkingType::ContractPrice);
        assert_eq!(order.price_protect, false);
        assert_eq!(order.orig_type, OrderType::Limit);
        assert_eq!(order.time, 1625097600000);
        assert_eq!(order.update_time, 1625097600001);
    }

    #[test]
    fn test_force_order_response_deserialization_empty() {
        let json = r#"[]"#;
        let response: GetForceOrdersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 0);
    }
}
