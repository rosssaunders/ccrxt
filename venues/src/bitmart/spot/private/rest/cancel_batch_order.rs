use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bitmart::{RestResult, rate_limit::EndpointType};

const CANCEL_BATCH_ORDER_ENDPOINT: &str = "/spot/v4/cancel_orders";

/// Request parameters for canceling batch orders.
#[derive(Debug, Clone, Serialize, Default)]
pub struct CancelBatchOrderRequest {
    /// Trading pair (e.g., BTC_USDT).
    pub symbol: String,

    /// Order ID list (max 10 IDs) - mutually exclusive with client_order_ids.
    #[serde(rename = "orderIds", skip_serializing_if = "Option::is_none")]
    pub order_ids: Option<Vec<String>>,

    /// Client order ID list (max 10 IDs) - mutually exclusive with order_ids.
    #[serde(rename = "clientOrderIds", skip_serializing_if = "Option::is_none")]
    pub client_order_ids: Option<Vec<String>>,

    /// Trade time limit in milliseconds, allowed range (0,60000], default: 5000.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Response for canceling batch orders.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelBatchOrderResponse {
    /// Successfully canceled order IDs.
    #[serde(rename = "successIds")]
    pub success_ids: Vec<String>,

    /// Order IDs that failed to cancel.
    #[serde(rename = "failIds")]
    pub fail_ids: Vec<String>,

    /// Total number of submissions.
    #[serde(rename = "totalCount")]
    pub total_count: i32,

    /// Number of successful cancellations.
    #[serde(rename = "successCount")]
    pub success_count: i32,

    /// Number of failed cancellations.
    #[serde(rename = "failedCount")]
    pub failed_count: i32,
}

impl RestClient {
    /// Cancel Batch Order(v4)
    ///
    /// Cancels multiple orders by order IDs or client order IDs. Maximum 10 orders per batch.
    /// Must specify either order_ids or client_order_ids, but not both.
    ///
    /// [docs]: https://developer-pro.bitmart.com/en/spot/#cancel-batch-order-v4-signed
    ///
    /// Rate limit: UID-based, 40 times/2 sec
    ///
    /// # Arguments
    /// * `request` - The batch cancel request parameters
    ///
    /// # Returns
    /// Batch cancel response with success/failure details
    pub async fn cancel_batch_order(
        &self,
        request: CancelBatchOrderRequest,
    ) -> RestResult<CancelBatchOrderResponse> {
        self.send_post_signed_request(
            CANCEL_BATCH_ORDER_ENDPOINT,
            request,
            EndpointType::SpotTrading,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel_batch_order_request_with_order_ids() {
        let request = CancelBatchOrderRequest {
            symbol: "BTC_USDT".to_string(),
            order_ids: Some(vec!["12345".to_string(), "67890".to_string()]),
            client_order_ids: None,
            recv_window: Some(5000),
        };

        assert_eq!(request.symbol, "BTC_USDT");
        assert!(request.order_ids.is_some());
        assert!(request.client_order_ids.is_none());
        assert_eq!(request.recv_window, Some(5000));
    }

    #[test]
    fn test_cancel_batch_order_request_with_client_order_ids() {
        let request = CancelBatchOrderRequest {
            symbol: "ETH_USDT".to_string(),
            order_ids: None,
            client_order_ids: Some(vec!["client_123".to_string()]),
            recv_window: None,
        };

        assert_eq!(request.symbol, "ETH_USDT");
        assert!(request.order_ids.is_none());
        assert!(request.client_order_ids.is_some());
        assert!(request.recv_window.is_none());
    }

    #[test]
    fn test_cancel_batch_order_request_default() {
        let request = CancelBatchOrderRequest::default();

        assert!(request.symbol.is_empty());
        assert!(request.order_ids.is_none());
        assert!(request.client_order_ids.is_none());
        assert!(request.recv_window.is_none());
    }

    #[test]
    fn test_cancel_batch_order_response_structure() {
        let response = CancelBatchOrderResponse {
            success_ids: vec!["123".to_string(), "456".to_string()],
            fail_ids: vec!["789".to_string()],
            total_count: 3,
            success_count: 2,
            failed_count: 1,
        };

        assert_eq!(response.success_ids.len(), 2);
        assert_eq!(response.fail_ids.len(), 1);
        assert_eq!(response.total_count, 3);
        assert_eq!(response.success_count, 2);
        assert_eq!(response.failed_count, 1);
    }

    #[test]
    fn test_request_serialization() {
        let request = CancelBatchOrderRequest {
            symbol: "BTC_USDT".to_string(),
            order_ids: Some(vec!["123".to_string()]),
            client_order_ids: None,
            recv_window: Some(5000),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"symbol\":\"BTC_USDT\""));
        assert!(json.contains("\"orderIds\":[\"123\"]"));
        assert!(json.contains("\"recvWindow\":5000"));
        assert!(!json.contains("\"clientOrderIds\""));
    }

    #[test]
    fn test_response_deserialization() {
        let json = r#"{
            "successIds": ["123", "456"],
            "failIds": ["789"],
            "totalCount": 3,
            "successCount": 2,
            "failedCount": 1
        }"#;

        let response: CancelBatchOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.success_ids, vec!["123", "456"]);
        assert_eq!(response.fail_ids, vec!["789"]);
        assert_eq!(response.total_count, 3);
        assert_eq!(response.success_count, 2);
        assert_eq!(response.failed_count, 1);
    }

    #[test]
    fn test_clone_derives() {
        let request = CancelBatchOrderRequest {
            symbol: "BTC_USDT".to_string(),
            order_ids: Some(vec!["123".to_string()]),
            client_order_ids: None,
            recv_window: Some(5000),
        };

        let cloned_request = request.clone();
        assert_eq!(request.symbol, cloned_request.symbol);
        assert_eq!(request.order_ids, cloned_request.order_ids);

        let response = CancelBatchOrderResponse {
            success_ids: vec!["123".to_string()],
            fail_ids: vec![],
            total_count: 1,
            success_count: 1,
            failed_count: 0,
        };

        let cloned_response = response.clone();
        assert_eq!(response.success_ids, cloned_response.success_ids);
        assert_eq!(response.total_count, cloned_response.total_count);
    }
}
