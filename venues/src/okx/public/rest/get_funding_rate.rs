use serde::{Deserialize, Serialize};
use super::client::RestClient;
use crate::okx::{EndpointType, RestResult};

/// Request parameters for getting funding rate
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateRequest {
    /// Instrument ID (e.g., "BTC-USD-SWAP" or "ANY" to return all swap symbols)
    #[serde(rename = "instId")]
    pub inst_id: String,
}

/// Response for getting funding rate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFundingRateResponse {
    /// Response code ("0" for success)
    pub code: String,
    /// Response message
    pub msg: String,
    /// Funding rate data
    pub data: Vec<FundingRateData>,
}

/// Funding rate data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundingRateData {
    /// Instrument type
    #[serde(rename = "instType")]
    pub inst_type: String,
    /// Instrument ID
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// Funding rate mechanism
    pub method: String,
    /// Formula type
    #[serde(rename = "formulaType")]
    pub formula_type: String,
    /// Current funding rate
    #[serde(rename = "fundingRate")]
    pub funding_rate: String,
    /// Forecasted funding rate for the next period
    #[serde(rename = "nextFundingRate")]
    pub next_funding_rate: String,
    /// Settlement time, Unix timestamp format in milliseconds
    #[serde(rename = "fundingTime")]
    pub funding_time: String,
    /// Forecasted funding time for the next period
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
    /// Settlement state of funding rate
    #[serde(rename = "settState")]
    pub sett_state: String,
    /// If settState = processing, it is the funding rate that is being used for current settlement cycle
    #[serde(rename = "settFundingRate")]
    pub sett_funding_rate: String,
    /// Premium index
    pub premium: String,
    /// Data return time, Unix timestamp format in milliseconds
    pub ts: String,
}

impl RestClient {
    /// Get funding rate
    ///
    /// Retrieve funding rate.
    ///
    /// See: https://www.okx.com/docs-v5/en/#rest-api-public-data-get-funding-rate
    ///
    /// Rate limit: 10 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The funding rate request parameters
    ///
    /// # Returns
    /// Response containing the funding rate data
    pub async fn get_funding_rate(
        &self,
        request: GetFundingRateRequest,
    ) -> RestResult<GetFundingRateResponse> {
        self.send_request(
            "api/v5/public/funding-rate",
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
    fn test_get_funding_rate_request_structure() {
        let request = GetFundingRateRequest {
            inst_id: "BTC-USD-SWAP".to_string(),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("instId").and_then(|v| v.as_str()), Some("BTC-USD-SWAP"));
    }

    #[test]
    fn test_funding_rate_data_structure() {
        let funding_rate_json = json!({
            "instType": "SWAP",
            "instId": "BTC-USD-SWAP",
            "method": "current_period",
            "formulaType": "withRate",
            "fundingRate": "0.0001",
            "nextFundingRate": "0.00015",
            "fundingTime": "1597026383085",
            "nextFundingTime": "1597055183085",
            "minFundingRate": "-0.005",
            "maxFundingRate": "0.005",
            "interestRate": "0.0001",
            "impactValue": "1000",
            "settState": "settled",
            "settFundingRate": "0.0001",
            "premium": "0.0002",
            "ts": "1597026383085"
        });

        let funding_rate_data: FundingRateData = serde_json::from_value(funding_rate_json).unwrap();
        assert_eq!(funding_rate_data.inst_type, "SWAP");
        assert_eq!(funding_rate_data.inst_id, "BTC-USD-SWAP");
        assert_eq!(funding_rate_data.funding_rate, "0.0001");
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
                    "nextFundingRate": "0.00015",
                    "fundingTime": "1597026383085",
                    "nextFundingTime": "1597055183085",
                    "minFundingRate": "-0.005",
                    "maxFundingRate": "0.005",
                    "interestRate": "0.0001",
                    "impactValue": "1000",
                    "settState": "settled",
                    "settFundingRate": "0.0001",
                    "premium": "0.0002",
                    "ts": "1597026383085"
                }
            ]
        });

        let response: GetFundingRateResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data.first().unwrap().inst_id, "BTC-USD-SWAP");
    }
}