use serde::{Deserialize, Serialize};

use crate::bybit::{enums::*, EndpointType, RestResult};

use super::client::RestClient;
use super::amend_order::AmendOrderRequest;

#[derive(Debug, Clone, Serialize)]
pub struct BatchAmendOrdersRequest {
    pub category: Category,
    pub request: Vec<AmendOrderRequest>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchAmendOrderResult {
    pub order_id: String,
    pub order_link_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchAmendOrderError {
    pub code: i32,
    pub msg: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BatchAmendOrdersData {
    pub result: BatchAmendOrdersResult,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: BatchAmendOrdersExtInfo,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BatchAmendOrdersResult {
    pub list: Vec<BatchAmendOrderResult>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BatchAmendOrdersExtInfo {
    pub list: Vec<BatchAmendOrderError>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BatchAmendOrdersResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: BatchAmendOrdersData,
    pub time: u64,
}

impl RestClient {
    /// Batch amend orders
    ///
    /// Amend multiple orders in a single request. Maximum 20 orders per batch.
    ///
    /// # Arguments
    /// * `request` - The batch amend orders request parameters
    ///
    /// # Returns
    /// A result containing the batch amend orders response or an error
    pub async fn batch_amend_orders(
        &self,
        request: BatchAmendOrdersRequest,
    ) -> RestResult<BatchAmendOrdersResponse> {
        self.send_signed_request(
            "/v5/order/amend-batch",
            reqwest::Method::POST,
            request,
            EndpointType::Trade,
        )
        .await
    }
}

impl BatchAmendOrdersRequest {
    /// Create a new batch amend orders request
    pub fn new(category: Category, orders: Vec<AmendOrderRequest>) -> Self {
        Self {
            category,
            request: orders,
        }
    }

    /// Add an order amendment to the batch
    pub fn add_order(mut self, order: AmendOrderRequest) -> Self {
        self.request.push(order);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_amend_orders_request() {
        let amend1 = AmendOrderRequest::by_order_id(
            Category::Linear,
            "BTCUSDT".to_string(),
            "order1".to_string(),
        )
        .price("51000".to_string());

        let amend2 = AmendOrderRequest::by_order_link_id(
            Category::Linear,
            "ETHUSDT".to_string(),
            "custom-order-2".to_string(),
        )
        .qty("0.2".to_string());

        let request = BatchAmendOrdersRequest::new(Category::Linear, vec![amend1, amend2]);

        assert_eq!(request.category, Category::Linear);
        assert_eq!(request.request.len(), 2);
        assert_eq!(request.request[0].symbol, "BTCUSDT");
        assert_eq!(request.request[1].symbol, "ETHUSDT");
    }

    #[test]
    fn test_batch_amend_orders_request_builder() {
        let amend1 = AmendOrderRequest::by_order_id(
            Category::Spot,
            "BTCUSDT".to_string(),
            "order1".to_string(),
        );

        let amend2 = AmendOrderRequest::by_order_id(
            Category::Spot,
            "ETHUSDT".to_string(),
            "order2".to_string(),
        );

        let request = BatchAmendOrdersRequest::new(Category::Spot, vec![amend1])
            .add_order(amend2);

        assert_eq!(request.category, Category::Spot);
        assert_eq!(request.request.len(), 2);
    }

    #[test]
    fn test_batch_amend_orders_request_serialization() {
        let amend = AmendOrderRequest::by_order_id(
            Category::Linear,
            "BTCUSDT".to_string(),
            "order123".to_string(),
        );

        let request = BatchAmendOrdersRequest::new(Category::Linear, vec![amend]);

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"category\":\"linear\""));
        assert!(json.contains("\"request\":["));
        assert!(json.contains("\"symbol\":\"BTCUSDT\""));
    }
}