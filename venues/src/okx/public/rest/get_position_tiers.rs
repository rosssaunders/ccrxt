use super::client::RestClient;
use crate::okx::{EndpointType, InstrumentType, RestResult};
use serde::{Deserialize, Serialize};

/// Request parameters for getting position tiers
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionTiersRequest {
    /// Instrument type (required)
    #[serde(rename = "instType")]
    pub inst_type: InstrumentType,
    
    /// Trade mode (required)
    /// Margin mode: "cross", "isolated"
    #[serde(rename = "tdMode")]
    pub td_mode: String,
    
    /// Single underlying or multiple underlyings (no more than 3) separated with comma
    /// If instType is SWAP/FUTURES/OPTION, either uly or instFamily is required
    /// If both are passed, instFamily will be used
    #[serde(rename = "uly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying: Option<String>,
    
    /// Single instrument family or multiple instrument families (no more than 5) separated with comma
    /// If instType is SWAP/FUTURES/OPTION, either uly or instFamily is required
    /// If both are passed, instFamily will be used
    #[serde(rename = "instFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    
    /// Single instrument or multiple instruments (no more than 5) separated with comma
    /// Either instId or ccy is required, if both are passed, instId will be used
    /// Ignore when instType is one of SWAP, FUTURES, OPTION
    #[serde(rename = "instId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    
    /// Margin currency
    /// Only applicable to cross MARGIN. It will return borrowing amount for 
    /// Multi-currency margin and Portfolio margin when ccy takes effect
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    
    /// Tiers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}

/// Individual position tier details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionTier {
    /// Underlying
    /// Applicable to FUTURES/SWAP/OPTION
    #[serde(rename = "uly")]
    pub underlying: Option<String>,
    
    /// Instrument family
    /// Applicable to FUTURES/SWAP/OPTION
    #[serde(rename = "instFamily")]
    pub inst_family: Option<String>,
    
    /// Instrument ID
    #[serde(rename = "instId")]
    pub inst_id: String,
    
    /// Tiers
    pub tier: String,
    
    /// The minimum borrowing amount or position of this gear
    /// Only applicable to margin/options/perpetual/delivery, the minimum position is 0 by default
    /// It will return the minimum borrowing amount when ccy takes effect
    #[serde(rename = "minSz")]
    pub min_sz: String,
    
    /// The maximum borrowing amount or number of positions held in this position
    /// Only applicable to margin/options/perpetual/delivery
    /// It will return the maximum borrowing amount when ccy takes effect
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
    #[serde(rename = "optMgnFactor")]
    pub opt_mgn_factor: Option<String>,
    
    /// Quote currency borrowing amount (only applicable to leverage and the case when instId takes effect)
    #[serde(rename = "quoteMaxLoan")]
    pub quote_max_loan: Option<String>,
    
    /// Base currency borrowing amount (only applicable to leverage and the case when instId takes effect)
    #[serde(rename = "baseMaxLoan")]
    pub base_max_loan: Option<String>,
}

/// Response for getting position tiers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPositionTiersResponse {
    /// Response code ("0" for success)
    pub code: String,
    /// Response message
    pub msg: String,
    /// Position tier data
    pub data: Vec<PositionTier>,
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
    /// Response containing the position tiers information
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
            tier: None,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instType").and_then(|v| v.as_str()),
            Some("SWAP")
        );
        assert_eq!(
            serialized.get("tdMode").and_then(|v| v.as_str()),
            Some("cross")
        );
        assert_eq!(
            serialized.get("uly").and_then(|v| v.as_str()),
            Some("BTC-USD")
        );
    }

    #[test]
    fn test_get_position_tiers_request_margin() {
        let request = GetPositionTiersRequest {
            inst_type: InstrumentType::Margin,
            td_mode: "isolated".to_string(),
            underlying: None,
            inst_family: None,
            inst_id: Some("BTC-USDT".to_string()),
            ccy: None,
            tier: None,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instType").and_then(|v| v.as_str()),
            Some("MARGIN")
        );
        assert_eq!(
            serialized.get("tdMode").and_then(|v| v.as_str()),
            Some("isolated")
        );
        assert_eq!(
            serialized.get("instId").and_then(|v| v.as_str()),
            Some("BTC-USDT")
        );
        // Should not serialize None values
        assert!(serialized.get("uly").is_none());
        assert!(serialized.get("instFamily").is_none());
        assert!(serialized.get("ccy").is_none());
        assert!(serialized.get("tier").is_none());
    }

    #[test]
    fn test_position_tier_structure() {
        let tier_json = json!({
            "uly": "BTC-USD",
            "instFamily": "BTC-USD",
            "instId": "BTC-USD-SWAP",
            "tier": "1",
            "minSz": "1",
            "maxSz": "100",
            "mmr": "0.01",
            "imr": "0.02",
            "maxLever": "125",
            "optMgnFactor": "",
            "quoteMaxLoan": "10000",
            "baseMaxLoan": "10"
        });

        let tier: PositionTier = serde_json::from_value(tier_json).unwrap();
        assert_eq!(tier.underlying, Some("BTC-USD".to_string()));
        assert_eq!(tier.inst_family, Some("BTC-USD".to_string()));
        assert_eq!(tier.inst_id, "BTC-USD-SWAP");
        assert_eq!(tier.tier, "1");
        assert_eq!(tier.min_sz, "1");
        assert_eq!(tier.max_sz, "100");
        assert_eq!(tier.mmr, "0.01");
        assert_eq!(tier.imr, "0.02");
        assert_eq!(tier.max_lever, "125");
        assert_eq!(tier.quote_max_loan, Some("10000".to_string()));
        assert_eq!(tier.base_max_loan, Some("10".to_string()));
    }

    #[test]
    fn test_position_tier_minimal_structure() {
        let tier_json = json!({
            "instId": "BTC-USDT",
            "tier": "1",
            "minSz": "0.001",
            "maxSz": "5",
            "mmr": "0.1",
            "imr": "0.15",
            "maxLever": "10"
        });

        let tier: PositionTier = serde_json::from_value(tier_json).unwrap();
        assert_eq!(tier.underlying, None);
        assert_eq!(tier.inst_family, None);
        assert_eq!(tier.inst_id, "BTC-USDT");
        assert_eq!(tier.tier, "1");
        assert_eq!(tier.min_sz, "0.001");
        assert_eq!(tier.max_sz, "5");
        assert_eq!(tier.mmr, "0.1");
        assert_eq!(tier.imr, "0.15");
        assert_eq!(tier.max_lever, "10");
        assert_eq!(tier.opt_mgn_factor, None);
        assert_eq!(tier.quote_max_loan, None);
        assert_eq!(tier.base_max_loan, None);
    }

    #[test]
    fn test_get_position_tiers_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "uly": "BTC-USD",
                    "instFamily": "BTC-USD",
                    "instId": "BTC-USD-SWAP",
                    "tier": "1",
                    "minSz": "1",
                    "maxSz": "100",
                    "mmr": "0.01",
                    "imr": "0.02",
                    "maxLever": "125"
                },
                {
                    "uly": "BTC-USD",
                    "instFamily": "BTC-USD",
                    "instId": "BTC-USD-SWAP",
                    "tier": "2",
                    "minSz": "101",
                    "maxSz": "500",
                    "mmr": "0.015",
                    "imr": "0.025",
                    "maxLever": "100"
                }
            ]
        });

        let response: GetPositionTiersResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 2);
        assert_eq!(response.data.first().unwrap().inst_id, "BTC-USD-SWAP");
        assert_eq!(response.data.first().unwrap().tier, "1");
        assert_eq!(response.data.get(1).unwrap().tier, "2");
    }

    #[test]
    fn test_position_tiers_serialization_roundtrip() {
        let original = GetPositionTiersRequest {
            inst_type: InstrumentType::Futures,
            td_mode: "cross".to_string(),
            underlying: None,
            inst_family: Some("BTC-USD".to_string()),
            inst_id: None,
            ccy: None,
            tier: Some("1".to_string()),
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: GetPositionTiersRequest = serde_json::from_value(serialized).unwrap();

        assert_eq!(original.inst_type, deserialized.inst_type);
        assert_eq!(original.td_mode, deserialized.td_mode);
        assert_eq!(original.underlying, deserialized.underlying);
        assert_eq!(original.inst_family, deserialized.inst_family);
        assert_eq!(original.inst_id, deserialized.inst_id);
        assert_eq!(original.ccy, deserialized.ccy);
        assert_eq!(original.tier, deserialized.tier);
    }

    #[test]
    fn test_position_tiers_with_all_fields() {
        let request = GetPositionTiersRequest {
            inst_type: InstrumentType::Option,
            td_mode: "isolated".to_string(),
            underlying: Some("BTC-USD,ETH-USD".to_string()),
            inst_family: Some("BTC-USD,ETH-USD".to_string()),
            inst_id: Some("BTC-USDT,ETH-USDT".to_string()),
            ccy: Some("USDT".to_string()),
            tier: Some("1,2,3".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instType").and_then(|v| v.as_str()),
            Some("OPTION")
        );
        assert_eq!(
            serialized.get("tdMode").and_then(|v| v.as_str()),
            Some("isolated")
        );
        assert_eq!(
            serialized.get("uly").and_then(|v| v.as_str()),
            Some("BTC-USD,ETH-USD")
        );
        assert_eq!(
            serialized.get("instFamily").and_then(|v| v.as_str()),
            Some("BTC-USD,ETH-USD")
        );
        assert_eq!(
            serialized.get("instId").and_then(|v| v.as_str()),
            Some("BTC-USDT,ETH-USDT")
        );
        assert_eq!(
            serialized.get("ccy").and_then(|v| v.as_str()),
            Some("USDT")
        );
        assert_eq!(
            serialized.get("tier").and_then(|v| v.as_str()),
            Some("1,2,3")
        );
    }
}