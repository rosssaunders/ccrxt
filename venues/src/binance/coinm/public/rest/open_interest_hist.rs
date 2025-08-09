use std::borrow::Cow;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::coinm::{
    RestResult,
    enums::{ContractTypeFilter, Period},
    public::rest::RestClient,
};

/// Endpoint path for Open Interest Statistics.
const OPEN_INTEREST_HIST_ENDPOINT: &str = "/futures/data/openInterestHist";

/// Request parameters for the Open Interest Statistics endpoint.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterestHistRequest {
    /// Trading pair (e.g., "BTCUSD").
    /// Must be a valid pair listed on Binance Coin-Margined Futures.
    pub pair: Cow<'static, str>,

    /// Contract type to filter results.
    /// Valid values: ALL, PERPETUAL, CURRENT_QUARTER, NEXT_QUARTER.
    /// See [`ContractTypeFilter`] enum for details.
    pub contract_type: ContractTypeFilter,

    /// Time interval for statistics.
    /// Valid values: "5m", "15m", "30m", "1h", "2h", "4h", "6h", "12h", "1d".
    /// See [`Period`] enum for details.
    pub period: Period,

    /// Maximum number of data points to return (default 30, max 500).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Start time in milliseconds since epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time in milliseconds since epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
}

/// Represents a single open interest statistics record returned by the endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterestHist {
    /// Trading pair (e.g., "BTCUSD").
    pub pair: Cow<'static, str>,

    /// Contract type for the record.
    pub contract_type: ContractTypeFilter,

    /// Sum of open interest (unit: contracts).
    pub sum_open_interest: Decimal,

    /// Sum of open interest value (unit: base asset).
    pub sum_open_interest_value: Decimal,

    /// Timestamp in milliseconds since epoch.
    pub timestamp: u64,
}

impl RestClient {
    /// Open Interest Statistics
    ///
    /// Queries open interest statistics for a given trading pair, contract type, and period.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Open-Interest-Statistics
    ///
    /// Rate limit: 1 request per second
    ///
    /// # Arguments
    /// * `request` - The request parameters for open interest statistics.
    ///
    /// # Returns
    /// Returns a vector of [`OpenInterestHist`] records for the specified query.
    pub async fn get_open_interest_hist(
        &self,
        request: OpenInterestHistRequest,
    ) -> RestResult<Vec<OpenInterestHist>> {
        self.send_get_request(OPEN_INTEREST_HIST_ENDPOINT, Some(request), 1)
            .await
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_open_interest_hist_request_serialization() {
        let req = OpenInterestHistRequest {
            pair: Cow::Borrowed("BTCUSD"),
            contract_type: ContractTypeFilter::Perpetual,
            period: Period::I5m,
            limit: Some(100),
            start_time: Some(1625097600000),
            end_time: Some(1625184000000),
        };
        let serialized = serde_urlencoded::to_string(&req).unwrap();
        assert!(serialized.contains("pair=BTCUSD"));
        assert!(serialized.contains("contractType=PERPETUAL"));
        assert!(serialized.contains("period=5m"));
        assert!(serialized.contains("limit=100"));
        assert!(serialized.contains("startTime=1625097600000"));
        assert!(serialized.contains("endTime=1625184000000"));
    }

    #[test]
    fn test_open_interest_hist_request_minimal() {
        let req = OpenInterestHistRequest {
            pair: Cow::Borrowed("ETHUSD"),
            contract_type: ContractTypeFilter::CurrentQuarter,
            period: Period::I1h,
            limit: None,
            start_time: None,
            end_time: None,
        };
        let serialized = serde_urlencoded::to_string(&req).unwrap();
        assert_eq!(
            serialized,
            "pair=ETHUSD&contractType=CURRENT_QUARTER&period=1h"
        );
    }

    #[test]
    fn test_open_interest_hist_response_deserialization() {
        let json = r#"[
            {
                "pair": "BTCUSD",
                "contractType": "PERPETUAL",
                "sumOpenInterest": "12345.678",
                "sumOpenInterestValue": "567890123.456",
                "timestamp": 1625097600000
            },
            {
                "pair": "BTCUSD",
                "contractType": "PERPETUAL",
                "sumOpenInterest": "13456.789",
                "sumOpenInterestValue": "612345678.901",
                "timestamp": 1625098500000
            }
        ]"#;
        let hist: Vec<OpenInterestHist> = serde_json::from_str(json).unwrap();
        assert_eq!(hist.len(), 2);
        let first_hist = &hist[0];
        assert_eq!(first_hist.pair, "BTCUSD");
        assert_eq!(first_hist.contract_type, ContractTypeFilter::Perpetual);
        assert_eq!(first_hist.sum_open_interest, dec!(12345.678));
        assert_eq!(first_hist.sum_open_interest_value, dec!(567890123.456));
        assert_eq!(first_hist.timestamp, 1625097600000);
        let second_hist = &hist[1];
        assert_eq!(second_hist.pair, "BTCUSD");
        assert_eq!(second_hist.contract_type, ContractTypeFilter::Perpetual);
        assert_eq!(second_hist.sum_open_interest, dec!(13456.789));
        assert_eq!(second_hist.sum_open_interest_value, dec!(612345678.901));
        assert_eq!(second_hist.timestamp, 1625098500000);
    }

    #[test]
    fn test_open_interest_hist_different_contract_types() {
        let contract_types = [
            ContractTypeFilter::Perpetual,
            ContractTypeFilter::CurrentQuarter,
            ContractTypeFilter::NextQuarter,
            ContractTypeFilter::All,
        ];
        for ct in contract_types {
            let req = OpenInterestHistRequest {
                pair: Cow::Borrowed("BTCUSD"),
                contract_type: ct,
                period: Period::I5m,
                limit: None,
                start_time: None,
                end_time: None,
            };
            let serialized = serde_urlencoded::to_string(&req).unwrap();
            assert!(serialized.contains(&format!("contractType={}", ct)));
        }
    }

    #[test]
    fn test_open_interest_hist_large_values() {
        let json = r#"[
            {
                "pair": "BTCUSD",
                "contractType": "PERPETUAL",
                "sumOpenInterest": "999999999.999",
                "sumOpenInterestValue": "999999999999.999",
                "timestamp": 1625097600000
            }
        ]"#;
        let hist: Vec<OpenInterestHist> = serde_json::from_str(json).unwrap();
        assert_eq!(hist.len(), 1);
        assert_eq!(hist[0].sum_open_interest, dec!(999999999.999));
        assert_eq!(hist[0].sum_open_interest_value, dec!(999999999999.999));
    }
}
