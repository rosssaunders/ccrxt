use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::RestResult;
use crate::binance::usdm::enums::*;

const QUERY_ORDER_ENDPOINT: &str = "/fapi/v1/order";

/// Request parameters for the query order endpoint.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryOrderRequest {
    /// Trading symbol (e.g., "BTCUSDT"). Required.
    pub symbol: std::borrow::Cow<'static, str>,

    /// Order ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,

    /// Original client order ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id: Option<std::borrow::Cow<'static, str>>,
}

/// Response for a queried order.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryOrderResponse {
    /// Trading symbol.
    pub symbol: std::borrow::Cow<'static, str>,

    /// Order ID.
    pub order_id: u64,

    /// Client order ID.
    pub client_order_id: std::borrow::Cow<'static, str>,

    /// Price for the order.
    pub price: std::borrow::Cow<'static, str>,

    /// Original quantity.
    pub orig_qty: std::borrow::Cow<'static, str>,

    /// Executed quantity.
    pub executed_qty: std::borrow::Cow<'static, str>,

    /// Order status.
    pub status: OrderStatus,

    /// Time in force.
    pub time_in_force: TimeInForce,

    /// Order type.
    pub order_type: OrderType,

    /// Order side.
    pub side: OrderSide,

    /// Position side.
    pub position_side: PositionSide,

    /// Working type.
    pub working_type: WorkingType,
}

impl UsdmClient {
    /// Query Order
    ///
    /// Retrieves information about a specific order on Binance USDM Futures.
    ///
    /// [docs]: https://binance-docs.github.io/apidocs/futures/en/#query-order-user_data
    ///
    /// Rate limit: 2 requests per second
    ///
    /// # Arguments
    /// * `request` - The query order request parameters
    ///
    /// # Returns
    /// Returns a `QueryOrderResponse` containing order details.
    pub async fn query_order(&self, request: QueryOrderRequest) -> RestResult<QueryOrderResponse> {
        self.send_signed_request(
            QUERY_ORDER_ENDPOINT,
            reqwest::Method::GET,
            request,
            2,
            false,
        )
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
            order_id: Some(1234567890),
            orig_client_order_id: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("order_id=1234567890"));
        assert!(!serialized.contains("orig_client_order_id="));
        assert!(!serialized.contains("api_key"));
        assert!(!serialized.contains("api_secret"));
    }

    #[test]
    fn test_query_order_request_serialization_with_client_order_id() {
        let request = QueryOrderRequest {
            symbol: "ETHUSDT".into(),
            order_id: None,
            orig_client_order_id: Some("my_order_123".into()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=ETHUSDT"));
        assert!(serialized.contains("orig_client_order_id=my_order_123"));
        assert!(!serialized.contains("order_id="));
    }

    #[test]
    fn test_query_order_request_with_both_ids() {
        let request = QueryOrderRequest {
            symbol: "BTCUSDT".into(),
            order_id: Some(1234567890),
            orig_client_order_id: Some("my_order_123".into()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("order_id=1234567890"));
        assert!(serialized.contains("orig_client_order_id=my_order_123"));
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
            "type": "LIMIT",
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
            "type": "MARKET",
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
            "type": "STOP",
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
            "type": "STOP_MARKET",
            "side": "SELL",
            "positionSide": "LONG",
            "workingType": "CONTRACT_PRICE"
        }"#;

        let response: QueryOrderResponse = serde_json::from_str(json).unwrap();
        assert!(matches!(response.order_type, OrderType::StopMarket));
        assert_eq!(response.price, "0");
    }
}
