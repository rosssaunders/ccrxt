use serde::{Deserialize, Serialize};

use super::super::RestClient;
use crate::bitget::spot::RestResult;

/// Endpoint for batch canceling orders
const BATCH_CANCEL_ORDERS_ENDPOINT: &str = "/api/v2/spot/trade/batch-cancel-order";

/// Single order cancellation request within a batch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCancelOrderItem {
    /// Trading pair name, e.g. BTCUSDT
    pub symbol: String,

    /// Order ID (either orderId or clientOrderId is required)
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    /// Client order ID (either orderId or clientOrderId is required)
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
}

/// Request parameters for batch cancelling orders
#[derive(Debug, Clone, Serialize)]
pub struct BatchCancelOrdersRequest {
    /// List of orders to cancel (maximum 20 orders per batch)
    #[serde(rename = "orderList")]
    pub order_list: Vec<BatchCancelOrderItem>,

    /// Request timestamp (Unix milliseconds)
    #[serde(rename = "requestTime", skip_serializing_if = "Option::is_none")]
    pub request_time: Option<i64>,

    /// Valid time window (Unix milliseconds)
    /// If set, request is valid only when server time is within receiveWindow
    #[serde(rename = "receiveWindow", skip_serializing_if = "Option::is_none")]
    pub receive_window: Option<i64>,
}

/// Result of a single order cancellation in the batch
#[derive(Debug, Clone, Deserialize)]
pub struct BatchCancelOrderResult {
    /// Order ID
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

/// Response from batch cancelling orders
#[derive(Debug, Clone, Deserialize)]
pub struct BatchCancelOrdersResponse {
    /// List of cancellation results
    #[serde(rename = "orderInfo")]
    pub order_info: Vec<BatchCancelOrderResult>,

    /// List of failed cancellations (if any)
    #[serde(rename = "failure")]
    pub failure: Option<Vec<BatchCancelOrderResult>>,

    /// List of successful cancellations (if any)
    #[serde(rename = "success")]
    pub success: Option<Vec<BatchCancelOrderResult>>,
}

impl RestClient {
    /// Cancel multiple spot trading orders in batch
    ///
    /// Cancels multiple orders for spot trading with the specified parameters.
    /// Maximum 20 orders per batch.
    ///
    /// [docs]: https://www.bitget.com/api-doc/spot/trade/Batch-Cancel-Orders
    /// Endpoint: POST /api/v2/spot/trade/batch-cancel-order
    /// * `request` - The batch order cancellation request parameters
    ///
    /// # Rate Limit
    /// 5 requests per second per UID
    /// Maximum 20 orders per batch
    ///
    /// # Returns
    /// A result containing the batch order cancellation response or an error
    pub async fn batch_cancel_orders(
        &self,
        request: BatchCancelOrdersRequest,
    ) -> RestResult<BatchCancelOrdersResponse> {
        // Validate that we don't exceed the maximum batch size
        if request.order_list.len() > 20 {
            return Err(crate::bitget::spot::Errors::Error(
                "Maximum 20 orders allowed per batch".to_string(),
            ));
        }

        if request.order_list.is_empty() {
            return Err(crate::bitget::spot::Errors::Error(
                "At least one order is required".to_string(),
            ));
        }

        let body = serde_json::to_string(&request).map_err(|e| {
            crate::bitget::spot::Errors::Error(format!("Failed to serialize request: {e}"))
        })?;

        self.send_signed_request(
            BATCH_CANCEL_ORDERS_ENDPOINT,
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
    fn test_batch_cancel_order_item_by_order_id() {
        let item = BatchCancelOrderItem {
            symbol: "BTCUSDT".to_string(),
            order_id: Some("1234567890".to_string()),
            client_order_id: None,
        };
        assert_eq!(item.symbol, "BTCUSDT");
        assert_eq!(item.order_id, Some("1234567890".to_string()));
        assert!(item.client_order_id.is_none());
    }

    #[test]
    fn test_batch_cancel_order_item_by_client_order_id() {
        let item = BatchCancelOrderItem {
            symbol: "ETHUSDT".to_string(),
            order_id: None,
            client_order_id: Some("my-order-123".to_string()),
        };
        assert_eq!(item.symbol, "ETHUSDT");
        assert!(item.order_id.is_none());
        assert_eq!(item.client_order_id, Some("my-order-123".to_string()));
    }

    #[test]
    fn test_batch_cancel_orders_request() {
        let orders = vec![
            BatchCancelOrderItem {
                symbol: "BTCUSDT".to_string(),
                order_id: Some("1001".to_string()),
                client_order_id: None,
            },
            BatchCancelOrderItem {
                symbol: "ETHUSDT".to_string(),
                order_id: None,
                client_order_id: Some("my-order-123".to_string()),
            },
        ];
        let request = BatchCancelOrdersRequest {
            order_list: orders,
            request_time: None,
            receive_window: None,
        };
        assert_eq!(request.order_list.len(), 2);
        assert_eq!(request.order_list[0].symbol, "BTCUSDT");
        assert_eq!(request.order_list[1].symbol, "ETHUSDT");
    }

    #[test]
    fn test_batch_cancel_orders_request_serialization() {
        let orders = vec![
            BatchCancelOrderItem {
                symbol: "BTCUSDT".to_string(),
                order_id: Some("1001".to_string()),
                client_order_id: None,
            },
            BatchCancelOrderItem {
                symbol: "ETHUSDT".to_string(),
                order_id: None,
                client_order_id: Some("my-order-123".to_string()),
            },
        ];
        let request = BatchCancelOrdersRequest {
            order_list: orders,
            request_time: None,
            receive_window: None,
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"orderList\""));
        assert!(json.contains("\"symbol\":\"BTCUSDT\""));
        assert!(json.contains("\"orderId\":\"1001\""));
        assert!(json.contains("\"symbol\":\"ETHUSDT\""));
        assert!(json.contains("\"clientOrderId\":\"my-order-123\""));
    }

    #[test]
    fn test_batch_cancel_order_result_deserialization() {
        let json = r#"{
            "orderId": "1001",
            "clientOid": "my-order-123",
            "success": true,
            "errorCode": null,
            "errorMsg": null
        }"#;

        let result: BatchCancelOrderResult = serde_json::from_str(json).unwrap();

        assert_eq!(result.order_id, Some("1001".to_string()));
        assert_eq!(result.client_order_id, Some("my-order-123".to_string()));
        assert!(result.success);
        assert!(result.error_code.is_none());
        assert!(result.error_msg.is_none());
    }

    #[test]
    fn test_batch_cancel_order_result_deserialization_failure() {
        let json = r#"{
            "orderId": "1002",
            "clientOid": null,
            "success": false,
            "errorCode": "43025",
            "errorMsg": "Order does not exist"
        }"#;

        let result: BatchCancelOrderResult = serde_json::from_str(json).unwrap();

        assert_eq!(result.order_id, Some("1002".to_string()));
        assert!(result.client_order_id.is_none());
        assert!(!result.success);
        assert_eq!(result.error_code, Some("43025".to_string()));
        assert_eq!(result.error_msg, Some("Order does not exist".to_string()));
    }

    #[test]
    fn test_batch_cancel_orders_response_deserialization() {
        let json = r#"{
            "orderInfo": [
                {
                    "orderId": "1001",
                    "clientOid": "order-1",
                    "success": true,
                    "errorCode": null,
                    "errorMsg": null
                },
                {
                    "orderId": "1002",
                    "clientOid": "order-2",
                    "success": false,
                    "errorCode": "43025",
                    "errorMsg": "Order does not exist"
                }
            ]
        }"#;

        let response: BatchCancelOrdersResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.order_info.len(), 2);
        assert!(response.order_info[0].success);
        assert!(!response.order_info[1].success);
    }
}
