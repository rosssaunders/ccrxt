//! Batch Cancel Replace Order endpoint for Bitget Spot API
//!
//! This endpoint allows modifying multiple existing orders by cancelling and replacing them in batch.
//!
//! Reference: https://www.bitget.com/api-doc/spot/trade/Batch-Cancel-Replace-Order
//! Endpoint: POST /api/v2/spot/trade/batch-cancel-replace-order
//! Rate limit: 5 requests/second/UID, maximum 20 orders per batch

use serde::{Deserialize, Serialize};

use super::super::RestClient;
use crate::bitget::{OrderSide, OrderType, RestResult};
use super::place_order::{Force, STPMode};

/// Single order cancel-replace request within a batch
#[derive(Debug, Clone, Serialize)]
pub struct BatchCancelReplaceOrderItem {
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
}

impl BatchCancelReplaceOrderItem {
    /// Create a limit order cancel-replace item by order ID
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
        }
    }

    /// Create a limit order cancel-replace item by client order ID
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
        }
    }

    /// Create a market order cancel-replace item by order ID
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
        }
    }

    /// Create a market order cancel-replace item by client order ID
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
}

/// Request parameters for batch cancel-replace orders
#[derive(Debug, Clone, Serialize)]
pub struct BatchCancelReplaceOrdersRequest {
    /// List of orders to cancel and replace (maximum 20 orders per batch)
    #[serde(rename = "orderList")]
    pub order_list: Vec<BatchCancelReplaceOrderItem>,

    /// Request timestamp (Unix milliseconds)
    #[serde(rename = "requestTime", skip_serializing_if = "Option::is_none")]
    pub request_time: Option<i64>,

    /// Valid time window (Unix milliseconds)
    /// If set, request is valid only when server time is within receiveWindow
    #[serde(rename = "receiveWindow", skip_serializing_if = "Option::is_none")]
    pub receive_window: Option<i64>,
}

impl BatchCancelReplaceOrdersRequest {
    /// Create a new batch cancel-replace orders request
    pub fn new(orders: Vec<BatchCancelReplaceOrderItem>) -> Self {
        Self {
            order_list: orders,
            request_time: None,
            receive_window: None,
        }
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

/// Result of a single order cancel-replace in the batch
#[derive(Debug, Clone, Deserialize)]
pub struct BatchCancelReplaceOrderResult {
    /// New order ID assigned by the system (if successful)
    #[serde(rename = "orderId")]
    pub order_id: Option<String>,

    /// New custom order ID (if provided in request)
    #[serde(rename = "clientOid")]
    pub client_order_id: Option<String>,

    /// Cancelled order ID (if successful)
    #[serde(rename = "cancelOrderId")]
    pub cancel_order_id: Option<String>,

    /// Cancelled client order ID (if applicable)
    #[serde(rename = "cancelClientOid")]
    pub cancel_client_order_id: Option<String>,

    /// Success status
    pub success: bool,

    /// Error code (if failed)
    #[serde(rename = "errorCode")]
    pub error_code: Option<String>,

    /// Error message (if failed)
    #[serde(rename = "errorMsg")]
    pub error_msg: Option<String>,
}

/// Response from batch cancel-replace orders
#[derive(Debug, Clone, Deserialize)]
pub struct BatchCancelReplaceOrdersResponse {
    /// List of cancel-replace results
    #[serde(rename = "orderInfo")]
    pub order_info: Vec<BatchCancelReplaceOrderResult>,

    /// List of failed cancel-replace operations (if any)
    #[serde(rename = "failure")]
    pub failure: Option<Vec<BatchCancelReplaceOrderResult>>,

    /// List of successful cancel-replace operations (if any)
    #[serde(rename = "success")]
    pub success: Option<Vec<BatchCancelReplaceOrderResult>>,
}

impl RestClient {
    /// Cancel and replace multiple spot trading orders in batch
    ///
    /// Cancels and replaces multiple orders for spot trading with the specified parameters.
    /// Maximum 20 orders per batch.
    ///
    /// # Arguments
    /// * `request` - The batch cancel-replace order request parameters
    ///
    /// # Rate Limit
    /// 5 requests per second per UID
    /// Maximum 20 orders per batch
    ///
    /// # Returns
    /// A result containing the batch cancel-replace order response or an error
    pub async fn batch_cancel_replace_orders(
        &self,
        request: BatchCancelReplaceOrdersRequest,
    ) -> RestResult<BatchCancelReplaceOrdersResponse> {
        // Validate that we don't exceed the maximum batch size
        if request.order_list.len() > 20 {
            return Err(crate::bitget::Errors::Error(
                "Maximum 20 orders allowed per batch".to_string(),
            ));
        }

        if request.order_list.is_empty() {
            return Err(crate::bitget::Errors::Error(
                "At least one order is required".to_string(),
            ));
        }

        let body = serde_json::to_string(&request).map_err(|e| {
            crate::bitget::Errors::Error(format!("Failed to serialize request: {e}"))
        })?;

        self.send_signed_request(
            "/api/v2/spot/trade/batch-cancel-replace-order",
            reqwest::Method::POST,
            None,        // No query parameters
            Some(&body), // JSON body
            5,           // 5 requests per second rate limit
            true,        // This is an order endpoint
            Some(5),     // Order-specific rate limit
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_cancel_replace_order_item_limit_by_order_id() {
        let item = BatchCancelReplaceOrderItem::limit_by_order_id(
            "BTCUSDT",
            "1234567890",
            OrderSide::Buy,
            "51000",
            "0.002",
        );

        assert_eq!(item.symbol, "BTCUSDT");
        assert_eq!(item.order_id, Some("1234567890".to_string()));
        assert!(item.client_order_id.is_none());
        assert_eq!(item.side, OrderSide::Buy);
        assert_eq!(item.order_type, OrderType::Limit);
        assert_eq!(item.force, Force::GTC);
        assert_eq!(item.price, Some("51000".to_string()));
        assert_eq!(item.size, "0.002");
    }

    #[test]
    fn test_batch_cancel_replace_order_item_market_by_client_order_id() {
        let item = BatchCancelReplaceOrderItem::market_by_client_order_id(
            "ETHUSDT",
            "my-order-123",
            OrderSide::Sell,
            "1.5",
        );

        assert_eq!(item.symbol, "ETHUSDT");
        assert!(item.order_id.is_none());
        assert_eq!(item.client_order_id, Some("my-order-123".to_string()));
        assert_eq!(item.side, OrderSide::Sell);
        assert_eq!(item.order_type, OrderType::Market);
        assert!(item.price.is_none());
        assert_eq!(item.size, "1.5");
    }

    #[test]
    fn test_batch_cancel_replace_order_item_builder() {
        let item = BatchCancelReplaceOrderItem::limit_by_order_id(
            "BTCUSDT",
            "1234567890",
            OrderSide::Buy,
            "51000",
            "0.002",
        )
        .force(Force::PostOnly)
        .new_client_order_id("new-batch-order-1")
        .stp_mode(STPMode::CancelTaker);

        assert_eq!(item.force, Force::PostOnly);
        assert_eq!(item.new_client_order_id, Some("new-batch-order-1".to_string()));
        assert_eq!(item.stp_mode, Some(STPMode::CancelTaker));
    }

    #[test]
    fn test_batch_cancel_replace_orders_request() {
        let orders = vec![
            BatchCancelReplaceOrderItem::limit_by_order_id(
                "BTCUSDT",
                "1001",
                OrderSide::Buy,
                "51000",
                "0.002",
            ),
            BatchCancelReplaceOrderItem::market_by_client_order_id(
                "ETHUSDT",
                "my-order-123",
                OrderSide::Sell,
                "1.5",
            ),
        ];

        let request = BatchCancelReplaceOrdersRequest::new(orders);

        assert_eq!(request.order_list.len(), 2);
        assert_eq!(request.order_list[0].symbol, "BTCUSDT");
        assert_eq!(request.order_list[1].symbol, "ETHUSDT");
    }

    #[test]
    fn test_batch_cancel_replace_orders_request_serialization() {
        let orders = vec![
            BatchCancelReplaceOrderItem::limit_by_order_id(
                "BTCUSDT",
                "1001",
                OrderSide::Buy,
                "51000",
                "0.002",
            )
            .new_client_order_id("new-order-1"),
        ];

        let request = BatchCancelReplaceOrdersRequest::new(orders);
        let json = serde_json::to_string(&request).unwrap();

        assert!(json.contains("\"orderList\""));
        assert!(json.contains("\"symbol\":\"BTCUSDT\""));
        assert!(json.contains("\"orderId\":\"1001\""));
        assert!(json.contains("\"side\":\"buy\""));
        assert!(json.contains("\"orderType\":\"limit\""));
        assert!(json.contains("\"newClientOid\":\"new-order-1\""));
    }

    #[test]
    fn test_batch_cancel_replace_order_result_deserialization() {
        let json = r#"{
            "orderId": "2001",
            "clientOid": "new-order-1",
            "cancelOrderId": "1001",
            "cancelClientOid": "old-order-1",
            "success": true,
            "errorCode": null,
            "errorMsg": null
        }"#;

        let result: BatchCancelReplaceOrderResult = serde_json::from_str(json).unwrap();

        assert_eq!(result.order_id, Some("2001".to_string()));
        assert_eq!(result.client_order_id, Some("new-order-1".to_string()));
        assert_eq!(result.cancel_order_id, Some("1001".to_string()));
        assert_eq!(result.cancel_client_order_id, Some("old-order-1".to_string()));
        assert!(result.success);
        assert!(result.error_code.is_none());
        assert!(result.error_msg.is_none());
    }

    #[test]
    fn test_batch_cancel_replace_order_result_deserialization_failure() {
        let json = r#"{
            "orderId": null,
            "clientOid": null,
            "cancelOrderId": null,
            "cancelClientOid": "old-order-2",
            "success": false,
            "errorCode": "43025",
            "errorMsg": "Order does not exist"
        }"#;

        let result: BatchCancelReplaceOrderResult = serde_json::from_str(json).unwrap();

        assert!(result.order_id.is_none());
        assert!(result.client_order_id.is_none());
        assert!(result.cancel_order_id.is_none());
        assert_eq!(result.cancel_client_order_id, Some("old-order-2".to_string()));
        assert!(!result.success);
        assert_eq!(result.error_code, Some("43025".to_string()));
        assert_eq!(result.error_msg, Some("Order does not exist".to_string()));
    }
}
