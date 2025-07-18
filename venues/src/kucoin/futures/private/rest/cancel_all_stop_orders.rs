use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

/// Cancel all stop orders request
#[derive(Debug, Clone, Default, Serialize)]
pub struct CancelAllStopOrdersRequest {
    /// Optional symbol to cancel stop orders for specific contract
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

/// Cancel all stop orders response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllStopOrdersResponse {
    pub cancelled_order_ids: Vec<String>,
}

impl super::RestClient {
    /// Cancel all untriggered stop orders
    ///
    /// <https://www.kucoin.com/docs-new/rest/futures-trading/orders/cancel-all-stop-orders>
    pub async fn cancel_all_stop_orders(
        &self,
        request: CancelAllStopOrdersRequest,
    ) -> Result<(RestResponse<CancelAllStopOrdersResponse>, ResponseHeaders)> {
        const CANCEL_ALL_STOP_ORDERS_ENDPOINT: &str = "/api/v1/stopOrders";
        
        let params = if let Some(symbol) = request.symbol {
            let mut params = std::collections::HashMap::new();
            params.insert("symbol".to_string(), symbol);
            Some(params)
        } else {
            None
        };
        
        self.delete(CANCEL_ALL_STOP_ORDERS_ENDPOINT, params).await
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
        let expected = r#"{"symbol":"XBTUSDTM"}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_cancel_all_stop_orders_request_serialization_without_symbol() {
        let request = CancelAllStopOrdersRequest {
            symbol: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        let expected = r#"{}"#;
        assert_eq!(json, expected);
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
}