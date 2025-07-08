use serde::{Deserialize, Serialize};

use super::{amend_order::AmendOrderRequest, client::RestClient};
use crate::bybit::{EndpointType, RestResult, enums::*};

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
