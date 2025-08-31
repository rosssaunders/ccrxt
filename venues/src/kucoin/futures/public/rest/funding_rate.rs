use serde::{Deserialize, Serialize};

use crate::kucoin::futures::{ResponseHeaders, RestResponse, Result, public_client::RestClient};

// API endpoints
const CURRENT_FUNDING_RATE_ENDPOINT_PREFIX: &str = "/api/v1/funding-rate/";
const CURRENT_FUNDING_RATE_ENDPOINT_SUFFIX: &str = "/current";

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
    pub predicted_value: Option<f64>,
}

impl RestClient {
    /// Get current funding rate for a specific symbol
    ///
    /// [docs](https://www.kucoin.com/docs-new/rest/futures-trading/funding-fees/get-current-funding-rate)
    pub async fn get_current_funding_rate(
        &self,
        request: GetCurrentFundingRateRequest,
    ) -> Result<(RestResponse<CurrentFundingRate>, ResponseHeaders)> {
        let endpoint = format!(
            "{}{}{}",
            CURRENT_FUNDING_RATE_ENDPOINT_PREFIX,
            request.symbol,
            CURRENT_FUNDING_RATE_ENDPOINT_SUFFIX
        );
        self.get(&endpoint, None).await
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
            "predictedValue": null
        }"#;

        let funding_rate: CurrentFundingRate = serde_json::from_str(json).unwrap();
        assert_eq!(funding_rate.symbol, "XBTUSDTM");
        assert_eq!(funding_rate.granularity, 28800000);
        assert_eq!(funding_rate.value, 0.000100);
        assert_eq!(funding_rate.predicted_value, None);
    }
}
