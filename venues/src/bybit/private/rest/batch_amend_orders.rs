use serde::{Deserialize, Serialize};

use super::{amend_order::AmendOrderRequest, client::RestClient};
use crate::bybit::{EndpointType, RestResult, enums::*};

/// Endpoint URL for batch amending orders
const BATCH_AMEND_ORDERS_ENDPOINT: &str = "/v5/order/amend-batch";

/// Request parameters for batch amending multiple orders.
///
/// Allows amending up to 20 orders in a single request for improved efficiency.
#[derive(Debug, Clone, Serialize)]
pub struct BatchAmendOrdersRequest {
    /// Product type (linear, spot, option, inverse)
    pub category: Category,

    /// List of order amendment requests (maximum 20 orders)
    pub request: Vec<AmendOrderRequest>,
}

/// Individual order amendment result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchAmendOrderResult {
    /// Unique order ID of the amended order
    pub order_id: String,

    /// User-defined order ID of the amended order
    pub order_link_id: String,
}

/// Error information for failed order amendments.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchAmendOrderError {
    /// Error code
    pub code: i32,

    /// Error message
    pub msg: String,
}

/// Batch amendment response data.
#[derive(Debug, Clone, Deserialize)]
pub struct BatchAmendOrdersData {
    /// Successful amendment results
    pub result: BatchAmendOrdersResult,

    /// Extended information containing error details
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: BatchAmendOrdersExtInfo,
}

/// Container for successful batch amendment results.
#[derive(Debug, Clone, Deserialize)]
pub struct BatchAmendOrdersResult {
    /// List of successfully amended orders
    pub list: Vec<BatchAmendOrderResult>,
}

/// Extended information containing error details for failed amendments.
#[derive(Debug, Clone, Deserialize)]
pub struct BatchAmendOrdersExtInfo {
    /// List of errors for failed order amendments
    pub list: Vec<BatchAmendOrderError>,
}

/// Response from the batch amend orders API endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct BatchAmendOrdersResponse {
    /// Return code (0 indicates success)
    #[serde(rename = "retCode")]
    pub ret_code: i32,

    /// Return message
    #[serde(rename = "retMsg")]
    pub ret_msg: String,

    /// Batch amendment results and error information
    pub result: BatchAmendOrdersData,

    /// Response timestamp in milliseconds
    pub time: u64,
}

impl RestClient {
    /// Batch amend orders
    ///
    /// Amend multiple orders in a single request for improved efficiency.
    /// Maximum 20 orders per batch. Supports all order types and markets.
    ///
    /// [API Documentation](https://bybit-exchange.github.io/docs/v5/order/batch-amend)
    ///
    /// Rate limit: 10 requests per second per UID
    ///
    /// # Arguments
    /// * `request` - The batch amendment request containing up to 20 order modifications
    ///
    /// # Returns
    /// A result containing both successful amendments and error details for failed ones
    pub async fn batch_amend_orders(
        &self,
        request: BatchAmendOrdersRequest,
    ) -> RestResult<BatchAmendOrdersResponse> {
        self.send_post_signed_request(BATCH_AMEND_ORDERS_ENDPOINT, request, EndpointType::Trade)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_amend_orders_request() {
        let amend1 = AmendOrderRequest {
            category: Category::Linear,
            symbol: "BTCUSDT".to_string(),
            order_id: Some("order1".to_string()),
            order_link_id: None,
            order_iv: None,
            trigger_price: None,
            qty: None,
            price: Some("51000".to_string()),
            tpsl_mode: None,
            take_profit: None,
            stop_loss: None,
            tp_trigger_by: None,
            sl_trigger_by: None,
            trigger_by: None,
            tp_limit_price: None,
            sl_limit_price: None,
        };

        let amend2 = AmendOrderRequest {
            category: Category::Linear,
            symbol: "ETHUSDT".to_string(),
            order_id: None,
            order_link_id: Some("custom-order-2".to_string()),
            order_iv: None,
            trigger_price: None,
            qty: Some("0.2".to_string()),
            price: None,
            tpsl_mode: None,
            take_profit: None,
            stop_loss: None,
            tp_trigger_by: None,
            sl_trigger_by: None,
            trigger_by: None,
            tp_limit_price: None,
            sl_limit_price: None,
        };

        let request = BatchAmendOrdersRequest {
            category: Category::Linear,
            request: vec![amend1, amend2],
        };

        assert_eq!(request.category, Category::Linear);
        assert_eq!(request.request.len(), 2);
        assert_eq!(request.request[0].symbol, "BTCUSDT");
        assert_eq!(request.request[1].symbol, "ETHUSDT");
    }

    #[test]
    fn test_batch_amend_orders_request_builder() {
        let amend1 = AmendOrderRequest {
            category: Category::Spot,
            symbol: "BTCUSDT".to_string(),
            order_id: Some("order1".to_string()),
            order_link_id: None,
            order_iv: None,
            trigger_price: None,
            qty: None,
            price: None,
            tpsl_mode: None,
            take_profit: None,
            stop_loss: None,
            tp_trigger_by: None,
            sl_trigger_by: None,
            trigger_by: None,
            tp_limit_price: None,
            sl_limit_price: None,
        };

        let amend2 = AmendOrderRequest {
            category: Category::Spot,
            symbol: "ETHUSDT".to_string(),
            order_id: Some("order2".to_string()),
            order_link_id: None,
            order_iv: None,
            trigger_price: None,
            qty: None,
            price: None,
            tpsl_mode: None,
            take_profit: None,
            stop_loss: None,
            tp_trigger_by: None,
            sl_trigger_by: None,
            trigger_by: None,
            tp_limit_price: None,
            sl_limit_price: None,
        };

        let request = BatchAmendOrdersRequest {
            category: Category::Spot,
            request: vec![amend1, amend2],
        };

        assert_eq!(request.category, Category::Spot);
        assert_eq!(request.request.len(), 2);
    }

    #[test]
    fn test_batch_amend_orders_request_serialization() {
        let amend = AmendOrderRequest {
            category: Category::Linear,
            symbol: "BTCUSDT".to_string(),
            order_id: Some("order123".to_string()),
            order_link_id: None,
            order_iv: None,
            trigger_price: None,
            qty: None,
            price: None,
            tpsl_mode: None,
            take_profit: None,
            stop_loss: None,
            tp_trigger_by: None,
            sl_trigger_by: None,
            trigger_by: None,
            tp_limit_price: None,
            sl_limit_price: None,
        };

        let request = BatchAmendOrdersRequest {
            category: Category::Linear,
            request: vec![amend],
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"category\":\"linear\""));
        assert!(json.contains("\"request\":["));
        assert!(json.contains("\"symbol\":\"BTCUSDT\""));
    }
}
