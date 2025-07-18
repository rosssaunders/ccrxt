// Query Current Open Order (USER_DATA) endpoint implementation for GET /dapi/v1/openOrder
// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Query-Current-Open-Order>

use serde::{Deserialize, Serialize};

use crate::binance::{
    coinm::{
        OrderSide, OrderStatus, OrderType, PositionSide, PriceMatch, RestResult,
        SelfTradePreventionMode, TimeInForce, WorkingType, private::rest::client::RestClient,
    },
    shared,
};

const OPEN_ORDER_ENDPOINT: &str = "/dapi/v1/openOrder";

/// Request parameters for querying a current open order (GET /dapi/v1/openOrder).
#[derive(Debug, Clone, Serialize, Default)]
pub struct QueryCurrentOpenOrderRequest {
    /// Trading symbol (e.g., "BTCUSD_PERP").
    pub symbol: String,

    /// Order ID. Either orderId or origClientOrderId must be sent.
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,

    /// Original client order ID. Either orderId or origClientOrderId must be sent.
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id: Option<String>,

    /// Receive window in milliseconds.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp in milliseconds.
    pub timestamp: u64,
}

/// Response for querying a current open order (GET /dapi/v1/openOrder).
#[derive(Debug, Clone, Deserialize)]
pub struct QueryCurrentOpenOrderResponse {
    /// Average price.
    #[serde(rename = "avgPrice")]
    pub avg_price: String,

    /// Client order ID.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,

    /// Cumulative base quantity.
    #[serde(rename = "cumBase")]
    pub cum_base: String,

    /// Executed quantity.
    #[serde(rename = "executedQty")]
    pub executed_qty: String,

    /// Order ID.
    #[serde(rename = "orderId")]
    pub order_id: u64,

    /// Original quantity.
    #[serde(rename = "origQty")]
    pub orig_qty: String,

    /// Original order type.
    #[serde(rename = "origType")]
    pub orig_type: OrderType,

    /// Order price.
    pub price: String,

    /// Reduce only flag.
    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,

    /// Order side.
    pub side: OrderSide,

    /// Position side.
    #[serde(rename = "positionSide")]
    pub position_side: PositionSide,

    /// Order status.
    pub status: OrderStatus,

    /// Stop price.
    #[serde(rename = "stopPrice")]
    pub stop_price: String,

    /// Close position flag.
    #[serde(rename = "closePosition")]
    pub close_position: bool,

    /// Trading symbol.
    pub symbol: String,

    /// Trading pair.
    pub pair: String,

    /// Order time.
    pub time: u64,

    /// Time in force.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,

    /// Order type.
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Activation price (for trailing stop orders).
    #[serde(rename = "activatePrice", skip_serializing_if = "Option::is_none")]
    pub activate_price: Option<String>,

    /// Price rate (for trailing stop orders).
    #[serde(rename = "priceRate", skip_serializing_if = "Option::is_none")]
    pub price_rate: Option<String>,

    /// Update time.
    #[serde(rename = "updateTime")]
    pub update_time: u64,

    /// Working type.
    #[serde(rename = "workingType")]
    pub working_type: WorkingType,

    /// Price protect flag.
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
    /// Queries a current open order (USER_DATA) on Binance Coin-M Futures.
    ///
    /// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Query-Current-Open-Order>
    /// GET /dapi/v1/openOrder
    /// Weight: 1
    /// Requires API key and signature.
    ///
    /// Either orderId or origClientOrderId must be sent.
    /// If the queried order has been filled or cancelled, the error message "Order does not exist" will be returned.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`QueryCurrentOpenOrderRequest`])
    ///
    /// # Returns
    /// A [`QueryCurrentOpenOrderResponse`] with the open order details.
    pub async fn query_current_open_order(
        &self,
        params: QueryCurrentOpenOrderRequest,
    ) -> RestResult<QueryCurrentOpenOrderResponse> {
        let weight = 1;
        shared::send_signed_request(
            self,
            OPEN_ORDER_ENDPOINT,
            reqwest::Method::GET,
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
    fn test_query_current_open_order_request_with_order_id() {
        let request = QueryCurrentOpenOrderRequest {
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
    fn test_query_current_open_order_request_with_client_order_id() {
        let request = QueryCurrentOpenOrderRequest {
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
    fn test_query_current_open_order_response_deserialization() {
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
            "positionSide": "LONG",
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

        let response: QueryCurrentOpenOrderResponse = serde_json::from_str(json).unwrap();
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
        assert_eq!(response.position_side, PositionSide::Long);
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
    fn test_query_current_open_order_response_with_execution() {
        let json = r#"{
            "avgPrice": "44999.5",
            "clientOrderId": "order_456",
            "cumBase": "0.00111",
            "executedQty": "5.0",
            "orderId": 67890,
            "origQty": "20.0",
            "origType": "LIMIT",
            "price": "45000.0",
            "reduceOnly": true,
            "side": "SELL",
            "positionSide": "SHORT",
            "status": "PARTIALLY_FILLED",
            "stopPrice": "0",
            "closePosition": false,
            "symbol": "ETHUSD_PERP",
            "pair": "ETHUSD",
            "time": 1625097600000,
            "timeInForce": "IOC",
            "type": "LIMIT",
            "updateTime": 1625097700000,
            "workingType": "MARK_PRICE",
            "priceProtect": true,
            "priceMatch": "QUEUE",
            "selfTradePreventionMode": "EXPIRE_TAKER"
        }"#;

        let response: QueryCurrentOpenOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.avg_price, "44999.5");
        assert_eq!(response.executed_qty, "5.0");
        assert_eq!(response.orig_qty, "20.0");
        assert_eq!(response.status, OrderStatus::PartiallyFilled);
        assert!(response.reduce_only);
        assert_eq!(response.side, OrderSide::Sell);
        assert_eq!(response.position_side, PositionSide::Short);
        assert_eq!(response.time_in_force, TimeInForce::IOC);
        assert_eq!(response.working_type, WorkingType::MarkPrice);
        assert!(response.price_protect);
        assert_eq!(response.price_match, PriceMatch::Queue);
        assert_eq!(
            response.self_trade_prevention_mode,
            SelfTradePreventionMode::ExpireTaker
        );
    }

    #[test]
    fn test_query_current_open_order_response_stop_order() {
        let json = r#"{
            "avgPrice": "0.0",
            "clientOrderId": "stop_order_789",
            "cumBase": "0",
            "executedQty": "0",
            "orderId": 11111,
            "origQty": "15.0",
            "origType": "STOP",
            "price": "43000.0",
            "reduceOnly": false,
            "side": "BUY",
            "positionSide": "LONG",
            "status": "NEW",
            "stopPrice": "42500.0",
            "closePosition": false,
            "symbol": "BTCUSD_PERP",
            "pair": "BTCUSD",
            "time": 1625097600000,
            "timeInForce": "GTC",
            "type": "STOP",
            "activatePrice": "42000.0",
            "priceRate": "0.05",
            "updateTime": 1625097600000,
            "workingType": "CONTRACT_PRICE",
            "priceProtect": false,
            "priceMatch": "OPPONENT",
            "selfTradePreventionMode": "NONE"
        }"#;

        let response: QueryCurrentOpenOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.order_type, OrderType::Stop);
        assert_eq!(response.stop_price, "42500.0");
        assert_eq!(response.activate_price, Some("42000.0".to_string()));
        assert_eq!(response.price_rate, Some("0.05".to_string()));
        assert_eq!(response.price_match, PriceMatch::Opponent);
    }
}
