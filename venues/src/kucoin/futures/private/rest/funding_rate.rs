use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

// API endpoints
const FUNDING_RATE_HISTORY_ENDPOINT_PREFIX: &str = "/api/v1/funding-rate/";
const FUNDING_RATE_HISTORY_ENDPOINT_SUFFIX: &str = "/history";

/// Get funding rate history request
#[derive(Debug, Clone, Serialize)]
pub struct GetFundingRateHistoryRequest {
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward: Option<bool>,
    #[serde(rename = "maxCount", skip_serializing_if = "Option::is_none")]
    pub max_count: Option<i32>,
}

/// Funding rate history item
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingRateHistoryItem {
    /// Symbol of the contract
    pub symbol: String,
    /// Granularity (funding rate interval in milliseconds)
    pub granularity: i64,
    /// Time point (milliseconds)
    pub time_point: i64,
    /// Funding rate
    pub value: f64,
}

/// Response for funding rate history
pub type GetFundingRateHistoryResponse = Vec<FundingRateHistoryItem>;

impl super::RestClient {
    /// Get funding rate history for a specific symbol
    ///
    /// <https://www.kucoin.com/docs-new/rest/futures-trading/funding-fees/get-private-funding-history>
    pub async fn get_funding_rate_history(
        &self,
        request: GetFundingRateHistoryRequest,
    ) -> Result<(RestResponse<GetFundingRateHistoryResponse>, ResponseHeaders)> {
        let endpoint = format!(
            "{}{}{}",
            FUNDING_RATE_HISTORY_ENDPOINT_PREFIX,
            request.symbol,
            FUNDING_RATE_HISTORY_ENDPOINT_SUFFIX
        );
        self.send_request(&endpoint, Some(&request)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_funding_rate_history_item_deserialization() {
        let json = r#"{
            "symbol": "XBTUSDTM",
            "granularity": 28800000,
            "timePoint": 1700000000000,
            "value": 0.0001
        }"#;

        let item: FundingRateHistoryItem = serde_json::from_str(json).unwrap();
        assert_eq!(item.symbol, "XBTUSDTM");
        assert_eq!(item.granularity, 28800000);
        assert_eq!(item.time_point, 1700000000000);
        assert_eq!(item.value, 0.0001);
    }

    #[test]
    fn test_get_funding_rate_history_request_serialization() {
        let request = GetFundingRateHistoryRequest {
            symbol: "XBTUSDTM".to_string(),
            from: Some(1700000000000),
            to: Some(1700100000000),
            offset: None,
            forward: None,
            max_count: Some(100),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"symbol\":\"XBTUSDTM\""));
        assert!(json.contains("\"from\":1700000000000"));
        assert!(json.contains("\"to\":1700100000000"));
        assert!(json.contains("\"maxCount\":100"));
        assert!(!json.contains("\"offset\""));
        assert!(!json.contains("\"forward\""));
    }
}
