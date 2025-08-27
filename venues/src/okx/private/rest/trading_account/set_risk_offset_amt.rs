use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

/// Endpoint URL for setting risk offset amount
const ACCOUNT_SET_RISK_OFFSET_AMT_ENDPOINT: &str = "api/v5/account/set-riskOffset-amt";

/// Request parameters for setting risk offset amount
#[derive(Debug, Clone, Serialize)]
pub struct SetRiskOffsetAmtRequest {
    /// Instrument ID, e.g. BTC-USD-SWAP
    #[serde(rename = "instId")]
    pub inst_id: String,

    /// Risk offset amount
    pub amt: String,
}

/// Set risk offset amount response
#[derive(Debug, Clone, Deserialize)]
pub struct SetRiskOffsetAmtResponse {
    /// Instrument ID
    #[serde(rename = "instId")]
    pub inst_id: String,

    /// Risk offset amount
    pub amt: String,
}

impl RestClient {
    /// Set risk offset amount
    ///
    /// You can only set a risk offset amount for positions you hold in portfolio margin mode.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#trading-account-rest-api-set-risk-offset-amount)
    ///
    /// Rate limit: 20 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The set risk offset amount request parameters
    ///
    /// # Returns
    /// A result containing the set risk offset amount response
    pub async fn set_risk_offset_amt(
        &self,
        request: SetRiskOffsetAmtRequest,
    ) -> RestResult<SetRiskOffsetAmtResponse> {
        self.send_post_request(
            ACCOUNT_SET_RISK_OFFSET_AMT_ENDPOINT,
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
    fn test_set_risk_offset_amt_request_serialization() {
        let request = SetRiskOffsetAmtRequest {
            inst_id: "BTC-USD-SWAP".to_string(),
            amt: "100".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"instId\":\"BTC-USD-SWAP\""));
        assert!(json.contains("\"amt\":\"100\""));
    }

    #[test]
    fn test_set_risk_offset_amt_response_deserialization() {
        let response_json = json!({
            "instId": "BTC-USD-SWAP",
            "amt": "100"
        });

        let response: SetRiskOffsetAmtResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.inst_id, "BTC-USD-SWAP");
        assert_eq!(response.amt, "100");
    }

    #[test]
    fn test_full_response_deserialization() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instId": "BTC-USD-SWAP",
                    "amt": "100"
                }
            ]
        });

        let response: ApiResponse<SetRiskOffsetAmtResponse> =
            serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].inst_id, "BTC-USD-SWAP");
        assert_eq!(response.data[0].amt, "100");
    }
}
