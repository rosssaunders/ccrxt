use serde::{Deserialize, Serialize};

use super::{RestClient, common::OkxApiResponse};
use crate::okx::{EndpointType, RestResult};

const TRADE_CANCEL_ORDER_ENDPOINT: &str = "/api/v5/trade/cancel-order";


/// Request to cancel an existing order
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest {
    /// Instrument ID, e.g. "BTC-USDT"
    pub inst_id: String,

    /// Order ID
    /// Either ordId or clOrdId is required. If both are passed, ordId will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,

    /// Client Order ID as assigned by the client
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
}

/// Response from canceling an order
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {
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
    /// Cancel an existing order
    ///
    /// # Arguments
    /// * `request` - The order cancellation request
    ///
    /// # Returns
    /// A result containing the order cancellation response or an error
    pub async fn cancel_order(
        &self,
        request: &CancelOrderRequest,
    ) -> RestResult<OkxApiResponse<CancelOrderResponse>> {
        self.send_request(
            TRADE_CANCEL_ORDER_ENDPOINT,
            reqwest::Method::POST,
            Some(request),
            EndpointType::PrivateTrading,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel_order_request_serialization() {
        let request = CancelOrderRequest {
            inst_id: "BTC-USDT".to_string(),
            ord_id: Some("312269865356374016".to_string()),
            cl_ord_id: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"instId\":\"BTC-USDT\""));
        assert!(json.contains("\"ordId\":\"312269865356374016\""));
        assert!(!json.contains("clOrdId"));
    }

    #[test]
    fn test_cancel_order_request_with_client_id() {
        let request = CancelOrderRequest {
            inst_id: "BTC-USDT".to_string(),
            ord_id: None,
            cl_ord_id: Some("my_order_123".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"instId\":\"BTC-USDT\""));
        assert!(json.contains("\"clOrdId\":\"my_order_123\""));
        assert!(!json.contains("ordId"));
    }

    #[test]
    fn test_cancel_order_response_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "clOrdId": "my_order_123",
                    "ordId": "312269865356374016",
                    "sCode": "0",
                    "sMsg": ""
                }
            ]
        }"#;

        let response: OkxApiResponse<CancelOrderResponse> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].cl_ord_id, Some("my_order_123".to_string()));
        assert_eq!(response.data[0].ord_id, "312269865356374016");
        assert_eq!(response.data[0].s_code, "0");
    }
}
