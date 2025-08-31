use serde::{Deserialize, Serialize};

use crate::kucoin::futures::{ResponseHeaders, RestResponse, Result, private_client::RestClient};

/// Endpoint URL for funding rate history
const FUNDING_RATE_HISTORY_ENDPOINT: &str = "/api/v1/funding-rate/{symbol}/history";

/// Request parameters for getting funding rate history.
#[derive(Debug, Clone, Serialize)]
pub struct GetFundingRateHistoryRequest {
    /// Trading symbol (e.g., "XBTUSDTM"). Required parameter.
    pub symbol: String,

    /// Start time in milliseconds. Optional parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End time in milliseconds. Optional parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,

    /// Offset for pagination. Optional parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,

    /// Whether to query forward or backward. Optional parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward: Option<bool>,

    /// Maximum number of records to return (1-200). Optional parameter.
    #[serde(rename = "maxCount", skip_serializing_if = "Option::is_none")]
    pub max_count: Option<i32>,
}

/// Funding rate history item representing a single funding rate record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingRateHistoryItem {
    /// Symbol of the contract (e.g., "XBTUSDTM").
    pub symbol: String,

    /// Granularity (funding rate interval in milliseconds).
    pub granularity: i64,

    /// Time point (milliseconds since epoch).
    pub time_point: i64,

    /// Funding rate value (can be positive or negative).
    pub value: f64,
}

/// Response type for funding rate history.
pub type GetFundingRateHistoryResponse = Vec<FundingRateHistoryItem>;

impl RestClient {
    /// Get Private Funding History
    ///
    /// Retrieve the funding history of the current user.
    ///
    /// [docs](https://www.kucoin.com/docs-new/rest/futures-trading/funding-fees/get-private-funding-history)
    ///
    /// Rate limit: 9
    ///
    /// # Arguments
    /// * `request` - The funding rate history request parameters
    ///
    /// # Returns
    /// List of funding rate history items for the specified symbol
    pub async fn get_funding_rate_history(
        &self,
        request: GetFundingRateHistoryRequest,
    ) -> Result<(RestResponse<GetFundingRateHistoryResponse>, ResponseHeaders)> {
        let endpoint = FUNDING_RATE_HISTORY_ENDPOINT.replace("{symbol}", &request.symbol);
        self.get_with_request(&endpoint, &request).await
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

    #[test]
    fn test_funding_rate_history_endpoint_formatting() {
        let endpoint = FUNDING_RATE_HISTORY_ENDPOINT.replace("{symbol}", "XBTUSDTM");
        assert_eq!(endpoint, "/api/v1/funding-rate/XBTUSDTM/history");
    }

    #[test]
    fn test_request_with_all_optional_fields() {
        let request = GetFundingRateHistoryRequest {
            symbol: "ETHUSDTM".to_string(),
            from: Some(1700000000000),
            to: Some(1700100000000),
            offset: Some(10),
            forward: Some(true),
            max_count: Some(50),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "ETHUSDTM");
        assert_eq!(json["from"], 1700000000000i64);
        assert_eq!(json["to"], 1700100000000i64);
        assert_eq!(json["offset"], 10);
        assert_eq!(json["forward"], true);
        assert_eq!(json["maxCount"], 50);
    }

    #[test]
    fn test_request_minimal_fields() {
        let request = GetFundingRateHistoryRequest {
            symbol: "ADAUSDTM".to_string(),
            from: None,
            to: None,
            offset: None,
            forward: None,
            max_count: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "ADAUSDTM");

        // Optional fields should not be present
        assert!(json.get("from").is_none());
        assert!(json.get("to").is_none());
        assert!(json.get("offset").is_none());
        assert!(json.get("forward").is_none());
        assert!(json.get("maxCount").is_none());
    }

    #[test]
    fn test_funding_rate_negative_value() {
        let json = r#"{
            "symbol": "DOTUSDTM",
            "granularity": 28800000,
            "timePoint": 1700000000000,
            "value": -0.0001
        }"#;

        let item: FundingRateHistoryItem = serde_json::from_str(json).unwrap();
        assert_eq!(item.value, -0.0001);
    }

    #[test]
    fn test_funding_rate_history_response_deserialization() {
        let json = r#"[
            {
                "symbol": "XBTUSDTM",
                "granularity": 28800000,
                "timePoint": 1700000000000,
                "value": 0.0001
            },
            {
                "symbol": "XBTUSDTM",
                "granularity": 28800000,
                "timePoint": 1700028800000,
                "value": 0.0002
            }
        ]"#;

        let response: GetFundingRateHistoryResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 2);
        assert_eq!(response[0].value, 0.0001);
        assert_eq!(response[1].value, 0.0002);
    }

    #[test]
    fn test_field_types() {
        let request = GetFundingRateHistoryRequest {
            symbol: "XBTUSDTM".to_string(),
            from: Some(1700000000000),
            to: Some(1700100000000),
            offset: Some(10),
            forward: Some(false),
            max_count: Some(200),
        };

        let json = serde_json::to_value(&request).unwrap();

        assert!(json["symbol"].is_string());
        assert!(json["from"].is_number());
        assert!(json["to"].is_number());
        assert!(json["offset"].is_number());
        assert!(json["forward"].is_boolean());
        assert!(json["maxCount"].is_number());
    }

    #[test]
    fn test_max_count_limit() {
        let request = GetFundingRateHistoryRequest {
            symbol: "XBTUSDTM".to_string(),
            from: None,
            to: None,
            offset: None,
            forward: None,
            max_count: Some(200), // Maximum allowed
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["maxCount"], 200);
    }

    #[test]
    fn test_camel_case_conversion() {
        let request = GetFundingRateHistoryRequest {
            symbol: "XBTUSDTM".to_string(),
            from: None,
            to: None,
            offset: None,
            forward: None,
            max_count: Some(100),
        };

        let json = serde_json::to_value(&request).unwrap();

        // Verify camelCase field exists
        assert!(json.get("maxCount").is_some());
        // Verify snake_case field does not exist
        assert!(json.get("max_count").is_none());
    }

    #[test]
    fn test_symbol_variations() {
        let symbols = ["XBTUSDTM", "ETHUSDTM", "ADAUSDTM", "DOTUSDTM"];

        for symbol in symbols.iter() {
            let request = GetFundingRateHistoryRequest {
                symbol: symbol.to_string(),
                from: None,
                to: None,
                offset: None,
                forward: None,
                max_count: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["symbol"], *symbol);
        }
    }

    #[test]
    fn test_endpoint_constant() {
        assert_eq!(
            FUNDING_RATE_HISTORY_ENDPOINT,
            "/api/v1/funding-rate/{symbol}/history"
        );
    }
}
