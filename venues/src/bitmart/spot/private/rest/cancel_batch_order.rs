//! BitMart cancel batch order REST API endpoint
//!
//! This module implements the BitMart batch order cancellation API endpoint.

use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bitmart::rate_limit::EndpointType;
use crate::bitmart::{OrderSide, RestResult};

/// Request parameters for canceling batch orders
#[derive(Debug, Serialize)]
pub struct CancelBatchOrderRequest {
    /// Trading pair (e.g. BTC_USDT)
    pub symbol: String,
    /// Order ID list (max 10 IDs) - mutually exclusive with client_order_ids
    #[serde(rename = "orderIds", skip_serializing_if = "Option::is_none")]
    pub order_ids: Option<Vec<String>>,
    /// Client order ID list (max 10 IDs) - mutually exclusive with order_ids
    #[serde(rename = "clientOrderIds", skip_serializing_if = "Option::is_none")]
    pub client_order_ids: Option<Vec<String>>,
    /// Trade time limit in milliseconds, allowed range (0,60000], default: 5000
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Response for canceling batch orders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelBatchOrderResponse {
    /// Successfully canceled order IDs
    #[serde(rename = "successIds")]
    pub success_ids: Vec<String>,
    /// Order IDs that failed to cancel
    #[serde(rename = "failIds")]
    pub fail_ids: Vec<String>,
    /// Total number of submissions
    #[serde(rename = "totalCount")]
    pub total_count: i32,
    /// Number of successful cancellations
    #[serde(rename = "successCount")]
    pub success_count: i32,
    /// Number of failed cancellations
    #[serde(rename = "failedCount")]
    pub failed_count: i32,
}

/// Request parameters for canceling all orders
#[derive(Debug, Serialize)]
pub struct CancelAllOrdersRequest {
    /// Trading pair (optional, e.g. BTC_USDT)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Order side (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<OrderSide>,
}

/// Response for canceling all orders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelAllOrdersResponse {
    // Empty response data - success is indicated by HTTP status code
}

impl RestClient {
    /// Cancel batch orders (v4)
    ///
    /// Cancels multiple orders by order IDs or client order IDs. Maximum 10 orders per batch.
    /// Must specify either order_ids or client_order_ids, but not both.
    ///
    /// See: https://raw.githubusercontent.com/rosssaunders/coincise/refs/heads/main/docs/bitmart/spot/spot___margin_trading.md
    ///
    /// Rate limit: varies by endpoint type
    ///
    /// # Arguments
    /// * `request` - The batch cancel request parameters
    ///
    /// # Returns
    /// Batch cancel response with success/failure details
    pub async fn cancel_batch_order(
        &self,
        request: CancelBatchOrderRequest,
    ) -> RestResult<CancelBatchOrderResponse> {
        // Validate that exactly one of order_ids or client_order_ids is provided
        match (&request.order_ids, &request.client_order_ids) {
            (Some(order_ids), None) => {
                if order_ids.is_empty() || order_ids.len() > 10 {
                    return Err(crate::bitmart::Errors::Error(
                        "order_ids must contain 1-10 order IDs".to_string(),
                    ));
                }
            }
            (None, Some(client_order_ids)) => {
                if client_order_ids.is_empty() || client_order_ids.len() > 10 {
                    return Err(crate::bitmart::Errors::Error(
                        "client_order_ids must contain 1-10 client order IDs".to_string(),
                    ));
                }
            }
            (Some(_), Some(_)) => {
                return Err(crate::bitmart::Errors::Error(
                    "Cannot specify both order_ids and client_order_ids".to_string(),
                ));
            }
            (None, None) => {
                return Err(crate::bitmart::Errors::Error(
                    "Must specify either order_ids or client_order_ids".to_string(),
                ));
            }
        }

        self.send_request(
            "/spot/v4/cancel_orders",
            reqwest::Method::POST,
            Some(&request),
            EndpointType::SpotTrading,
        )
        .await
    }

    /// Cancel all orders (v4)
    ///
    /// Cancels all outstanding orders for a symbol and/or side.
    ///
    /// See: https://raw.githubusercontent.com/rosssaunders/coincise/refs/heads/main/docs/bitmart/spot/spot___margin_trading.md
    ///
    /// Rate limit: varies by endpoint type
    ///
    /// # Arguments
    /// * `request` - The cancel all request parameters
    ///
    /// # Returns
    /// Empty response - success indicated by HTTP status
    pub async fn cancel_all_orders(
        &self,
        request: CancelAllOrdersRequest,
    ) -> RestResult<CancelAllOrdersResponse> {
        self.send_request(
            "/spot/v4/cancel_all",
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
    fn test_cancel_batch_order_request_with_order_ids() {
        let request = CancelBatchOrderRequest {
            symbol: "BTC_USDT".to_string(),
            order_ids: Some(vec!["12345".to_string(), "67890".to_string()]),
            client_order_ids: None,
            recv_window: Some(5000),
        };

        assert_eq!(request.symbol, "BTC_USDT");
        assert!(request.order_ids.is_some());
        assert!(request.client_order_ids.is_none());
        assert_eq!(request.recv_window, Some(5000));
    }

    #[test]
    fn test_cancel_batch_order_request_with_client_order_ids() {
        let request = CancelBatchOrderRequest {
            symbol: "ETH_USDT".to_string(),
            order_ids: None,
            client_order_ids: Some(vec!["client_123".to_string()]),
            recv_window: None,
        };

        assert_eq!(request.symbol, "ETH_USDT");
        assert!(request.order_ids.is_none());
        assert!(request.client_order_ids.is_some());
        assert!(request.recv_window.is_none());
    }

    #[test]
    fn test_cancel_all_orders_request() {
        let request = CancelAllOrdersRequest {
            symbol: Some("BTC_USDT".to_string()),
            side: Some(OrderSide::Buy),
        };

        assert_eq!(request.symbol, Some("BTC_USDT".to_string()));
        assert_eq!(request.side, Some(OrderSide::Buy));
    }

    #[test]
    fn test_cancel_all_orders_request_empty() {
        let request = CancelAllOrdersRequest {
            symbol: None,
            side: None,
        };

        assert!(request.symbol.is_none());
        assert!(request.side.is_none());
    }
}
