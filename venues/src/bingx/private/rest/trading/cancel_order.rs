use serde::{Deserialize, Serialize};

use crate::bingx::{
    EndpointType, PrivateRestClient as RestClient, RestResult,
    enums::{CancelRestriction, OrderSide, OrderStatus, OrderType},
};

const CANCEL_ORDER_ENDPOINT: &str = "/openApi/spot/v1/trade/cancel";

/// Request to cancel an order
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest {
    /// Trading pair, e.g., BTC-USDT (required)
    pub symbol: String,

    /// Order ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i64>,

    /// Customized order ID for users, with a limit of characters from 1 to 40. Different orders cannot use the same clientOrderID. Only supports a query range of 2 hours (optional)
    #[serde(rename = "clientOrderID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,

    /// Cancel orders with specified status: NEW: new order, PENDING: order in progress, PARTIALLY_FILLED: partially filled (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_restrictions: Option<CancelRestriction>,

    /// Request valid time window value in milliseconds (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,

    /// Timestamp of initiating the request in milliseconds (required)
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

    /// Order status: NEW, PENDING, PARTIALLY_FILLED, FILLED, CANCELED, FAILED
    pub status: OrderStatus,

    /// Order type: MARKET/LIMIT/TAKE_STOP_LIMIT/TAKE_STOP_MARKET/TRIGGER_LIMIT/TRIGGER_MARKET
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Order side: BUY/SELL
    pub side: OrderSide,

    /// Customized order ID for users
    #[serde(rename = "clientOrderID")]
    pub client_order_id: Option<String>,

    /// Trigger price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<String>,
}

impl RestClient {
    /// Cancel Order
    ///
    /// [docs](https://bingx-api.github.io/docs/#/en-us/spot/trade-api.html#Cancel%20Order)
    ///
    /// Rate limit: UID 5/s & IP rate limit group 3
    ///
    /// # Arguments
    /// * `request` - The cancel order request containing either order ID or client order ID
    ///
    /// # Returns
    /// A result containing the cancel order response or an error
    pub async fn cancel_order(
        &self,
        request: &CancelOrderRequest,
    ) -> RestResult<CancelOrderResponse> {
        self.send_post_signed_request(CANCEL_ORDER_ENDPOINT, request, EndpointType::Trading)
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
            cancel_restrictions: Some(CancelRestriction::New),
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
            serde_json::to_string(&CancelRestriction::New).unwrap(),
            "\"NEW\""
        );
        assert_eq!(
            serde_json::to_string(&CancelRestriction::Pending).unwrap(),
            "\"PENDING\""
        );
        assert_eq!(
            serde_json::to_string(&CancelRestriction::PartiallyFilled).unwrap(),
            "\"PARTIALLY_FILLED\""
        );
    }
}
