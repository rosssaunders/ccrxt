use serde::{Deserialize, Serialize};

use super::{
    RestClient,
    place_order::{OrderSide, OrderStatus, OrderType},
};
use crate::bingx::spot::{EndpointType, RestResult};

const CANCEL_ORDER_ENDPOINT: &str = "/openApi/spot/v1/trade/cancel";

/// Cancel restrictions enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CancelRestrictions {
    /// New order
    New,
    /// Order in progress
    Pending,
    /// Partially filled
    PartiallyFilled,
}

/// Request to cancel an order
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest {
    /// Trading pair, e.g., BTC-USDT
    pub symbol: String,

    /// Order ID (either orderId or clientOrderId must be provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i64>,

    /// Customized order ID for users
    #[serde(rename = "clientOrderID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,

    /// Cancel orders with specified status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_restrictions: Option<CancelRestrictions>,

    /// Request valid time window value, Unit: milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,

    /// Timestamp of initiating the request, Unit: milliseconds
    pub timestamp: i64,
}

/// Response from canceling an order
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {
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

    /// Customized order ID for users
    #[serde(rename = "clientOrderID")]
    pub client_order_id: Option<String>,

    /// Trigger price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<String>,
}

impl RestClient {
    /// Cancel an order
    ///
    /// Cancels an active order.
    /// Rate limit: 5/s by UID
    ///
    /// # Arguments
    /// * `request` - The cancel order request containing either order ID or client order ID
    ///
    /// # Returns
    /// A result containing the cancel order response or an error
    ///
    /// # Notes
    /// - Either orderId or clientOrderId must be provided
    /// - Only orders with status NEW, PENDING, or PARTIALLY_FILLED can be canceled
    ///
    /// # API Documentation
    /// - [docs]: https://bingx-api.github.io/docs/#/en-us/spot/trade-api.html#Cancel%20Order
    pub async fn cancel_order(
        &self,
        request: &CancelOrderRequest,
    ) -> RestResult<CancelOrderResponse> {
        self.send_request(
            CANCEL_ORDER_ENDPOINT,
            reqwest::Method::POST,
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
    fn test_cancel_order_request_serialization_with_order_id() {
        let request = CancelOrderRequest {
            symbol: "BTC-USDT".to_string(),
            order_id: Some(123456789),
            client_order_id: None,
            cancel_restrictions: Some(CancelRestrictions::New),
            recv_window: Some(5000),
            timestamp: 1658748648396,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTC-USDT"));
        assert!(serialized.contains("orderId=123456789"));
        assert!(serialized.contains("cancelRestrictions=NEW"));
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1658748648396"));
        assert!(!serialized.contains("clientOrderID"));
    }

    #[test]
    fn test_cancel_order_request_serialization_with_client_order_id() {
        let request = CancelOrderRequest {
            symbol: "BTC-USDT".to_string(),
            order_id: None,
            client_order_id: Some("my_order_123".to_string()),
            cancel_restrictions: None,
            recv_window: None,
            timestamp: 1658748648396,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTC-USDT"));
        assert!(serialized.contains("clientOrderID=my_order_123"));
        assert!(!serialized.contains("orderId"));
        assert!(!serialized.contains("cancelRestrictions"));
        assert!(!serialized.contains("recvWindow"));
        assert!(serialized.contains("timestamp=1658748648396"));
    }

    #[test]
    fn test_cancel_order_response_deserialization() {
        let json = r#"{
            "symbol": "BTC-USDT",
            "orderId": 123456789,
            "price": "50000.0",
            "origQty": "0.001",
            "executedQty": "0.0005",
            "cummulativeQuoteQty": "25.0",
            "status": "CANCELED",
            "type": "LIMIT",
            "side": "BUY",
            "clientOrderID": "my_order_123",
            "stopPrice": "49000.0"
        }"#;

        let response: CancelOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTC-USDT");
        assert_eq!(response.order_id, 123456789);
        assert_eq!(response.price, "50000.0");
        assert_eq!(response.orig_qty, "0.001");
        assert_eq!(response.executed_qty, "0.0005");
        assert_eq!(response.cummulative_quote_qty, "25.0");
        assert!(matches!(response.status, OrderStatus::Canceled));
        assert!(matches!(response.order_type, OrderType::Limit));
        assert!(matches!(response.side, OrderSide::Buy));
        assert_eq!(response.client_order_id, Some("my_order_123".to_string()));
        assert_eq!(response.stop_price, Some("49000.0".to_string()));
    }

    #[test]
    fn test_cancel_restrictions_serialization() {
        assert_eq!(
            serde_json::to_string(&CancelRestrictions::New).unwrap(),
            "\"NEW\""
        );
        assert_eq!(
            serde_json::to_string(&CancelRestrictions::Pending).unwrap(),
            "\"PENDING\""
        );
        assert_eq!(
            serde_json::to_string(&CancelRestrictions::PartiallyFilled).unwrap(),
            "\"PARTIALLY_FILLED\""
        );
    }
}
