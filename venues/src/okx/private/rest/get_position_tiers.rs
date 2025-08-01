use serde::{Deserialize, Serialize};

use super::{RestClient, common::OkxApiResponse};
use crate::okx::{EndpointType, InstrumentType, RestResult};


const ACCOUNT_POSITION_TIERS_ENDPOINT: &str = "api/v5/account/position-tiers";
/// Request to get position tiers
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionTiersRequest {
    /// Instrument type: FUTURES, SWAP, OPTION
    pub inst_type: InstrumentType,

    /// Trade mode: cross, isolated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub td_mode: Option<String>,

    /// Underlying, e.g. BTC-USD. Only applicable to FUTURES/SWAP/OPTION
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,

    /// Instrument family, e.g. BTC-USD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,

    /// Instrument ID, e.g. BTC-USDT-SWAP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,

    /// Currency, only applicable to OPTION
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,

    /// Tier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}

/// Position tier information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionTier {
    /// Base currency
    pub base_ccy: Option<String>,

    /// Instrument family
    pub inst_family: String,

    /// Instrument ID
    pub inst_id: String,

    /// Maximum leverage
    pub max_lever: String,

    /// Maximum position size
    pub max_sz: String,

    /// Minimum position size
    pub min_sz: String,

    /// Opt margin rate
    pub opt_mgn_factor: Option<String>,

    /// Quote currency
    pub quote_ccy: Option<String>,

    /// Tier
    pub tier: String,

    /// Underlying
    pub uly: String,

    /// Initial margin requirement
    pub imr: String,

    /// Maintenance margin requirement
    pub mmr: String,
}

impl RestClient {
    /// Get position tiers
    ///
    /// # Arguments
    /// * `request` - The get position tiers request
    ///
    /// # Returns
    /// A result containing the position tiers or an error
    pub async fn get_position_tiers(
        &self,
        request: &GetPositionTiersRequest,
    ) -> RestResult<OkxApiResponse<PositionTier>> {
        self.send_request(
            ACCOUNT_POSITION_TIERS_ENDPOINT,
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
    fn test_get_position_tiers_request_serialization() {
        let request = GetPositionTiersRequest {
            inst_type: InstrumentType::Swap,
            td_mode: Some("cross".to_string()),
            uly: Some("BTC-USD".to_string()),
            inst_family: None,
            inst_id: None,
            ccy: None,
            tier: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("instType=SWAP"));
        assert!(serialized.contains("tdMode=cross"));
        assert!(serialized.contains("uly=BTC-USD"));
    }

    #[test]
    fn test_position_tier_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "baseCcy": "BTC",
                    "instFamily": "BTC-USD",
                    "instId": "BTC-USD-SWAP",
                    "maxLever": "125",
                    "maxSz": "1000",
                    "minSz": "1",
                    "optMgnFactor": "0.1",
                    "quoteCcy": "USD",
                    "tier": "1",
                    "uly": "BTC-USD",
                    "imr": "0.008",
                    "mmr": "0.005"
                }
            ]
        }"#;

        let response: OkxApiResponse<PositionTier> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let tier = &response.data[0];
        assert_eq!(tier.inst_family, "BTC-USD");
        assert_eq!(tier.inst_id, "BTC-USD-SWAP");
        assert_eq!(tier.max_lever, "125");
        assert_eq!(tier.tier, "1");
        assert_eq!(tier.imr, "0.008");
        assert_eq!(tier.mmr, "0.005");
    }
}
