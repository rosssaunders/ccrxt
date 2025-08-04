use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::okx::{EndpointType, RestResult};

const ACCOUNT_MMP_CONFIG_ENDPOINT: &str = "api/v5/account/mmp-config";

/// Request to get mmp config
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMmpConfigRequest {
    /// Instrument family
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
}

/// MMP configuration
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MmpConfig {
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
    /// Get MMP config
    ///
    /// [docs]: https://www.okx.com/docs-v5/en/#trading-account-rest-api-get-mmp-config
    ///
    /// # Arguments
    /// * `request` - The get MMP config request
    ///
    /// # Returns
    /// A result containing the MMP config or an error
    pub async fn get_mmp_config(&self, request: &GetMmpConfigRequest) -> RestResult<MmpConfig> {
        self.send_request(
            ACCOUNT_MMP_CONFIG_ENDPOINT,
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
    use crate::okx::response::OkxApiResponse;

    #[test]
    fn test_get_mmp_config_request_serialization() {
        let request = GetMmpConfigRequest {
            inst_family: Some("BTC-USD".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("instFamily=BTC-USD"));
    }

    #[test]
    fn test_mmp_config_deserialization() {
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

        let response: OkxApiResponse<MmpConfig> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let config = &response.data[0];
        assert_eq!(config.inst_family, "BTC-USD");
        assert_eq!(config.time_interval, "5000");
        assert_eq!(config.frozen_interval, "1000");
        assert_eq!(config.qty_limit, "100");
    }
}
