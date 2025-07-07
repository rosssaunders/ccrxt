use serde::{Deserialize, Serialize};

use crate::bitget::{OrderSide, OrderType, RestResult};

use super::super::RestClient;
use super::place_order::{Force, STPMode};

const BATCH_ORDERS_ENDPOINT: &str = "/api/v2/spot/trade/batch-orders";
/// Single order request within a batch
#[derive(Debug, Clone, Serialize)]
pub struct BatchOrderItem {
    /// Trading pair name, e.g. BTCUSDT
    pub symbol: String,

    /// Order direction: buy or sell
    pub side: OrderSide,

    /// Order type: limit or market
    #[serde(rename = "orderType")]
    pub order_type: OrderType,

    /// Execution strategy (invalid when orderType is market)
    pub force: Force,

    /// Limit price (required for limit orders)
    /// The decimal places of price can be obtained from Get Symbol Info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Order amount
    /// For Limit and Market-Sell orders: represents the number of base coins
    /// For Market-Buy orders: represents the number of quote coins
    pub size: String,

    /// Custom order ID (optional)
    #[serde(rename = "clientOid", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,

    /// Self-trade prevention mode
    #[serde(rename = "stpMode", skip_serializing_if = "Option::is_none")]
    pub stp_mode: Option<STPMode>,
}

/// Request parameters for placing batch orders
#[derive(Debug, Clone, Serialize)]
pub struct BatchOrdersRequest {
    /// List of orders to place (maximum 20 orders per batch)
    #[serde(rename = "orderList")]
    pub order_list: Vec<BatchOrderItem>,

    /// Request timestamp (Unix milliseconds)
    #[serde(rename = "requestTime", skip_serializing_if = "Option::is_none")]
    pub request_time: Option<i64>,

    /// Valid time window (Unix milliseconds)
    /// If set, request is valid only when server time is within receiveWindow
    #[serde(rename = "receiveWindow", skip_serializing_if = "Option::is_none")]
    pub receive_window: Option<i64>,
}

/// Result of a single order in the batch
#[derive(Debug, Clone, Deserialize)]
pub struct BatchOrderResult {
    /// Order ID assigned by the system (if successful)
    #[serde(rename = "orderId")]
    pub order_id: Option<String>,

    /// Custom order ID (if provided in request)
    #[serde(rename = "clientOid")]
    pub client_order_id: Option<String>,

    /// Success status
    pub success: bool,

    /// Error code (if failed)
    #[serde(rename = "errorCode")]
    pub error_code: Option<String>,

    /// Error message (if failed)
    #[serde(rename = "errorMsg")]
    pub error_msg: Option<String>,
}

/// Response from placing batch orders
#[derive(Debug, Clone, Deserialize)]
pub struct BatchOrdersResponse {
    /// List of order results
    #[serde(rename = "orderInfo")]
    pub order_info: Vec<BatchOrderResult>,

    /// List of failed orders (if any)
    #[serde(rename = "failure")]
    pub failure: Option<Vec<BatchOrderResult>>,

    /// List of successful orders (if any)
    #[serde(rename = "success")]
    pub success: Option<Vec<BatchOrderResult>>,
}

impl RestClient {
    /// Place multiple spot trading orders in batch
    ///
    /// Places multiple orders for spot trading with the specified parameters.
    /// Maximum 20 orders per batch.
    ///
    /// # Arguments
    /// * `request` - The batch order placement request parameters
    ///
    /// # Rate Limit
    /// 5 requests per second per UID
    /// Maximum 20 orders per batch
    ///
    /// # Returns
    /// A result containing the batch order placement response or an error
    pub async fn batch_orders(
        &self,
        request: BatchOrdersRequest,
    ) -> RestResult<BatchOrdersResponse> {
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
            BATCH_ORDERS_ENDPOINT,
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
    fn test_batch_order_item_limit() {
        let item = BatchOrderItem {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            force: Force::GTC,
            price: Some("50000".to_string()),
            size: "0.001".to_string(),
            client_order_id: None,
            stp_mode: None,
        };

        assert_eq!(item.symbol, "BTCUSDT");
        assert_eq!(item.side, OrderSide::Buy);
        assert_eq!(item.order_type, OrderType::Limit);
        assert_eq!(item.force, Force::GTC);
        assert_eq!(item.price, Some("50000".to_string()));
        assert_eq!(item.size, "0.001");
        assert!(item.client_order_id.is_none());
    }

    #[test]
    fn test_batch_order_item_market() {
        let item = BatchOrderItem {
            symbol: "ETHUSDT".to_string(),
            side: OrderSide::Sell,
            order_type: OrderType::Market,
            force: Force::GTC,
            price: None,
            size: "1.0".to_string(),
            client_order_id: None,
            stp_mode: None,
        };

        assert_eq!(item.symbol, "ETHUSDT");
        assert_eq!(item.side, OrderSide::Sell);
        assert_eq!(item.order_type, OrderType::Market);
        assert!(item.price.is_none());
        assert_eq!(item.size, "1.0");
    }

    #[test]
    fn test_batch_order_item_builder() {
        let item = BatchOrderItem {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            force: Force::PostOnly,
            price: Some("50000".to_string()),
            size: "0.001".to_string(),
            client_order_id: Some("batch-order-1".to_string()),
            stp_mode: Some(STPMode::CancelTaker),
        };

        assert_eq!(item.force, Force::PostOnly);
        assert_eq!(item.client_order_id, Some("batch-order-1".to_string()));
        assert_eq!(item.stp_mode, Some(STPMode::CancelTaker));
    }

    #[test]
    fn test_batch_orders_request() {
        let orders = vec![
            BatchOrderItem {
                symbol: "BTCUSDT".to_string(),
                side: OrderSide::Buy,
                order_type: OrderType::Limit,
                force: Force::GTC,
                price: Some("50000".to_string()),
                size: "0.001".to_string(),
                client_order_id: None,
                stp_mode: None,
            },
            BatchOrderItem {
                symbol: "ETHUSDT".to_string(),
                side: OrderSide::Sell,
                order_type: OrderType::Market,
                force: Force::GTC,
                price: None,
                size: "1.0".to_string(),
                client_order_id: None,
                stp_mode: None,
            },
        ];

        let request = BatchOrdersRequest {
            order_list: orders,
            request_time: None,
            receive_window: None,
        };

        assert_eq!(request.order_list.len(), 2);
        assert_eq!(request.order_list[0].symbol, "BTCUSDT");
        assert_eq!(request.order_list[1].symbol, "ETHUSDT");
    }

    #[test]
    fn test_batch_orders_request_serialization() {
        let orders = vec![BatchOrderItem {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            force: Force::GTC,
            price: Some("50000".to_string()),
            size: "0.001".to_string(),
            client_order_id: Some("order-1".to_string()),
            stp_mode: None,
        }];

        let request = BatchOrdersRequest {
            order_list: orders,
            request_time: None,
            receive_window: None,
        };
        let json = serde_json::to_string(&request).unwrap();

        assert!(json.contains("\"orderList\""));
        assert!(json.contains("\"symbol\":\"BTCUSDT\""));
        assert!(json.contains("\"side\":\"buy\""));
        assert!(json.contains("\"orderType\":\"limit\""));
        assert!(json.contains("\"clientOid\":\"order-1\""));
    }

    #[test]
    fn test_batch_order_result_deserialization() {
        let json = r#"{
            "orderId": "1001",
            "clientOid": "order-1",
            "success": true,
            "errorCode": null,
            "errorMsg": null
        }"#;

        let result: BatchOrderResult = serde_json::from_str(json).unwrap();

        assert_eq!(result.order_id, Some("1001".to_string()));
        assert_eq!(result.client_order_id, Some("order-1".to_string()));
        assert!(result.success);
        assert!(result.error_code.is_none());
        assert!(result.error_msg.is_none());
    }

    #[test]
    fn test_batch_order_result_deserialization_failure() {
        let json = r#"{
            "orderId": null,
            "clientOid": "order-2",
            "success": false,
            "errorCode": "40001",
            "errorMsg": "Invalid symbol"
        }"#;

        let result: BatchOrderResult = serde_json::from_str(json).unwrap();

        assert!(result.order_id.is_none());
        assert_eq!(result.client_order_id, Some("order-2".to_string()));
        assert!(!result.success);
        assert_eq!(result.error_code, Some("40001".to_string()));
        assert_eq!(result.error_msg, Some("Invalid symbol".to_string()));
    }
}
