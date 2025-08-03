use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::okx::{EndpointType, RestResult};

const PUBLIC_FUNDING_RATE_HISTORY_ENDPOINT: &str = "api/v5/public/funding-rate-history";

/// Request parameters for getting funding rate history
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateHistoryRequest {
    /// Instrument ID, e.g. "BTC-USD-SWAP". Only applicable to SWAP
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// Pagination of data to return records newer than the requested fundingTime
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Pagination of data to return records earlier than the requested fundingTime
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of results per request. The maximum is 100. The default is 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Individual funding rate history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingRateHistory {
    /// Instrument type (SWAP)
    /// Instrument type, should be "SWAP"
    #[serde(rename = "instType")]
    pub inst_type: String,
    /// Instrument ID, e.g. "BTC-USD-SWAP"
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// Formula type: "noRate" (old funding rate formula) or "withRate" (new funding rate formula)
    #[serde(rename = "formulaType")]
    pub formula_type: String,
    /// Predicted funding rate
    #[serde(rename = "fundingRate")]
    pub funding_rate: String,
    /// Actual funding rate
    #[serde(rename = "realizedRate")]
    pub realized_rate: String,
    /// Settlement time, Unix timestamp format in milliseconds, e.g. "1597026383085"
    #[serde(rename = "fundingTime")]
    pub funding_time: String,
    /// Funding rate mechanism (current_period, next_period)
    pub method: String,
}

/// Response for getting funding rate history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFundingRateHistoryResponse {
    /// Response code ("0" for success)
    pub code: String,
    /// Response message
    pub msg: String,
    /// Funding rate history data
    pub data: Vec<FundingRateHistory>,
}

impl RestClient {
    /// Get funding rate history
    ///
    /// Retrieve funding rate history. This endpoint can retrieve data from the last 3 months.
    ///
    /// [docs]: https://www.okx.com/docs-v5/en/#rest-api-public-rest-api-get-funding-rate-history
    ///
    /// Rate limit: 10 requests per 2 seconds
    /// Rate limit rule: IP + Instrument ID
    ///
    /// # Arguments
    /// * `request` - The funding rate history request parameters
    ///
    /// # Returns
    /// Response containing the list of funding rate history entries
    pub async fn get_funding_rate_history(
        &self,
        request: &GetFundingRateHistoryRequest,
    ) -> RestResult<GetFundingRateHistoryResponse> {
        self.send_request(
            PUBLIC_FUNDING_RATE_HISTORY_ENDPOINT,
            reqwest::Method::GET,
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

    #[test]
    fn test_get_funding_rate_history_request_structure() {
        let request = GetFundingRateHistoryRequest {
            inst_id: "BTC-USD-SWAP".to_string(),
            before: None,
            after: None,
            limit: Some("50".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instId").and_then(|v| v.as_str()),
            Some("BTC-USD-SWAP")
        );
        assert_eq!(serialized.get("limit").and_then(|v| v.as_str()), Some("50"));
        assert!(serialized.get("before").is_none());
        assert!(serialized.get("after").is_none());
    }

    #[test]
    fn test_get_funding_rate_history_request_with_pagination() {
        let request = GetFundingRateHistoryRequest {
            inst_id: "ETH-USD-SWAP".to_string(),
            before: Some("1597026383085".to_string()),
            after: Some("1597026283085".to_string()),
            limit: Some("100".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instId").and_then(|v| v.as_str()),
            Some("ETH-USD-SWAP")
        );
        assert_eq!(
            serialized.get("before").and_then(|v| v.as_str()),
            Some("1597026383085")
        );
        assert_eq!(
            serialized.get("after").and_then(|v| v.as_str()),
            Some("1597026283085")
        );
        assert_eq!(
            serialized.get("limit").and_then(|v| v.as_str()),
            Some("100")
        );
    }

    #[test]
    fn test_funding_rate_history_structure() {
        let funding_rate_history_json = json!({
            "instType": "SWAP",
            "instId": "BTC-USD-SWAP",
            "formulaType": "withRate",
            "realizedRate": "0.00009",
            "fundingRate": "0.000123",
            "fundingTime": "1597026383085",
            "method": "current_period"
        });

        let funding_rate_history: FundingRateHistory =
            serde_json::from_value(funding_rate_history_json).unwrap();
        assert_eq!(funding_rate_history.inst_type, "SWAP");
        assert_eq!(funding_rate_history.inst_id, "BTC-USD-SWAP");
        assert_eq!(funding_rate_history.formula_type, "withRate");
        assert_eq!(funding_rate_history.funding_rate, "0.000123");
        assert_eq!(funding_rate_history.realized_rate, "0.00009");
        assert_eq!(funding_rate_history.funding_time, "1597026383085");
        assert_eq!(funding_rate_history.method, "current_period");
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
                    "realizedRate": "0.00009",
                    "fundingRate": "0.000123",
                    "fundingTime": "1597026383085",
                    "method": "current_period"
                },
                {
                    "instType": "SWAP",
                    "instId": "BTC-USD-SWAP",
                    "formulaType": "noRate",
                    "fundingRate": "0.000789",
                    "realizedRate": "0.000012",
                    "fundingTime": "1597026383086",
                    "method": "next_period"
                }
            ]
        });

        let response: GetFundingRateHistoryResponse =
            serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.msg, "");
        assert_eq!(response.data.len(), 2);

        let first_entry = response.data.first().unwrap();
        assert_eq!(first_entry.inst_type, "SWAP");
        assert_eq!(first_entry.inst_id, "BTC-USD-SWAP");
        assert_eq!(first_entry.formula_type, "withRate");
        assert_eq!(first_entry.funding_rate, "0.000123");
        assert_eq!(first_entry.realized_rate, "0.00009");
        assert_eq!(first_entry.funding_time, "1597026383085");
        assert_eq!(first_entry.method, "current_period");

        let second_entry = response.data.get(1).unwrap();
        assert_eq!(second_entry.formula_type, "noRate");
        assert_eq!(second_entry.funding_rate, "0.000789");
        assert_eq!(second_entry.realized_rate, "0.000012");
        assert_eq!(second_entry.funding_time, "1597026383086");
        assert_eq!(second_entry.method, "next_period");
    }

    #[test]
    fn test_funding_rate_history_serialization_roundtrip() {
        let original = GetFundingRateHistoryRequest {
            inst_id: "SOL-USD-SWAP".to_string(),
            after: Some("1597026383085".to_string()),
            before: None,
            limit: Some("25".to_string()),
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: GetFundingRateHistoryRequest =
            serde_json::from_value(serialized).unwrap();

        assert_eq!(original.inst_id, deserialized.inst_id);
        assert_eq!(original.after, deserialized.after);
        assert_eq!(original.before, deserialized.before);
        assert_eq!(original.limit, deserialized.limit);
    }

    #[test]
    fn test_funding_rate_history_minimal_request() {
        let request = GetFundingRateHistoryRequest {
            inst_id: "BTC-USD-SWAP".to_string(),
            after: None,
            before: None,
            limit: None,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instId").and_then(|v| v.as_str()),
            Some("BTC-USD-SWAP")
        );
        // Optional fields should not be present when None
        assert!(serialized.get("after").is_none());
        assert!(serialized.get("before").is_none());
        assert!(serialized.get("limit").is_none());
    }

    #[test]
    fn test_funding_rate_history_structure_with_different_formula_types() {
        let old_formula_json = json!({
            "instType": "SWAP",
            "instId": "ETH-USD-SWAP",
            "formulaType": "noRate",
            "fundingRate": "0.001000",
            "realizedRate": "0.001100",
            "fundingTime": "1597026383000",
            "method": "current_period"
        });

        let new_formula_json = json!({
            "instType": "SWAP",
            "instId": "ETH-USD-SWAP",
            "formulaType": "withRate",
            "fundingRate": "0.002000",
            "realizedRate": "0.002100",
            "fundingTime": "1597026383000",
            "method": "next_period"
        });

        let old_formula: FundingRateHistory = serde_json::from_value(old_formula_json).unwrap();
        let new_formula: FundingRateHistory = serde_json::from_value(new_formula_json).unwrap();

        assert_eq!(old_formula.formula_type, "noRate");
        assert_eq!(new_formula.formula_type, "withRate");
        assert_eq!(old_formula.method, "current_period");
        assert_eq!(new_formula.method, "next_period");
    }
}
