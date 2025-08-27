use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::okx::{EndpointType, RestResult};

const TRADE_AMEND_ORDER_ENDPOINT: &str = "api/v5/trade/amend-order";

/// Request to amend an existing order
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AmendOrderRequest {
    /// Instrument ID, e.g. "BTC-USDT"
    pub inst_id: String,

    /// Order ID
    /// Either ordId or clOrdId is required. If both are passed, ordId will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,

    /// Client Order ID as assigned by the client
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,

    /// Client Request ID as assigned by the client for order amendment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_id: Option<String>,

    /// New quantity to buy or sell. When amending the quantity, only one order can be amended at a time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_sz: Option<String>,

    /// New order price. Only applicable to limit orders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_px: Option<String>,

    /// New take-profit trigger price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_tp_trigger_px: Option<String>,

    /// New take-profit order price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_tp_ord_px: Option<String>,

    /// New stop-loss trigger price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_sl_trigger_px: Option<String>,

    /// New stop-loss order price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_sl_ord_px: Option<String>,

    /// New take-profit trigger price type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_tp_trigger_px_type: Option<String>,

    /// New stop-loss trigger price type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_sl_trigger_px_type: Option<String>,
}

/// Response from amending an order
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmendOrderResponse {
    /// Client Order ID as assigned by the client
    pub cl_ord_id: Option<String>,

    /// Order ID
    pub ord_id: String,

    /// Client Request ID as assigned by the client for order amendment
    pub req_id: Option<String>,

    /// Response code for individual order: "0" means success
    pub s_code: String,

    /// Response message for individual order
    pub s_msg: String,
}

impl RestClient {
    /// Amend an existing order
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#order-book-trading-trade-post-amend-order)
    ///
    /// # Arguments
    /// * `request` - The order amendment request
    ///
    /// # Returns
    /// A result containing the order amendment response or an error
    pub async fn amend_order(&self, request: &AmendOrderRequest) -> RestResult<AmendOrderResponse> {
        self.send_post_request(
            TRADE_AMEND_ORDER_ENDPOINT,
            Some(request),
            EndpointType::PrivateTrading,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::okx::response::ApiResponse;

    #[test]
    fn test_amend_order_request_serialization() {
        let request = AmendOrderRequest {
            inst_id: "BTC-USDT".to_string(),
            ord_id: Some("312269865356374016".to_string()),
            cl_ord_id: None,
            req_id: Some("amend_req_123".to_string()),
            new_sz: Some("0.02".to_string()),
            new_px: Some("51000.0".to_string()),
            new_tp_trigger_px: None,
            new_tp_ord_px: None,
            new_sl_trigger_px: None,
            new_sl_ord_px: None,
            new_tp_trigger_px_type: None,
            new_sl_trigger_px_type: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"instId\":\"BTC-USDT\""));
        assert!(json.contains("\"ordId\":\"312269865356374016\""));
        assert!(json.contains("\"reqId\":\"amend_req_123\""));
        assert!(json.contains("\"newSz\":\"0.02\""));
        assert!(json.contains("\"newPx\":\"51000.0\""));
        assert!(!json.contains("clOrdId"));
    }

    #[test]
    fn test_amend_order_request_with_client_id() {
        let request = AmendOrderRequest {
            inst_id: "BTC-USDT".to_string(),
            ord_id: None,
            cl_ord_id: Some("my_order_123".to_string()),
            req_id: None,
            new_sz: None,
            new_px: Some("52000.0".to_string()),
            new_tp_trigger_px: Some("55000.0".to_string()),
            new_tp_ord_px: Some("55000.0".to_string()),
            new_sl_trigger_px: Some("48000.0".to_string()),
            new_sl_ord_px: Some("48000.0".to_string()),
            new_tp_trigger_px_type: None,
            new_sl_trigger_px_type: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"instId\":\"BTC-USDT\""));
        assert!(json.contains("\"clOrdId\":\"my_order_123\""));
        assert!(json.contains("\"newPx\":\"52000.0\""));
        assert!(json.contains("\"newTpTriggerPx\":\"55000.0\""));
        assert!(json.contains("\"newSlTriggerPx\":\"48000.0\""));
        assert!(!json.contains("ordId"));
    }

    #[test]
    fn test_amend_order_response_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "clOrdId": "my_order_123",
                    "ordId": "312269865356374016",
                    "reqId": "amend_req_123",
                    "sCode": "0",
                    "sMsg": ""
                }
            ]
        }"#;

        let response: ApiResponse<AmendOrderResponse> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].cl_ord_id, Some("my_order_123".to_string()));
        assert_eq!(response.data[0].ord_id, "312269865356374016");
        assert_eq!(response.data[0].req_id, Some("amend_req_123".to_string()));
        assert_eq!(response.data[0].s_code, "0");
    }
}
