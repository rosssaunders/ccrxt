use serde::{Deserialize, Serialize};

use super::place_order::{Force, STPMode};
use crate::bitget::{OrderSide, OrderType, PrivateRestClient as RestClient, RestResult};

const CANCEL_REPLACE_ORDER_ENDPOINT: &str = "/api/v2/spot/trade/cancel-replace-order";

/// Request parameters for cancelling and replacing an order
#[derive(Debug, Clone, Serialize)]
pub struct CancelReplaceOrderRequest {
    /// Trading pair name, e.g. BTCUSDT
    pub symbol: String,

    /// Order ID to cancel (either orderId or clientOrderId is required)
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    /// Client order ID to cancel (either orderId or clientOrderId is required)
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,

    /// New order direction: buy or sell
    pub side: OrderSide,

    /// New order type: limit or market
    #[serde(rename = "orderType")]
    pub order_type: OrderType,

    /// New execution strategy (invalid when orderType is market)
    pub force: Force,

    /// New limit price (required for limit orders)
    /// The decimal places of price can be obtained from Get Symbol Info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// New order amount
    /// For Limit and Market-Sell orders: represents the number of base coins
    /// For Market-Buy orders: represents the number of quote coins
    pub size: String,

    /// New custom order ID (optional)
    #[serde(rename = "newClientOid", skip_serializing_if = "Option::is_none")]
    pub new_client_order_id: Option<String>,

    /// Self-trade prevention mode
    #[serde(rename = "stpMode", skip_serializing_if = "Option::is_none")]
    pub stp_mode: Option<STPMode>,

    /// Request timestamp (Unix milliseconds)
    #[serde(rename = "requestTime", skip_serializing_if = "Option::is_none")]
    pub request_time: Option<i64>,

    /// Valid time window (Unix milliseconds)
    /// If set, request is valid only when server time is within receiveWindow
    #[serde(rename = "receiveWindow", skip_serializing_if = "Option::is_none")]
    pub receive_window: Option<i64>,
}

/// Response from cancelling and replacing an order
#[derive(Debug, Clone, Deserialize)]
pub struct CancelReplaceOrderResponse {
    /// New order ID assigned by the system
    #[serde(rename = "orderId")]
    pub order_id: String,

    /// New custom order ID (if provided in request)
    #[serde(rename = "clientOid")]
    pub client_order_id: Option<String>,

    /// Cancelled order ID
    #[serde(rename = "cancelOrderId")]
    pub cancel_order_id: String,

    /// Cancelled client order ID (if applicable)
    #[serde(rename = "cancelClientOid")]
    pub cancel_client_order_id: Option<String>,
}

impl RestClient {
    /// Cancel and replace a spot trading order
    ///
    /// Cancels an existing order and places a new order with the specified parameters.
    /// This is an atomic operation that ensures the old order is cancelled before
    /// the new order is placed.
    ///
    /// # Arguments
    /// * `request` - The cancel-replace order request parameters
    ///
    /// # Rate Limit
    /// 10 requests per second per UID
    ///
    /// # Returns
    /// A result containing the cancel-replace order response or an error
    pub async fn cancel_replace_order(
        &self,
        request: CancelReplaceOrderRequest,
    ) -> RestResult<CancelReplaceOrderResponse> {
        self.send_post_signed_request(
            CANCEL_REPLACE_ORDER_ENDPOINT,
            request,
            10,       // 10 requests per second rate limit
            true,     // This is an order endpoint
            Some(10), // Order-specific rate limit
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel_replace_order_request_limit_by_order_id() {
        let request = CancelReplaceOrderRequest {
            symbol: "BTCUSDT".to_string(),
            order_id: Some("1234567890".to_string()),
            client_order_id: None,
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            force: Force::GTC,
            price: Some("51000".to_string()),
            size: "0.002".to_string(),
            new_client_order_id: None,
            stp_mode: None,
            request_time: None,
            receive_window: None,
        };

        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.order_id, Some("1234567890".to_string()));
        assert!(request.client_order_id.is_none());
        assert_eq!(request.side, OrderSide::Buy);
        assert_eq!(request.order_type, OrderType::Limit);
        assert_eq!(request.force, Force::GTC);
        assert_eq!(request.price, Some("51000".to_string()));
        assert_eq!(request.size, "0.002");
    }

    #[test]
    fn test_cancel_replace_order_request_limit_by_client_order_id() {
        let request = CancelReplaceOrderRequest {
            symbol: "ETHUSDT".to_string(),
            order_id: None,
            client_order_id: Some("my-order-123".to_string()),
            side: OrderSide::Sell,
            order_type: OrderType::Limit,
            force: Force::GTC,
            price: Some("3000".to_string()),
            size: "1.5".to_string(),
            new_client_order_id: None,
            stp_mode: None,
            request_time: None,
            receive_window: None,
        };

        assert_eq!(request.symbol, "ETHUSDT");
        assert!(request.order_id.is_none());
        assert_eq!(request.client_order_id, Some("my-order-123".to_string()));
        assert_eq!(request.side, OrderSide::Sell);
        assert_eq!(request.order_type, OrderType::Limit);
        assert_eq!(request.price, Some("3000".to_string()));
        assert_eq!(request.size, "1.5");
    }

    #[test]
    fn test_cancel_replace_order_request_market() {
        let request = CancelReplaceOrderRequest {
            symbol: "BTCUSDT".to_string(),
            order_id: Some("1234567890".to_string()),
            client_order_id: None,
            side: OrderSide::Sell,
            order_type: OrderType::Market,
            force: Force::GTC,
            price: None,
            size: "0.5".to_string(),
            new_client_order_id: None,
            stp_mode: None,
            request_time: None,
            receive_window: None,
        };

        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.order_id, Some("1234567890".to_string()));
        assert_eq!(request.side, OrderSide::Sell);
        assert_eq!(request.order_type, OrderType::Market);
        assert!(request.price.is_none());
        assert_eq!(request.size, "0.5");
    }

    #[test]
    fn test_cancel_replace_order_request_builder() {
        let request = CancelReplaceOrderRequest {
            symbol: "BTCUSDT".to_string(),
            order_id: Some("1234567890".to_string()),
            client_order_id: None,
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            force: Force::PostOnly,
            price: Some("51000".to_string()),
            size: "0.002".to_string(),
            new_client_order_id: Some("new-order-456".to_string()),
            stp_mode: Some(STPMode::CancelTaker),
            request_time: None,
            receive_window: None,
        };

        assert_eq!(request.force, Force::PostOnly);
        assert_eq!(
            request.new_client_order_id,
            Some("new-order-456".to_string())
        );
        assert_eq!(request.stp_mode, Some(STPMode::CancelTaker));
    }

    #[test]
    fn test_cancel_replace_order_request_serialization() {
        let request = CancelReplaceOrderRequest {
            symbol: "BTCUSDT".to_string(),
            order_id: Some("1234567890".to_string()),
            client_order_id: None,
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            force: Force::GTC,
            price: Some("51000".to_string()),
            size: "0.002".to_string(),
            new_client_order_id: Some("new-order-456".to_string()),
            stp_mode: None,
            request_time: None,
            receive_window: None,
        };

        let json = serde_json::to_string(&request).unwrap();

        assert!(json.contains("\"symbol\":\"BTCUSDT\""));
        assert!(json.contains("\"orderId\":\"1234567890\""));
        assert!(json.contains("\"side\":\"buy\""));
        assert!(json.contains("\"orderType\":\"limit\""));
        assert!(json.contains("\"force\":\"gtc\""));
        assert!(json.contains("\"price\":\"51000\""));
        assert!(json.contains("\"size\":\"0.002\""));
        assert!(json.contains("\"newClientOid\":\"new-order-456\""));
    }

    #[test]
    fn test_cancel_replace_order_response_deserialization() {
        let json = r#"{
            "orderId": "2001",
            "clientOid": "new-order-456",
            "cancelOrderId": "1001",
            "cancelClientOid": "old-order-123"
        }"#;

        let response: CancelReplaceOrderResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.order_id, "2001");
        assert_eq!(response.client_order_id, Some("new-order-456".to_string()));
        assert_eq!(response.cancel_order_id, "1001");
        assert_eq!(
            response.cancel_client_order_id,
            Some("old-order-123".to_string())
        );
    }

    #[test]
    fn test_cancel_replace_order_response_deserialization_no_client_ids() {
        let json = r#"{
            "orderId": "2001",
            "clientOid": null,
            "cancelOrderId": "1001",
            "cancelClientOid": null
        }"#;

        let response: CancelReplaceOrderResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.order_id, "2001");
        assert!(response.client_order_id.is_none());
        assert_eq!(response.cancel_order_id, "1001");
        assert!(response.cancel_client_order_id.is_none());
    }
}
