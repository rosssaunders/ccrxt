use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::okx::{EndpointType, RestResult};

const PUBLIC_FUNDING_RATE_ENDPOINT: &str = "api/v5/public/funding-rate";

/// Request parameters for getting funding rate
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateRequest {
    /// Instrument ID, e.g. "BTC-USD-SWAP" or "ANY" to return the funding rate info of all swap symbols
    /// Only applicable to SWAP
    #[serde(rename = "instId")]
    pub inst_id: String,
}

/// Individual funding rate entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingRate {
    /// Instrument type (SWAP)
    #[serde(rename = "instType")]
    pub inst_type: String,

    /// Instrument ID, e.g. "BTC-USD-SWAP" or "ANY"
    #[serde(rename = "instId")]
    pub inst_id: String,

    /// Funding rate mechanism (current_period, next_period - no longer supported)
    pub method: String,

    /// Formula type (noRate: old funding rate formula, withRate: new funding rate formula)
    #[serde(rename = "formulaType")]
    pub formula_type: String,

    /// Current funding rate
    #[serde(rename = "fundingRate")]
    pub funding_rate: String,

    /// Forecasted funding rate for the next period (no longer supported, will be "")
    #[serde(rename = "nextFundingRate")]
    pub next_funding_rate: String,

    /// Settlement time, Unix timestamp format in milliseconds, e.g. "1597026383085"
    #[serde(rename = "fundingTime")]
    pub funding_time: String,

    /// Forecasted funding time for the next period, Unix timestamp format in milliseconds, e.g. "1597026383085"
    #[serde(rename = "nextFundingTime")]
    pub next_funding_time: String,

    /// The lower limit of the funding rate
    #[serde(rename = "minFundingRate")]
    pub min_funding_rate: String,

    /// The upper limit of the funding rate
    #[serde(rename = "maxFundingRate")]
    pub max_funding_rate: String,

    /// Interest rate
    #[serde(rename = "interestRate")]
    pub interest_rate: String,

    /// Depth weighted amount (in the unit of quote currency)
    #[serde(rename = "impactValue")]
    pub impact_value: String,

    /// Settlement state of funding rate (processing, settled)
    #[serde(rename = "settState")]
    pub sett_state: String,

    /// If settState = processing, it is the funding rate that is being used for current settlement cycle.
    /// If settState = settled, it is the funding rate that is being used for previous settlement cycle
    #[serde(rename = "settFundingRate")]
    pub sett_funding_rate: String,

    /// Premium index. Formula: [(Best bid + Best ask) / 2 – Index price] / Index price
    pub premium: String,

    /// Data return time, Unix timestamp format in milliseconds, e.g. "1597026383085"
    pub ts: String,
}

impl RestClient {
    /// Get funding rate
    ///
    /// Retrieve funding rate.
    ///
    /// [docs]: https://www.okx.com/docs-v5/en/#rest-api-public-rest-api-get-funding-rate
    ///
    /// Rate limit: 10 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The funding rate request parameters
    ///
    /// # Returns
    /// Response containing the funding rate information
    pub async fn get_funding_rate(
        &self,
        request: &GetFundingRateRequest,
    ) -> RestResult<Vec<FundingRate>> {
        self.send_get_request(
            PUBLIC_FUNDING_RATE_ENDPOINT,
            Some(request),
            EndpointType::PublicMarketData,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::okx::response::OkxApiResponse;

    #[test]
    fn test_get_funding_rate_request_structure() {
        let request = GetFundingRateRequest {
            inst_id: "BTC-USD-SWAP".to_string(),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instId").and_then(|v| v.as_str()),
            Some("BTC-USD-SWAP")
        );
    }

    #[test]
    fn test_get_funding_rate_request_any_symbol() {
        let request = GetFundingRateRequest {
            inst_id: "ANY".to_string(),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instId").and_then(|v| v.as_str()),
            Some("ANY")
        );
    }

    #[test]
    fn test_funding_rate_structure() {
        let funding_rate_json = json!({
            "instType": "SWAP",
            "instId": "BTC-USD-SWAP",
            "method": "current_period",
            "formulaType": "withRate",
            "fundingRate": "0.0001",
            "nextFundingRate": "",
            "fundingTime": "1597026383085",
            "nextFundingTime": "1597055183085",
            "minFundingRate": "-0.005",
            "maxFundingRate": "0.005",
            "interestRate": "0.0001",
            "impactValue": "1000",
            "settState": "settled",
            "settFundingRate": "0.0001",
            "premium": "0.000123",
            "ts": "1597026383085"
        });

        let funding_rate: FundingRate = serde_json::from_value(funding_rate_json).unwrap();
        assert_eq!(funding_rate.inst_type, "SWAP");
        assert_eq!(funding_rate.inst_id, "BTC-USD-SWAP");
        assert_eq!(funding_rate.method, "current_period");
        assert_eq!(funding_rate.formula_type, "withRate");
        assert_eq!(funding_rate.funding_rate, "0.0001");
        assert_eq!(funding_rate.next_funding_rate, "");
        assert_eq!(funding_rate.funding_time, "1597026383085");
        assert_eq!(funding_rate.next_funding_time, "1597055183085");
        assert_eq!(funding_rate.min_funding_rate, "-0.005");
        assert_eq!(funding_rate.max_funding_rate, "0.005");
        assert_eq!(funding_rate.interest_rate, "0.0001");
        assert_eq!(funding_rate.impact_value, "1000");
        assert_eq!(funding_rate.sett_state, "settled");
        assert_eq!(funding_rate.sett_funding_rate, "0.0001");
        assert_eq!(funding_rate.premium, "0.000123");
        assert_eq!(funding_rate.ts, "1597026383085");
    }

    #[test]
    fn test_get_funding_rate_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instType": "SWAP",
                    "instId": "BTC-USD-SWAP",
                    "method": "current_period",
                    "formulaType": "withRate",
                    "fundingRate": "0.0001",
                    "nextFundingRate": "",
                    "fundingTime": "1597026383085",
                    "nextFundingTime": "1597055183085",
                    "minFundingRate": "-0.005",
                    "maxFundingRate": "0.005",
                    "interestRate": "0.0001",
                    "impactValue": "1000",
                    "settState": "settled",
                    "settFundingRate": "0.0001",
                    "premium": "0.000123",
                    "ts": "1597026383085"
                }
            ]
        });

        let response: OkxApiResponse<FundingRate> = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.msg, "");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data.first().unwrap().inst_id, "BTC-USD-SWAP");
        assert_eq!(response.data.first().unwrap().funding_rate, "0.0001");
        assert_eq!(response.data.first().unwrap().method, "current_period");
    }

    #[test]
    fn test_funding_rate_serialization_roundtrip() {
        let original = GetFundingRateRequest {
            inst_id: "ETH-USD-SWAP".to_string(),
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: GetFundingRateRequest = serde_json::from_value(serialized).unwrap();

        assert_eq!(original.inst_id, deserialized.inst_id);
    }

    #[test]
    fn test_funding_rate_multiple_instruments() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instType": "SWAP",
                    "instId": "BTC-USD-SWAP",
                    "method": "current_period",
                    "formulaType": "withRate",
                    "fundingRate": "0.0001",
                    "nextFundingRate": "",
                    "fundingTime": "1597026383085",
                    "nextFundingTime": "1597055183085",
                    "minFundingRate": "-0.005",
                    "maxFundingRate": "0.005",
                    "interestRate": "0.0001",
                    "impactValue": "1000",
                    "settState": "settled",
                    "settFundingRate": "0.0001",
                    "premium": "0.000123",
                    "ts": "1597026383085"
                },
                {
                    "instType": "SWAP",
                    "instId": "ETH-USD-SWAP",
                    "method": "current_period",
                    "formulaType": "withRate",
                    "fundingRate": "0.0002",
                    "nextFundingRate": "",
                    "fundingTime": "1597026383085",
                    "nextFundingTime": "1597055183085",
                    "minFundingRate": "-0.005",
                    "maxFundingRate": "0.005",
                    "interestRate": "0.0001",
                    "impactValue": "500",
                    "settState": "processing",
                    "settFundingRate": "0.00015",
                    "premium": "0.000456",
                    "ts": "1597026383085"
                }
            ]
        });

        let response: OkxApiResponse<FundingRate> = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 2);
        assert_eq!(response.data.first().unwrap().inst_id, "BTC-USD-SWAP");
        assert_eq!(response.data.get(1).unwrap().inst_id, "ETH-USD-SWAP");
        assert_eq!(response.data.get(1).unwrap().funding_rate, "0.0002");
        assert_eq!(response.data.get(1).unwrap().sett_state, "processing");
    }
}
