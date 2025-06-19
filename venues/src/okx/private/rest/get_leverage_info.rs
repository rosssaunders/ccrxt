use serde::{Deserialize, Serialize};

use super::{RestClient, common::OkxApiResponse};
use crate::okx::{EndpointType, RestResult};

/// Request to get leverage info
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLeverageInfoRequest {
    /// Instrument ID, e.g. "BTC-USDT"
    pub inst_id: String,

    /// Margin mode
    /// "cross", "isolated"
    pub mgn_mode: String,
}

/// Leverage info details
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeverageInfo {
    /// Instrument ID
    pub inst_id: String,

    /// Margin mode
    pub mgn_mode: String,

    /// Position side
    pub pos_side: String,

    /// Leverage
    pub lever: String,
}

impl RestClient {
    /// Get leverage info
    ///
    /// # Arguments
    /// * `request` - The get leverage info request
    ///
    /// # Returns
    /// A result containing the leverage info or an error
    pub async fn get_leverage_info(&self, request: &GetLeverageInfoRequest) -> RestResult<OkxApiResponse<LeverageInfo>> {
        self.send_request(
            "api/v5/account/leverage-info",
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
    fn test_get_leverage_info_request_serialization() {
        let request = GetLeverageInfoRequest {
            inst_id: "BTC-USDT-SWAP".to_string(),
            mgn_mode: "cross".to_string(),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("instId=BTC-USDT-SWAP"));
        assert!(serialized.contains("mgnMode=cross"));
    }

    #[test]
    fn test_leverage_info_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instId": "BTC-USDT-SWAP",
                    "mgnMode": "cross",
                    "posSide": "long",
                    "lever": "10"
                }
            ]
        }"#;

        let response: OkxApiResponse<LeverageInfo> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let info = &response.data[0];
        assert_eq!(info.inst_id, "BTC-USDT-SWAP");
        assert_eq!(info.mgn_mode, "cross");
        assert_eq!(info.lever, "10");
    }
}