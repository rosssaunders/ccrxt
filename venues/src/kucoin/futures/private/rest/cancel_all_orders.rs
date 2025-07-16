use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

/// Endpoint URL for cancel all orders
pub const CANCEL_ALL_ORDERS_ENDPOINT: &str = "/api/v1/orders";

#[derive(Debug, Clone, Default, Serialize)]
pub struct CancelAllOrdersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrdersResponse {
    pub cancelled_order_ids: Vec<String>,
}

impl super::RestClient {
    /// Cancel all orders
    pub async fn cancel_all_orders(
        &self,
        request: CancelAllOrdersRequest,
    ) -> Result<(RestResponse<CancelAllOrdersResponse>, ResponseHeaders)> {
        let endpoint = CANCEL_ALL_ORDERS_ENDPOINT;
        let mut params = HashMap::new();
        if let Some(symbol) = request.symbol {
            params.insert("symbol".to_string(), symbol);
        }
        let params = if params.is_empty() {
            None
        } else {
            Some(params)
        };
        self.delete(endpoint, params).await
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
        let expected = r#"{"symbol":"XBTUSDTM"}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_cancel_all_orders_request_serialization_without_symbol() {
        let request = CancelAllOrdersRequest {
            symbol: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        let expected = r#"{}"#;
        assert_eq!(json, expected);
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
    fn test_cancel_all_orders_endpoint() {
        assert_eq!(CANCEL_ALL_ORDERS_ENDPOINT, "/api/v1/orders");
    }
}
