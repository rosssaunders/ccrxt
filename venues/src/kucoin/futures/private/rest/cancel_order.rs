use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

/// Endpoint URL for Cancel Order (format string)
const CANCEL_ORDER_ENDPOINT: &str = "/api/v1/orders/";

/// Request parameters for cancelling a specific order.
#[derive(Debug, Clone, Serialize)]
pub struct CancelOrderRequest {
    /// The order ID to cancel.
    pub order_id: String,
}

/// Response data from the cancel order endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {
    /// List of cancelled order IDs.
    pub cancelled_order_ids: Vec<String>,
}

impl super::RestClient {
    /// Cancel Order
    ///
    /// Cancel a specific order by its order ID. This will cancel the order immediately
    /// if it's still active and has not been filled.
    ///
    /// [docs]: https://www.kucoin.com/docs-new/rest/futures-trading/orders/cancel-order-by-orderld
    ///
    /// Rate limit: 40
    ///
    /// # Arguments
    /// * `request` - The cancel order request containing the order ID
    ///
    /// # Returns
    /// List of cancelled order IDs (usually contains single order ID)
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
    fn test_cancel_order_request_with_different_order_id_formats() {
        // Test various order ID formats
        let order_ids = vec![
            "5e8c8c2f1a3b4a001c5d8e31", // Standard KuCoin format
            "order123",                 // Simple format
            "test-order-12345",         // With hyphens
            "64c12345a987b654321",      // Hex format
        ];

        for order_id in order_ids {
            let request = CancelOrderRequest {
                order_id: order_id.to_string(),
            };
            assert_eq!(request.order_id, order_id);
        }
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
    fn test_cancel_order_response_deserialization_empty() {
        let json = r#"{"cancelledOrderIds": []}"#;
        let response: CancelOrderResponse = serde_json::from_str(json).unwrap();
        assert!(response.cancelled_order_ids.is_empty());
    }

    #[test]
    fn test_cancel_order_response_deserialization_multiple() {
        let json = r#"{
            "cancelledOrderIds": [
                "5e8c8c2f1a3b4a001c5d8e31",
                "5e8c8c2f1a3b4a001c5d8e32",
                "5e8c8c2f1a3b4a001c5d8e33"
            ]
        }"#;

        let response: CancelOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.cancelled_order_ids.len(), 3);
        assert!(
            response
                .cancelled_order_ids
                .contains(&"5e8c8c2f1a3b4a001c5d8e31".to_string())
        );
        assert!(
            response
                .cancelled_order_ids
                .contains(&"5e8c8c2f1a3b4a001c5d8e32".to_string())
        );
        assert!(
            response
                .cancelled_order_ids
                .contains(&"5e8c8c2f1a3b4a001c5d8e33".to_string())
        );
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

    #[test]
    fn test_cancel_order_endpoint_formatting_with_various_ids() {
        let test_cases = vec![
            (
                "5e8c8c2f1a3b4a001c5d8e31",
                "/api/v1/orders/5e8c8c2f1a3b4a001c5d8e31",
            ),
            ("short123", "/api/v1/orders/short123"),
            (
                "order-with-hyphens-123",
                "/api/v1/orders/order-with-hyphens-123",
            ),
            ("64c12345a987b654321", "/api/v1/orders/64c12345a987b654321"),
        ];

        for (order_id, expected_endpoint) in test_cases {
            let endpoint = format!("{}{}", CANCEL_ORDER_ENDPOINT, order_id);
            assert_eq!(endpoint, expected_endpoint);
        }
    }

    #[test]
    fn test_cancel_order_request_field_types() {
        let request = CancelOrderRequest {
            order_id: "test_order_123".to_string(),
        };

        // Test field type
        assert_eq!(request.order_id, "test_order_123");
        assert!(request.order_id.len() > 0);

        // Test that order_id is a String type
        let _: String = request.order_id;
    }

    #[test]
    fn test_cancel_order_response_field_types() {
        let json = r#"{
            "cancelledOrderIds": ["5e8c8c2f1a3b4a001c5d8e31", "5e8c8c2f1a3b4a001c5d8e32"]
        }"#;

        let response: CancelOrderResponse = serde_json::from_str(json).unwrap();

        // Test field types
        assert_eq!(response.cancelled_order_ids.len(), 2);
        assert!(response.cancelled_order_ids.iter().all(|id| id.len() > 0));

        // Verify order ID format (typical KuCoin order ID length)
        for order_id in &response.cancelled_order_ids {
            assert!(order_id.len() >= 20); // KuCoin order IDs are typically 24 characters
        }

        // Test that cancelled_order_ids is Vec<String>
        let _: Vec<String> = response.cancelled_order_ids;
    }

    #[test]
    fn test_cancel_order_order_id_length_constraints() {
        // Test minimum reasonable order ID length
        let short_request = CancelOrderRequest {
            order_id: "a".to_string(),
        };
        assert!(short_request.order_id.len() >= 1);

        // Test typical KuCoin order ID length
        let typical_request = CancelOrderRequest {
            order_id: "5e8c8c2f1a3b4a001c5d8e31".to_string(),
        };
        assert_eq!(typical_request.order_id.len(), 24);

        // Test longer order ID
        let long_request = CancelOrderRequest {
            order_id: "very-long-order-id-with-many-characters-1234567890".to_string(),
        };
        assert!(long_request.order_id.len() > 24);
    }

    #[test]
    fn test_cancel_order_json_field_names() {
        let response = CancelOrderResponse {
            cancelled_order_ids: vec!["order1".to_string(), "order2".to_string()],
        };

        let json = serde_json::to_string(&response).unwrap();
        // Verify camelCase field name
        assert!(json.contains("cancelledOrderIds"));
        assert!(!json.contains("cancelled_order_ids"));
    }
}
