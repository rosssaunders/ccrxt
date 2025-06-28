//! Cancel Order endpoint for Bitget Spot API
//!
//! This endpoint allows cancelling a specific order.
//!
//! Reference: https://www.bitget.com/api-doc/spot/trade/Cancel-Order
//! Endpoint: POST /api/v2/spot/trade/cancel-order
//! Rate limit: 10 times/1s (UID)

use serde::{Deserialize, Serialize};

use super::super::RestClient;
use crate::bitget::RestResult;

/// Request parameters for cancelling an order
#[derive(Debug, Clone, Serialize)]
pub struct CancelOrderRequest {
    /// Trading pair name, e.g. BTCUSDT
    pub symbol: String,

    /// Order ID (either orderId or clientOrderId is required)
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    /// Client order ID (either orderId or clientOrderId is required)
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
}

impl CancelOrderRequest {
    /// Create a request to cancel an order by order ID
    pub fn by_order_id(symbol: impl Into<String>, order_id: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            order_id: Some(order_id.into()),
            client_order_id: None,
        }
    }

    /// Create a request to cancel an order by client order ID
    pub fn by_client_order_id(
        symbol: impl Into<String>,
        client_order_id: impl Into<String>,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            order_id: None,
            client_order_id: Some(client_order_id.into()),
        }
    }
}

/// Response from cancelling an order
#[derive(Debug, Clone, Deserialize)]
pub struct CancelOrderResponse {
    /// Order ID
    #[serde(rename = "orderId")]
    pub order_id: String,

    /// Client order ID (if provided)
    #[serde(rename = "clientOid")]
    pub client_order_id: Option<String>,
}

impl RestClient {
    /// Cancel a specific spot trading order
    ///
    /// Cancels an existing order by order ID or client order ID.
    ///
    /// # Arguments
    /// * `request` - The cancel order request parameters
    ///
    /// # Rate Limit
    /// 10 requests per second per UID
    ///
    /// # Returns
    /// A result containing the cancel order response or an error
    pub async fn cancel_order(
        &self,
        request: CancelOrderRequest,
    ) -> RestResult<CancelOrderResponse> {
        let body = serde_json::to_string(&request).map_err(|e| {
            crate::bitget::Errors::Error(format!("Failed to serialize request: {e}"))
        })?;

        self.send_signed_request(
            "/api/v2/spot/trade/cancel-order",
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
    fn test_cancel_order_request_by_order_id() {
        let request = CancelOrderRequest::by_order_id("BTCUSDT", "1234567890");

        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.order_id, Some("1234567890".to_string()));
        assert!(request.client_order_id.is_none());
    }

    #[test]
    fn test_cancel_order_request_by_client_order_id() {
        let request = CancelOrderRequest::by_client_order_id("ETHUSDT", "my-order-123");

        assert_eq!(request.symbol, "ETHUSDT");
        assert!(request.order_id.is_none());
        assert_eq!(request.client_order_id, Some("my-order-123".to_string()));
    }

    #[test]
    fn test_cancel_order_request_serialization() {
        let request = CancelOrderRequest::by_order_id("BTCUSDT", "1234567890");
        let json = serde_json::to_string(&request).unwrap();

        assert!(json.contains("\"symbol\":\"BTCUSDT\""));
        assert!(json.contains("\"orderId\":\"1234567890\""));
        assert!(!json.contains("clientOrderId"));
    }

    #[test]
    fn test_cancel_order_response_deserialization() {
        let json = r#"{
            "orderId": "121211212122",
            "clientOid": "xx001"
        }"#;

        let response: CancelOrderResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.order_id, "121211212122");
        assert_eq!(response.client_order_id, Some("xx001".to_string()));
    }

    #[test]
    fn test_cancel_order_response_deserialization_no_client_id() {
        let json = r#"{
            "orderId": "121211212122",
            "clientOid": null
        }"#;

        let response: CancelOrderResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.order_id, "121211212122");
        assert_eq!(response.client_order_id, None);
    }
}
