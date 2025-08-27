use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

/// Endpoint URL for canceling batch RFQs
const CANCEL_BATCH_RFQS_ENDPOINT: &str = "api/v5/rfq/cancel-batch-rfqs";

/// Request to cancel multiple existing RFQs
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelBatchRfqsRequest {
    /// RFQ IDs created by system
    #[serde(rename = "rfqIds", skip_serializing_if = "Option::is_none")]
    pub rfq_ids: Option<Vec<String>>,

    /// Client-supplied RFQ IDs
    /// Either rfqIds or clRfqIds is required
    /// If both attributes are sent, rfqIds will be used as primary identifier
    #[serde(rename = "clRfqIds", skip_serializing_if = "Option::is_none")]
    pub cl_rfq_ids: Option<Vec<String>>,
}

/// Response from canceling batch RFQs
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelBatchRfqsResponse {
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
    /// Cancel batch RFQs
    ///
    /// Cancel multiple existing active RFQs that you have created previously.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#block-trading-rest-api-cancel-batch-rfqs)
    ///
    /// Rate limit: 5 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The batch RFQ cancellation request parameters
    ///
    /// # Returns
    /// A result containing the batch RFQ cancellation responses with status for each RFQ
    pub async fn cancel_batch_rfqs(
        &self,
        request: CancelBatchRfqsRequest,
    ) -> RestResult<CancelBatchRfqsResponse> {
        self.send_post_request(
            CANCEL_BATCH_RFQS_ENDPOINT,
            request,
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::okx::response::ApiResponse;

    #[test]
    fn test_cancel_batch_rfqs_response_deserialization() {
        let response_json = json!({
            "rfqId": "rfq_123",
            "clRfqId": "client_123",
            "sCode": "0",
            "sMsg": ""
        });

        let response: CancelBatchRfqsResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.rfq_id, "rfq_123");
        assert_eq!(response.cl_rfq_id, "client_123");
        assert_eq!(response.s_code, "0");
        assert_eq!(response.s_msg, "");
    }

    #[test]
    fn test_cancel_batch_rfqs_full_response() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "rfqId": "rfq_123",
                    "clRfqId": "client_123",
                    "sCode": "0",
                    "sMsg": ""
                },
                {
                    "rfqId": "rfq_456",
                    "clRfqId": "client_456",
                    "sCode": "1",
                    "sMsg": "RFQ not found"
                }
            ]
        });

        let response: ApiResponse<CancelBatchRfqsResponse> =
            serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 2);

        // First success
        assert_eq!(response.data[0].rfq_id, "rfq_123");
        assert_eq!(response.data[0].s_code, "0");

        // Second failure
        assert_eq!(response.data[1].rfq_id, "rfq_456");
        assert_eq!(response.data[1].s_code, "1");
        assert_eq!(response.data[1].s_msg, "RFQ not found");
    }
}
