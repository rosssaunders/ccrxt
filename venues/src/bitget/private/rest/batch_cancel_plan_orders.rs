use serde::{Deserialize, Serialize};

use super::super::RestClient;
use crate::bitget::RestResult;

/// Endpoint for batch canceling plan orders
const BATCH_CANCEL_PLAN_ORDERS_ENDPOINT: &str = "/api/v2/spot/plan/batch-cancel-plan-order";

/// Single plan order cancellation request within a batch
#[derive(Debug, Clone, Serialize)]
pub struct BatchCancelPlanOrderItem {
    /// Trading pair name, e.g. BTCUSDT
    pub symbol: String,

    /// Plan order ID (either orderId or clientOrderId is required)
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    /// Client order ID (either orderId or clientOrderId is required)
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
}

/// Request parameters for batch cancelling plan orders
#[derive(Debug, Clone, Serialize)]
pub struct BatchCancelPlanOrdersRequest {
    /// List of plan orders to cancel (maximum 20 orders per batch)
    #[serde(rename = "orderList")]
    pub order_list: Vec<BatchCancelPlanOrderItem>,

    /// Request timestamp (Unix milliseconds)
    #[serde(rename = "requestTime", skip_serializing_if = "Option::is_none")]
    pub request_time: Option<i64>,

    /// Valid time window (Unix milliseconds)
    /// If set, request is valid only when server time is within receiveWindow
    #[serde(rename = "receiveWindow", skip_serializing_if = "Option::is_none")]
    pub receive_window: Option<i64>,
}

/// Result of a single plan order cancellation in the batch
#[derive(Debug, Clone, Deserialize)]
pub struct BatchCancelPlanOrderResult {
    /// Plan order ID
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

/// Response from batch cancelling plan orders
#[derive(Debug, Clone, Deserialize)]
pub struct BatchCancelPlanOrdersResponse {
    /// List of plan order cancellation results
    #[serde(rename = "orderInfo")]
    pub order_info: Vec<BatchCancelPlanOrderResult>,

    /// List of failed cancellations (if any)
    #[serde(rename = "failure")]
    pub failure: Option<Vec<BatchCancelPlanOrderResult>>,

    /// List of successful cancellations (if any)
    #[serde(rename = "success")]
    pub success: Option<Vec<BatchCancelPlanOrderResult>>,
}

impl RestClient {
    /// Cancel multiple spot plan orders in batch
    ///
    /// Cancels multiple plan orders for spot trading with the specified parameters.
    /// Maximum 20 orders per batch.
    ///
    /// # Arguments
    /// * `request` - The batch plan order cancellation request parameters
    ///
    /// # Rate Limit
    /// 5 requests per second per UID
    /// Maximum 20 orders per batch
    ///
    /// # Returns
    /// A result containing the batch plan order cancellation response or an error
    pub async fn batch_cancel_plan_orders(
        &self,
        request: BatchCancelPlanOrdersRequest,
    ) -> RestResult<BatchCancelPlanOrdersResponse> {
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
            BATCH_CANCEL_PLAN_ORDERS_ENDPOINT,
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
    fn test_batch_cancel_plan_order_item_by_order_id() {
        let item = BatchCancelPlanOrderItem {
            symbol: "BTCUSDT".to_string(),
            order_id: Some("plan_1234567890".to_string()),
            client_order_id: None,
        };

        assert_eq!(item.symbol, "BTCUSDT");
        assert_eq!(item.order_id, Some("plan_1234567890".to_string()));
        assert!(item.client_order_id.is_none());
    }

    #[test]
    fn test_batch_cancel_plan_order_item_by_client_order_id() {
        let item = BatchCancelPlanOrderItem {
            symbol: "ETHUSDT".to_string(),
            order_id: None,
            client_order_id: Some("my-plan-order-123".to_string()),
        };

        assert_eq!(item.symbol, "ETHUSDT");
        assert!(item.order_id.is_none());
        assert_eq!(item.client_order_id, Some("my-plan-order-123".to_string()));
    }

    #[test]
    fn test_batch_cancel_plan_orders_request() {
        let orders = vec![
            BatchCancelPlanOrderItem {
                symbol: "BTCUSDT".to_string(),
                order_id: Some("plan_1001".to_string()),
                client_order_id: None,
            },
            BatchCancelPlanOrderItem {
                symbol: "ETHUSDT".to_string(),
                order_id: None,
                client_order_id: Some("my-plan-order-123".to_string()),
            },
        ];

        let request = BatchCancelPlanOrdersRequest {
            order_list: orders,
            request_time: None,
            receive_window: None,
        };

        assert_eq!(request.order_list.len(), 2);
        assert_eq!(request.order_list[0].symbol, "BTCUSDT");
        assert_eq!(request.order_list[1].symbol, "ETHUSDT");
        assert!(request.request_time.is_none());
        assert!(request.receive_window.is_none());
    }

    #[test]
    fn test_batch_cancel_plan_orders_request_builder() {
        let orders = vec![BatchCancelPlanOrderItem {
            symbol: "BTCUSDT".to_string(),
            order_id: Some("plan_1001".to_string()),
            client_order_id: None,
        }];

        let request = BatchCancelPlanOrdersRequest {
            order_list: orders,
            request_time: Some(1640995200000),
            receive_window: Some(5000),
        };

        assert_eq!(request.request_time, Some(1640995200000));
        assert_eq!(request.receive_window, Some(5000));
    }

    #[test]
    fn test_batch_cancel_plan_orders_request_serialization() {
        let orders = vec![
            BatchCancelPlanOrderItem {
                symbol: "BTCUSDT".to_string(),
                order_id: Some("plan_1001".to_string()),
                client_order_id: None,
            },
            BatchCancelPlanOrderItem {
                symbol: "ETHUSDT".to_string(),
                order_id: None,
                client_order_id: Some("my-plan-order-123".to_string()),
            },
        ];

        let request = BatchCancelPlanOrdersRequest {
            order_list: orders,
            request_time: None,
            receive_window: None,
        };
        let json = serde_json::to_string(&request).unwrap();

        assert!(json.contains("\"orderList\""));
        assert!(json.contains("\"symbol\":\"BTCUSDT\""));
        assert!(json.contains("\"orderId\":\"plan_1001\""));
        assert!(json.contains("\"symbol\":\"ETHUSDT\""));
        assert!(json.contains("\"clientOrderId\":\"my-plan-order-123\""));
    }

    #[test]
    fn test_batch_cancel_plan_order_result_deserialization() {
        let json = r#"{
            "orderId": "plan_1001",
            "clientOid": "my-plan-order-123",
            "success": true,
            "errorCode": null,
            "errorMsg": null
        }"#;

        let result: BatchCancelPlanOrderResult = serde_json::from_str(json).unwrap();

        assert_eq!(result.order_id, Some("plan_1001".to_string()));
        assert_eq!(
            result.client_order_id,
            Some("my-plan-order-123".to_string())
        );
        assert!(result.success);
        assert!(result.error_code.is_none());
        assert!(result.error_msg.is_none());
    }

    #[test]
    fn test_batch_cancel_plan_order_result_deserialization_failure() {
        let json = r#"{
            "orderId": "plan_1002",
            "clientOid": null,
            "success": false,
            "errorCode": "43025",
            "errorMsg": "Plan order does not exist"
        }"#;

        let result: BatchCancelPlanOrderResult = serde_json::from_str(json).unwrap();

        assert_eq!(result.order_id, Some("plan_1002".to_string()));
        assert!(result.client_order_id.is_none());
        assert!(!result.success);
        assert_eq!(result.error_code, Some("43025".to_string()));
        assert_eq!(
            result.error_msg,
            Some("Plan order does not exist".to_string())
        );
    }

    #[test]
    fn test_batch_cancel_plan_orders_response_deserialization() {
        let json = r#"{
            "orderInfo": [
                {
                    "orderId": "plan_1001",
                    "clientOid": "plan-order-1",
                    "success": true,
                    "errorCode": null,
                    "errorMsg": null
                },
                {
                    "orderId": "plan_1002",
                    "clientOid": "plan-order-2",
                    "success": false,
                    "errorCode": "43025",
                    "errorMsg": "Plan order does not exist"
                }
            ]
        }"#;

        let response: BatchCancelPlanOrdersResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.order_info.len(), 2);
        assert!(response.order_info[0].success);
        assert!(!response.order_info[1].success);
    }
}
