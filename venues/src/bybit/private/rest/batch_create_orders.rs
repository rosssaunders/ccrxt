use serde::{Deserialize, Serialize};

use super::{client::RestClient, create_order::CreateOrderRequest};
use crate::bybit::{EndpointType, RestResult, enums::*};

/// Endpoint URL for batch creating orders
const BATCH_CREATE_ORDERS_ENDPOINT: &str = "/v5/order/create-batch";

/// Request parameters for batch creating multiple orders.
///
/// Allows creating up to 20 orders in a single request for improved efficiency.
#[derive(Debug, Clone, Serialize)]
pub struct BatchCreateOrdersRequest {
    /// Product type (linear, spot, option, inverse)
    pub category: Category,

    /// List of order creation requests (maximum 20 orders)
    pub request: Vec<CreateOrderRequest>,
}

/// Individual order creation result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchCreateOrderResult {
    /// Unique order ID of the created order
    pub order_id: String,

    /// User-defined order ID of the created order
    pub order_link_id: String,
}

/// Error information for failed order creations.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchCreateOrderError {
    /// Error code
    pub code: i32,

    /// Error message
    pub msg: String,
}

/// Batch creation response data.
#[derive(Debug, Clone, Deserialize)]
pub struct BatchCreateOrdersData {
    /// Successful creation results
    pub result: BatchCreateOrdersResult,

    /// Extended information containing error details
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: BatchCreateOrdersExtInfo,
}

/// Container for successful batch creation results.
#[derive(Debug, Clone, Deserialize)]
pub struct BatchCreateOrdersResult {
    /// List of successfully created orders
    pub list: Vec<BatchCreateOrderResult>,
}

/// Extended information containing error details for failed creations.
#[derive(Debug, Clone, Deserialize)]
pub struct BatchCreateOrdersExtInfo {
    /// List of errors for failed order creations
    pub list: Vec<BatchCreateOrderError>,
}

/// Response from the batch create orders API endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct BatchCreateOrdersResponse {
    /// Return code (0 indicates success)
    #[serde(rename = "retCode")]
    pub ret_code: i32,

    /// Return message
    #[serde(rename = "retMsg")]
    pub ret_msg: String,

    /// Batch creation results and error information
    pub result: BatchCreateOrdersData,

    /// Response timestamp in milliseconds
    pub time: u64,
}

impl RestClient {
    /// Batch create orders
    ///
    /// Place multiple orders in a single request for improved efficiency.
    /// Maximum 20 orders per batch. Supports all order types and markets.
    ///
    /// [API Documentation](https://bybit-exchange.github.io/docs/v5/order/batch-place)
    ///
    /// Rate limit: 10 requests per second per UID
    ///
    /// # Arguments
    /// * `request` - The batch creation request containing up to 20 new orders
    ///
    /// # Returns
    /// A result containing both successful order creations and error details for failed ones
    pub async fn batch_create_orders(
        &self,
        request: BatchCreateOrdersRequest,
    ) -> RestResult<BatchCreateOrdersResponse> {
        self.send_signed_request(
            BATCH_CREATE_ORDERS_ENDPOINT,
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
    use crate::bybit::enums::{OrderType, Side};

    #[test]
    fn test_batch_create_orders_request() {
        let order1 = CreateOrderRequest {
            category: Category::Linear,
            symbol: "BTCUSDT".to_string(),
            side: Side::Buy,
            order_type: OrderType::Limit,
            qty: "0.001".to_string(),
            price: Some("50000".to_string()),
            ..Default::default()
        };

        let order2 = CreateOrderRequest {
            category: Category::Linear,
            symbol: "ETHUSDT".to_string(),
            side: Side::Sell,
            order_type: OrderType::Market,
            qty: "0.1".to_string(),
            ..Default::default()
        };

        let request = BatchCreateOrdersRequest {
            category: Category::Linear,
            request: vec![order1, order2],
        };

        assert_eq!(request.category, Category::Linear);
        assert_eq!(request.request.len(), 2);
        assert_eq!(request.request[0].symbol, "BTCUSDT");
        assert_eq!(request.request[1].symbol, "ETHUSDT");
    }

    #[test]
    fn test_batch_create_orders_request_builder() {
        let order1 = CreateOrderRequest {
            category: Category::Spot,
            symbol: "BTCUSDT".to_string(),
            side: Side::Buy,
            order_type: OrderType::Limit,
            qty: "0.001".to_string(),
            ..Default::default()
        };

        let order2 = CreateOrderRequest {
            category: Category::Spot,
            symbol: "ETHUSDT".to_string(),
            side: Side::Sell,
            order_type: OrderType::Market,
            qty: "0.1".to_string(),
            ..Default::default()
        };

        let request = BatchCreateOrdersRequest {
            category: Category::Spot,
            request: vec![order1, order2],
        };

        assert_eq!(request.category, Category::Spot);
        assert_eq!(request.request.len(), 2);
    }

    #[test]
    fn test_batch_create_orders_request_serialization() {
        let order = CreateOrderRequest {
            category: Category::Linear,
            symbol: "BTCUSDT".to_string(),
            side: Side::Buy,
            order_type: OrderType::Limit,
            qty: "0.001".to_string(),
            ..Default::default()
        };

        let request = BatchCreateOrdersRequest {
            category: Category::Linear,
            request: vec![order],
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"category\":\"linear\""));
        assert!(json.contains("\"request\":["));
        assert!(json.contains("\"symbol\":\"BTCUSDT\""));
    }
}
