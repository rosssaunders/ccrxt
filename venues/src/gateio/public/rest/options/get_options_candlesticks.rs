use serde::Serialize;

use super::{RestClient, candlestick::OptionsCandlestick};
use crate::gateio::RestResult;

const OPTIONS_CANDLESTICKS_ENDPOINT: &str = "/options/candlesticks";

/// Request parameters for options candlesticks
#[derive(Debug, Clone, Serialize, Default)]
pub struct OptionsCandlesticksRequest {
    /// Contract name
    pub contract: String,

    /// Start time (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End time (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,

    /// Maximum number of records to return (1-1000, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Interval time frame (10s, 1m, 5m, 15m, 30m, 1h, 4h, 8h, 1d, 7d, 30d)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
}

impl RestClient {
    /// Get options candlesticks
    ///
    /// Retrieves candlestick data for a specific options contract.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#get-options-candlesticks)
    pub async fn get_options_candlesticks(
        &self,
        params: OptionsCandlesticksRequest,
    ) -> RestResult<Vec<OptionsCandlestick>> {
        self.get_with_query(OPTIONS_CANDLESTICKS_ENDPOINT, Some(&params))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_options_candlesticks_request_minimal() {
        let request = OptionsCandlesticksRequest {
            contract: "BTC-20240101-50000-C".to_string(),
            from: None,
            to: None,
            limit: None,
            interval: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["contract"], "BTC-20240101-50000-C");
        assert!(!json.as_object().unwrap().contains_key("from"));
        assert!(!json.as_object().unwrap().contains_key("to"));
        assert!(!json.as_object().unwrap().contains_key("limit"));
        assert!(!json.as_object().unwrap().contains_key("interval"));
    }

    #[test]
    fn test_options_candlesticks_request_with_time_range() {
        let request = OptionsCandlesticksRequest {
            contract: "ETH-20240101-3000-P".to_string(),
            from: Some(1640995200),
            to: Some(1640995800),
            limit: None,
            interval: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["contract"], "ETH-20240101-3000-P");
        assert_eq!(json["from"], 1640995200);
        assert_eq!(json["to"], 1640995800);
    }

    #[test]
    fn test_options_candlesticks_request_full() {
        let request = OptionsCandlesticksRequest {
            contract: "ADA-20240301-1-C".to_string(),
            from: Some(1640995200),
            to: Some(1640995800),
            limit: Some(200),
            interval: Some("5m".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["contract"], "ADA-20240301-1-C");
        assert_eq!(json["from"], 1640995200);
        assert_eq!(json["to"], 1640995800);
        assert_eq!(json["limit"], 200);
        assert_eq!(json["interval"], "5m");
    }

    #[test]
    fn test_options_candlesticks_request_intervals() {
        let intervals = vec![
            "10s", "1m", "5m", "15m", "30m", "1h", "4h", "8h", "1d", "7d", "30d",
        ];

        for interval in intervals {
            let request = OptionsCandlesticksRequest {
                contract: "BTC-20240101-50000-C".to_string(),
                from: None,
                to: None,
                limit: None,
                interval: Some(interval.to_string()),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["interval"], interval);
        }
    }

    #[test]
    fn test_options_candlesticks_request_limits() {
        let limits = vec![1, 50, 100, 500, 1000];

        for limit in limits {
            let request = OptionsCandlesticksRequest {
                contract: "BTC-20240101-50000-C".to_string(),
                from: None,
                to: None,
                limit: Some(limit),
                interval: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["limit"], limit);
        }
    }

    #[test]
    fn test_options_candlesticks_request_contracts() {
        let contracts = vec![
            "BTC-20240101-50000-C",
            "ETH-20240215-3000-P",
            "BNB-20240301-400-C",
            "SOL-20240315-150-P",
        ];

        for contract in contracts {
            let request = OptionsCandlesticksRequest {
                contract: contract.to_string(),
                from: None,
                to: None,
                limit: None,
                interval: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["contract"], contract);
        }
    }

    #[test]
    fn test_options_candlesticks_endpoint() {
        assert_eq!(OPTIONS_CANDLESTICKS_ENDPOINT, "/options/candlesticks");
    }
}
