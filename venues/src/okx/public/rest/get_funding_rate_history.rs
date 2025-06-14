use serde::{Deserialize, Serialize};
use super::client::RestClient;
use crate::okx::{EndpointType, RestResult};

/// Request parameters for getting funding rate history
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateHistoryRequest {
    /// Instrument ID (e.g., "BTC-USD-SWAP")
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// Pagination of data to return records newer than the requested fundingTime
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Pagination of data to return records earlier than the requested fundingTime
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of results per request. The maximum is 100; The default is 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Response for getting funding rate history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFundingRateHistoryResponse {
    /// Response code ("0" for success)
    pub code: String,
    /// Response message
    pub msg: String,
    /// Funding rate history data
    pub data: Vec<FundingRateHistoryData>,
}

/// Funding rate history data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundingRateHistoryData {
    /// Instrument type
    #[serde(rename = "instType")]
    pub inst_type: String,
    /// Instrument ID
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// Formula type
    #[serde(rename = "formulaType")]
    pub formula_type: String,
    /// Predicted funding rate
    #[serde(rename = "fundingRate")]
    pub funding_rate: String,
    /// Actual funding rate
    #[serde(rename = "realizedRate")]
    pub realized_rate: String,
    /// Settlement time, Unix timestamp format in milliseconds
    #[serde(rename = "fundingTime")]
    pub funding_time: String,
    /// Funding rate mechanism
    pub method: String,
}

impl RestClient {
    /// Get funding rate history
    ///
    /// Retrieve funding rate history. This endpoint can retrieve data from the last 3 months.
    ///
    /// See: https://www.okx.com/docs-v5/en/#rest-api-public-data-get-funding-rate-history
    ///
    /// Rate limit: 10 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The funding rate history request parameters
    ///
    /// # Returns
    /// Response containing the funding rate history data
    pub async fn get_funding_rate_history(
        &self,
        request: GetFundingRateHistoryRequest,
    ) -> RestResult<GetFundingRateHistoryResponse> {
        self.send_request(
            "api/v5/public/funding-rate-history",
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
    fn test_get_funding_rate_history_request_structure() {
        let request = GetFundingRateHistoryRequest {
            inst_id: "BTC-USD-SWAP".to_string(),
            before: Some("1597026383085".to_string()),
            after: None,
            limit: Some("100".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("instId").and_then(|v| v.as_str()), Some("BTC-USD-SWAP"));
        assert_eq!(serialized.get("before").and_then(|v| v.as_str()), Some("1597026383085"));
        assert_eq!(serialized.get("limit").and_then(|v| v.as_str()), Some("100"));
    }

    #[test]
    fn test_funding_rate_history_data_structure() {
        let funding_rate_history_json = json!({
            "instType": "SWAP",
            "instId": "BTC-USD-SWAP",
            "formulaType": "withRate",
            "fundingRate": "0.0001",
            "realizedRate": "0.00012",
            "fundingTime": "1597026383085",
            "method": "current_period"
        });

        let funding_rate_history_data: FundingRateHistoryData = serde_json::from_value(funding_rate_history_json).unwrap();
        assert_eq!(funding_rate_history_data.inst_type, "SWAP");
        assert_eq!(funding_rate_history_data.inst_id, "BTC-USD-SWAP");
        assert_eq!(funding_rate_history_data.funding_rate, "0.0001");
        assert_eq!(funding_rate_history_data.realized_rate, "0.00012");
    }

    #[test]
    fn test_get_funding_rate_history_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instType": "SWAP",
                    "instId": "BTC-USD-SWAP",
                    "formulaType": "withRate",
                    "fundingRate": "0.0001",
                    "realizedRate": "0.00012",
                    "fundingTime": "1597026383085",
                    "method": "current_period"
                }
            ]
        });

        let response: GetFundingRateHistoryResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data.first().unwrap().inst_id, "BTC-USD-SWAP");
    }
}