use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

const CANCEL_SPREAD_ORDER_ENDPOINT: &str = "/api/v5/sprd/cancel-order";

/// Request parameters for canceling a spread order
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelSpreadOrderRequest {
    /// Order ID
    /// Either `ord_id` or `cl_ord_id` is required. If both are passed, `ord_id` will be used.
    #[serde(rename = "ordId", skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,

    /// Client Order ID as assigned by the client
    /// Either `ord_id` or `cl_ord_id` is required. If both are passed, `ord_id` will be used.
    #[serde(rename = "clOrdId", skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
}

/// Response data for canceling a spread order
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelSpreadOrderResponse {
    /// Order ID
    #[serde(rename = "ordId")]
    pub ord_id: String,

    /// Client Order ID as assigned by the client
    #[serde(rename = "clOrdId")]
    pub cl_ord_id: String,

    /// The code of the event execution result, 0 means success
    #[serde(rename = "sCode")]
    pub s_code: String,

    /// Rejection message if the request is unsuccessful
    #[serde(rename = "sMsg")]
    pub s_msg: String,
}

impl RestClient {
    /// Cancel spread order
    ///
    /// Cancel a pending spread order
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#spread-trading-rest-api-cancel-order)
    pub async fn cancel_spread_order(
        &self,
        request: CancelSpreadOrderRequest,
    ) -> RestResult<CancelSpreadOrderResponse> {
        self.send_post_request(
            CANCEL_SPREAD_ORDER_ENDPOINT,
            Some(&request),
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_cancel_spread_order_request_with_ord_id() {
        let request = CancelSpreadOrderRequest {
            ord_id: Some("312269865356374016".to_string()),
            cl_ord_id: None,
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: CancelSpreadOrderRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_cancel_spread_order_request_with_cl_ord_id() {
        let request = CancelSpreadOrderRequest {
            ord_id: None,
            cl_ord_id: Some("client123".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("clOrdId"));
        assert!(!serialized.contains("ordId"));
    }

    #[test]
    fn test_cancel_spread_order_request_with_both_ids() {
        let request = CancelSpreadOrderRequest {
            ord_id: Some("312269865356374016".to_string()),
            cl_ord_id: Some("client123".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("ordId"));
        assert!(serialized.contains("clOrdId"));
    }

    #[test]
    fn test_cancel_spread_order_response_success() {
        let json_response = r#"{
            "ordId": "312269865356374016",
            "clOrdId": "client123",
            "sCode": "0",
            "sMsg": ""
        }"#;

        let response: CancelSpreadOrderResponse = serde_json::from_str(json_response).unwrap();
        assert_eq!(response.ord_id, "312269865356374016");
        assert_eq!(response.cl_ord_id, "client123");
        assert_eq!(response.s_code, "0");
        assert_eq!(response.s_msg, "");
    }

    #[test]
    fn test_cancel_spread_order_response_error() {
        let json_response = r#"{
            "ordId": "",
            "clOrdId": "client456",
            "sCode": "51000",
            "sMsg": "Order does not exist"
        }"#;

        let response: CancelSpreadOrderResponse = serde_json::from_str(json_response).unwrap();
        assert_eq!(response.ord_id, "");
        assert_eq!(response.s_code, "51000");
        assert_eq!(response.s_msg, "Order does not exist");
    }
}
