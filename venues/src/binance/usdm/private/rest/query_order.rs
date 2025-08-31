use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::binance::usdm::{RestResult, enums::*, private_client::UsdmClient};

const QUERY_ORDER_ENDPOINT: &str = "/fapi/v1/order";

/// Request parameters for the query order endpoint.
///
/// Used to retrieve details for a specific order on Binance USDM Futures.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueryOrderRequest {
    /// Trading symbol (e.g., "BTCUSDT"). Required.
    /// Must match a valid symbol listed on Binance USDM Futures.
    pub symbol: Cow<'static, str>,

    /// Order ID. Optional.
    /// Either `order_id` or `orig_client_order_id` must be provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,

    /// Original client order ID. Optional.
    /// Either `order_id` or `orig_client_order_id` must be provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id: Option<Cow<'static, str>>,

    /// The timestamp of the request (milliseconds since epoch). Required.
    pub timestamp: u64,

    /// The number of milliseconds after timestamp the request is valid for. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Response for a queried order.
///
/// Contains all details returned by Binance for a queried order.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryOrderResponse {
    /// Trading symbol.
    pub symbol: Cow<'static, str>,

    /// Order ID.
    pub order_id: u64,

    /// Client order ID.
    pub client_order_id: Cow<'static, str>,

    /// Price for the order.
    pub price: Cow<'static, str>,

    /// Original quantity.
    pub orig_qty: Cow<'static, str>,

    /// Executed quantity.
    pub executed_qty: Cow<'static, str>,

    /// Cumulative quote asset transacted.
    pub cum_quote: Option<Cow<'static, str>>,

    /// Average price for the order.
    pub avg_price: Option<Cow<'static, str>>,

    /// Status of the order.
    pub status: OrderStatus,

    /// Time in force for the order.
    pub time_in_force: TimeInForce,

    /// Order type.
    pub order_type: OrderType,

    /// Original order type (for trailing stop market).
    pub orig_type: Option<OrderType>,

    /// Order side.
    pub side: OrderSide,

    /// Position side.
    pub position_side: PositionSide,

    /// Working type.
    pub working_type: WorkingType,

    /// Stop price (for stop orders).
    pub stop_price: Option<Cow<'static, str>>,

    /// Activation price (for trailing stop market).
    pub activate_price: Option<Cow<'static, str>>,

    /// Callback rate (for trailing stop market).
    pub price_rate: Option<Cow<'static, str>>,

    /// Reduce only flag.
    pub reduce_only: Option<bool>,

    /// Close position flag.
    pub close_position: Option<bool>,

    /// Price protect flag.
    pub price_protect: Option<bool>,

    /// Price match mode.
    pub price_match: Option<Cow<'static, str>>,

    /// Self trade prevention mode.
    pub self_trade_prevention_mode: Option<Cow<'static, str>>,

    /// Good till date (for GTD orders).
    pub good_till_date: Option<u64>,

    /// Order creation time (milliseconds since epoch).
    pub time: Option<u64>,

    /// Order update time (milliseconds since epoch).
    pub update_time: Option<u64>,
}

impl UsdmClient {
    /// Query Order (USER_DATA)
    ///
    /// Retrieves information about a specific order on Binance USDM Futures.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Query-Order)
    ///
    /// Rate limit: 2 requests per second
    ///
    /// # Arguments
    /// * `request` - The query order request parameters
    ///
    /// # Returns
    /// Returns a `QueryOrderResponse` containing order details.
    pub async fn query_order(&self, request: QueryOrderRequest) -> RestResult<QueryOrderResponse> {
        self.send_get_signed_request(QUERY_ORDER_ENDPOINT, request, 2, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_order_request_serialization_with_order_id() {
        let request = QueryOrderRequest {
            symbol: "BTCUSDT".into(),
            order_id: Some(1234567890u64),
            orig_client_order_id: None,
            timestamp: 1620000000000,
            recv_window: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("orderId=1234567890"));
        assert!(serialized.contains("timestamp=1620000000000"));
        assert!(!serialized.contains("origClientOrderId="));
        assert!(!serialized.contains("api_key"));
        assert!(!serialized.contains("api_secret"));
    }

    #[test]
    fn test_query_order_request_serialization_with_client_order_id() {
        let request = QueryOrderRequest {
            symbol: "ETHUSDT".into(),
            order_id: None,
            orig_client_order_id: Some("my_order_123".into()),
            timestamp: 1620000000001,
            recv_window: Some(5000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=ETHUSDT"));
        assert!(serialized.contains("origClientOrderId=my_order_123"));
        assert!(serialized.contains("timestamp=1620000000001"));
        assert!(serialized.contains("recvWindow=5000"));
        assert!(!serialized.contains("orderId="));
    }

    #[test]
    fn test_query_order_request_with_both_ids() {
        let request = QueryOrderRequest {
            symbol: "BTCUSDT".into(),
            order_id: Some(1234567890u64),
            orig_client_order_id: Some("my_order_123".into()),
            timestamp: 1620000000002,
            recv_window: Some(10000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("orderId=1234567890"));
        assert!(serialized.contains("origClientOrderId=my_order_123"));
        assert!(serialized.contains("timestamp=1620000000002"));
        assert!(serialized.contains("recvWindow=10000"));
    }

    #[test]
    fn test_query_order_response_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "orderId": 1234567890,
            "clientOrderId": "my_order_123",
            "price": "45380.10",
            "origQty": "0.100",
            "executedQty": "0.050",
            "status": "PARTIALLY_FILLED",
            "timeInForce": "GTC",
            "orderType": "LIMIT",
            "side": "BUY",
            "positionSide": "LONG",
            "workingType": "CONTRACT_PRICE"
        }"#;

        let response: QueryOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTCUSDT");
        assert_eq!(response.order_id, 1234567890);
        assert_eq!(response.client_order_id, "my_order_123");
        assert_eq!(response.price, "45380.10");
        assert_eq!(response.orig_qty, "0.100");
        assert_eq!(response.executed_qty, "0.050");
        assert!(matches!(response.status, OrderStatus::PartiallyFilled));
        assert!(matches!(response.time_in_force, TimeInForce::GTC));
        assert!(matches!(response.order_type, OrderType::Limit));
        assert!(matches!(response.side, OrderSide::Buy));
        assert!(matches!(response.position_side, PositionSide::Long));
        assert!(matches!(response.working_type, WorkingType::ContractPrice));
    }

    #[test]
    fn test_query_order_filled_status() {
        let json = r#"{
            "symbol": "ETHUSDT",
            "orderId": 9876543210,
            "clientOrderId": "eth_order_456",
            "price": "3070.50",
            "origQty": "1.000",
            "executedQty": "1.000",
            "status": "FILLED",
            "timeInForce": "IOC",
            "orderType": "MARKET",
            "side": "SELL",
            "positionSide": "SHORT",
            "workingType": "MARK_PRICE"
        }"#;

        let response: QueryOrderResponse = serde_json::from_str(json).unwrap();
        assert!(matches!(response.status, OrderStatus::Filled));
        assert_eq!(response.orig_qty, response.executed_qty);
        assert!(matches!(response.time_in_force, TimeInForce::IOC));
        assert!(matches!(response.order_type, OrderType::Market));
    }

    #[test]
    fn test_query_order_canceled_status() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "orderId": 1111111111,
            "clientOrderId": "cancel_order",
            "price": "40000.00",
            "origQty": "0.500",
            "executedQty": "0.000",
            "status": "CANCELED",
            "timeInForce": "FOK",
            "orderType": "STOP",
            "side": "BUY",
            "positionSide": "BOTH",
            "workingType": "CONTRACT_PRICE"
        }"#;

        let response: QueryOrderResponse = serde_json::from_str(json).unwrap();
        assert!(matches!(response.status, OrderStatus::Canceled));
        assert_eq!(response.executed_qty, "0.000");
        assert!(matches!(response.time_in_force, TimeInForce::FOK));
    }

    #[test]
    fn test_query_order_stop_market() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "orderId": 2222222222,
            "clientOrderId": "stop_market_order",
            "price": "0",
            "origQty": "0.300",
            "executedQty": "0.000",
            "status": "NEW",
            "timeInForce": "GTC",
            "orderType": "STOP_MARKET",
            "side": "SELL",
            "positionSide": "LONG",
            "workingType": "CONTRACT_PRICE"
        }"#;

        let response: QueryOrderResponse = serde_json::from_str(json).unwrap();
        assert!(matches!(response.order_type, OrderType::StopMarket));
        assert_eq!(response.price, "0");
    }
}
