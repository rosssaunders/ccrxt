use serde::Deserialize;

use super::{RestClient, cancel_order::CancelOrderRequest, common::OkxApiResponse};
use crate::okx::{EndpointType, RestResult};

const TRADE_CANCEL_BATCH_ORDERS_ENDPOINT: &str = "api/v5/trade/cancel-batch-orders";

/// Response from canceling multiple orders
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelBatchOrdersResponse {
    /// Client Order ID as assigned by the client
    pub cl_ord_id: Option<String>,

    /// Order ID
    pub ord_id: String,

    /// Response code for individual order: "0" means success
    pub s_code: String,

    /// Response message for individual order
    pub s_msg: String,
}

impl RestClient {
    /// Cancel batch orders
    ///
    /// Cancels multiple orders in a single request. Maximum 20 orders can be canceled per request.
    ///
    /// [docs]: https://www.okx.com/docs-v5/en/#order-book-trading-trade-post-cancel-multiple-orders
    ///
    /// Rate limit: 300 orders per 2 seconds
    ///
    /// # Arguments
    /// * `orders` - Array of order cancellation requests (maximum 20 orders)
    ///
    /// # Returns
    /// Response containing the batch order cancellation results
    pub async fn cancel_batch_orders(
        &self,
        orders: &[CancelOrderRequest],
    ) -> RestResult<OkxApiResponse<CancelBatchOrdersResponse>> {
        self.send_request(
            TRADE_CANCEL_BATCH_ORDERS_ENDPOINT,
            reqwest::Method::POST,
            Some(orders),
            EndpointType::PrivateTrading,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel_batch_orders_serialization() {
        let orders = vec![
            CancelOrderRequest {
                inst_id: "BTC-USDT".to_string(),
                ord_id: Some("312269865356374016".to_string()),
                cl_ord_id: None,
            },
            CancelOrderRequest {
                inst_id: "ETH-USDT".to_string(),
                ord_id: None,
                cl_ord_id: Some("order_2".to_string()),
            },
        ];

        let json = serde_json::to_string(&orders).unwrap();
        assert!(json.contains("\"instId\":\"BTC-USDT\""));
        assert!(json.contains("\"instId\":\"ETH-USDT\""));
        assert!(json.contains("\"ordId\":\"312269865356374016\""));
        assert!(json.contains("\"clOrdId\":\"order_2\""));
    }

    #[test]
    fn test_cancel_batch_orders_response_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "clOrdId": "",
                    "ordId": "312269865356374016",
                    "sCode": "0",
                    "sMsg": ""
                },
                {
                    "clOrdId": "order_2",
                    "ordId": "312269865356374017",
                    "sCode": "0",
                    "sMsg": ""
                }
            ]
        }"#;

        let response: OkxApiResponse<CancelBatchOrdersResponse> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 2);
        assert_eq!(response.data[0].ord_id, "312269865356374016");
        assert_eq!(response.data[1].cl_ord_id, Some("order_2".to_string()));
        assert_eq!(response.data[1].ord_id, "312269865356374017");
    }
}
