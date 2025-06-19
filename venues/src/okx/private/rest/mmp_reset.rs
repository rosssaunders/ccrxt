use serde::{Deserialize, Serialize};

use super::{RestClient, common::OkxApiResponse};
use crate::okx::{EndpointType, RestResult};

/// Request to reset MMP
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MmpResetRequest {
    /// Instrument family
    pub inst_family: String,
}

/// Response for reset MMP
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MmpResetResponse {
    /// Instrument family
    pub inst_family: String,

    /// Reset result
    pub result: bool,
}

impl RestClient {
    /// Reset MMP
    ///
    /// # Arguments
    /// * `request` - The reset MMP request
    ///
    /// # Returns
    /// A result containing the reset MMP response or an error
    pub async fn mmp_reset(
        &self,
        request: &MmpResetRequest,
    ) -> RestResult<OkxApiResponse<MmpResetResponse>> {
        self.send_request(
            "api/v5/account/mmp-reset",
            reqwest::Method::POST,
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
    fn test_mmp_reset_request_serialization() {
        let request = MmpResetRequest {
            inst_family: "BTC-USD".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"instFamily\":\"BTC-USD\""));
    }

    #[test]
    fn test_mmp_reset_response_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instFamily": "BTC-USD",
                    "result": true
                }
            ]
        }"#;

        let response: OkxApiResponse<MmpResetResponse> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let result = &response.data[0];
        assert_eq!(result.inst_family, "BTC-USD");
        assert!(result.result);
    }
}