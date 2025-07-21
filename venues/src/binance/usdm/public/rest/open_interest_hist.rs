use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::binance::usdm::enums::Period;

const OPEN_INTEREST_HIST_ENDPOINT: &str = "/futures/data/openInterestHist";

/// Request parameters for the Open Interest Statistics endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct OpenInterestHistRequest<'a> {
    /// The symbol to query (e.g., "BTCUSDT").
    pub symbol: Cow<'a, str>,

    /// The period interval. Valid values: "5m", "15m", "30m", "1h", "2h", "4h", "6h", "12h", "1d".
    pub period: Period,

    /// Number of data points to return. Default 30, max 500.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Start time in milliseconds since epoch.
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time in milliseconds since epoch.
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
}

/// Open interest statistics data point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenInterestHistResponse<'a> {
    /// Trading symbol.
    pub symbol: Cow<'a, str>,

    /// Total open interest quantity.
    #[serde(rename = "sumOpenInterest")]
    pub sum_open_interest: Cow<'a, str>,

    /// Total open interest value in USDT.
    #[serde(rename = "sumOpenInterestValue")]
    pub sum_open_interest_value: Cow<'a, str>,

    /// Circulating supply provided by CMC (CoinMarketCap).
    #[serde(rename = "CMCCirculatingSupply")]
    pub cmc_circulating_supply: Option<Cow<'a, str>>,

    /// Timestamp in milliseconds since epoch.
    pub timestamp: u64,
}

impl RestClient {
    /// Open Interest Statistics
    ///
    /// Gets open interest statistics for a symbol over time periods.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Open-Interest-Statistics
    ///
    /// Rate limit: 0 (no weight)
    ///
    /// # Arguments
    /// * `params` - Request parameters including symbol, period, and optional time range
    ///
    /// # Returns
    /// Vector of open interest statistics data points
    pub async fn open_interest_hist<'a>(
        &self,
        params: OpenInterestHistRequest<'a>,
    ) -> crate::binance::usdm::RestResult<Vec<OpenInterestHistResponse<'a>>> {
        self.send_public_request(
            OPEN_INTEREST_HIST_ENDPOINT,
            reqwest::Method::GET,
            Some(params),
            0,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_interest_hist_request_serialization() {
        let request = OpenInterestHistRequest {
            symbol: "BTCUSDT".into(),
            period: Period::I5m,
            limit: Some(100),
            start_time: Some(1625097600000),
            end_time: Some(1625184000000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("period=5m"));
        assert!(serialized.contains("limit=100"));
        assert!(serialized.contains("startTime=1625097600000"));
        assert!(serialized.contains("endTime=1625184000000"));
    }

    #[test]
    fn test_open_interest_hist_request_minimal() {
        let request = OpenInterestHistRequest {
            symbol: "ETHUSDT".into(),
            period: Period::I1h,
            limit: None,
            start_time: None,
            end_time: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=ETHUSDT"));
        assert!(serialized.contains("period=1h"));
        assert!(!serialized.contains("limit"));
        assert!(!serialized.contains("startTime"));
        assert!(!serialized.contains("endTime"));
    }

    #[test]
    fn test_open_interest_hist_response_deserialization() {
        let json = r#"[
            {
                "symbol": "BTCUSDT",
                "sumOpenInterest": "123456789.00000000",
                "sumOpenInterestValue": "5617234567890.12345678",
                "CMCCirculatingSupply": "165880.538",
                "timestamp": 1625097600000
            },
            {
                "symbol": "BTCUSDT",
                "sumOpenInterest": "124567890.00000000",
                "sumOpenInterestValue": "5667890123456.78901234",
                "CMCCirculatingSupply": "165900.14853",
                "timestamp": 1625098500000
            }
        ]"#;

        let hist: Vec<OpenInterestHistResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(hist.len(), 2);

        let first = &hist[0];
        assert_eq!(first.symbol, "BTCUSDT");
        assert_eq!(first.sum_open_interest, "123456789.00000000");
        assert_eq!(first.sum_open_interest_value, "5617234567890.12345678");
        assert_eq!(first.cmc_circulating_supply, Some("165880.538".into()));
        assert_eq!(first.timestamp, 1625097600000);

        let second = &hist[1];
        assert_eq!(second.symbol, "BTCUSDT");
        assert_eq!(second.sum_open_interest, "124567890.00000000");
        assert_eq!(second.sum_open_interest_value, "5667890123456.78901234");
        assert_eq!(second.cmc_circulating_supply, Some("165900.14853".into()));
        assert_eq!(second.timestamp, 1625098500000);
    }

    #[test]
    fn test_open_interest_hist_small_values() {
        let json = r#"[
            {
                "symbol": "NEWUSDT",
                "sumOpenInterest": "1.00000000",
                "sumOpenInterestValue": "10.00000000",
                "timestamp": 1625097600000
            }
        ]"#;

        let hist: Vec<OpenInterestHistResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(hist.len(), 1);
        assert_eq!(hist[0].sum_open_interest, "1.00000000");
        assert_eq!(hist[0].sum_open_interest_value, "10.00000000");
        assert_eq!(hist[0].cmc_circulating_supply, None);
    }

    #[test]
    fn test_open_interest_hist_different_periods() {
        let periods = vec![
            Period::I5m,
            Period::I15m,
            Period::I30m,
            Period::I1h,
            Period::I2h,
            Period::I4h,
            Period::I6h,
            Period::I12h,
            Period::I1d,
        ];

        for period in periods {
            let request = OpenInterestHistRequest {
                symbol: "BTCUSDT".into(),
                period,
                limit: None,
                start_time: None,
                end_time: None,
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("period={}", period.as_str())));
        }
    }

    #[test]
    fn test_open_interest_hist_response_with_cmc_field() {
        let json = r#"[
            {
                "symbol": "BTCUSDT",
                "sumOpenInterest": "20403.63700000",
                "sumOpenInterestValue": "150570784.07809979",
                "CMCCirculatingSupply": "165880.538",
                "timestamp": 1583127900000
            }
        ]"#;

        let hist: Vec<OpenInterestHistResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(hist.len(), 1);

        let item = &hist[0];
        assert_eq!(item.symbol, "BTCUSDT");
        assert_eq!(item.sum_open_interest, "20403.63700000");
        assert_eq!(item.sum_open_interest_value, "150570784.07809979");
        assert_eq!(item.cmc_circulating_supply, Some("165880.538".into()));
        assert_eq!(item.timestamp, 1583127900000);
    }

    #[test]
    fn test_open_interest_hist_response_without_cmc_field() {
        let json = r#"[
            {
                "symbol": "BTCUSDT",
                "sumOpenInterest": "20403.63700000",
                "sumOpenInterestValue": "150570784.07809979",
                "timestamp": 1583127900000
            }
        ]"#;

        let hist: Vec<OpenInterestHistResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(hist.len(), 1);

        let item = &hist[0];
        assert_eq!(item.symbol, "BTCUSDT");
        assert_eq!(item.sum_open_interest, "20403.63700000");
        assert_eq!(item.sum_open_interest_value, "150570784.07809979");
        assert_eq!(item.cmc_circulating_supply, None);
        assert_eq!(item.timestamp, 1583127900000);
    }
}
