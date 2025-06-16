use serde::{Deserialize, Serialize};

use super::{RestClient, common::OkxApiResponse, place_order::PlaceOrderRequest};
use crate::okx::{EndpointType, RestResult};

/// Request to place multiple orders at once
#[derive(Debug, Clone, Serialize)]
pub struct PlaceBatchOrdersRequest {
    /// Array of order requests (maximum 20 orders)
    #[serde(flatten)]
    pub orders: Vec<PlaceOrderRequest>,
}

/// Response from placing multiple orders
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceBatchOrdersResponse {
    /// Client Order ID as assigned by the client
    pub cl_ord_id: Option<String>,

    /// Order ID assigned by the system
    pub ord_id: String,

    /// Order tag
    pub tag: Option<String>,

    /// Response code for individual order: "0" means success
    pub s_code: String,

    /// Response message for individual order
    pub s_msg: String,
}

impl RestClient {
    /// Place multiple orders at once
    ///
    /// # Arguments
    /// * `orders` - Vector of order placement requests (maximum 20 orders)
    ///
    /// # Returns
    /// A result containing the batch order placement responses or an error
    pub async fn place_batch_orders(&self, orders: &[PlaceOrderRequest]) -> RestResult<OkxApiResponse<PlaceBatchOrdersResponse>> {
        self.send_request(
            "api/v5/trade/batch-orders",
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
    use crate::okx::{OrderSide, OrderType};

    #[test]
    fn test_place_batch_orders_serialization() {
        let orders = vec![
            PlaceOrderRequest {
                inst_id: "BTC-USDT".to_string(),
                td_mode: "cash".to_string(),
                ccy: None,
                cl_ord_id: Some("order_1".to_string()),
                tag: None,
                side: OrderSide::Buy,
                ord_type: OrderType::Limit,
                sz: "0.01".to_string(),
                px: Some("50000.0".to_string()),
                reduce_only: None,
                tgt_ccy: None,
                attach_algo_ords: None,
            },
            PlaceOrderRequest {
                inst_id: "ETH-USDT".to_string(),
                td_mode: "cash".to_string(),
                ccy: None,
                cl_ord_id: Some("order_2".to_string()),
                tag: None,
                side: OrderSide::Sell,
                ord_type: OrderType::Market,
                sz: "0.1".to_string(),
                px: None,
                reduce_only: None,
                tgt_ccy: None,
                attach_algo_ords: None,
            },
        ];

        let json = serde_json::to_string(&orders).unwrap();
        assert!(json.contains("\"instId\":\"BTC-USDT\""));
        assert!(json.contains("\"instId\":\"ETH-USDT\""));
        assert!(json.contains("\"clOrdId\":\"order_1\""));
        assert!(json.contains("\"clOrdId\":\"order_2\""));
        assert!(json.contains("\"side\":\"buy\""));
        assert!(json.contains("\"side\":\"sell\""));
    }

    #[test]
    fn test_place_batch_orders_response_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "clOrdId": "order_1",
                    "ordId": "312269865356374016",
                    "tag": "",
                    "sCode": "0",
                    "sMsg": ""
                },
                {
                    "clOrdId": "order_2",
                    "ordId": "312269865356374017",
                    "tag": "",
                    "sCode": "0",
                    "sMsg": ""
                }
            ]
        }"#;

        let response: OkxApiResponse<PlaceBatchOrdersResponse> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 2);
        assert_eq!(response.data[0].cl_ord_id, Some("order_1".to_string()));
        assert_eq!(response.data[0].ord_id, "312269865356374016");
        assert_eq!(response.data[1].cl_ord_id, Some("order_2".to_string()));
        assert_eq!(response.data[1].ord_id, "312269865356374017");
    }
}
