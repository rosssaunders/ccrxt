use serde::{Deserialize, Serialize};

use super::super::RestClient;
use crate::bitget::spot::RestResult;

const CANCEL_SYMBOL_ORDER_ENDPOINT: &str = "/api/v2/spot/trade/cancel-symbol-order";
/// Request parameters for cancelling all orders for a symbol
#[derive(Debug, Clone, Serialize)]
pub struct CancelSymbolOrderRequest {
    /// Trading pair name, e.g. BTCUSDT
    pub symbol: String,

    /// Request timestamp (Unix milliseconds)
    #[serde(rename = "requestTime", skip_serializing_if = "Option::is_none")]
    pub request_time: Option<i64>,

    /// Valid time window (Unix milliseconds)
    /// If set, request is valid only when server time is within receiveWindow
    #[serde(rename = "receiveWindow", skip_serializing_if = "Option::is_none")]
    pub receive_window: Option<i64>,
}

/// Result of cancelling orders for a symbol
#[derive(Debug, Clone, Deserialize)]
pub struct CancelSymbolOrderResult {
    /// Order ID
    #[serde(rename = "orderId")]
    pub order_id: String,

    /// Client order ID (if provided)
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

/// Response from cancelling all orders for a symbol
#[derive(Debug, Clone, Deserialize)]
pub struct CancelSymbolOrderResponse {
    /// List of cancelled order results
    #[serde(rename = "orderInfo")]
    pub order_info: Vec<CancelSymbolOrderResult>,

    /// List of failed cancellations (if any)
    #[serde(rename = "failure")]
    pub failure: Option<Vec<CancelSymbolOrderResult>>,

    /// List of successful cancellations (if any)
    #[serde(rename = "success")]
    pub success: Option<Vec<CancelSymbolOrderResult>>,
}

impl RestClient {
    /// Cancel all spot trading orders for a specific symbol
    ///
    /// Cancels all existing orders for the specified trading symbol.
    ///
    /// # Arguments
    /// * `request` - The cancel symbol orders request parameters
    ///
    /// # Rate Limit
    /// 5 requests per second per UID
    ///
    /// # Returns
    /// A result containing the cancel symbol orders response or an error
    pub async fn cancel_symbol_order(
        &self,
        request: CancelSymbolOrderRequest,
    ) -> RestResult<CancelSymbolOrderResponse> {
        let body = serde_json::to_string(&request).map_err(|e| {
            crate::bitget::spot::Errors::Error(format!("Failed to serialize request: {e}"))
        })?;

        self.send_signed_request(
            CANCEL_SYMBOL_ORDER_ENDPOINT,
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
    fn test_cancel_symbol_order_request() {
        let request = CancelSymbolOrderRequest {
            symbol: "BTCUSDT".to_string(),
            request_time: None,
            receive_window: None,
        };

        assert_eq!(request.symbol, "BTCUSDT");
        assert!(request.request_time.is_none());
        assert!(request.receive_window.is_none());
    }

    #[test]
    fn test_cancel_symbol_order_request_builder() {
        let request = CancelSymbolOrderRequest {
            symbol: "ETHUSDT".to_string(),
            request_time: Some(1640995200000),
            receive_window: Some(5000),
        };

        assert_eq!(request.symbol, "ETHUSDT");
        assert_eq!(request.request_time, Some(1640995200000));
        assert_eq!(request.receive_window, Some(5000));
    }

    #[test]
    fn test_cancel_symbol_order_request_serialization() {
        let request = CancelSymbolOrderRequest {
            symbol: "BTCUSDT".to_string(),
            request_time: Some(1640995200000),
            receive_window: None,
        };

        let json = serde_json::to_string(&request).unwrap();

        assert!(json.contains("\"symbol\":\"BTCUSDT\""));
        assert!(json.contains("\"requestTime\":1640995200000"));
    }

    #[test]
    fn test_cancel_symbol_order_result_deserialization() {
        let json = r#"{
            "orderId": "1001",
            "clientOid": "my-order-123",
            "success": true,
            "errorCode": null,
            "errorMsg": null
        }"#;

        let result: CancelSymbolOrderResult = serde_json::from_str(json).unwrap();

        assert_eq!(result.order_id, "1001");
        assert_eq!(result.client_order_id, Some("my-order-123".to_string()));
        assert!(result.success);
        assert!(result.error_code.is_none());
        assert!(result.error_msg.is_none());
    }

    #[test]
    fn test_cancel_symbol_order_result_deserialization_failure() {
        let json = r#"{
            "orderId": "1002",
            "clientOid": null,
            "success": false,
            "errorCode": "43025",
            "errorMsg": "Order does not exist"
        }"#;

        let result: CancelSymbolOrderResult = serde_json::from_str(json).unwrap();

        assert_eq!(result.order_id, "1002");
        assert!(result.client_order_id.is_none());
        assert!(!result.success);
        assert_eq!(result.error_code, Some("43025".to_string()));
        assert_eq!(result.error_msg, Some("Order does not exist".to_string()));
    }

    #[test]
    fn test_cancel_symbol_order_response_deserialization() {
        let json = r#"{
            "orderInfo": [
                {
                    "orderId": "1001",
                    "clientOid": "order-1",
                    "success": true,
                    "errorCode": null,
                    "errorMsg": null
                },
                {
                    "orderId": "1002",
                    "clientOid": "order-2",
                    "success": true,
                    "errorCode": null,
                    "errorMsg": null
                }
            ]
        }"#;

        let response: CancelSymbolOrderResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.order_info.len(), 2);
        assert!(response.order_info[0].success);
        assert!(response.order_info[1].success);
    }
}
