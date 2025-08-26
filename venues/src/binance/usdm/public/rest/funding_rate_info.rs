use std::borrow::Cow;

use serde::Deserialize;

use crate::binance::usdm::PublicRestClient as RestClient;
use crate::binance::usdm::RestResult;
/// Endpoint for Funding Rate Info
const FUNDING_RATE_INFO_ENDPOINT: &str = "/fapi/v1/fundingInfo";

/// Represents a funding rate info record returned by Binance USDM.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingRateInfo {
    /// Trading pair symbol (e.g., "BTCUSDT").
    pub symbol: Cow<'static, str>,

    /// The adjusted funding rate cap as a string (decimal).
    pub adjusted_funding_rate_cap: Cow<'static, str>,

    /// The adjusted funding rate floor as a string (decimal).
    pub adjusted_funding_rate_floor: Cow<'static, str>,

    /// Funding interval in hours.
    pub funding_interval_hours: u32,
    // disclaimer: bool (ignored)
}

impl RestClient {
    /// Get Funding Rate Info
    ///
    /// Retrieves funding rate cap, floor, and interval settings for USDM futures.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Get-Funding-Rate-Info)
    ///
    /// Rate limit: share 500/5min/IP rate limit with GET /fapi/v1/fundingInfo
    ///
    /// # Returns
    /// A vector of `FundingRateInfo` records wrapped in `RestResult`.
    pub async fn get_funding_rate_info(&self) -> RestResult<Vec<FundingRateInfo>> {
        self.send_get_request(FUNDING_RATE_INFO_ENDPOINT, Some(()), 1)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_funding_rate_info_deserialization() {
        let json = r#"[
            {
                "symbol": "BTCUSDT",
                "adjustedFundingRateCap": "0.02",
                "adjustedFundingRateFloor": "-0.02",
                "fundingIntervalHours": 8,
                "disclaimer": false
            },
            {
                "symbol": "ETHUSDT",
                "adjustedFundingRateCap": "0.02",
                "adjustedFundingRateFloor": "-0.02",
                "fundingIntervalHours": 8,
                "disclaimer": false
            }
        ]"#;

        let funding_info: Vec<FundingRateInfo> = serde_json::from_str(json).unwrap();
        assert_eq!(funding_info.len(), 2);

        assert_eq!(funding_info[0].symbol, "BTCUSDT");
        assert_eq!(funding_info[0].adjusted_funding_rate_cap, "0.02");
        assert_eq!(funding_info[0].adjusted_funding_rate_floor, "-0.02");
        assert_eq!(funding_info[0].funding_interval_hours, 8);

        assert_eq!(funding_info[1].symbol, "ETHUSDT");
        assert_eq!(funding_info[1].adjusted_funding_rate_cap, "0.02");
        assert_eq!(funding_info[1].adjusted_funding_rate_floor, "-0.02");
        assert_eq!(funding_info[1].funding_interval_hours, 8);
    }

    #[test]
    fn test_funding_rate_info_different_intervals() {
        let json = r#"[
            {
                "symbol": "BTCUSDT",
                "adjustedFundingRateCap": "0.02",
                "adjustedFundingRateFloor": "-0.02",
                "fundingIntervalHours": 8
            },
            {
                "symbol": "SPECIAL",
                "adjustedFundingRateCap": "0.03",
                "adjustedFundingRateFloor": "-0.03",
                "fundingIntervalHours": 4
            }
        ]"#;

        let funding_info: Vec<FundingRateInfo> = serde_json::from_str(json).unwrap();
        assert_eq!(funding_info.len(), 2);
        assert_eq!(funding_info[0].funding_interval_hours, 8);
        assert_eq!(funding_info[1].funding_interval_hours, 4);
    }

    #[test]
    fn test_funding_rate_info_extreme_caps() {
        let json = r#"[
            {
                "symbol": "VOLATILE",
                "adjustedFundingRateCap": "0.10",
                "adjustedFundingRateFloor": "-0.10",
                "fundingIntervalHours": 8
            }
        ]"#;

        let funding_info: Vec<FundingRateInfo> = serde_json::from_str(json).unwrap();
        assert_eq!(funding_info.len(), 1);
        assert_eq!(funding_info[0].adjusted_funding_rate_cap, "0.10");
        assert_eq!(funding_info[0].adjusted_funding_rate_floor, "-0.10");
    }

    #[test]
    fn test_funding_rate_info_asymmetric_caps() {
        let json = r#"[
            {
                "symbol": "ASYMMETRIC",
                "adjustedFundingRateCap": "0.03",
                "adjustedFundingRateFloor": "-0.01",
                "fundingIntervalHours": 8
            }
        ]"#;

        let funding_info: Vec<FundingRateInfo> = serde_json::from_str(json).unwrap();
        assert_eq!(funding_info.len(), 1);
        assert_eq!(funding_info[0].adjusted_funding_rate_cap, "0.03");
        assert_eq!(funding_info[0].adjusted_funding_rate_floor, "-0.01");
    }

    #[test]
    fn test_funding_rate_info_empty_response() {
        let json = r#"[]"#;
        let funding_info: Vec<FundingRateInfo> = serde_json::from_str(json).unwrap();
        assert_eq!(funding_info.len(), 0);
    }

    #[test]
    fn test_funding_rate_info_high_precision() {
        let json = r#"[
            {
                "symbol": "PRECISE",
                "adjustedFundingRateCap": "0.00375",
                "adjustedFundingRateFloor": "-0.00375",
                "fundingIntervalHours": 1
            }
        ]"#;

        let funding_info: Vec<FundingRateInfo> = serde_json::from_str(json).unwrap();
        assert_eq!(funding_info.len(), 1);
        assert_eq!(funding_info[0].adjusted_funding_rate_cap, "0.00375");
        assert_eq!(funding_info[0].adjusted_funding_rate_floor, "-0.00375");
        assert_eq!(funding_info[0].funding_interval_hours, 1);
    }

    #[test]
    fn test_funding_rate_info_ignores_extra_fields() {
        // Test that extra fields like "disclaimer" are ignored
        let json = r#"[
            {
                "symbol": "BTCUSDT",
                "adjustedFundingRateCap": "0.02",
                "adjustedFundingRateFloor": "-0.02",
                "fundingIntervalHours": 8,
                "disclaimer": true,
                "extraField": "ignored"
            }
        ]"#;

        let funding_info: Vec<FundingRateInfo> = serde_json::from_str(json).unwrap();
        assert_eq!(funding_info.len(), 1);
        assert_eq!(funding_info[0].symbol, "BTCUSDT");
    }
}
