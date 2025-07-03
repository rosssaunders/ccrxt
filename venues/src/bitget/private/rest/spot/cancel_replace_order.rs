//! Cancel Replace Order endpoint for Bitget Spot API
//!
//! This endpoint allows modifying an existing order by cancelling and replacing it.
//!
//! Reference: https://www.bitget.com/api-doc/spot/trade/Cancel-Replace-Order
//! Endpoint: POST /api/v2/spot/trade/cancel-replace-order
//! Rate limit: 10 requests/second/UID

use serde::{Deserialize, Serialize};

use super::super::RestClient;
use crate::bitget::{OrderSide, OrderType, RestResult};
use super::place_order::{Force, STPMode};

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

impl CancelReplaceOrderRequest {
    /// Create a request to cancel and replace an order by order ID with a limit order
    pub fn limit_by_order_id(
        symbol: impl Into<String>,
        order_id: impl Into<String>,
        side: OrderSide,
        price: impl Into<String>,
        size: impl Into<String>,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            order_id: Some(order_id.into()),
            client_order_id: None,
            side,
            order_type: OrderType::Limit,
            force: Force::GTC,
            price: Some(price.into()),
            size: size.into(),
            new_client_order_id: None,
            stp_mode: None,
            request_time: None,
            receive_window: None,
        }
    }

    /// Create a request to cancel and replace an order by client order ID with a limit order
    pub fn limit_by_client_order_id(
        symbol: impl Into<String>,
        client_order_id: impl Into<String>,
        side: OrderSide,
        price: impl Into<String>,
        size: impl Into<String>,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            order_id: None,
            client_order_id: Some(client_order_id.into()),
            side,
            order_type: OrderType::Limit,
            force: Force::GTC,
            price: Some(price.into()),
            size: size.into(),
            new_client_order_id: None,
            stp_mode: None,
            request_time: None,
            receive_window: None,
        }
    }

    /// Create a request to cancel and replace an order by order ID with a market order
    pub fn market_by_order_id(
        symbol: impl Into<String>,
        order_id: impl Into<String>,
        side: OrderSide,
        size: impl Into<String>,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            order_id: Some(order_id.into()),
            client_order_id: None,
            side,
            order_type: OrderType::Market,
            force: Force::GTC, // Force is ignored for market orders
            price: None,
            size: size.into(),
            new_client_order_id: None,
            stp_mode: None,
            request_time: None,
            receive_window: None,
        }
    }

    /// Create a request to cancel and replace an order by client order ID with a market order
    pub fn market_by_client_order_id(
        symbol: impl Into<String>,
        client_order_id: impl Into<String>,
        side: OrderSide,
        size: impl Into<String>,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            order_id: None,
            client_order_id: Some(client_order_id.into()),
            side,
            order_type: OrderType::Market,
            force: Force::GTC, // Force is ignored for market orders
            price: None,
            size: size.into(),
            new_client_order_id: None,
            stp_mode: None,
            request_time: None,
            receive_window: None,
        }
    }

    /// Set the execution force/strategy
    pub fn force(mut self, force: Force) -> Self {
        self.force = force;
        self
    }

    /// Set a new custom client order ID
    pub fn new_client_order_id(mut self, new_client_order_id: impl Into<String>) -> Self {
        self.new_client_order_id = Some(new_client_order_id.into());
        self
    }

    /// Set the self-trade prevention mode
    pub fn stp_mode(mut self, stp_mode: STPMode) -> Self {
        self.stp_mode = Some(stp_mode);
        self
    }

    /// Set the request timestamp
    pub fn request_time(mut self, request_time: i64) -> Self {
        self.request_time = Some(request_time);
        self
    }

    /// Set the receive window
    pub fn receive_window(mut self, receive_window: i64) -> Self {
        self.receive_window = Some(receive_window);
        self
    }
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
        let body = serde_json::to_string(&request).map_err(|e| {
            crate::bitget::Errors::Error(format!("Failed to serialize request: {e}"))
        })?;

        self.send_signed_request(
            "/api/v2/spot/trade/cancel-replace-order",
            reqwest::Method::POST,
            None,        // No query parameters
            Some(&body), // JSON body
            10,          // 10 requests per second rate limit
            true,        // This is an order endpoint
            Some(10),    // Order-specific rate limit
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel_replace_order_request_limit_by_order_id() {
        let request = CancelReplaceOrderRequest::limit_by_order_id(
            "BTCUSDT",
            "1234567890",
            OrderSide::Buy,
            "51000",
            "0.002",
        );

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
        let request = CancelReplaceOrderRequest::limit_by_client_order_id(
            "ETHUSDT",
            "my-order-123",
            OrderSide::Sell,
            "3000",
            "1.5",
        );

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
        let request = CancelReplaceOrderRequest::market_by_order_id(
            "BTCUSDT",
            "1234567890",
            OrderSide::Sell,
            "0.5",
        );

        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.order_id, Some("1234567890".to_string()));
        assert_eq!(request.side, OrderSide::Sell);
        assert_eq!(request.order_type, OrderType::Market);
        assert!(request.price.is_none());
        assert_eq!(request.size, "0.5");
    }

    #[test]
    fn test_cancel_replace_order_request_builder() {
        let request = CancelReplaceOrderRequest::limit_by_order_id(
            "BTCUSDT",
            "1234567890",
            OrderSide::Buy,
            "51000",
            "0.002",
        )
        .force(Force::PostOnly)
        .new_client_order_id("new-order-456")
        .stp_mode(STPMode::CancelTaker);

        assert_eq!(request.force, Force::PostOnly);
        assert_eq!(request.new_client_order_id, Some("new-order-456".to_string()));
        assert_eq!(request.stp_mode, Some(STPMode::CancelTaker));
    }

    #[test]
    fn test_cancel_replace_order_request_serialization() {
        let request = CancelReplaceOrderRequest::limit_by_order_id(
            "BTCUSDT",
            "1234567890",
            OrderSide::Buy,
            "51000",
            "0.002",
        )
        .new_client_order_id("new-order-456");

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
        assert_eq!(response.cancel_client_order_id, Some("old-order-123".to_string()));
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
