use serde::{Deserialize, Serialize};

use crate::bybit::{enums::*, EndpointType, RestResult};

use super::client::RestClient;
use super::cancel_order::CancelOrderRequest;

#[derive(Debug, Clone, Serialize)]
pub struct BatchCancelOrdersRequest {
    pub category: Category,
    pub request: Vec<CancelOrderRequest>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchCancelOrderResult {
    pub order_id: String,
    pub order_link_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchCancelOrderError {
    pub code: i32,
    pub msg: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BatchCancelOrdersData {
    pub result: BatchCancelOrdersResult,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: BatchCancelOrdersExtInfo,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BatchCancelOrdersResult {
    pub list: Vec<BatchCancelOrderResult>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BatchCancelOrdersExtInfo {
    pub list: Vec<BatchCancelOrderError>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BatchCancelOrdersResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: BatchCancelOrdersData,
    pub time: u64,
}

impl RestClient {
    /// Batch cancel orders
    ///
    /// Cancel multiple orders in a single request. Maximum 20 orders per batch.
    ///
    /// # Arguments
    /// * `request` - The batch cancel orders request parameters
    ///
    /// # Returns
    /// A result containing the batch cancel orders response or an error
    pub async fn batch_cancel_orders(
        &self,
        request: BatchCancelOrdersRequest,
    ) -> RestResult<BatchCancelOrdersResponse> {
        self.send_signed_request(
            "/v5/order/cancel-batch",
            reqwest::Method::POST,
            request,
            EndpointType::Trade,
        )
        .await
    }
}

impl BatchCancelOrdersRequest {
    /// Create a new batch cancel orders request
    pub fn new(category: Category, orders: Vec<CancelOrderRequest>) -> Self {
        Self {
            category,
            request: orders,
        }
    }

    /// Add an order cancellation to the batch
    pub fn add_order(mut self, order: CancelOrderRequest) -> Self {
        self.request.push(order);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_cancel_orders_request() {
        let cancel1 = CancelOrderRequest::by_order_id(
            Category::Linear,
            "BTCUSDT".to_string(),
            "order1".to_string(),
        );

        let cancel2 = CancelOrderRequest::by_order_link_id(
            Category::Linear,
            "ETHUSDT".to_string(),
            "custom-order-2".to_string(),
        );

        let request = BatchCancelOrdersRequest::new(Category::Linear, vec![cancel1, cancel2]);

        assert_eq!(request.category, Category::Linear);
        assert_eq!(request.request.len(), 2);
        assert_eq!(request.request[0].symbol, "BTCUSDT");
        assert_eq!(request.request[1].symbol, "ETHUSDT");
    }

    #[test]
    fn test_batch_cancel_orders_request_builder() {
        let cancel1 = CancelOrderRequest::by_order_id(
            Category::Spot,
            "BTCUSDT".to_string(),
            "order1".to_string(),
        );

        let cancel2 = CancelOrderRequest::by_order_id(
            Category::Spot,
            "ETHUSDT".to_string(),
            "order2".to_string(),
        );

        let request = BatchCancelOrdersRequest::new(Category::Spot, vec![cancel1])
            .add_order(cancel2);

        assert_eq!(request.category, Category::Spot);
        assert_eq!(request.request.len(), 2);
    }

    #[test]
    fn test_batch_cancel_orders_request_serialization() {
        let cancel = CancelOrderRequest::by_order_id(
            Category::Linear,
            "BTCUSDT".to_string(),
            "order123".to_string(),
        );

        let request = BatchCancelOrdersRequest::new(Category::Linear, vec![cancel]);

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"category\":\"linear\""));
        assert!(json.contains("\"request\":["));
        assert!(json.contains("\"symbol\":\"BTCUSDT\""));
    }
}