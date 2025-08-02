use serde::{Deserialize, Serialize};

use crate::binance::coinm::{
    RestResult,
    enums::{
        OrderSide, OrderStatus, OrderType, PriceMatch, SelfTradePreventionMode, TimeInForce,
        WorkingType,
    },
    private::rest::client::RestClient,
};

const ORDER_ENDPOINT: &str = "/dapi/v1/order";

/// Request parameters for querying an order (GET /dapi/v1/order).
#[derive(Debug, Clone, Serialize, Default)]
pub struct QueryOrderRequest {
    /// Trading symbol (e.g., "BTCUSD_200925").
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Order ID. Either `orderId` or `origClientOrderId` must be sent.
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,

    /// Original client order ID. Either `orderId` or `origClientOrderId` must be sent.
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id: Option<String>,

    /// The value cannot be greater than 60000.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp (milliseconds since epoch).
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

/// Response for querying an order (GET /dapi/v1/order).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryOrderResponse {
    /// Average price of filled trades for this order.
    #[serde(rename = "avgPrice")]
    pub avg_price: String,

    /// Client order ID assigned by the user.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,

    /// Cumulative base asset amount filled.
    #[serde(rename = "cumBase")]
    pub cum_base: String,

    /// Executed quantity for this order.
    #[serde(rename = "executedQty")]
    pub executed_qty: String,

    /// Exchange-assigned order ID.
    #[serde(rename = "orderId")]
    pub order_id: u64,

    /// Original order quantity.
    #[serde(rename = "origQty")]
    pub orig_qty: String,

    /// Original order type (e.g., LIMIT, MARKET).
    #[serde(rename = "origType")]
    pub orig_type: OrderType,

    /// Price specified in the order.
    pub price: String,

    /// True if the order is reduce-only.
    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,

    /// Order side (BUY or SELL).
    pub side: OrderSide,

    /// Order status (e.g., NEW, FILLED, CANCELED).
    pub status: OrderStatus,

    /// Stop price for conditional orders.
    #[serde(rename = "stopPrice")]
    pub stop_price: String,

    /// True if this is a close-all position order.
    #[serde(rename = "closePosition")]
    pub close_position: bool,

    /// Trading symbol (e.g., BTCUSD_PERP).
    pub symbol: String,

    /// Trading pair (e.g., BTCUSD).
    pub pair: String,

    /// Order creation time (milliseconds since epoch).
    pub time: u64,

    /// Time in force for the order (e.g., GTC, IOC).
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,

    /// Order type (e.g., LIMIT, MARKET).
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Activation price (for trailing stop orders).
    #[serde(rename = "activatePrice")]
    pub activate_price: Option<String>,

    /// Callback rate (for trailing stop orders).
    #[serde(rename = "priceRate")]
    pub price_rate: Option<String>,

    /// Last update time for the order (milliseconds since epoch).
    #[serde(rename = "updateTime")]
    pub update_time: u64,

    /// Working type (e.g., CONTRACT_PRICE, MARK_PRICE).
    #[serde(rename = "workingType")]
    pub working_type: WorkingType,

    /// True if price protection is enabled for this order.
    #[serde(rename = "priceProtect")]
    pub price_protect: bool,

    /// Price match mode.
    #[serde(rename = "priceMatch")]
    pub price_match: PriceMatch,

    /// Self-trade prevention mode.
    #[serde(rename = "selfTradePreventionMode")]
    pub self_trade_prevention_mode: SelfTradePreventionMode,
}

impl RestClient {
    /// Query an order's status on Binance Coin-M Futures.
    ///
    /// Check an order's status.
    /// These orders will not be found:
    /// order status is CANCELED or EXPIRED AND order has NO filled trade AND created time + 3 days < current time
    /// order create time + 90 days < current time
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Query-Order
    ///
    /// GET /dapi/v1/order
    /// Weight: 1
    /// Requires API key and signature.
    pub async fn get_query_order(
        &self,
        params: QueryOrderRequest,
    ) -> RestResult<QueryOrderResponse> {
        let weight = 1;
        // HIGH PERFORMANCE: Use GET-specific function, no HTTP verb branching
        self.send_get_signed_request(ORDER_ENDPOINT, params, weight, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_order_request_with_order_id() {
        let request = QueryOrderRequest {
            symbol: "BTCUSD_PERP".to_string(),
            order_id: Some(12345),
            orig_client_order_id: None,
            recv_window: None,
            timestamp: 1625097600000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSD_PERP"));
        assert!(serialized.contains("orderId=12345"));
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(!serialized.contains("origClientOrderId"));
        assert!(!serialized.contains("recvWindow"));
    }

    #[test]
    fn test_query_order_request_with_client_order_id() {
        let request = QueryOrderRequest {
            symbol: "ETHUSD_PERP".to_string(),
            order_id: None,
            orig_client_order_id: Some("my_order_123".to_string()),
            recv_window: Some(5000),
            timestamp: 1625097600000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=ETHUSD_PERP"));
        assert!(serialized.contains("origClientOrderId=my_order_123"));
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(!serialized.contains("orderId"));
    }

    #[test]
    fn test_query_order_response_deserialization() {
        let json = r#"{
            "avgPrice": "0.0",
            "clientOrderId": "my_order_123",
            "cumBase": "0",
            "executedQty": "0",
            "orderId": 12345,
            "origQty": "10.5",
            "origType": "LIMIT",
            "price": "45000.0",
            "reduceOnly": false,
            "side": "BUY",
            "status": "NEW",
            "stopPrice": "0",
            "closePosition": false,
            "symbol": "BTCUSD_PERP",
            "pair": "BTCUSD",
            "time": 1625097600000,
            "timeInForce": "GTC",
            "type": "LIMIT",
            "activatePrice": null,
            "priceRate": null,
            "updateTime": 1625097600000,
            "workingType": "CONTRACT_PRICE",
            "priceProtect": false,
            "priceMatch": "NONE",
            "selfTradePreventionMode": "NONE"
        }"#;

        let response: QueryOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.avg_price, "0.0");
        assert_eq!(response.client_order_id, "my_order_123");
        assert_eq!(response.cum_base, "0");
        assert_eq!(response.executed_qty, "0");
        assert_eq!(response.order_id, 12345);
        assert_eq!(response.orig_qty, "10.5");
        assert_eq!(response.orig_type, OrderType::Limit);
        assert_eq!(response.price, "45000.0");
        assert!(!response.reduce_only);
        assert_eq!(response.side, OrderSide::Buy);
        assert_eq!(response.status, OrderStatus::New);
        assert_eq!(response.stop_price, "0");
        assert!(!response.close_position);
        assert_eq!(response.symbol, "BTCUSD_PERP");
        assert_eq!(response.pair, "BTCUSD");
        assert_eq!(response.time, 1625097600000);
        assert_eq!(response.time_in_force, TimeInForce::GTC);
        assert_eq!(response.order_type, OrderType::Limit);
        assert!(response.activate_price.is_none());
        assert!(response.price_rate.is_none());
        assert_eq!(response.update_time, 1625097600000);
        assert_eq!(response.working_type, WorkingType::ContractPrice);
        assert!(!response.price_protect);
        assert_eq!(response.price_match, PriceMatch::None);
        assert_eq!(
            response.self_trade_prevention_mode,
            SelfTradePreventionMode::None
        );
    }

    #[test]
    fn test_query_order_response_filled() {
        let json = r#"{
            "avgPrice": "44999.5",
            "clientOrderId": "order_456",
            "cumBase": "0.00222",
            "executedQty": "10.0",
            "orderId": 67890,
            "origQty": "10.0",
            "origType": "MARKET",
            "price": "0",
            "reduceOnly": true,
            "side": "SELL",
            "status": "FILLED",
            "stopPrice": "0",
            "closePosition": false,
            "symbol": "ETHUSD_PERP",
            "pair": "ETHUSD",
            "time": 1625097600000,
            "timeInForce": "IOC",
            "type": "MARKET",
            "updateTime": 1625097700000,
            "workingType": "CONTRACT_PRICE",
            "priceProtect": true,
            "priceMatch": "NONE",
            "selfTradePreventionMode": "EXPIRE_TAKER"
        }"#;

        let response: QueryOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.avg_price, "44999.5");
        assert_eq!(response.executed_qty, "10.0");
        assert_eq!(response.status, OrderStatus::Filled);
        assert_eq!(response.orig_type, OrderType::Market);
        assert!(response.reduce_only);
        assert_eq!(response.side, OrderSide::Sell);
        assert_eq!(
            response.self_trade_prevention_mode,
            SelfTradePreventionMode::ExpireTaker
        );
    }

    #[test]
    fn test_query_order_response_cancelled() {
        let json = r#"{
            "avgPrice": "0.0",
            "clientOrderId": "cancelled_order",
            "cumBase": "0",
            "executedQty": "0",
            "orderId": 99999,
            "origQty": "5.0",
            "origType": "STOP",
            "price": "43000.0",
            "reduceOnly": false,
            "side": "BUY",
            "status": "CANCELED",
            "stopPrice": "42500.0",
            "closePosition": false,
            "symbol": "BTCUSD_PERP",
            "pair": "BTCUSD",
            "time": 1625097600000,
            "timeInForce": "GTC",
            "type": "STOP",
            "activatePrice": "42000.0",
            "priceRate": "0.02",
            "updateTime": 1625097800000,
            "workingType": "MARK_PRICE",
            "priceProtect": false,
            "priceMatch": "OPPONENT",
            "selfTradePreventionMode": "NONE"
        }"#;

        let response: QueryOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.status, OrderStatus::Canceled);
        assert_eq!(response.executed_qty, "0");
        assert_eq!(response.order_type, OrderType::Stop);
        assert_eq!(response.stop_price, "42500.0");
        assert_eq!(response.activate_price, Some("42000.0".to_string()));
        assert_eq!(response.price_rate, Some("0.02".to_string()));
        assert_eq!(response.working_type, WorkingType::MarkPrice);
        assert_eq!(response.price_match, PriceMatch::Opponent);
    }
}
