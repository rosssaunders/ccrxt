use serde::{Deserialize, Serialize};

use super::{RestClient, common::OkxApiResponse};
use crate::okx::{EndpointType, RestResult};


const ACCOUNT_RISK_STATE_ENDPOINT: &str = "api/v5/account/risk-state";
/// Request to get risk state
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRiskStateRequest {
    // This endpoint has no parameters
}

/// Risk state details
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskState {
    /// State of auto-deleveraging
    /// "1": Auto-deleveraging enabled, "2": Auto-deleveraging disabled
    pub at_risk: String,

    /// Risk unit of auto-deleveraging
    /// "1": USD, "2": Coin
    pub at_risk_idx: String,

    /// Risk state of auto-deleveraging
    /// "1", "2", "3", "4", "5"
    pub at_risk_mgn: String,

    /// Timestamp
    pub ts: String,
}

impl RestClient {
    /// Get risk state
    ///
    /// # Arguments
    /// * `request` - The get risk state request
    ///
    /// # Returns
    /// A result containing the risk state or an error
    pub async fn get_risk_state(
        &self,
        request: &GetRiskStateRequest,
    ) -> RestResult<OkxApiResponse<RiskState>> {
        self.send_request(
            ACCOUNT_RISK_STATE_ENDPOINT,
            reqwest::Method::GET,
            Some(request),
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_risk_state_request_serialization() {
        let request = GetRiskStateRequest {};

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_risk_state_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "atRisk": "1",
                    "atRiskIdx": "1",
                    "atRiskMgn": "3",
                    "ts": "1597026383085"
                }
            ]
        }"#;

        let response: OkxApiResponse<RiskState> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let state = &response.data[0];
        assert_eq!(state.at_risk, "1");
        assert_eq!(state.at_risk_idx, "1");
        assert_eq!(state.at_risk_mgn, "3");
        assert_eq!(state.ts, "1597026383085");
    }
}
