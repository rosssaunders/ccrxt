use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

/// Endpoint URL for canceling RFQ
const CANCEL_RFQ_ENDPOINT: &str = "api/v5/rfq/cancel-rfq";

/// Request to cancel an existing RFQ
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelRfqRequest {
    /// RFQ ID created by system
    #[serde(rename = "rfqId", skip_serializing_if = "Option::is_none")]
    pub rfq_id: Option<String>,

    /// Client-supplied RFQ ID
    /// Either rfqId or clRfqId is required. If both are passed, rfqId will be used
    #[serde(rename = "clRfqId", skip_serializing_if = "Option::is_none")]
    pub cl_rfq_id: Option<String>,
}

/// Response from canceling an RFQ
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelRfqResponse {
    /// RFQ ID
    #[serde(rename = "rfqId")]
    pub rfq_id: String,

    /// Client-supplied RFQ ID
    #[serde(rename = "clRfqId")]
    pub cl_rfq_id: String,

    /// The code of the event execution result ("0" means success)
    #[serde(rename = "sCode")]
    pub s_code: String,

    /// Rejection message if the request is unsuccessful
    #[serde(rename = "sMsg")]
    pub s_msg: String,
}

impl RestClient {
    /// Cancel RFQ
    ///
    /// Cancel an existing active RFQ that you have created previously.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#block-trading-rest-api-cancel-rfq)
    ///
    /// Rate limit: 5 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The RFQ cancellation request parameters
    ///
    /// # Returns
    /// A result containing the RFQ cancellation response with status
    pub async fn cancel_rfq(&self, request: CancelRfqRequest) -> RestResult<CancelRfqResponse> {
        self.send_post_request(CANCEL_RFQ_ENDPOINT, request, EndpointType::PrivateAccount)
            .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::okx::response::ApiResponse;

    #[test]
    fn test_cancel_rfq_request_by_rfq_id() {
        let request = CancelRfqRequest {
            rfq_id: Some("rfq_123456".to_string()),
            cl_rfq_id: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"rfqId\":\"rfq_123456\""));
        assert!(!json.contains("\"clRfqId\""));

        assert_eq!(request.rfq_id, Some("rfq_123456".to_string()));
        assert_eq!(request.cl_rfq_id, None);
    }

    #[test]
    fn test_cancel_rfq_request_by_client_id() {
        let request = CancelRfqRequest {
            rfq_id: None,
            cl_rfq_id: Some("client_rfq_123".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"clRfqId\":\"client_rfq_123\""));
        assert!(!json.contains("\"rfqId\""));

        assert_eq!(request.rfq_id, None);
        assert_eq!(request.cl_rfq_id, Some("client_rfq_123".to_string()));
    }

    #[test]
    fn test_cancel_rfq_response_deserialization() {
        let response_json = json!({
            "rfqId": "rfq_123456",
            "clRfqId": "client_rfq_123",
            "sCode": "0",
            "sMsg": ""
        });

        let response: CancelRfqResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.rfq_id, "rfq_123456");
        assert_eq!(response.cl_rfq_id, "client_rfq_123");
        assert_eq!(response.s_code, "0");
        assert_eq!(response.s_msg, "");
    }

    #[test]
    fn test_cancel_rfq_full_response_deserialization() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "rfqId": "rfq_123456",
                    "clRfqId": "client_rfq_123",
                    "sCode": "0",
                    "sMsg": ""
                }
            ]
        });

        let response: ApiResponse<CancelRfqResponse> =
            serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let cancel_response = &response.data[0];
        assert_eq!(cancel_response.rfq_id, "rfq_123456");
        assert_eq!(cancel_response.s_code, "0");
    }

    #[test]
    fn test_cancel_rfq_error_response() {
        let response_json = json!({
            "rfqId": "rfq_123456",
            "clRfqId": "",
            "sCode": "1",
            "sMsg": "RFQ not found or already canceled"
        });

        let response: CancelRfqResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.rfq_id, "rfq_123456");
        assert_eq!(response.cl_rfq_id, "");
        assert_eq!(response.s_code, "1");
        assert_eq!(response.s_msg, "RFQ not found or already canceled");
    }

    #[test]
    fn test_cancel_rfq_request_serialization_roundtrip() {
        let original = CancelRfqRequest {
            rfq_id: Some("rfq_789".to_string()),
            cl_rfq_id: Some("client_789".to_string()),
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: CancelRfqRequest = serde_json::from_value(serialized).unwrap();

        assert_eq!(original.rfq_id, deserialized.rfq_id);
        assert_eq!(original.cl_rfq_id, deserialized.cl_rfq_id);
    }
}
