use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

/// Endpoint URL for resetting MMP status
const MMP_RESET_ENDPOINT: &str = "api/v5/rfq/mmp-reset";

/// Response for MMP reset
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MmpResetResponse {
    /// The timestamp of re-setting successfully (Unix timestamp in milliseconds)
    pub ts: String,
}

impl RestClient {
    /// Reset MMP status
    ///
    /// Reset the MMP status to be inactive.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#block-trading-rest-api-reset-mmp-status)
    ///
    /// Rate limit: 5 requests per 2 seconds
    ///
    /// # Returns
    /// Response containing the reset timestamp
    pub async fn mmp_reset(&self) -> RestResult<MmpResetResponse> {
        self.send_post_request(
            MMP_RESET_ENDPOINT,
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
    fn test_mmp_reset_response_deserialization() {
        let response_json = json!({
            "ts": "1597026383085"
        });

        let response: MmpResetResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.ts, "1597026383085");
    }

    #[test]
    fn test_mmp_reset_api_response() {
        let api_response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "ts": "1597026383085"
                }
            ]
        });

        let api_response: ApiResponse<MmpResetResponse> =
            serde_json::from_value(api_response_json).unwrap();
        assert_eq!(api_response.code, "0");
        assert_eq!(api_response.data.len(), 1);
        assert_eq!(api_response.data[0].ts, "1597026383085");
    }
}
