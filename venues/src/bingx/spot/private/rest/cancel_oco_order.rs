use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bingx::spot::{EndpointType, RestResult};

const CANCEL_OCO_ORDER_ENDPOINT: &str = "/openApi/spot/v1/oco/cancel";

/// Request for canceling an OCO order list
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOcoOrderRequest {
    /// Order ID of the limit order or stop-limit order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// User-defined order ID of the limit order or stop-limit order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    /// Request validity window in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
    /// Request timestamp in milliseconds
    pub timestamp: u64,
}

/// Response for canceling an OCO order
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOcoOrderResponse {
    /// Order ID
    pub order_id: String,
    /// User-defined order ID
    pub client_order_id: String,
}

impl RestClient {
    /// Cancel an OCO order list
    ///
    /// Used to cancel the entire OCO order.
    ///
    /// # Arguments
    /// * `request` - The cancel OCO order request
    ///
    /// # Returns
    /// * `RestResult<CancelOcoOrderResponse>` - The cancel OCO order response or error
    pub async fn cancel_oco_order(
        &self,
        request: &CancelOcoOrderRequest,
    ) -> RestResult<CancelOcoOrderResponse> {
        self.send_post_signed_request(CANCEL_OCO_ORDER_ENDPOINT, request, EndpointType::Trading,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel_oco_order_request_serialization_with_order_id() {
        let request = CancelOcoOrderRequest {
            order_id: Some("123456789".to_string()),
            client_order_id: None,
            recv_window: Some(5000),
            timestamp: 1658748648396,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("123456789"));
        assert!(!json.contains("clientOrderId"));
        assert!(json.contains("1658748648396"));
    }

    #[test]
    fn test_cancel_oco_order_request_serialization_with_client_order_id() {
        let request = CancelOcoOrderRequest {
            order_id: None,
            client_order_id: Some("my_oco_order".to_string()),
            recv_window: None,
            timestamp: 1658748648396,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(!json.contains("orderId"));
        assert!(json.contains("my_oco_order"));
        assert!(json.contains("1658748648396"));
    }

    #[test]
    fn test_cancel_oco_order_response_deserialization() {
        let json = r#"{
            "orderId": "123456789",
            "clientOrderId": "my_oco_order"
        }"#;

        let response: CancelOcoOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.order_id, "123456789");
        assert_eq!(response.client_order_id, "my_oco_order");
    }
}
