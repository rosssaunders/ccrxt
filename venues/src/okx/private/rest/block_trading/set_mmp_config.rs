use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

/// Endpoint URL for setting MMP config
const SET_MMP_CONFIG_ENDPOINT: &str = "api/v5/rfq/mmp-config";

/// Request to set MMP configuration
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetMmpConfigRequest {
    /// Time window in milliseconds for MMP (Market Maker Protection)
    pub time_interval: String,

    /// Frozen period in milliseconds
    pub frozen_interval: String,

    /// Quantity threshold for MMP trigger
    pub qty_limit: String,
}

/// Response from setting MMP config
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetMmpConfigResponse {
    /// Time window in milliseconds
    pub time_interval: String,

    /// Frozen period in milliseconds
    pub frozen_interval: String,

    /// Quantity threshold for MMP trigger
    pub qty_limit: String,
}

impl RestClient {
    /// Set block trading MMP config
    ///
    /// Set Market Maker Protection configuration for block trading.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#block-trading-rest-api-set-mmp-config)
    ///
    /// Rate limit: 5 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The MMP configuration parameters
    ///
    /// # Returns
    /// A result containing the MMP configuration response
    pub async fn set_block_trading_mmp_config(
        &self,
        request: SetMmpConfigRequest,
    ) -> RestResult<SetMmpConfigResponse> {
        self.send_post_request(
            SET_MMP_CONFIG_ENDPOINT,
            request,
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
    fn test_set_mmp_config_request() {
        let request = SetMmpConfigRequest {
            time_interval: "5000".to_string(),
            frozen_interval: "10000".to_string(),
            qty_limit: "1000".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"timeInterval\":\"5000\""));
        assert!(json.contains("\"frozenInterval\":\"10000\""));
        assert!(json.contains("\"qtyLimit\":\"1000\""));
    }

    #[test]
    fn test_set_mmp_config_response() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "timeInterval": "5000",
                    "frozenInterval": "10000",
                    "qtyLimit": "1000"
                }
            ]
        });

        let response: ApiResponse<SetMmpConfigResponse> =
            serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].time_interval, "5000");
    }
}
