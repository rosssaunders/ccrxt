//! BitMart cancel order REST API endpoint
//!
//! This module implements the BitMart cancel order API endpoint for canceling orders.

use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bitmart::RestResult;
use crate::bitmart::rate_limit::EndpointType;

/// Request parameters for canceling an order
#[derive(Debug, Serialize)]
pub struct CancelOrderRequest {
    /// Trading pair (e.g. BTC_USDT)
    pub symbol: String,
    /// Order ID (required if client_order_id not provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// Client-defined Order ID (required if order_id not provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
}

/// Response for canceling an order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelOrderResponse {
    /// Whether the cancellation was successful
    pub result: bool,
}

impl RestClient {
    /// Cancel an order (v3)
    ///
    /// Cancels a specified unfinished order.
    ///
    /// See: https://raw.githubusercontent.com/rosssaunders/coincise/refs/heads/main/docs/bitmart/spot/spot___margin_trading.md
    ///
    /// Rate limit: varies by endpoint type
    ///
    /// # Arguments
    /// * `request` - The cancel order request parameters
    ///
    /// # Returns
    /// Cancel order response with result status
    pub async fn cancel_order(
        &self,
        request: CancelOrderRequest,
    ) -> RestResult<CancelOrderResponse> {
        self.send_request(
            "/spot/v3/cancel_order",
            reqwest::Method::POST,
            Some(&request),
            EndpointType::SpotTrading,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel_order_request_by_order_id() {
        let request = CancelOrderRequest {
            symbol: "BTC_USDT".to_string(),
            order_id: Some("12345".to_string()),
            client_order_id: None,
        };

        assert_eq!(request.symbol, "BTC_USDT");
        assert_eq!(request.order_id, Some("12345".to_string()));
        assert!(request.client_order_id.is_none());
    }

    #[test]
    fn test_cancel_order_request_by_client_order_id() {
        let request = CancelOrderRequest {
            symbol: "ETH_USDT".to_string(),
            order_id: None,
            client_order_id: Some("my_order_123".to_string()),
        };

        assert_eq!(request.symbol, "ETH_USDT");
        assert!(request.order_id.is_none());
        assert_eq!(request.client_order_id, Some("my_order_123".to_string()));
    }

    #[test]
    fn test_cancel_order_response_structure() {
        let json = r#"{"result": true}"#;
        let response: CancelOrderResponse = serde_json::from_str(json).unwrap();
        assert!(response.result);
    }

    #[test]
    fn test_cancel_order_response_serialization() {
        let response = CancelOrderResponse { result: false };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"result\":false"));
    }
}
