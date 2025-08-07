use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

/// Endpoint URL for Cancel All Stop Orders
const CANCEL_ALL_STOP_ORDERS_ENDPOINT: &str = "/api/v1/stopOrders";

/// Request parameters for cancelling all stop orders.
#[derive(Debug, Clone, Default, Serialize)]
pub struct CancelAllStopOrdersRequest {
    /// Optional symbol to cancel stop orders for a specific trading pair only.
    /// If not provided, cancels all stop orders across all symbols.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

/// Response data from the cancel all stop orders endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllStopOrdersResponse {
    /// List of cancelled stop order IDs.
    pub cancelled_order_ids: Vec<String>,
}

impl super::RestClient {
    /// Cancel All Stop Orders
    ///
    /// Cancel all untriggered stop orders for the account. Optionally filter by symbol
    /// to cancel stop orders for a specific trading pair only.
    ///
    /// [docs]: https://www.kucoin.com/docs-new/rest/futures-trading/orders/cancel-all-stop-orders
    ///
    /// Rate limit: 40
    ///
    /// # Arguments
    /// * `request` - The cancel all stop orders request parameters
    ///
    /// # Returns
    /// List of cancelled stop order IDs
    pub async fn cancel_all_stop_orders(
        &self,
        request: CancelAllStopOrdersRequest,
    ) -> Result<(CancelAllStopOrdersResponse, ResponseHeaders)> {
        let (response, headers): (RestResponse<CancelAllStopOrdersResponse>, ResponseHeaders) =
            self.delete_with_request(CANCEL_ALL_STOP_ORDERS_ENDPOINT, &request).await?;

        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel_all_stop_orders_request_serialization_with_symbol() {
        let request = CancelAllStopOrdersRequest {
            symbol: Some("XBTUSDTM".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("XBTUSDTM"));
        assert!(!json.contains("null"));
    }

    #[test]
    fn test_cancel_all_stop_orders_request_serialization_without_symbol() {
        let request = CancelAllStopOrdersRequest { symbol: None };

        let json = serde_json::to_string(&request).unwrap();
        assert_eq!(json, "{}");
        assert!(!json.contains("symbol"));
    }

    #[test]
    fn test_cancel_all_stop_orders_request_serialization_minimal() {
        let request = CancelAllStopOrdersRequest::default();

        let json = serde_json::to_string(&request).unwrap();
        assert_eq!(json, "{}");
        assert!(!json.contains("symbol"));
    }

    #[test]
    fn test_cancel_all_stop_orders_request_default() {
        let request = CancelAllStopOrdersRequest::default();
        assert!(request.symbol.is_none());
    }

    #[test]
    fn test_cancel_all_stop_orders_response_deserialization() {
        let json = r#"{
            "cancelledOrderIds": ["order1", "order2", "order3"]
        }"#;

        let response: CancelAllStopOrdersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.cancelled_order_ids.len(), 3);
        assert_eq!(response.cancelled_order_ids[0], "order1");
        assert_eq!(response.cancelled_order_ids[1], "order2");
        assert_eq!(response.cancelled_order_ids[2], "order3");
    }

    #[test]
    fn test_cancel_all_stop_orders_response_deserialization_empty() {
        let json = r#"{"cancelledOrderIds": []}"#;
        let response: CancelAllStopOrdersResponse = serde_json::from_str(json).unwrap();
        assert!(response.cancelled_order_ids.is_empty());
    }

    #[test]
    fn test_cancel_all_stop_orders_response_deserialization_large_list() {
        let order_ids: Vec<String> = (1..=50).map(|i| format!("stop_order_{}", i)).collect();
        let order_ids_json: Vec<String> = order_ids.iter().map(|id| format!("\"{}\"", id)).collect();
        let json = format!(r#"{{"cancelledOrderIds": [{}]}}"#, order_ids_json.join(", "));

        let response: CancelAllStopOrdersResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(response.cancelled_order_ids.len(), 50);
        assert_eq!(response.cancelled_order_ids[0], "stop_order_1");
        assert_eq!(response.cancelled_order_ids[49], "stop_order_50");
    }

    #[test]
    fn test_cancel_all_stop_orders_endpoint() {
        assert_eq!(CANCEL_ALL_STOP_ORDERS_ENDPOINT, "/api/v1/stopOrders");
    }

    #[test]
    fn test_cancel_all_stop_orders_request_field_types() {
        let request = CancelAllStopOrdersRequest {
            symbol: Some("ETHUSDTM".to_string()),
        };

        // Test field types and constraints
        assert_eq!(request.symbol, Some("ETHUSDTM".to_string()));
        
        // Test that symbol can be None
        let minimal_request = CancelAllStopOrdersRequest { symbol: None };
        assert!(minimal_request.symbol.is_none());
    }

    #[test]
    fn test_cancel_all_stop_orders_response_field_types() {
        let json = r#"{
            "cancelledOrderIds": ["5e8c8c2f1a3b4a001c5d8e31", "5e8c8c2f1a3b4a001c5d8e32"]
        }"#;

        let response: CancelAllStopOrdersResponse = serde_json::from_str(json).unwrap();
        
        // Test field types
        assert_eq!(response.cancelled_order_ids.len(), 2);
        assert!(response.cancelled_order_ids.iter().all(|id| id.len() > 0));
        
        // Verify order ID format (typical KuCoin order ID length)
        for order_id in &response.cancelled_order_ids {
            assert!(order_id.len() >= 20); // KuCoin order IDs are typically 24 characters
        }
    }

    #[test]
    fn test_cancel_all_stop_orders_symbol_variations() {
        let symbols = vec![
            "XBTUSDTM",
            "ETHUSDTM", 
            "ADAUSDTM",
            "DOTUSDTM",
            "LINKUSDTM"
        ];

        for symbol in symbols {
            let request = CancelAllStopOrdersRequest {
                symbol: Some(symbol.to_string()),
            };

            let json = serde_json::to_string(&request).unwrap();
            assert!(json.contains(symbol));
        }
    }

    #[test]
    fn test_cancel_all_stop_orders_stop_order_types() {
        // Test response handling for different stop order types
        let json = r#"{
            "cancelledOrderIds": [
                "tp_order_001",
                "sl_order_002", 
                "stop_order_003",
                "conditional_order_004"
            ]
        }"#;

        let response: CancelAllStopOrdersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.cancelled_order_ids.len(), 4);
        
        // Verify all different stop order ID patterns are handled
        assert!(response.cancelled_order_ids.contains(&"tp_order_001".to_string()));
        assert!(response.cancelled_order_ids.contains(&"sl_order_002".to_string()));
        assert!(response.cancelled_order_ids.contains(&"stop_order_003".to_string()));
        assert!(response.cancelled_order_ids.contains(&"conditional_order_004".to_string()));
    }
}
