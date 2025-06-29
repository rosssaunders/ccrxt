use serde::{Deserialize, Serialize};

use crate::bybit::{enums::*, EndpointType, RestResult};

use super::client::RestClient;
use super::create_order::CreateOrderRequest;

#[derive(Debug, Clone, Serialize)]
pub struct BatchCreateOrdersRequest {
    pub category: Category,
    pub request: Vec<CreateOrderRequest>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchCreateOrderResult {
    pub order_id: String,
    pub order_link_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchCreateOrderError {
    pub code: i32,
    pub msg: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BatchCreateOrdersData {
    pub result: BatchCreateOrdersResult,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: BatchCreateOrdersExtInfo,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BatchCreateOrdersResult {
    pub list: Vec<BatchCreateOrderResult>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BatchCreateOrdersExtInfo {
    pub list: Vec<BatchCreateOrderError>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BatchCreateOrdersResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: BatchCreateOrdersData,
    pub time: u64,
}

impl RestClient {
    /// Batch create orders
    ///
    /// Place multiple orders in a single request. Maximum 20 orders per batch.
    ///
    /// # Arguments
    /// * `request` - The batch create orders request parameters
    ///
    /// # Returns
    /// A result containing the batch create orders response or an error
    pub async fn batch_create_orders(
        &self,
        request: BatchCreateOrdersRequest,
    ) -> RestResult<BatchCreateOrdersResponse> {
        self.send_signed_request(
            "/v5/order/create-batch",
            reqwest::Method::POST,
            request,
            EndpointType::Trade,
        )
        .await
    }
}

impl BatchCreateOrdersRequest {
    /// Create a new batch create orders request
    pub fn new(category: Category, orders: Vec<CreateOrderRequest>) -> Self {
        Self {
            category,
            request: orders,
        }
    }

    /// Add an order to the batch
    pub fn add_order(mut self, order: CreateOrderRequest) -> Self {
        self.request.push(order);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bybit::enums::{OrderType, Side};

    #[test]
    fn test_batch_create_orders_request() {
        let order1 = CreateOrderRequest::new(
            Category::Linear,
            "BTCUSDT".to_string(),
            Side::Buy,
            OrderType::Limit,
            "0.001".to_string(),
        )
        .price("50000".to_string());

        let order2 = CreateOrderRequest::new(
            Category::Linear,
            "ETHUSDT".to_string(),
            Side::Sell,
            OrderType::Market,
            "0.1".to_string(),
        );

        let request = BatchCreateOrdersRequest::new(Category::Linear, vec![order1, order2]);

        assert_eq!(request.category, Category::Linear);
        assert_eq!(request.request.len(), 2);
        assert_eq!(request.request[0].symbol, "BTCUSDT");
        assert_eq!(request.request[1].symbol, "ETHUSDT");
    }

    #[test]
    fn test_batch_create_orders_request_builder() {
        let order1 = CreateOrderRequest::new(
            Category::Spot,
            "BTCUSDT".to_string(),
            Side::Buy,
            OrderType::Limit,
            "0.001".to_string(),
        );

        let order2 = CreateOrderRequest::new(
            Category::Spot,
            "ETHUSDT".to_string(),
            Side::Sell,
            OrderType::Market,
            "0.1".to_string(),
        );

        let request = BatchCreateOrdersRequest::new(Category::Spot, vec![order1])
            .add_order(order2);

        assert_eq!(request.category, Category::Spot);
        assert_eq!(request.request.len(), 2);
    }

    #[test]
    fn test_batch_create_orders_request_serialization() {
        let order = CreateOrderRequest::new(
            Category::Linear,
            "BTCUSDT".to_string(),
            Side::Buy,
            OrderType::Limit,
            "0.001".to_string(),
        );

        let request = BatchCreateOrdersRequest::new(Category::Linear, vec![order]);

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"category\":\"linear\""));
        assert!(json.contains("\"request\":["));
        assert!(json.contains("\"symbol\":\"BTCUSDT\""));
    }
}