use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

/// Endpoint URL for setting auto earn
const ACCOUNT_SET_AUTO_EARN_ENDPOINT: &str = "api/v5/account/set-auto-earn";

/// Request parameters for setting auto earn
#[derive(Debug, Clone, Serialize)]
pub struct SetAutoEarnRequest {
    /// Currency, e.g. BTC
    pub ccy: String,

    /// Auto earn switch
    /// true: on
    /// false: off
    #[serde(rename = "autoEarn")]
    pub auto_earn: bool,
}

/// Set auto earn response
#[derive(Debug, Clone, Deserialize)]
pub struct SetAutoEarnResponse {
    /// Currency, e.g. BTC
    pub ccy: String,

    /// Auto earn switch
    #[serde(rename = "autoEarn")]
    pub auto_earn: bool,
}

impl RestClient {
    /// Set auto earn
    ///
    /// Set the auto earning switch for the currency.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#trading-account-rest-api-set-auto-earn)
    ///
    /// Rate limit: 5 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The set auto earn request parameters
    ///
    /// # Returns
    /// A result containing the set auto earn response
    pub async fn set_auto_earn(
        &self,
        request: SetAutoEarnRequest,
    ) -> RestResult<SetAutoEarnResponse> {
        self.send_post_request(
            ACCOUNT_SET_AUTO_EARN_ENDPOINT,
            Some(&request),
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
    fn test_set_auto_earn_request_serialization() {
        let request = SetAutoEarnRequest {
            ccy: "BTC".to_string(),
            auto_earn: true,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"ccy\":\"BTC\""));
        assert!(json.contains("\"autoEarn\":true"));
    }

    #[test]
    fn test_set_auto_earn_response_deserialization() {
        let response_json = json!({
            "ccy": "BTC",
            "autoEarn": true
        });

        let response: SetAutoEarnResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.ccy, "BTC");
        assert!(response.auto_earn);
    }

    #[test]
    fn test_full_response_deserialization() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "ccy": "USDT",
                    "autoEarn": false
                }
            ]
        });

        let response: ApiResponse<SetAutoEarnResponse> =
            serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].ccy, "USDT");
        assert!(!response.data[0].auto_earn);
    }
}
