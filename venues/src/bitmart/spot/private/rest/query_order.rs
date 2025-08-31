//! BitMart query order REST API endpoint
//!
//! This module implements the BitMart query order API endpoint for retrieving order details.

use serde::{Deserialize, Serialize};

use crate::bitmart::{
    OrderMode, OrderSide, OrderStatus, OrderType, RestResult, rate_limit::EndpointType,
    spot::private_client::RestClient,
};

const QUERY_ORDER_ENDPOINT: &str = "/spot/v4/query/order";

/// Request parameters for querying order details
#[derive(Debug, Serialize)]
pub struct QueryOrderRequest {
    /// Order ID (required if client_order_id not provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// Client-defined Order ID (required if order_id not provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    /// Trading pair (e.g. BTC_USDT)
    pub symbol: String,
}

/// Order details information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderDetails {
    /// Order ID
    #[serde(rename = "orderId")]
    pub order_id: String,
    /// Client-defined Order ID
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
    /// Trading pair (e.g. BTC_USDT)
    pub symbol: String,
    /// Order side
    pub side: OrderSide,
    /// Order mode
    #[serde(rename = "orderMode")]
    pub order_mode: OrderMode,
    /// Order type
    #[serde(rename = "type")]
    pub order_type: OrderType,
    /// Order status
    pub state: OrderStatus,
    /// Order price
    pub price: String,
    /// Average filled price
    #[serde(rename = "priceAvg")]
    pub price_avg: String,
    /// Order size
    pub size: String,
    /// Filled size
    #[serde(rename = "filledSize")]
    pub filled_size: String,
    /// Notional amount
    pub notional: String,
    /// Filled notional amount
    #[serde(rename = "filledNotional")]
    pub filled_notional: String,
    /// Order creation time in milliseconds
    #[serde(rename = "createTime")]
    pub create_time: i64,
    /// Last update time in milliseconds
    #[serde(rename = "updateTime")]
    pub update_time: i64,
}

/// Response for querying order details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryOrderResponse {
    /// Order details
    #[serde(flatten)]
    pub order: OrderDetails,
}

impl RestClient {
    /// Query Order By Id (v4)
    ///
    /// Retrieves details for a specific order.
    ///
    /// [docs](https://developer-pro.bitmart.com/en/spot/#query-order-by-id-v4-signed)
    ///
    /// Rate limit: varies by endpoint type
    ///
    /// # Arguments
    /// * `request` - The query order request parameters
    ///
    /// # Returns
    /// Order details response
    pub async fn query_order(&self, request: QueryOrderRequest) -> RestResult<QueryOrderResponse> {
        self.send_post_signed_request(QUERY_ORDER_ENDPOINT, request, EndpointType::SpotTrading)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_order_request_by_order_id() {
        let request = QueryOrderRequest {
            order_id: Some("12345".to_string()),
            client_order_id: None,
            symbol: "BTC_USDT".to_string(),
        };

        assert_eq!(request.order_id, Some("12345".to_string()));
        assert!(request.client_order_id.is_none());
        assert_eq!(request.symbol, "BTC_USDT");
    }

    #[test]
    fn test_query_order_request_by_client_order_id() {
        let request = QueryOrderRequest {
            order_id: None,
            client_order_id: Some("my_order_123".to_string()),
            symbol: "ETH_USDT".to_string(),
        };

        assert!(request.order_id.is_none());
        assert_eq!(request.client_order_id, Some("my_order_123".to_string()));
        assert_eq!(request.symbol, "ETH_USDT");
    }

    #[test]
    fn test_order_details_structure() {
        let json = r#"{
            "orderId": "12345",
            "clientOrderId": "my_order_123",
            "symbol": "BTC_USDT",
            "side": "buy",
            "orderMode": "spot",
            "type": "limit",
            "state": "new",
            "price": "50000.00",
            "priceAvg": "0.00",
            "size": "0.001",
            "filledSize": "0.000",
            "notional": "50.00",
            "filledNotional": "0.00",
            "createTime": 1609459200000,
            "updateTime": 1609459200000
        }"#;

        let order: OrderDetails = serde_json::from_str(json).unwrap();
        assert_eq!(order.order_id, "12345");
        assert_eq!(order.client_order_id, "my_order_123");
        assert_eq!(order.symbol, "BTC_USDT");
        assert_eq!(order.side, OrderSide::Buy);
        assert_eq!(order.order_mode, OrderMode::Spot);
        assert_eq!(order.order_type, OrderType::Limit);
        assert_eq!(order.state, OrderStatus::New);
        assert_eq!(order.price, "50000.00");
        assert_eq!(order.price_avg, "0.00");
        assert_eq!(order.size, "0.001");
        assert_eq!(order.filled_size, "0.000");
        assert_eq!(order.notional, "50.00");
        assert_eq!(order.filled_notional, "0.00");
        assert_eq!(order.create_time, 1609459200000);
        assert_eq!(order.update_time, 1609459200000);
    }
}
