use serde::{Deserialize, Serialize};

use super::{cancel_order::CancelOrderRequest, client::RestClient};
use crate::bybit::{EndpointType, RestResult, enums::*};

/// Endpoint URL for batch cancelling orders
const BATCH_CANCEL_ORDERS_ENDPOINT: &str = "/v5/order/cancel-batch";

/// Request parameters for batch cancelling multiple orders.
///
/// Allows cancelling up to 20 orders in a single request for improved efficiency.
#[derive(Debug, Clone, Serialize)]
pub struct BatchCancelOrdersRequest {
    /// Product type (linear, spot, option, inverse)
    pub category: Category,

    /// List of order cancellation requests (maximum 20 orders)
    pub request: Vec<CancelOrderRequest>,
}

/// Individual order cancellation result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchCancelOrderResult {
    /// Unique order ID of the cancelled order
    pub order_id: String,

    /// User-defined order ID of the cancelled order
    pub order_link_id: String,
}

/// Error information for failed order cancellations.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchCancelOrderError {
    /// Error code
    pub code: i32,

    /// Error message
    pub msg: String,
}

/// Batch cancellation response data.
#[derive(Debug, Clone, Deserialize)]
pub struct BatchCancelOrdersData {
    /// Successful cancellation results
    pub result: BatchCancelOrdersResult,

    /// Extended information containing error details
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: BatchCancelOrdersExtInfo,
}

/// Container for successful batch cancellation results.
#[derive(Debug, Clone, Deserialize)]
pub struct BatchCancelOrdersResult {
    /// List of successfully cancelled orders
    pub list: Vec<BatchCancelOrderResult>,
}

/// Extended information containing error details for failed cancellations.
#[derive(Debug, Clone, Deserialize)]
pub struct BatchCancelOrdersExtInfo {
    /// List of errors for failed order cancellations
    pub list: Vec<BatchCancelOrderError>,
}

/// Response from the batch cancel orders API endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct BatchCancelOrdersResponse {
    /// Return code (0 indicates success)
    #[serde(rename = "retCode")]
    pub ret_code: i32,

    /// Return message
    #[serde(rename = "retMsg")]
    pub ret_msg: String,

    /// Batch cancellation results and error information
    pub result: BatchCancelOrdersData,

    /// Response timestamp in milliseconds
    pub time: u64,
}

impl RestClient {
    /// Batch cancel orders
    ///
    /// Cancel multiple orders in a single request for improved efficiency.
    /// Maximum 20 orders per batch. Supports all order types and markets.
    ///
    /// [API Documentation](https://bybit-exchange.github.io/docs/v5/order/batch-cancel)
    ///
    /// Rate limit: 10 requests per second per UID
    ///
    /// # Arguments
    /// * `request` - The batch cancellation request containing up to 20 order identifiers
    ///
    /// # Returns
    /// A result containing both successful cancellations and error details for failed ones
    pub async fn batch_cancel_orders(
        &self,
        request: BatchCancelOrdersRequest,
    ) -> RestResult<BatchCancelOrdersResponse> {
        self.send_signed_request(
            BATCH_CANCEL_ORDERS_ENDPOINT,
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
