use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

// API endpoints
const INTEREST_RATE_INDEX_ENDPOINT: &str = "/api/v1/interest/query";
const PREMIUM_INDEX_ENDPOINT: &str = "/api/v1/premium/query";

/// Get interest rate index request
#[derive(Debug, Clone, Serialize)]
pub struct GetInterestRateIndexRequest {
    /// Symbol of the contract (e.g., .XBTINT8H, .USDTINT8H, .XBTINT, .USDTINT)
    pub symbol: String,

    /// Start time (milliseconds)
    #[serde(rename = "startAt", skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i64>,

    /// End time (milliseconds)
    #[serde(rename = "endAt", skip_serializing_if = "Option::is_none")]
    pub end_at: Option<i64>,

    /// Whether to reverse the results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse: Option<bool>,

    /// Start offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    /// Whether to search forward
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward: Option<bool>,

    /// Max record count (default: 10, max: 100)
    #[serde(rename = "maxCount", skip_serializing_if = "Option::is_none")]
    pub max_count: Option<i64>,
}

/// Interest rate index item
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterestRateIndexItem {
    /// Symbol of the contract
    pub symbol: String,

    /// Granularity (milliseconds)
    pub granularity: i64,

    /// Timestamp (milliseconds)
    pub time_point: i64,

    /// Interest rate value
    pub value: f64,
}

/// Interest rate index response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterestRateIndexResponse {
    /// List of interest rate index data
    pub data_list: Vec<InterestRateIndexItem>,

    /// Whether there are more pages
    pub has_more: bool,
}

/// Get premium index request
#[derive(Debug, Clone, Serialize)]
pub struct GetPremiumIndexRequest {
    /// Symbol of the contract (e.g., .XBTUSDTMPI, .XBTUSDTMPI8H)
    pub symbol: String,

    /// Start time (milliseconds)
    #[serde(rename = "startAt", skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i64>,

    /// End time (milliseconds)
    #[serde(rename = "endAt", skip_serializing_if = "Option::is_none")]
    pub end_at: Option<i64>,

    /// Whether to reverse the results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse: Option<bool>,

    /// Start offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    /// Whether to search forward
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward: Option<bool>,

    /// Max record count (default: 10, max: 100)
    #[serde(rename = "maxCount", skip_serializing_if = "Option::is_none")]
    pub max_count: Option<i64>,
}

/// Premium index item
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PremiumIndexItem {
    /// Symbol of the contract
    pub symbol: String,

    /// Granularity (milliseconds)
    pub granularity: i64,

    /// Timestamp (milliseconds)
    pub time_point: i64,

    /// Premium index value
    pub value: f64,
}

/// Premium index response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PremiumIndexResponse {
    /// List of premium index data
    pub data_list: Vec<PremiumIndexItem>,

    /// Whether there are more pages
    pub has_more: bool,
}

impl super::RestClient {
    /// Get interest rate index data
    ///
    /// [docs](https://www.kucoin.com/docs-new/rest/futures-trading/market-data/get-interest-rate-index)
    pub async fn get_interest_rate_index(
        &self,
        request: GetInterestRateIndexRequest,
    ) -> Result<(RestResponse<InterestRateIndexResponse>, ResponseHeaders)> {
        self.send_request(INTEREST_RATE_INDEX_ENDPOINT, Some(&request))
            .await
    }

    /// Get premium index data
    ///
    /// [docs](https://www.kucoin.com/docs-new/rest/futures-trading/market-data/get-premium-index)
    pub async fn get_premium_index(
        &self,
        request: GetPremiumIndexRequest,
    ) -> Result<(RestResponse<PremiumIndexResponse>, ResponseHeaders)> {
        self.send_request(PREMIUM_INDEX_ENDPOINT, Some(&request))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interest_rate_index_request() {
        let request = GetInterestRateIndexRequest {
            symbol: ".XBTINT8H,.USDTINT8H".to_string(),
            start_at: Some(1634567890000),
            end_at: Some(1634654290000),
            reverse: Some(true),
            offset: None,
            forward: Some(true),
            max_count: Some(50),
        };

        assert_eq!(request.symbol, ".XBTINT8H,.USDTINT8H");
        assert_eq!(request.start_at, Some(1634567890000));
        assert_eq!(request.max_count, Some(50));
    }

    #[test]
    fn test_interest_rate_index_response_deserialization() {
        let json = r#"{
            "dataList": [
                {
                    "symbol": ".XBTINT8H",
                    "granularity": 3600000,
                    "timePoint": 1634567890000,
                    "value": 0.0001
                },
                {
                    "symbol": ".XBTINT8H",
                    "granularity": 3600000,
                    "timePoint": 1634571490000,
                    "value": 0.00015
                }
            ],
            "hasMore": true
        }"#;

        let response: InterestRateIndexResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.data_list.len(), 2);
        assert_eq!(response.data_list[0].symbol, ".XBTINT8H");
        assert_eq!(response.data_list[0].value, 0.0001);
        assert!(response.has_more);
    }

    #[test]
    fn test_premium_index_request() {
        let request = GetPremiumIndexRequest {
            symbol: ".XBTUSDTMPI,.XBTUSDTMPI8H".to_string(),
            start_at: None,
            end_at: None,
            reverse: None,
            offset: None,
            forward: None,
            max_count: None,
        };

        assert_eq!(request.symbol, ".XBTUSDTMPI,.XBTUSDTMPI8H");
    }

    #[test]
    fn test_premium_index_response_deserialization() {
        let json = r#"{
            "dataList": [
                {
                    "symbol": ".XBTUSDTMPI",
                    "granularity": 60000,
                    "timePoint": 1634567890000,
                    "value": 0.0002
                },
                {
                    "symbol": ".XBTUSDTMPI",
                    "granularity": 60000,
                    "timePoint": 1634567950000,
                    "value": 0.00025
                }
            ],
            "hasMore": false
        }"#;

        let response: PremiumIndexResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.data_list.len(), 2);
        assert_eq!(response.data_list[0].symbol, ".XBTUSDTMPI");
        assert_eq!(response.data_list[0].value, 0.0002);
        assert!(!response.has_more);
    }
}
