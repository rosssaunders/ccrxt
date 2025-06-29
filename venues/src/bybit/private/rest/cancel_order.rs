use serde::{Deserialize, Serialize};

use crate::bybit::{enums::*, EndpointType, RestResult};

use super::client::RestClient;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest {
    pub category: Category,
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_filter: Option<OrderFilter>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderData {
    pub order_id: String,
    pub order_link_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CancelOrderResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: CancelOrderData,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

impl RestClient {
    /// Cancel an existing order
    ///
    /// Cancel unfilled or partially filled orders.
    ///
    /// # Arguments
    /// * `request` - The order cancellation request parameters
    ///
    /// # Returns
    /// A result containing the order cancellation response or an error
    pub async fn cancel_order(
        &self,
        request: CancelOrderRequest,
    ) -> RestResult<CancelOrderResponse> {
        self.send_signed_request(
            "/v5/order/cancel",
            reqwest::Method::POST,
            request,
            EndpointType::Trade,
        )
        .await
    }
}

impl CancelOrderRequest {
    /// Create a new cancel order request by order ID
    pub fn by_order_id(category: Category, symbol: String, order_id: String) -> Self {
        Self {
            category,
            symbol,
            order_id: Some(order_id),
            order_link_id: None,
            order_filter: None,
        }
    }

    /// Create a new cancel order request by order link ID
    pub fn by_order_link_id(category: Category, symbol: String, order_link_id: String) -> Self {
        Self {
            category,
            symbol,
            order_id: None,
            order_link_id: Some(order_link_id),
            order_filter: None,
        }
    }

    /// Set order filter (for spot trading only)
    pub fn order_filter(mut self, order_filter: OrderFilter) -> Self {
        self.order_filter = Some(order_filter);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel_order_request_by_order_id() {
        let request = CancelOrderRequest::by_order_id(
            Category::Linear,
            "BTCUSDT".to_string(),
            "12345".to_string(),
        );

        assert_eq!(request.category, Category::Linear);
        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.order_id, Some("12345".to_string()));
        assert!(request.order_link_id.is_none());
        assert!(request.order_filter.is_none());
    }

    #[test]
    fn test_cancel_order_request_by_order_link_id() {
        let request = CancelOrderRequest::by_order_link_id(
            Category::Spot,
            "ETHUSDT".to_string(),
            "custom-456".to_string(),
        )
        .order_filter(OrderFilter::Order);

        assert_eq!(request.category, Category::Spot);
        assert_eq!(request.symbol, "ETHUSDT");
        assert!(request.order_id.is_none());
        assert_eq!(request.order_link_id, Some("custom-456".to_string()));
        assert_eq!(request.order_filter, Some(OrderFilter::Order));
    }

    #[test]
    fn test_cancel_order_request_serialization() {
        let request = CancelOrderRequest::by_order_id(
            Category::Linear,
            "BTCUSDT".to_string(),
            "order123".to_string(),
        );

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"category\":\"linear\""));
        assert!(json.contains("\"symbol\":\"BTCUSDT\""));
        assert!(json.contains("\"orderId\":\"order123\""));
        assert!(!json.contains("orderLinkId")); // Should be skipped when None
        assert!(!json.contains("orderFilter")); // Should be skipped when None
    }
}