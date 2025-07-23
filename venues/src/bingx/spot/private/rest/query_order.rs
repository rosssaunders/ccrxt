use serde::{Deserialize, Serialize};

use super::{
    RestClient,
    place_order::{OrderSide, OrderStatus, OrderType},
};
use crate::bingx::spot::{EndpointType, RestResult};

const QUERY_ORDER_ENDPOINT: &str = "/openApi/spot/v1/trade/query";

/// Request to query order details
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryOrderRequest {
    /// Trading pair, e.g., BTC-USDT
    pub symbol: String,

    /// Order ID (either orderId or clientOrderId must be provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i64>,

    /// Customized order ID for users
    #[serde(rename = "clientOrderID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,

    /// Request valid time window value, Unit: milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,

    /// Timestamp of initiating the request, Unit: milliseconds
    pub timestamp: i64,
}

/// Detailed order information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderDetails {
    /// Trading pair
    pub symbol: String,

    /// Order ID
    pub order_id: i64,

    /// Price
    pub price: String,

    /// Original quantity
    pub orig_qty: String,

    /// Executed quantity
    pub executed_qty: String,

    /// Cumulative quote asset transacted quantity
    pub cummulative_quote_qty: String,

    /// Order status
    pub status: OrderStatus,

    /// Order type
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Order side
    pub side: OrderSide,

    /// Order timestamp
    pub time: i64,

    /// Update timestamp
    pub update_time: i64,

    /// Original quote order quantity
    pub orig_quote_order_qty: String,

    /// Fee
    pub fee: String,

    /// Fee asset
    pub fee_asset: String,

    /// Customized order ID for users
    #[serde(rename = "clientOrderID")]
    pub client_order_id: Option<String>,

    /// Trigger price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<String>,

    /// Average fill price
    pub avg_price: String,
}

impl RestClient {
    /// Query order details
    ///
    /// Retrieves detailed information about a specific order.
    /// Rate limit: 10/s by UID
    ///
    /// # Arguments
    /// * `request` - The query order request containing either order ID or client order ID
    ///
    /// # Returns
    /// A result containing the order details or an error
    ///
    /// # Notes
    /// - Either orderId or clientOrderId must be provided
    /// - Only supports a query range of 2 hours for client order IDs
    ///
    /// # API Documentation
    /// - [docs]: https://bingx-api.github.io/docs/#/en-us/spot/trade-api.html#Query%20Order%20details
    pub async fn query_order(&self, request: &QueryOrderRequest) -> RestResult<OrderDetails> {
        self.send_request(
            QUERY_ORDER_ENDPOINT,
            reqwest::Method::GET,
            Some(request),
            EndpointType::Trading,
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
            symbol: "BTC-USDT".to_string(),
            order_id: Some(123456789),
            client_order_id: None,
            recv_window: Some(5000),
            timestamp: 1658748648396,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTC-USDT"));
        assert!(serialized.contains("orderId=123456789"));
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1658748648396"));
        assert!(!serialized.contains("clientOrderID"));
    }

    #[test]
    fn test_query_order_request_serialization_with_client_order_id() {
        let request = QueryOrderRequest {
            symbol: "BTC-USDT".to_string(),
            order_id: None,
            client_order_id: Some("my_order_123".to_string()),
            recv_window: None,
            timestamp: 1658748648396,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTC-USDT"));
        assert!(serialized.contains("clientOrderID=my_order_123"));
        assert!(serialized.contains("timestamp=1658748648396"));
        assert!(!serialized.contains("orderId"));
        assert!(!serialized.contains("recvWindow"));
    }

    #[test]
    fn test_order_details_deserialization() {
        let json = r#"{
            "symbol": "BTC-USDT",
            "orderId": 123456789,
            "price": "50000.0",
            "origQty": "0.001",
            "executedQty": "0.001",
            "cummulativeQuoteQty": "50.0",
            "status": "FILLED",
            "type": "LIMIT",
            "side": "BUY",
            "time": 1658748648396,
            "updateTime": 1658748650000,
            "origQuoteOrderQty": "50.0",
            "fee": "0.05",
            "feeAsset": "USDT",
            "clientOrderID": "my_order_123",
            "stopPrice": "49000.0",
            "avgPrice": "50000.0"
        }"#;

        let order_details: OrderDetails = serde_json::from_str(json).unwrap();
        assert_eq!(order_details.symbol, "BTC-USDT");
        assert_eq!(order_details.order_id, 123456789);
        assert_eq!(order_details.price, "50000.0");
        assert_eq!(order_details.orig_qty, "0.001");
        assert_eq!(order_details.executed_qty, "0.001");
        assert_eq!(order_details.cummulative_quote_qty, "50.0");
        assert!(matches!(order_details.status, OrderStatus::Filled));
        assert!(matches!(order_details.order_type, OrderType::Limit));
        assert!(matches!(order_details.side, OrderSide::Buy));
        assert_eq!(order_details.time, 1658748648396);
        assert_eq!(order_details.update_time, 1658748650000);
        assert_eq!(order_details.orig_quote_order_qty, "50.0");
        assert_eq!(order_details.fee, "0.05");
        assert_eq!(order_details.fee_asset, "USDT");
        assert_eq!(
            order_details.client_order_id,
            Some("my_order_123".to_string())
        );
        assert_eq!(order_details.stop_price, Some("49000.0".to_string()));
        assert_eq!(order_details.avg_price, "50000.0");
    }
}
