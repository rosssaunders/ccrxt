//! Cancel orders endpoint for Coinbase Exchange REST API
//!
//! Cancel individual orders or all orders.

use serde::{Deserialize, Serialize};

use crate::coinbase::{EndpointType, RestResult};

use super::RestClient;

const ORDERS_ENDPOINT: &str = "orders";

/// Request to cancel all orders
#[derive(Debug, Clone, Serialize, Default)]
pub struct CancelAllOrdersRequest {
    /// Cancels orders on a specific profile
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<String>,

    /// Cancels orders on a specific product only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
}

/// Request to cancel a single order
#[derive(Debug, Clone, Serialize, Default)]
pub struct CancelOrderRequest {
    /// Cancels orders on a specific profile
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<String>,

    /// Optional product id of order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
}

/// Response from canceling all orders
pub type CancelAllOrdersResponse = Vec<String>;

/// Response from canceling a single order
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum CancelOrderResponse {
    /// Order ID when successfully canceled
    OrderId(String),
    /// Client order ID when successfully canceled with client_oid
    ClientOrderId(String),
}

impl RestClient {
    /// Cancel all orders
    ///
    /// With best effort, cancel all open orders. This may require you to make the
    /// request multiple times until all of the open orders are deleted.
    ///
    /// # Arguments
    /// * `request` - The cancel all orders request parameters
    ///
    /// # Returns
    /// A result containing the list of canceled order IDs or an error
    ///
    /// # API Key Permissions
    /// This endpoint requires the "trade" permission.
    pub async fn cancel_all_orders(
        &self,
        request: &CancelAllOrdersRequest,
    ) -> RestResult<CancelAllOrdersResponse> {
        self.send_request(
            ORDERS_ENDPOINT,
            reqwest::Method::DELETE,
            Some(request),
            EndpointType::Private,
        )
        .await
    }

    /// Cancel a single order
    ///
    /// Cancel a single open order by ID. Orders can be canceled using either the exchange assigned ID
    /// or the client assigned client_oid. When using client_oid it must be preceded by the "client:" namespace.
    ///
    /// # Arguments
    /// * `order_id` - The order ID (either exchange ID or "client:client_oid")
    /// * `request` - Optional request parameters
    ///
    /// # Returns
    /// A result containing the canceled order ID or an error
    ///
    /// # API Key Permissions
    /// This endpoint requires the "trade" permission.
    pub async fn cancel_order(
        &self,
        order_id: &str,
        request: &CancelOrderRequest,
    ) -> RestResult<CancelOrderResponse> {
        let endpoint = format!("orders/{}", order_id);
        self.send_request(
            &endpoint,
            reqwest::Method::DELETE,
            Some(request),
            EndpointType::Private,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel_all_orders_request_serialization() {
        let request = CancelAllOrdersRequest {
            profile_id: None,
            product_id: Some("BTC-USD".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("product_id=BTC-USD"));
        assert!(!serialized.contains("profile_id")); // Should be omitted when None
    }

    #[test]
    fn test_cancel_order_request_serialization() {
        let request = CancelOrderRequest {
            profile_id: Some("default".to_string()),
            product_id: Some("BTC-USD".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("profile_id=default"));
        assert!(serialized.contains("product_id=BTC-USD"));
    }

    #[test]
    fn test_cancel_all_orders_response_deserialization() {
        let json =
            r#"["d0c5340b-6d6c-49d9-b567-48c4bfca13d2", "e1d6451c-7e7d-5ae8-c678-59d5ceda24e3"]"#;

        let response: CancelAllOrdersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 2);
        assert_eq!(response[0], "d0c5340b-6d6c-49d9-b567-48c4bfca13d2");
        assert_eq!(response[1], "e1d6451c-7e7d-5ae8-c678-59d5ceda24e3");
    }

    #[test]
    fn test_cancel_order_response_deserialization() {
        // Test with order ID
        let json = r#""d0c5340b-6d6c-49d9-b567-48c4bfca13d2""#;
        let response: CancelOrderResponse = serde_json::from_str(json).unwrap();
        match response {
            CancelOrderResponse::OrderId(id) => {
                assert_eq!(id, "d0c5340b-6d6c-49d9-b567-48c4bfca13d2");
            }
            CancelOrderResponse::ClientOrderId(_) => panic!("Expected OrderId"),
        }

        // Test with client order ID
        let json = r#""client:my-order-123""#;
        let response: CancelOrderResponse = serde_json::from_str(json).unwrap();
        match response {
            CancelOrderResponse::OrderId(id) => {
                assert_eq!(id, "client:my-order-123");
            }
            CancelOrderResponse::ClientOrderId(_) => panic!("Expected OrderId for client order ID"),
        }
    }
}
