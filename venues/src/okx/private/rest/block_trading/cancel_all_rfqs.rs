use serde::Deserialize;

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

/// Endpoint URL for canceling all RFQs
const CANCEL_ALL_RFQS_ENDPOINT: &str = "api/v5/rfq/cancel-all-rfqs";

/// Response from canceling all RFQs
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllRfqsResponse {
    /// The timestamp of successful cancellation (Unix timestamp in milliseconds)
    pub ts: String,
}

impl RestClient {
    /// Cancel all RFQs
    ///
    /// Cancel all your active RFQs with one request.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#block-trading-rest-api-cancel-all-rfqs)
    ///
    /// Rate limit: 5 requests per 2 seconds
    ///
    /// # Returns
    /// A result containing the cancellation timestamp
    pub async fn cancel_all_rfqs(&self) -> RestResult<CancelAllRfqsResponse> {
        self.send_post_request(
            CANCEL_ALL_RFQS_ENDPOINT,
            serde_json::Value::Null,
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
    fn test_cancel_all_rfqs_response() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "ts": "1597026383085"
                }
            ]
        });

        let response: ApiResponse<CancelAllRfqsResponse> =
            serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].ts, "1597026383085");
    }
}
