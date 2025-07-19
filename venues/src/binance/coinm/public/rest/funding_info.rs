use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, public::rest::RestClient};

const FUNDING_INFO_ENDPOINT: &str = "/dapi/v1/fundingInfo";

/// Request parameters for the funding rate info endpoint.
/// No parameters required for this endpoint.
#[derive(Debug, Clone, Serialize, Default)]
struct FundingInfoRequest {}

/// Represents funding rate info for a symbol.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingInfo {
    /// Trading symbol.
    pub symbol: String,

    /// Adjusted funding rate cap.
    pub adjusted_funding_rate_cap: String,

    /// Adjusted funding rate floor.
    pub adjusted_funding_rate_floor: String,

    /// Funding interval hours.
    pub funding_interval_hours: u32,

    /// Disclaimer flag.
    pub disclaimer: bool,
}

/// Response from the funding rate info endpoint.
pub type FundingInfoResponse = Vec<FundingInfo>;

impl RestClient {
    /// Get funding rate info
    ///
    /// Query funding rate info for symbols that had FundingRateCap/FundingRateFloor/fundingIntervalHours adjustment.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Get-Funding-Info
    ///
    /// Rate limit: 1
    ///
    /// # Arguments
    /// This endpoint takes no parameters
    ///
    /// # Returns
    /// List of funding rate info for symbols with adjustments
    pub async fn get_funding_info(&self) -> RestResult<FundingInfoResponse> {
        self.send_request(FUNDING_INFO_ENDPOINT, reqwest::Method::GET, None::<()>, 1)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_funding_info_request_serialization() {
        let request = FundingInfoRequest {};
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_funding_info_response_deserialization() {
        let json = r#"[
            {
                "symbol": "BTCUSD_PERP",
                "adjustedFundingRateCap": "0.00300000",
                "adjustedFundingRateFloor": "-0.00300000",
                "fundingIntervalHours": 8,
                "disclaimer": false
            },
            {
                "symbol": "ETHUSD_PERP",
                "adjustedFundingRateCap": "0.00200000",
                "adjustedFundingRateFloor": "-0.00200000",
                "fundingIntervalHours": 4,
                "disclaimer": true
            }
        ]"#;

        let funding_info: FundingInfoResponse = serde_json::from_str(json).unwrap();
        assert_eq!(funding_info.len(), 2);

        let first_info = &funding_info[0];
        assert_eq!(first_info.symbol, "BTCUSD_PERP");
        assert_eq!(first_info.adjusted_funding_rate_cap, "0.00300000");
        assert_eq!(first_info.adjusted_funding_rate_floor, "-0.00300000");
        assert_eq!(first_info.funding_interval_hours, 8);
        assert!(!first_info.disclaimer);

        let second_info = &funding_info[1];
        assert_eq!(second_info.symbol, "ETHUSD_PERP");
        assert_eq!(second_info.adjusted_funding_rate_cap, "0.00200000");
        assert_eq!(second_info.adjusted_funding_rate_floor, "-0.00200000");
        assert_eq!(second_info.funding_interval_hours, 4);
        assert!(second_info.disclaimer);
    }

    #[test]
    fn test_funding_info_response_empty_array() {
        let json = r#"[]"#;
        let funding_info: FundingInfoResponse = serde_json::from_str(json).unwrap();
        assert_eq!(funding_info.len(), 0);
    }

    #[test]
    fn test_funding_info_disclaimer_variations() {
        let json = r#"[
            {
                "symbol": "SOLUSDT_PERP",
                "adjustedFundingRateCap": "0.00100000",
                "adjustedFundingRateFloor": "-0.00100000",
                "fundingIntervalHours": 1,
                "disclaimer": true
            }
        ]"#;

        let funding_info: FundingInfoResponse = serde_json::from_str(json).unwrap();
        assert_eq!(funding_info.len(), 1);
        assert!(funding_info[0].disclaimer);
        assert_eq!(funding_info[0].funding_interval_hours, 1);
    }
}
