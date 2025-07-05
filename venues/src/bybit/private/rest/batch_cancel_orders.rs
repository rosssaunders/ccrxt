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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_cancel_orders_request() {
        let cancel1 = CancelOrderRequest {
            category: Category::Linear,
            symbol: "BTCUSDT".to_string(),
            order_id: Some("order1".to_string()),
            order_link_id: None,
            order_filter: None,
        };

        let cancel2 = CancelOrderRequest {
            category: Category::Linear,
            symbol: "ETHUSDT".to_string(),
            order_id: None,
            order_link_id: Some("custom-order-2".to_string()),
            order_filter: None,
        };

        let request = BatchCancelOrdersRequest {
            category: Category::Linear,
            request: vec![cancel1, cancel2],
        };

        assert_eq!(request.category, Category::Linear);
        assert_eq!(request.request.len(), 2);
        assert_eq!(request.request[0].symbol, "BTCUSDT");
        assert_eq!(request.request[1].symbol, "ETHUSDT");
    }

    #[test]
    fn test_batch_cancel_orders_request_builder() {
        let cancel1 = CancelOrderRequest {
            category: Category::Spot,
            symbol: "BTCUSDT".to_string(),
            order_id: Some("order1".to_string()),
            order_link_id: None,
            order_filter: None,
        };

        let cancel2 = CancelOrderRequest {
            category: Category::Spot,
            symbol: "ETHUSDT".to_string(),
            order_id: Some("order2".to_string()),
            order_link_id: None,
            order_filter: None,
        };

        let request = BatchCancelOrdersRequest {
            category: Category::Spot,
            request: vec![cancel1, cancel2],
        };

        assert_eq!(request.category, Category::Spot);
        assert_eq!(request.request.len(), 2);
    }

    #[test]
    fn test_batch_cancel_orders_request_serialization() {
        let cancel = CancelOrderRequest {
            category: Category::Linear,
            symbol: "BTCUSDT".to_string(),
            order_id: Some("order123".to_string()),
            order_link_id: None,
            order_filter: None,
        };

        let request = BatchCancelOrdersRequest {
            category: Category::Linear,
            request: vec![cancel],
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"category\":\"linear\""));
        assert!(json.contains("\"request\":["));
        assert!(json.contains("\"symbol\":\"BTCUSDT\""));
    }
}