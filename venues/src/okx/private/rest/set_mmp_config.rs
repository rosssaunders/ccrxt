use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::okx::{EndpointType, RestResult};

const ACCOUNT_MMP_CONFIG_ENDPOINT: &str = "api/v5/account/mmp-config";

/// Request to set MMP config
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetMmpConfigRequest {
    /// Instrument family
    pub inst_family: String,

    /// Time interval (ms)
    pub time_interval: String,

    /// Frozen interval (ms)
    pub frozen_interval: String,

    /// Quantity limit
    pub qty_limit: String,
}

/// Response for set MMP config
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetMmpConfigResponse {
    /// Instrument family
    pub inst_family: String,

    /// Time interval (ms)
    pub time_interval: String,

    /// Frozen interval (ms)
    pub frozen_interval: String,

    /// Quantity limit
    pub qty_limit: String,
}

impl RestClient {
    /// Set MMP config
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#trading-account-rest-api-set-mmp-config)
    ///
    /// # Arguments
    /// * `request` - The set MMP config request
    ///
    /// # Returns
    /// A result containing the set MMP config response or an error
    pub async fn set_mmp_config(
        &self,
        request: &SetMmpConfigRequest,
    ) -> RestResult<SetMmpConfigResponse> {
        self.send_post_request(
            ACCOUNT_MMP_CONFIG_ENDPOINT,
            Some(request),
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::okx::response::OkxApiResponse;

    #[test]
    fn test_set_mmp_config_request_serialization() {
        let request = SetMmpConfigRequest {
            inst_family: "BTC-USD".to_string(),
            time_interval: "5000".to_string(),
            frozen_interval: "1000".to_string(),
            qty_limit: "100".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"instFamily\":\"BTC-USD\""));
        assert!(json.contains("\"timeInterval\":\"5000\""));
        assert!(json.contains("\"frozenInterval\":\"1000\""));
        assert!(json.contains("\"qtyLimit\":\"100\""));
    }

    #[test]
    fn test_set_mmp_config_response_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instFamily": "BTC-USD",
                    "timeInterval": "5000",
                    "frozenInterval": "1000",
                    "qtyLimit": "100"
                }
            ]
        }"#;

        let response: OkxApiResponse<SetMmpConfigResponse> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let result = response.data.first();
        assert!(result.is_some(), "Expected at least one result in response");
        let result = result.unwrap();
        assert_eq!(result.inst_family, "BTC-USD");
        assert_eq!(result.time_interval, "5000");
        assert_eq!(result.frozen_interval, "1000");
        assert_eq!(result.qty_limit, "100");
    }
}
