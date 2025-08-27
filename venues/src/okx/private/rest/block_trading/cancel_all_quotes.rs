use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

/// Endpoint URL for cancelling all quotes
const CANCEL_ALL_QUOTES_ENDPOINT: &str = "api/v5/rfq/cancel-all-quotes";

/// Response for cancelling all quotes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllQuotesResponse {
    /// The timestamp of cancellation successfully (Unix timestamp in milliseconds)
    pub ts: String,
}

impl RestClient {
    /// Cancel all Quotes
    ///
    /// Cancels all active Quotes.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#block-trading-rest-api-cancel-all-quotes)
    ///
    /// Rate limit: 2 requests per 2 seconds
    ///
    /// # Returns
    /// Response containing the cancellation timestamp
    pub async fn cancel_all_quotes(&self) -> RestResult<CancelAllQuotesResponse> {
        self.send_post_request(
            CANCEL_ALL_QUOTES_ENDPOINT,
            &serde_json::json!({}),
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
    fn test_cancel_all_quotes_response_deserialization() {
        let response_json = json!({
            "ts": "1597026383085"
        });

        let response: CancelAllQuotesResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.ts, "1597026383085");
    }

    #[test]
    fn test_cancel_all_quotes_api_response() {
        let api_response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "ts": "1597026383085"
                }
            ]
        });

        let api_response: ApiResponse<CancelAllQuotesResponse> =
            serde_json::from_value(api_response_json).unwrap();
        assert_eq!(api_response.code, "0");
        assert_eq!(api_response.data.len(), 1);
        assert_eq!(api_response.data[0].ts, "1597026383085");
    }
}
