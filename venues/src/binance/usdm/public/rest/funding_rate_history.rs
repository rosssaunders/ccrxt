use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::binance::usdm::RestResult;

/// Endpoint for Funding Rate History
const FUNDING_RATE_HISTORY_ENDPOINT: &str = "/fapi/v1/fundingRate";

/// Request parameters for Funding Rate History endpoint.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FundingRateHistoryRequest {
    /// Trading pair symbol (e.g., "BTCUSDT"). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Cow<'static, str>>,

    /// Timestamp in milliseconds to get funding rate from (inclusive). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// Timestamp in milliseconds to get funding rate until (inclusive). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Number of records to return. Default 100; max 1000. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
}

/// Represents a funding rate history record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingRateHistory {
    /// Trading pair symbol for this record.
    pub symbol: Cow<'static, str>,

    /// Funding rate as a decimal string (e.g., "0.00010000").
    pub funding_rate: String,

    /// Funding timestamp in milliseconds since epoch.
    pub funding_time: u64,

    /// Mark price at the time of funding. Optional.
    pub mark_price: Option<String>,
}

impl RestClient {
    /// Get Funding Rate History
    ///
    /// Retrieves historical funding rates for USDM futures.
    ///
    /// [API docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Get-Funding-Rate-History)
    ///
    /// Request Weight: share 500/5min/IP rate limit with GET /fapi/v1/fundingInfo
    ///
    /// # Arguments
    /// * `params` - The funding rate history request parameters.
    ///
    /// # Returns
    /// A vector of `FundingRateHistory` records wrapped in `RestResult`.
    pub async fn get_funding_rate_history(
        &self,
        params: FundingRateHistoryRequest,
    ) -> RestResult<Vec<FundingRateHistory>> {
        self.send_get_request(FUNDING_RATE_HISTORY_ENDPOINT, Some(params), 1)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_funding_rate_history_request_serialization_full() {
        let request = FundingRateHistoryRequest {
            symbol: Some("BTCUSDT".into()),
            start_time: Some(1625184000000),
            end_time: Some(1625270400000),
            limit: Some(100),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("startTime=1625184000000"));
        assert!(serialized.contains("endTime=1625270400000"));
        assert!(serialized.contains("limit=100"));
    }

    #[test]
    fn test_funding_rate_history_request_serialization_minimal() {
        let request = FundingRateHistoryRequest {
            symbol: None,
            start_time: None,
            end_time: None,
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_funding_rate_history_request_serialization_symbol_only() {
        let request = FundingRateHistoryRequest {
            symbol: Some("ETHUSDT".into()),
            start_time: None,
            end_time: None,
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=ETHUSDT");
    }

    #[test]
    fn test_funding_rate_history_response_deserialization() {
        let json = r#"[
            {
                "symbol": "BTCUSDT",
                "fundingRate": "0.00010000",
                "fundingTime": 1625184000000,
                "markPrice": "45380.10000000"
            },
            {
                "symbol": "BTCUSDT",
                "fundingRate": "0.00012500",
                "fundingTime": 1625212800000,
                "markPrice": "45920.50000000"
            }
        ]"#;

        let history: Vec<FundingRateHistory> = serde_json::from_str(json).unwrap();
        assert_eq!(history.len(), 2);

        assert_eq!(history[0].symbol, "BTCUSDT");
        assert_eq!(history[0].funding_rate, "0.00010000");
        assert_eq!(history[0].funding_time, 1625184000000);
        assert_eq!(history[0].mark_price, Some("45380.10000000".to_string()));

        assert_eq!(history[1].symbol, "BTCUSDT");
        assert_eq!(history[1].funding_rate, "0.00012500");
        assert_eq!(history[1].funding_time, 1625212800000);
        assert_eq!(history[1].mark_price, Some("45920.50000000".to_string()));
    }

    #[test]
    fn test_funding_rate_history_response_without_mark_price() {
        let json = r#"[
            {
                "symbol": "BTCUSDT",
                "fundingRate": "0.00010000",
                "fundingTime": 1625184000000
            }
        ]"#;

        let history: Vec<FundingRateHistory> = serde_json::from_str(json).unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].mark_price, None);
    }

    #[test]
    fn test_funding_rate_history_negative_rate() {
        let json = r#"[
            {
                "symbol": "BTCUSDT",
                "fundingRate": "-0.00025000",
                "fundingTime": 1625184000000,
                "markPrice": "45380.10000000"
            }
        ]"#;

        let history: Vec<FundingRateHistory> = serde_json::from_str(json).unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].funding_rate, "-0.00025000");
    }

    #[test]
    fn test_funding_rate_history_extreme_rates() {
        let json = r#"[
            {
                "symbol": "VOLATILE",
                "fundingRate": "0.02000000",
                "fundingTime": 1625184000000,
                "markPrice": "100.00000000"
            },
            {
                "symbol": "VOLATILE",
                "fundingRate": "-0.02000000",
                "fundingTime": 1625212800000,
                "markPrice": "150.00000000"
            }
        ]"#;

        let history: Vec<FundingRateHistory> = serde_json::from_str(json).unwrap();
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].funding_rate, "0.02000000"); // 2% cap
        assert_eq!(history[1].funding_rate, "-0.02000000"); // -2% floor
    }

    #[test]
    fn test_funding_rate_history_empty_response() {
        let json = r#"[]"#;
        let history: Vec<FundingRateHistory> = serde_json::from_str(json).unwrap();
        assert_eq!(history.len(), 0);
    }

    #[test]
    fn test_funding_rate_history_max_limit() {
        let request = FundingRateHistoryRequest {
            symbol: Some("BTCUSDT".into()),
            start_time: None,
            end_time: None,
            limit: Some(1000), // max limit
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("limit=1000"));
    }

    #[test]
    fn test_funding_rate_history_multiple_symbols() {
        let json = r#"[
            {
                "symbol": "BTCUSDT",
                "fundingRate": "0.00010000",
                "fundingTime": 1625184000000
            },
            {
                "symbol": "ETHUSDT",
                "fundingRate": "0.00015000",
                "fundingTime": 1625184000000
            },
            {
                "symbol": "BNBUSDT",
                "fundingRate": "0.00008000",
                "fundingTime": 1625184000000
            }
        ]"#;

        let history: Vec<FundingRateHistory> = serde_json::from_str(json).unwrap();
        assert_eq!(history.len(), 3);
        assert_eq!(history[0].symbol, "BTCUSDT");
        assert_eq!(history[1].symbol, "ETHUSDT");
        assert_eq!(history[2].symbol, "BNBUSDT");
    }

    #[test]
    fn test_funding_rate_history_request_default() {
        let request = FundingRateHistoryRequest::default();
        assert!(request.symbol.is_none());
        assert!(request.start_time.is_none());
        assert!(request.end_time.is_none());
        assert!(request.limit.is_none());
        // Default serialization should be empty
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_funding_rate_history_endpoint_constant() {
        assert_eq!(FUNDING_RATE_HISTORY_ENDPOINT, "/fapi/v1/fundingRate");
    }
}
