use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

/// Endpoint URL for canceling quote
const CANCEL_QUOTE_ENDPOINT: &str = "api/v5/rfq/cancel-quote";

/// Request to cancel a quote
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelQuoteRequest {
    /// Quote ID created by system
    #[serde(rename = "quoteId", skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<String>,

    /// Client-supplied Quote ID
    #[serde(rename = "clQuoteId", skip_serializing_if = "Option::is_none")]
    pub cl_quote_id: Option<String>,

    /// RFQ ID created by system
    #[serde(rename = "rfqId", skip_serializing_if = "Option::is_none")]
    pub rfq_id: Option<String>,

    /// Client-supplied RFQ ID
    #[serde(rename = "clRfqId", skip_serializing_if = "Option::is_none")]
    pub cl_rfq_id: Option<String>,
}

/// Response from canceling a quote
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelQuoteResponse {
    /// Quote ID
    #[serde(rename = "quoteId")]
    pub quote_id: String,

    /// Client-supplied Quote ID
    #[serde(rename = "clQuoteId")]
    pub cl_quote_id: String,

    /// The code of the event execution result ("0" means success)
    #[serde(rename = "sCode")]
    pub s_code: String,

    /// Rejection message if unsuccessful
    #[serde(rename = "sMsg")]
    pub s_msg: String,
}

impl RestClient {
    /// Cancel quote
    ///
    /// Cancel an active quote that you have created.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#block-trading-rest-api-cancel-quote)
    ///
    /// Rate limit: 5 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The quote cancellation request parameters
    ///
    /// # Returns
    /// A result containing the quote cancellation response
    pub async fn cancel_quote(
        &self,
        request: CancelQuoteRequest,
    ) -> RestResult<CancelQuoteResponse> {
        self.send_post_request(CANCEL_QUOTE_ENDPOINT, request, EndpointType::PrivateAccount)
            .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::okx::response::ApiResponse;

    #[test]
    fn test_cancel_quote_by_id() {
        let request = CancelQuoteRequest {
            quote_id: Some("quote_123".to_string()),
            cl_quote_id: None,
            rfq_id: None,
            cl_rfq_id: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"quoteId\":\"quote_123\""));
        assert!(!json.contains("\"clQuoteId\""));
    }

    #[test]
    fn test_cancel_quote_response() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "quoteId": "quote_123",
                    "clQuoteId": "client_quote_123",
                    "sCode": "0",
                    "sMsg": ""
                }
            ]
        });

        let response: ApiResponse<CancelQuoteResponse> =
            serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data[0].quote_id, "quote_123");
        assert_eq!(response.data[0].s_code, "0");
    }
}
