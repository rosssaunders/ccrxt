use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

const AMEND_SPREAD_ORDER_ENDPOINT: &str = "/api/v5/sprd/amend-order";

/// Request parameters for amending a spread order
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AmendSpreadOrderRequest {
    /// Order ID
    /// Either `ord_id` or `cl_ord_id` is required. If both are passed, `ord_id` will be used.
    #[serde(rename = "ordId", skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,

    /// Client Order ID as assigned by the client
    /// Either `ord_id` or `cl_ord_id` is required. If both are passed, `ord_id` will be used.
    #[serde(rename = "clOrdId", skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,

    /// Client Request ID as assigned by the client for order amendment
    /// A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 32 characters.
    #[serde(rename = "reqId", skip_serializing_if = "Option::is_none")]
    pub req_id: Option<String>,

    /// New quantity after amendment
    /// Either `new_sz` or `new_px` is required.
    /// When amending a partially-filled order, the new_sz should include the amount that has been filled.
    #[serde(rename = "newSz", skip_serializing_if = "Option::is_none")]
    pub new_sz: Option<String>,

    /// New price after amendment
    /// Either `new_sz` or `new_px` is required.
    #[serde(rename = "newPx", skip_serializing_if = "Option::is_none")]
    pub new_px: Option<String>,
}

/// Response data for amending a spread order
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AmendSpreadOrderResponse {
    /// Order ID
    #[serde(rename = "ordId")]
    pub ord_id: String,

    /// Client Order ID as assigned by the client
    #[serde(rename = "clOrdId")]
    pub cl_ord_id: String,

    /// Client Request ID as assigned by the client for order amendment
    #[serde(rename = "reqId")]
    pub req_id: String,

    /// The code of the event execution result, 0 means success
    #[serde(rename = "sCode")]
    pub s_code: String,

    /// Rejection message if the request is unsuccessful
    #[serde(rename = "sMsg")]
    pub s_msg: String,
}

impl RestClient {
    /// Amend spread order
    ///
    /// Amend an incomplete spread order
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#spread-trading-rest-api-amend-order)
    pub async fn amend_spread_order(
        &self,
        request: AmendSpreadOrderRequest,
    ) -> RestResult<AmendSpreadOrderResponse> {
        self.send_post_request(
            AMEND_SPREAD_ORDER_ENDPOINT,
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
    fn test_amend_spread_order_request_with_new_sz() {
        let request = AmendSpreadOrderRequest {
            ord_id: Some("312269865356374016".to_string()),
            cl_ord_id: None,
            req_id: Some("req123".to_string()),
            new_sz: Some("2".to_string()),
            new_px: None,
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: AmendSpreadOrderRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_amend_spread_order_request_with_new_px() {
        let request = AmendSpreadOrderRequest {
            ord_id: None,
            cl_ord_id: Some("client123".to_string()),
            req_id: None,
            new_sz: None,
            new_px: Some("51000".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("clOrdId"));
        assert!(serialized.contains("newPx"));
        assert!(!serialized.contains("ordId"));
        assert!(!serialized.contains("reqId"));
        assert!(!serialized.contains("newSz"));
    }

    #[test]
    fn test_amend_spread_order_request_with_both_new_sz_and_px() {
        let request = AmendSpreadOrderRequest {
            ord_id: Some("312269865356374016".to_string()),
            cl_ord_id: Some("client123".to_string()),
            req_id: Some("req456".to_string()),
            new_sz: Some("1.5".to_string()),
            new_px: Some("52000".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("ordId"));
        assert!(serialized.contains("clOrdId"));
        assert!(serialized.contains("reqId"));
        assert!(serialized.contains("newSz"));
        assert!(serialized.contains("newPx"));
    }

    #[test]
    fn test_amend_spread_order_response_success() {
        let json_response = r#"{
            "ordId": "312269865356374016",
            "clOrdId": "client123",
            "reqId": "req123",
            "sCode": "0",
            "sMsg": ""
        }"#;

        let response: AmendSpreadOrderResponse = serde_json::from_str(json_response).unwrap();
        assert_eq!(response.ord_id, "312269865356374016");
        assert_eq!(response.cl_ord_id, "client123");
        assert_eq!(response.req_id, "req123");
        assert_eq!(response.s_code, "0");
        assert_eq!(response.s_msg, "");
    }

    #[test]
    fn test_amend_spread_order_response_error() {
        let json_response = r#"{
            "ordId": "",
            "clOrdId": "client456",
            "reqId": "req456",
            "sCode": "51000",
            "sMsg": "Order does not exist"
        }"#;

        let response: AmendSpreadOrderResponse = serde_json::from_str(json_response).unwrap();
        assert_eq!(response.ord_id, "");
        assert_eq!(response.s_code, "51000");
        assert_eq!(response.s_msg, "Order does not exist");
    }

    #[test]
    fn test_amend_spread_order_serialization() {
        let response = AmendSpreadOrderResponse {
            ord_id: "312269865356374016".to_string(),
            cl_ord_id: "client123".to_string(),
            req_id: "req123".to_string(),
            s_code: "0".to_string(),
            s_msg: "".to_string(),
        };

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: AmendSpreadOrderResponse = serde_json::from_str(&serialized).unwrap();
        assert_eq!(response, deserialized);
    }
}
