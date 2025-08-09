use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

/// Endpoint URL for Cancel All Orders
const CANCEL_ALL_ORDERS_ENDPOINT: &str = "/api/v1/orders";

/// Request parameters for cancelling all orders.
#[derive(Debug, Clone, Default, Serialize)]
pub struct CancelAllOrdersRequest {
    /// Optional symbol to cancel orders for a specific trading pair only.
    /// If not provided, cancels all orders across all symbols.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

/// Response data from the cancel all orders endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrdersResponse {
    /// List of cancelled order IDs.
    pub cancelled_order_ids: Vec<String>,
}

impl super::RestClient {
    /// Cancel All Orders
    ///
    /// Cancel all outstanding orders for the account. Optionally filter by symbol
    /// to cancel orders for a specific trading pair only.
    ///
    /// [docs]: https://www.kucoin.com/docs-new/rest/futures-trading/orders/cancel-all-orders
    ///
    /// Rate limit: 40
    ///
    /// # Arguments
    /// * `request` - The cancel all orders request parameters
    ///
    /// # Returns
    /// List of cancelled order IDs
    pub async fn cancel_all_orders(
        &self,
        request: CancelAllOrdersRequest,
    ) -> Result<(CancelAllOrdersResponse, ResponseHeaders)> {
        let (response, headers): (RestResponse<CancelAllOrdersResponse>, ResponseHeaders) = self
            .delete_with_request(CANCEL_ALL_ORDERS_ENDPOINT, &request)
            .await?;

        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel_all_orders_request_serialization_with_symbol() {
        let request = CancelAllOrdersRequest {
            symbol: Some("XBTUSDTM".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("XBTUSDTM"));
        assert!(!json.contains("null"));
    }

    #[test]
    fn test_cancel_all_orders_request_serialization_without_symbol() {
        let request = CancelAllOrdersRequest { symbol: None };

        let json = serde_json::to_string(&request).unwrap();
        assert_eq!(json, "{}");
        assert!(!json.contains("symbol"));
    }

    #[test]
    fn test_cancel_all_orders_request_serialization_minimal() {
        let request = CancelAllOrdersRequest::default();

        let json = serde_json::to_string(&request).unwrap();
        assert_eq!(json, "{}");
        assert!(!json.contains("symbol"));
    }

    #[test]
    fn test_cancel_all_orders_request_default() {
        let request = CancelAllOrdersRequest::default();
        assert!(request.symbol.is_none());
    }

    #[test]
    fn test_cancel_all_orders_response_deserialization() {
        let json = r#"{
            "cancelledOrderIds": ["order1", "order2", "order3"]
        }"#;

        let response: CancelAllOrdersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.cancelled_order_ids.len(), 3);
        assert_eq!(response.cancelled_order_ids[0], "order1");
        assert_eq!(response.cancelled_order_ids[1], "order2");
        assert_eq!(response.cancelled_order_ids[2], "order3");
    }

    #[test]
    fn test_cancel_all_orders_response_deserialization_empty() {
        let json = r#"{"cancelledOrderIds": []}"#;
        let response: CancelAllOrdersResponse = serde_json::from_str(json).unwrap();
        assert!(response.cancelled_order_ids.is_empty());
    }

    #[test]
    fn test_cancel_all_orders_response_deserialization_large_list() {
        let order_ids: Vec<String> = (1..=100).map(|i| format!("order_{}", i)).collect();
        let order_ids_json: Vec<String> =
            order_ids.iter().map(|id| format!("\"{}\"", id)).collect();
        let json = format!(
            r#"{{"cancelledOrderIds": [{}]}}"#,
            order_ids_json.join(", ")
        );

        let response: CancelAllOrdersResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(response.cancelled_order_ids.len(), 100);
        assert_eq!(response.cancelled_order_ids[0], "order_1");
        assert_eq!(response.cancelled_order_ids[99], "order_100");
    }

    #[test]
    fn test_cancel_all_orders_endpoint() {
        assert_eq!(CANCEL_ALL_ORDERS_ENDPOINT, "/api/v1/orders");
    }

    #[test]
    fn test_cancel_all_orders_request_field_types() {
        let request = CancelAllOrdersRequest {
            symbol: Some("ETHUSDTM".to_string()),
        };

        // Test field types and constraints
        assert_eq!(request.symbol, Some("ETHUSDTM".to_string()));

        // Test that symbol can be None
        let minimal_request = CancelAllOrdersRequest { symbol: None };
        assert!(minimal_request.symbol.is_none());
    }

    #[test]
    fn test_cancel_all_orders_response_field_types() {
        let json = r#"{
            "cancelledOrderIds": ["5e8c8c2f1a3b4a001c5d8e31", "5e8c8c2f1a3b4a001c5d8e32"]
        }"#;

        let response: CancelAllOrdersResponse = serde_json::from_str(json).unwrap();

        // Test field types
        assert_eq!(response.cancelled_order_ids.len(), 2);
        assert!(response.cancelled_order_ids.iter().all(|id| id.len() > 0));

        // Verify order ID format (typical KuCoin order ID length)
        for order_id in &response.cancelled_order_ids {
            assert!(order_id.len() >= 20); // KuCoin order IDs are typically 24 characters
        }
    }

    #[test]
    fn test_cancel_all_orders_symbol_variations() {
        let symbols = vec!["XBTUSDTM", "ETHUSDTM", "ADAUSDTM", "DOTUSDTM", "LINKUSDTM"];

        for symbol in symbols {
            let request = CancelAllOrdersRequest {
                symbol: Some(symbol.to_string()),
            };

            let json = serde_json::to_string(&request).unwrap();
            assert!(json.contains(symbol));
        }
    }
}
