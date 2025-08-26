use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const FUNDING_RATE_ENDPOINT: &str = "/dapi/v1/fundingRate";

/// Request parameters for the funding rate history endpoint.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FundingRateRequest {
    /// Trading symbol (e.g., "BTCUSD_PERP").
    pub symbol: String,

    /// Timestamp in ms to get funding rate from INCLUSIVE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// Timestamp in ms to get funding rate until INCLUSIVE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Default 100; max 1000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Represents a single funding rate entry.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingRate {
    /// Trading symbol.
    pub symbol: String,

    /// Funding time (timestamp in milliseconds).
    pub funding_time: u64,

    /// Funding rate as a decimal string.
    pub funding_rate: String,
}

/// Response from the funding rate history endpoint.
pub type FundingRateResponse = Vec<FundingRate>;

impl RestClient {
    /// Get Funding Rate History of Perpetual Futures
    ///
    /// Empty array will be returned for delivery symbols.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Get-Funding-Rate-History-of-Perpetual-Futures)
    ///
    /// Rate limit: 1
    ///
    /// # Arguments
    /// * `params` - The funding rate request parameters
    ///
    /// # Returns
    /// Vector of funding rate entries for the specified symbol
    pub async fn get_funding_rate_history(
        &self,
        params: FundingRateRequest,
    ) -> RestResult<FundingRateResponse> {
        self.send_get_request(FUNDING_RATE_ENDPOINT, Some(params), 1)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_funding_rate_request_serialization() {
        let request = FundingRateRequest {
            symbol: "BTCUSD_PERP".to_string(),
            start_time: Some(1625097600000),
            end_time: Some(1625184000000),
            limit: Some(100),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSD_PERP"));
        assert!(serialized.contains("startTime=1625097600000"));
        assert!(serialized.contains("endTime=1625184000000"));
        assert!(serialized.contains("limit=100"));
    }

    #[test]
    fn test_funding_rate_request_serialization_minimal() {
        let request = FundingRateRequest {
            symbol: "ETHUSD_PERP".to_string(),
            start_time: None,
            end_time: None,
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=ETHUSD_PERP");
    }

    #[test]
    fn test_funding_rate_response_deserialization() {
        let json = r#"[
            {
                "symbol": "BTCUSD_PERP",
                "fundingTime": 1625097600000,
                "fundingRate": "0.00010000"
            },
            {
                "symbol": "BTCUSD_PERP",
                "fundingTime": 1625126400000,
                "fundingRate": "-0.00005000"
            }
        ]"#;

        let funding_rates: FundingRateResponse = serde_json::from_str(json).unwrap();
        assert_eq!(funding_rates.len(), 2);

        let first_rate = &funding_rates[0];
        assert_eq!(first_rate.symbol, "BTCUSD_PERP");
        assert_eq!(first_rate.funding_time, 1625097600000);
        assert_eq!(first_rate.funding_rate, "0.00010000");

        let second_rate = &funding_rates[1];
        assert_eq!(second_rate.symbol, "BTCUSD_PERP");
        assert_eq!(second_rate.funding_time, 1625126400000);
        assert_eq!(second_rate.funding_rate, "-0.00005000");
    }

    #[test]
    fn test_funding_rate_response_empty_array() {
        // Test that empty array (for delivery symbols) deserializes correctly
        let json = r#"[]"#;
        let funding_rates: FundingRateResponse = serde_json::from_str(json).unwrap();
        assert_eq!(funding_rates.len(), 0);
    }

    #[test]
    fn test_funding_rate_negative_rate() {
        let json = r#"[
            {
                "symbol": "ETHUSD_PERP",
                "fundingTime": 1625097600000,
                "fundingRate": "-0.00375000"
            }
        ]"#;

        let funding_rates: FundingRateResponse = serde_json::from_str(json).unwrap();
        assert_eq!(funding_rates.len(), 1);
        assert_eq!(funding_rates[0].funding_rate, "-0.00375000");
    }
}
