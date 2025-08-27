use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

/// Endpoint URL for getting MMP config
const GET_MMP_CONFIG_ENDPOINT: &str = "api/v5/rfq/mmp-config";

/// Response for MMP configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MmpConfig {
    /// Time window (ms). MMP interval where monitoring is done.
    /// "0" means MMP is disabled
    #[serde(rename = "timeInterval")]
    pub time_interval: String,

    /// Frozen period (ms). If it is "0", the trade will remain frozen until manually reset
    /// and mmp_frozen_until will be ""
    #[serde(rename = "frozenInterval")]
    pub frozen_interval: String,

    /// Limit in number of execution attempts
    #[serde(rename = "countLimit")]
    pub count_limit: String,

    /// Whether MMP is currently triggered
    #[serde(rename = "mmpFrozen")]
    pub mmp_frozen: bool,

    /// If frozen_interval is not "0" and mmp_frozen = true, it is the time interval (in ms)
    /// when MMP is no longer triggered, otherwise ""
    #[serde(rename = "mmpFrozenUntil")]
    pub mmp_frozen_until: String,
}

impl RestClient {
    /// Get MMP Config
    ///
    /// This endpoint is used to get MMP configure information and only applicable to
    /// block trading market makers.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#block-trading-rest-api-get-mmp-config)
    ///
    /// Rate limit: 5 requests per 2 seconds
    ///
    /// # Returns
    /// Response containing MMP configuration
    pub async fn get_mmp_config(&self) -> RestResult<MmpConfig> {
        self.send_get_request(
            GET_MMP_CONFIG_ENDPOINT,
            None::<&()>,
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
    fn test_mmp_config_deserialization() {
        let config_json = json!({
            "timeInterval": "60000",
            "frozenInterval": "10000",
            "countLimit": "5",
            "mmpFrozen": false,
            "mmpFrozenUntil": ""
        });

        let config: MmpConfig = serde_json::from_value(config_json).unwrap();
        assert_eq!(config.time_interval, "60000");
        assert_eq!(config.frozen_interval, "10000");
        assert_eq!(config.count_limit, "5");
        assert!(!config.mmp_frozen);
        assert_eq!(config.mmp_frozen_until, "");
    }

    #[test]
    fn test_mmp_config_frozen_deserialization() {
        let config_json = json!({
            "timeInterval": "60000",
            "frozenInterval": "20000",
            "countLimit": "3",
            "mmpFrozen": true,
            "mmpFrozenUntil": "1597026403085"
        });

        let config: MmpConfig = serde_json::from_value(config_json).unwrap();
        assert_eq!(config.time_interval, "60000");
        assert_eq!(config.frozen_interval, "20000");
        assert_eq!(config.count_limit, "3");
        assert!(config.mmp_frozen);
        assert_eq!(config.mmp_frozen_until, "1597026403085");
    }

    #[test]
    fn test_mmp_config_disabled_deserialization() {
        let config_json = json!({
            "timeInterval": "0",
            "frozenInterval": "0",
            "countLimit": "0",
            "mmpFrozen": false,
            "mmpFrozenUntil": ""
        });

        let config: MmpConfig = serde_json::from_value(config_json).unwrap();
        assert_eq!(config.time_interval, "0");
        assert_eq!(config.frozen_interval, "0");
        assert_eq!(config.count_limit, "0");
        assert!(!config.mmp_frozen);
        assert_eq!(config.mmp_frozen_until, "");
    }

    #[test]
    fn test_get_mmp_config_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "timeInterval": "120000",
                    "frozenInterval": "30000",
                    "countLimit": "10",
                    "mmpFrozen": false,
                    "mmpFrozenUntil": ""
                }
            ]
        });

        let response: ApiResponse<MmpConfig> = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].time_interval, "120000");
        assert_eq!(response.data[0].count_limit, "10");
        assert!(!response.data[0].mmp_frozen);
    }
}
