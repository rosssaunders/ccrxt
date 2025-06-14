use serde::{Deserialize, Serialize};
use super::client::RestClient;
use crate::okx::{EndpointType, InstrumentType, RestResult};

/// Request parameters for getting position tiers
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionTiersRequest {
    /// Instrument type
    #[serde(rename = "instType")]
    pub inst_type: InstrumentType,
    /// Trade mode (cross/isolated)
    #[serde(rename = "tdMode")]
    pub td_mode: String,
    /// Single underlying or multiple underlyings (no more than 3) separated with comma
    #[serde(rename = "uly", skip_serializing_if = "Option::is_none")]
    pub underlying: Option<String>,
    /// Single instrument family or multiple instrument families (no more than 5) separated with comma
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    /// Single instrument or multiple instruments (no more than 5) separated with comma
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    /// Margin currency (only applicable to cross MARGIN)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// Tiers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}

/// Response for getting position tiers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPositionTiersResponse {
    /// Response code ("0" for success)
    pub code: String,
    /// Response message
    pub msg: String,
    /// Position tiers data
    pub data: Vec<PositionTierData>,
}

/// Position tier data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionTierData {
    /// Underlying (applicable to FUTURES/SWAP/OPTION)
    #[serde(rename = "uly", skip_serializing_if = "Option::is_none")]
    pub underlying: Option<String>,
    /// Instrument family (applicable to FUTURES/SWAP/OPTION)
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    /// Instrument ID
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    /// Tiers
    pub tier: String,
    /// The minimum borrowing amount or position
    #[serde(rename = "minSz")]
    pub min_sz: String,
    /// The maximum borrowing amount or number of positions held in this position
    #[serde(rename = "maxSz")]
    pub max_sz: String,
    /// Position maintenance margin requirement rate
    pub mmr: String,
    /// Initial margin requirement rate
    pub imr: String,
    /// Maximum available leverage
    #[serde(rename = "maxLever")]
    pub max_lever: String,
    /// Option Margin Coefficient (only applicable to options)
    #[serde(rename = "optMgnFactor", skip_serializing_if = "Option::is_none")]
    pub opt_mgn_factor: Option<String>,
    /// Quote currency borrowing amount (only applicable to leverage)
    #[serde(rename = "quoteMaxLoan", skip_serializing_if = "Option::is_none")]
    pub quote_max_loan: Option<String>,
    /// Base currency borrowing amount (only applicable to leverage)
    #[serde(rename = "baseMaxLoan", skip_serializing_if = "Option::is_none")]
    pub base_max_loan: Option<String>,
}

impl RestClient {
    /// Get position tiers
    ///
    /// Retrieve position tiers information, maximum leverage depends on your borrowings
    /// and Maintenance margin ratio.
    ///
    /// See: https://www.okx.com/docs-v5/en/#rest-api-public-data-get-position-tiers
    ///
    /// Rate limit: 10 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The position tiers request parameters
    ///
    /// # Returns
    /// Response containing the position tiers data
    pub async fn get_position_tiers(
        &self,
        request: GetPositionTiersRequest,
    ) -> RestResult<GetPositionTiersResponse> {
        self.send_request(
            "api/v5/public/position-tiers",
            reqwest::Method::GET,
            Some(&request),
            EndpointType::PublicMarketData,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_get_position_tiers_request_structure() {
        let request = GetPositionTiersRequest {
            inst_type: InstrumentType::Swap,
            td_mode: "cross".to_string(),
            underlying: Some("BTC-USD".to_string()),
            inst_family: None,
            inst_id: None,
            ccy: None,
            tier: Some("1".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("instType").and_then(|v| v.as_str()), Some("SWAP"));
        assert_eq!(serialized.get("tdMode").and_then(|v| v.as_str()), Some("cross"));
        assert_eq!(serialized.get("uly").and_then(|v| v.as_str()), Some("BTC-USD"));
        assert_eq!(serialized.get("tier").and_then(|v| v.as_str()), Some("1"));
    }

    #[test]
    fn test_position_tier_data_structure() {
        let position_tier_json = json!({
            "uly": "BTC-USD",
            "tier": "1",
            "minSz": "0",
            "maxSz": "100",
            "mmr": "0.01",
            "imr": "0.1",
            "maxLever": "10"
        });

        let position_tier_data: PositionTierData = serde_json::from_value(position_tier_json).unwrap();
        assert_eq!(position_tier_data.underlying, Some("BTC-USD".to_string()));
        assert_eq!(position_tier_data.tier, "1");
        assert_eq!(position_tier_data.min_sz, "0");
        assert_eq!(position_tier_data.max_sz, "100");
        assert_eq!(position_tier_data.max_lever, "10");
    }

    #[test]
    fn test_get_position_tiers_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "uly": "BTC-USD",
                    "tier": "1",
                    "minSz": "0",
                    "maxSz": "100",
                    "mmr": "0.01",
                    "imr": "0.1",
                    "maxLever": "10"
                }
            ]
        });

        let response: GetPositionTiersResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data.first().unwrap().tier, "1");
    }
}