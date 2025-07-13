use serde::{Deserialize, Serialize};

use super::super::RestClient;
use crate::bitget::spot::RestResult;

/// Endpoint for canceling a plan order
const CANCEL_PLAN_ORDER_ENDPOINT: &str = "/api/v2/spot/plan/cancel-plan-order";

/// Request parameters for cancelling a plan order
#[derive(Debug, Clone, Serialize)]
pub struct CancelPlanOrderRequest {
    /// Trading pair name, e.g. BTCUSDT
    pub symbol: String,

    /// Plan order ID (either orderId or clientOrderId is required)
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    /// Client order ID (either orderId or clientOrderId is required)
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,

    /// Request timestamp (Unix milliseconds)
    #[serde(rename = "requestTime", skip_serializing_if = "Option::is_none")]
    pub request_time: Option<i64>,

    /// Valid time window (Unix milliseconds)
    /// If set, request is valid only when server time is within receiveWindow
    #[serde(rename = "receiveWindow", skip_serializing_if = "Option::is_none")]
    pub receive_window: Option<i64>,
}

/// Response from cancelling a plan order
#[derive(Debug, Clone, Deserialize)]
pub struct CancelPlanOrderResponse {
    /// Plan order ID
    #[serde(rename = "orderId")]
    pub order_id: String,

    /// Client order ID (if provided)
    #[serde(rename = "clientOid")]
    pub client_order_id: Option<String>,
}

impl RestClient {
    /// Cancel a specific spot plan order (trigger/stop order)
    ///
    /// Cancels an existing plan order by order ID or client order ID.
    ///
    /// # Arguments
    /// * `request` - The cancel plan order request parameters
    ///
    /// # Rate Limit
    /// 10 requests per second per UID
    ///
    /// # Returns
    /// A result containing the cancel plan order response or an error
    pub async fn cancel_plan_order(
        &self,
        request: CancelPlanOrderRequest,
    ) -> RestResult<CancelPlanOrderResponse> {
        let body = serde_json::to_string(&request).map_err(|e| {
            crate::bitget::spot::Errors::Error(format!("Failed to serialize request: {e}"))
        })?;

        self.send_signed_request(
            CANCEL_PLAN_ORDER_ENDPOINT,
            reqwest::Method::POST,
            None,        // No query parameters
            Some(&body), // JSON body
            10,          // 10 requests per second rate limit
            true,        // This is an order-related endpoint
            Some(10),    // Order-specific rate limit
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel_plan_order_request_by_order_id() {
        let request = CancelPlanOrderRequest {
            symbol: "BTCUSDT".to_string(),
            order_id: Some("plan_1234567890".to_string()),
            client_order_id: None,
            request_time: None,
            receive_window: None,
        };

        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.order_id, Some("plan_1234567890".to_string()));
        assert!(request.client_order_id.is_none());
        assert!(request.request_time.is_none());
        assert!(request.receive_window.is_none());
    }

    #[test]
    fn test_cancel_plan_order_request_by_client_order_id() {
        let request = CancelPlanOrderRequest {
            symbol: "ETHUSDT".to_string(),
            order_id: None,
            client_order_id: Some("my-plan-order-123".to_string()),
            request_time: None,
            receive_window: None,
        };

        assert_eq!(request.symbol, "ETHUSDT");
        assert!(request.order_id.is_none());
        assert_eq!(
            request.client_order_id,
            Some("my-plan-order-123".to_string())
        );
    }

    #[test]
    fn test_cancel_plan_order_request_builder() {
        let request = CancelPlanOrderRequest {
            symbol: "BTCUSDT".to_string(),
            order_id: Some("plan_1234567890".to_string()),
            client_order_id: None,
            request_time: Some(1640995200000),
            receive_window: Some(5000),
        };

        assert_eq!(request.request_time, Some(1640995200000));
        assert_eq!(request.receive_window, Some(5000));
    }

    #[test]
    fn test_cancel_plan_order_request_serialization() {
        let request = CancelPlanOrderRequest {
            symbol: "BTCUSDT".to_string(),
            order_id: Some("plan_1234567890".to_string()),
            client_order_id: None,
            request_time: Some(1640995200000),
            receive_window: None,
        };

        let json = serde_json::to_string(&request).unwrap();

        assert!(json.contains("\"symbol\":\"BTCUSDT\""));
        assert!(json.contains("\"orderId\":\"plan_1234567890\""));
        assert!(json.contains("\"requestTime\":1640995200000"));
        assert!(!json.contains("clientOrderId"));
    }

    #[test]
    fn test_cancel_plan_order_request_serialization_client_id() {
        let request = CancelPlanOrderRequest {
            symbol: "ETHUSDT".to_string(),
            order_id: None,
            client_order_id: Some("my-plan-order-123".to_string()),
            request_time: None,
            receive_window: None,
        };

        let json = serde_json::to_string(&request).unwrap();

        assert!(json.contains("\"symbol\":\"ETHUSDT\""));
        assert!(json.contains("\"clientOrderId\":\"my-plan-order-123\""));
        assert!(!json.contains("orderId"));
    }

    #[test]
    fn test_cancel_plan_order_response_deserialization() {
        let json = r#"{
            "orderId": "plan_121211212122",
            "clientOid": "xx001"
        }"#;

        let response: CancelPlanOrderResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.order_id, "plan_121211212122");
        assert_eq!(response.client_order_id, Some("xx001".to_string()));
    }

    #[test]
    fn test_cancel_plan_order_response_deserialization_no_client_id() {
        let json = r#"{
            "orderId": "plan_121211212122",
            "clientOid": null
        }"#;

        let response: CancelPlanOrderResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.order_id, "plan_121211212122");
        assert!(response.client_order_id.is_none());
    }
}
