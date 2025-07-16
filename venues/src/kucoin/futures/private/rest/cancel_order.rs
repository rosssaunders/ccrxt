use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

/// Endpoint URL for cancel order (format string)
pub const CANCEL_ORDER_ENDPOINT: &str = "/api/v1/orders/";

#[derive(Debug, Clone, Serialize)]
pub struct CancelOrderRequest {
    pub order_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {
    pub cancelled_order_ids: Vec<String>,
}

impl super::RestClient {
    /// Cancel an order
    pub async fn cancel_order(
        &self,
        request: CancelOrderRequest,
    ) -> Result<(RestResponse<CancelOrderResponse>, ResponseHeaders)> {
        let endpoint = format!("{}{}", CANCEL_ORDER_ENDPOINT, request.order_id);
        self.delete(&endpoint, None::<Option<()>>).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel_order_request_creation() {
        let request = CancelOrderRequest {
            order_id: "5e8c8c2f1a3b4a001c5d8e31".to_string(),
        };
        assert_eq!(request.order_id, "5e8c8c2f1a3b4a001c5d8e31");
    }

    #[test]
    fn test_cancel_order_response_deserialization() {
        let json = r#"{
            "cancelledOrderIds": ["order1", "order2"]
        }"#;

        let response: CancelOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.cancelled_order_ids.len(), 2);
        assert_eq!(response.cancelled_order_ids[0], "order1");
        assert_eq!(response.cancelled_order_ids[1], "order2");
    }

    #[test]
    fn test_cancel_order_response_deserialization_single() {
        let json = r#"{"cancelledOrderIds": ["5e8c8c2f1a3b4a001c5d8e31"]}"#;
        let response: CancelOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.cancelled_order_ids.len(), 1);
        assert_eq!(response.cancelled_order_ids[0], "5e8c8c2f1a3b4a001c5d8e31");
    }

    #[test]
    fn test_cancel_order_endpoint() {
        assert_eq!(CANCEL_ORDER_ENDPOINT, "/api/v1/orders/");
    }

    #[test]
    fn test_cancel_order_endpoint_formatting() {
        let order_id = "test123";
        let endpoint = format!("{}{}", CANCEL_ORDER_ENDPOINT, order_id);
        assert_eq!(endpoint, "/api/v1/orders/test123");
    }
}
