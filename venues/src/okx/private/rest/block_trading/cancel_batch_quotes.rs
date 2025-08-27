use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

/// Endpoint URL for cancelling multiple quotes
const CANCEL_BATCH_QUOTES_ENDPOINT: &str = "api/v5/rfq/cancel-batch-quotes";

/// Request parameters for cancelling multiple quotes
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelBatchQuotesRequest {
    /// Quote IDs
    #[serde(rename = "quoteIds", skip_serializing_if = "Option::is_none")]
    pub quote_ids: Option<Vec<String>>,

    /// Client-supplied Quote IDs
    #[serde(rename = "clQuoteIds", skip_serializing_if = "Option::is_none")]
    pub cl_quote_ids: Option<Vec<String>>,
}

/// Response for individual quote cancellation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelBatchQuoteResult {
    /// Quote ID
    #[serde(rename = "quoteId")]
    pub quote_id: String,

    /// Client-supplied Quote ID
    #[serde(rename = "clQuoteId")]
    pub cl_quote_id: String,

    /// The code of the event execution result, "0" means success
    #[serde(rename = "sCode")]
    pub s_code: String,

    /// Rejection message if the request is unsuccessful
    #[serde(rename = "sMsg")]
    pub s_msg: String,
}

impl RestClient {
    /// Cancel multiple Quotes
    ///
    /// Cancel one or multiple active Quote(s) in a single batch. Maximum 100 quote
    /// orders can be canceled per request.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#block-trading-rest-api-cancel-multiple-quotes)
    ///
    /// Rate limit: 2 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The batch quote cancellation request parameters
    ///
    /// # Returns
    /// Response containing the results of each quote cancellation
    pub async fn cancel_batch_quotes(
        &self,
        request: CancelBatchQuotesRequest,
    ) -> RestResult<CancelBatchQuoteResult> {
        self.send_post_request(
            CANCEL_BATCH_QUOTES_ENDPOINT,
            &request,
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
    fn test_cancel_batch_quotes_request_with_quote_ids() {
        let request = CancelBatchQuotesRequest {
            quote_ids: Some(vec!["quote_123".to_string(), "quote_456".to_string()]),
            cl_quote_ids: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"quoteIds\":[\"quote_123\",\"quote_456\"]"));
        assert!(!json.contains("\"clQuoteIds\""));
    }

    #[test]
    fn test_cancel_batch_quotes_request_with_client_quote_ids() {
        let request = CancelBatchQuotesRequest {
            quote_ids: None,
            cl_quote_ids: Some(vec!["cl_quote_123".to_string(), "cl_quote_456".to_string()]),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"clQuoteIds\":[\"cl_quote_123\",\"cl_quote_456\"]"));
        assert!(!json.contains("\"quoteIds\""));
    }

    #[test]
    fn test_cancel_batch_quote_result_deserialization() {
        let result_json = json!({
            "quoteId": "quote_123",
            "clQuoteId": "cl_quote_123",
            "sCode": "0",
            "sMsg": ""
        });

        let result: CancelBatchQuoteResult = serde_json::from_value(result_json).unwrap();
        assert_eq!(result.quote_id, "quote_123");
        assert_eq!(result.cl_quote_id, "cl_quote_123");
        assert_eq!(result.s_code, "0");
        assert_eq!(result.s_msg, "");
    }

    #[test]
    fn test_cancel_batch_quotes_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "quoteId": "quote_123",
                    "clQuoteId": "cl_quote_123",
                    "sCode": "0",
                    "sMsg": ""
                },
                {
                    "quoteId": "quote_456",
                    "clQuoteId": "cl_quote_456",
                    "sCode": "51008",
                    "sMsg": "Quote not found"
                }
            ]
        });

        let response: ApiResponse<CancelBatchQuoteResult> =
            serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 2);
        assert_eq!(response.data[0].s_code, "0");
        assert_eq!(response.data[1].s_code, "51008");
        assert_eq!(response.data[1].s_msg, "Quote not found");
    }

    #[test]
    fn test_cancel_batch_quotes_request_builder() {
        let request = CancelBatchQuotesRequest {
            quote_ids: Some(vec!["quote_123".to_string()]),
            cl_quote_ids: Some(vec!["cl_quote_456".to_string()]),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"quoteIds\":[\"quote_123\"]"));
        assert!(json.contains("\"clQuoteIds\":[\"cl_quote_456\"]"));
    }
}
