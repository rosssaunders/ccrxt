use serde::{Deserialize, Serialize};

use crate::kucoin::{ResponseHeaders, RestResponse, Result};

/// Get current funding rate request
#[derive(Debug, Clone, Serialize)]
pub struct GetCurrentFundingRateRequest {
    pub symbol: String,
}

/// Current funding rate response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentFundingRate {
    /// Symbol of the contract
    pub symbol: String,
    /// Granularity (funding rate interval in milliseconds)
    pub granularity: i64,
    /// Time point (milliseconds)
    pub time_point: i64,
    /// Funding rate
    pub value: f64,
    /// Predicted funding rate
    pub predicted_value: f64,
}

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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    /// Get current funding rate for a specific symbol
    pub async fn get_current_funding_rate(
        &self,
        request: GetCurrentFundingRateRequest,
    ) -> Result<(RestResponse<CurrentFundingRate>, ResponseHeaders)> {
        let endpoint = format!("/api/v1/funding-rate/{}/current", request.symbol);
        self.get(&endpoint, None).await
    }

    /// Get funding rate history for a specific symbol
    pub async fn get_funding_rate_history(
        &self,
        request: GetFundingRateHistoryRequest,
    ) -> Result<(RestResponse<GetFundingRateHistoryResponse>, ResponseHeaders)> {
        let endpoint = format!("/api/v1/funding-rate/{}/history", request.symbol);

        let mut params = std::collections::HashMap::new();

        if let Some(from) = request.from {
            params.insert("from".to_string(), from.to_string());
        }
        if let Some(to) = request.to {
            params.insert("to".to_string(), to.to_string());
        }
        if let Some(offset) = request.offset {
            params.insert("offset".to_string(), offset.to_string());
        }
        if let Some(forward) = request.forward {
            params.insert("forward".to_string(), forward.to_string());
        }
        if let Some(max_count) = request.max_count {
            params.insert("maxCount".to_string(), max_count.to_string());
        }

        let params = if params.is_empty() {
            None
        } else {
            Some(params)
        };

        self.get(&endpoint, params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_current_funding_rate_request_serialization() {
        let request = GetCurrentFundingRateRequest {
            symbol: "XBTUSDTM".to_string(),
        };

        assert_eq!(request.symbol, "XBTUSDTM");
    }

    #[test]
    fn test_current_funding_rate_deserialization() {
        let json = r#"{
            "symbol": "XBTUSDTM",
            "granularity": 28800000,
            "timePoint": 1637049600000,
            "value": 0.000100,
            "predictedValue": 0.000200
        }"#;

        let funding_rate: CurrentFundingRate = serde_json::from_str(json).unwrap();
        assert_eq!(funding_rate.symbol, "XBTUSDTM");
        assert_eq!(funding_rate.granularity, 28800000);
        assert_eq!(funding_rate.value, 0.000100);
        assert_eq!(funding_rate.predicted_value, 0.000200);
    }

    #[test]
    fn test_funding_rate_history_item_deserialization() {
        let json = r#"{
            "symbol": "XBTUSDTM",
            "granularity": 28800000,
            "timePoint": 1637049600000,
            "value": 0.000100
        }"#;

        let item: FundingRateHistoryItem = serde_json::from_str(json).unwrap();
        assert_eq!(item.symbol, "XBTUSDTM");
        assert_eq!(item.granularity, 28800000);
        assert_eq!(item.value, 0.000100);
    }
}
