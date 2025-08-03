use serde::{Deserialize, Serialize};

use super::{RestClient, common::OkxApiResponse};
use crate::okx::{EndpointType, RestResult};

const ACCOUNT_ADJUST_LEVERAGE_INFO_ENDPOINT: &str = "api/v5/account/adjust-leverage-info";

/// Request to get adjust leverage info
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAdjustLeverageInfoRequest {
    /// Instrument ID, e.g. "BTC-USDT"
    pub inst_id: String,

    /// Margin mode
    /// "cross", "isolated"  
    pub mgn_mode: String,

    /// Position side
    /// "long", "short", "net"
    pub pos_side: String,

    /// Leverage
    pub lever: String,
}

/// Adjust leverage info details
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdjustLeverageInfo {
    /// Instrument ID
    pub inst_id: String,

    /// Leverage
    pub lever: String,

    /// Position side
    pub pos_side: String,

    /// Maximum position size
    pub max_pos_sz: String,

    /// Maximum size in USD
    pub max_pos_sz_usd: String,

    /// Position
    pub pos: String,

    /// Available position size
    pub avail_pos_sz: String,
}

impl RestClient {
    /// Get adjust leverage info
    ///
    /// [docs]: https://www.okx.com/docs-v5/en/#trading-account-rest-api-get-leverage-estimated-info
    ///
    /// # Arguments
    /// * `request` - The get adjust leverage info request
    ///
    /// # Returns
    /// A result containing the adjust leverage info or an error
    pub async fn get_adjust_leverage_info(
        &self,
        request: &GetAdjustLeverageInfoRequest,
    ) -> RestResult<OkxApiResponse<AdjustLeverageInfo>> {
        self.send_request(
            ACCOUNT_ADJUST_LEVERAGE_INFO_ENDPOINT,
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
    fn test_get_adjust_leverage_info_request_serialization() {
        let request = GetAdjustLeverageInfoRequest {
            inst_id: "BTC-USDT-SWAP".to_string(),
            mgn_mode: "isolated".to_string(),
            pos_side: "long".to_string(),
            lever: "5".to_string(),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("instId=BTC-USDT-SWAP"));
        assert!(serialized.contains("mgnMode=isolated"));
        assert!(serialized.contains("posSide=long"));
        assert!(serialized.contains("lever=5"));
    }

    #[test]
    fn test_adjust_leverage_info_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instId": "BTC-USDT-SWAP",
                    "lever": "5",
                    "posSide": "long",
                    "maxPosSz": "100",
                    "maxPosSzUsd": "5000000",
                    "pos": "10",
                    "availPosSz": "90"
                }
            ]
        }"#;

        let response: OkxApiResponse<AdjustLeverageInfo> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let info = &response.data[0];
        assert_eq!(info.inst_id, "BTC-USDT-SWAP");
        assert_eq!(info.lever, "5");
        assert_eq!(info.pos_side, "long");
        assert_eq!(info.max_pos_sz, "100");
    }
}
