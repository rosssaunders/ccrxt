use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bybit::{EndpointType, RestResult, enums::*};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel_order_request_by_order_id() {
        let request = CancelOrderRequest {
            category: Category::Linear,
            symbol: "BTCUSDT".to_string(),
            order_id: Some("12345".to_string()),
            order_link_id: None,
            order_filter: None,
        };

        assert_eq!(request.category, Category::Linear);
        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.order_id, Some("12345".to_string()));
        assert!(request.order_link_id.is_none());
        assert!(request.order_filter.is_none());
    }

    #[test]
    fn test_cancel_order_request_by_order_link_id() {
        let request = CancelOrderRequest {
            category: Category::Spot,
            symbol: "ETHUSDT".to_string(),
            order_id: None,
            order_link_id: Some("custom-456".to_string()),
            order_filter: Some(OrderFilter::Order),
        };

        assert_eq!(request.category, Category::Spot);
        assert_eq!(request.symbol, "ETHUSDT");
        assert!(request.order_id.is_none());
        assert_eq!(request.order_link_id, Some("custom-456".to_string()));
        assert_eq!(request.order_filter, Some(OrderFilter::Order));
    }

    #[test]
    fn test_cancel_order_request_serialization() {
        let request = CancelOrderRequest {
            category: Category::Linear,
            symbol: "BTCUSDT".to_string(),
            order_id: Some("order123".to_string()),
            order_link_id: None,
            order_filter: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"category\":\"linear\""));
        assert!(json.contains("\"symbol\":\"BTCUSDT\""));
        assert!(json.contains("\"orderId\":\"order123\""));
        assert!(!json.contains("orderLinkId")); // Should be skipped when None
        assert!(!json.contains("orderFilter")); // Should be skipped when None
    }
}
