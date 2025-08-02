use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::gateio::shared::enums::CandlestickInterval;

/// Request parameters for futures premium index
#[derive(Debug, Clone, Serialize, Default)]
pub struct FuturesPremiumIndexRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract name
    pub contract: String,
    /// Start time in Unix seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    /// End time in Unix seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
    /// Interval time between data points
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<CandlestickInterval>,
    /// Maximum number of records to return (1-1000, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Premium index K-line data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesPremiumIndex {
    /// Unix timestamp in seconds
    pub t: i64,

    /// Close price
    pub c: String,

    /// Highest price
    pub h: String,

    /// Lowest price
    pub l: String,

    /// Open price
    pub o: String,
}

impl RestClient {
    /// Get premium index K-line
    ///
    /// Retrieves premium index candlestick data for a specific futures contract.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#premium-index-k-line>
    /// Premium index tracks the difference between mark price and index price.
    pub async fn get_futures_premium_index(
        &self,
        params: FuturesPremiumIndexRequest,
    ) -> crate::gateio::perpetual::Result<Vec<FuturesPremiumIndex>> {
        let endpoint = format!("/futures/{}/premium_index", params.settle);
        self.get_with_query(&endpoint, Some(&params)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_futures_premium_index_request_minimal() {
        let request = FuturesPremiumIndexRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            from: None,
            to: None,
            interval: None,
            limit: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "BTC_USDT");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 2); // Only settle and contract
        assert!(!obj.contains_key("from"));
        assert!(!obj.contains_key("to"));
        assert!(!obj.contains_key("interval"));
        assert!(!obj.contains_key("limit"));
    }

    #[test]
    fn test_futures_premium_index_request_full() {
        let request = FuturesPremiumIndexRequest {
            settle: "USDT".to_string(),
            contract: "ETH_USDT".to_string(),
            from: Some(1640995200),
            to: Some(1640998800),
            interval: Some(CandlestickInterval::Minutes1),
            limit: Some(500),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "ETH_USDT");
        assert_eq!(json["from"], 1640995200);
        assert_eq!(json["to"], 1640998800);
        assert_eq!(json["interval"], "1m");
        assert_eq!(json["limit"], 500);

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 6);
    }

    #[test]
    fn test_different_settlement_currencies() {
        let currencies = vec!["USDT", "BTC", "USD"];

        for settle in currencies {
            let request = FuturesPremiumIndexRequest {
                settle: settle.to_string(),
                contract: "BTC_USDT".to_string(),
                from: None,
                to: None,
                interval: None,
                limit: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["settle"], settle);
        }
    }

    #[test]
    fn test_different_contract_pairs() {
        let contracts = vec![
            "BTC_USDT",
            "ETH_USDT",
            "ADA_USDT",
            "SOL_USDT",
            "MATIC_USDT",
            "DOT_USDT",
            "AVAX_USDT",
            "LINK_USDT",
        ];

        for contract in contracts {
            let request = FuturesPremiumIndexRequest {
                settle: "USDT".to_string(),
                contract: contract.to_string(),
                from: None,
                to: None,
                interval: None,
                limit: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["contract"], contract);
        }
    }

    #[test]
    fn test_different_intervals() {
        let intervals = vec![
            CandlestickInterval::Minutes1,
            CandlestickInterval::Minutes5,
            CandlestickInterval::Minutes15,
            CandlestickInterval::Minutes30,
            CandlestickInterval::Hours1,
            CandlestickInterval::Hours4,
            CandlestickInterval::Days1,
        ];

        for interval in intervals {
            let request = FuturesPremiumIndexRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                from: None,
                to: None,
                interval: Some(interval),
                limit: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert!(json["interval"].is_string());
        }
    }

    #[test]
    fn test_time_range_scenarios() {
        let time_ranges = vec![
            (1640995200, 1640998800, "1 hour"),
            (1640995200, 1641081600, "24 hours"),
            (1640995200, 1641686400, "1 week"),
            (1640995200, 1643673600, "1 month"),
        ];

        for (from, to, _description) in time_ranges {
            let request = FuturesPremiumIndexRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                from: Some(from),
                to: Some(to),
                interval: Some(CandlestickInterval::Hours1),
                limit: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["from"], from);
            assert_eq!(json["to"], to);
            assert!(to > from);
        }
    }

    #[test]
    fn test_limit_variations() {
        let limits = vec![1, 10, 50, 100, 500, 1000];

        for limit in limits {
            let request = FuturesPremiumIndexRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                from: None,
                to: None,
                interval: Some(CandlestickInterval::Minutes1),
                limit: Some(limit),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["limit"], limit);
            assert!(limit >= 1 && limit <= 1000);
        }
    }

    #[test]
    fn test_futures_premium_index_deserialization() {
        let json = r#"{
            "t": 1640995200,
            "c": "0.000125",
            "h": "0.000150",
            "l": "0.000100",
            "o": "0.000110"
        }"#;

        let premium_index: FuturesPremiumIndex = serde_json::from_str(json).unwrap();
        assert_eq!(premium_index.t, 1640995200);
        assert_eq!(premium_index.c, "0.000125");
        assert_eq!(premium_index.h, "0.000150");
        assert_eq!(premium_index.l, "0.000100");
        assert_eq!(premium_index.o, "0.000110");
    }

    #[test]
    fn test_premium_index_ohlc_validation() {
        let json = r#"{
            "t": 1640995200,
            "c": "0.000125",
            "h": "0.000150",
            "l": "0.000100",
            "o": "0.000110"
        }"#;

        let premium_index: FuturesPremiumIndex = serde_json::from_str(json).unwrap();

        let open: f64 = premium_index.o.parse().unwrap();
        let high: f64 = premium_index.h.parse().unwrap();
        let low: f64 = premium_index.l.parse().unwrap();
        let close: f64 = premium_index.c.parse().unwrap();

        // Verify OHLC relationships
        assert!(high >= open);
        assert!(high >= close);
        assert!(high >= low);
        assert!(low <= open);
        assert!(low <= close);
        assert!(low <= high);
    }

    #[test]
    fn test_positive_premium_scenarios() {
        // When mark price > index price (positive premium)
        let positive_premiums = vec![
            (
                "0.000050",
                "0.000075",
                "0.000025",
                "0.000060",
                "Small positive premium",
            ),
            (
                "0.000100",
                "0.000150",
                "0.000080",
                "0.000130",
                "Medium positive premium",
            ),
            (
                "0.000200",
                "0.000300",
                "0.000150",
                "0.000280",
                "Large positive premium",
            ),
            (
                "0.000500",
                "0.000600",
                "0.000450",
                "0.000550",
                "Very high positive premium",
            ),
        ];

        for (open, high, low, close, _description) in positive_premiums {
            let json = format!(
                r#"{{
                "t": 1640995200,
                "c": "{}",
                "h": "{}",
                "l": "{}",
                "o": "{}"
            }}"#,
                close, high, low, open
            );

            let premium_index: FuturesPremiumIndex = serde_json::from_str(&json).unwrap();

            let o: f64 = premium_index.o.parse().unwrap();
            let h: f64 = premium_index.h.parse().unwrap();
            let l: f64 = premium_index.l.parse().unwrap();
            let c: f64 = premium_index.c.parse().unwrap();

            // All values should be positive (futures trading at premium)
            assert!(o > 0.0);
            assert!(h > 0.0);
            assert!(l > 0.0);
            assert!(c > 0.0);

            // Verify OHLC constraints
            assert!(h >= o && h >= c && h >= l);
            assert!(l <= o && l <= c && l <= h);
        }
    }

    #[test]
    fn test_negative_premium_scenarios() {
        // When mark price < index price (negative premium/discount)
        let negative_premiums = vec![
            (
                "-0.000060",
                "-0.000025",
                "-0.000075",
                "-0.000050",
                "Small negative premium",
            ),
            (
                "-0.000130",
                "-0.000080",
                "-0.000150",
                "-0.000100",
                "Medium negative premium",
            ),
            (
                "-0.000280",
                "-0.000150",
                "-0.000300",
                "-0.000200",
                "Large negative premium",
            ),
            (
                "-0.000550",
                "-0.000450",
                "-0.000600",
                "-0.000500",
                "Very high negative premium",
            ),
        ];

        for (open, high, low, close, _description) in negative_premiums {
            let json = format!(
                r#"{{
                "t": 1640995200,
                "c": "{}",
                "h": "{}",
                "l": "{}",
                "o": "{}"
            }}"#,
                close, high, low, open
            );

            let premium_index: FuturesPremiumIndex = serde_json::from_str(&json).unwrap();

            let o: f64 = premium_index.o.parse().unwrap();
            let h: f64 = premium_index.h.parse().unwrap();
            let l: f64 = premium_index.l.parse().unwrap();
            let c: f64 = premium_index.c.parse().unwrap();

            // All values should be negative (futures trading at discount)
            assert!(o < 0.0);
            assert!(h < 0.0);
            assert!(l < 0.0);
            assert!(c < 0.0);

            // Verify OHLC constraints (high is least negative, low is most negative)
            assert!(h >= o && h >= c && h >= l);
            assert!(l <= o && l <= c && l <= h);
        }
    }

    #[test]
    fn test_premium_crossing_zero() {
        // Premium index crossing from negative to positive or vice versa
        let crossing_scenarios = vec![
            (
                "-0.000050",
                "0.000025",
                "-0.000075",
                "0.000010",
                "Negative to positive",
            ),
            (
                "0.000050",
                "0.000075",
                "-0.000025",
                "-0.000010",
                "Positive to negative",
            ),
            (
                "-0.000025",
                "0.000050",
                "-0.000060",
                "0.000030",
                "Wide swing positive",
            ),
            (
                "0.000025",
                "0.000060",
                "-0.000050",
                "-0.000030",
                "Wide swing negative",
            ),
        ];

        for (open, high, low, close, _description) in crossing_scenarios {
            let json = format!(
                r#"{{
                "t": 1640995200,
                "c": "{}",
                "h": "{}",
                "l": "{}",
                "o": "{}"
            }}"#,
                close, high, low, open
            );

            let premium_index: FuturesPremiumIndex = serde_json::from_str(&json).unwrap();

            let o: f64 = premium_index.o.parse().unwrap();
            let h: f64 = premium_index.h.parse().unwrap();
            let l: f64 = premium_index.l.parse().unwrap();
            let c: f64 = premium_index.c.parse().unwrap();

            // Verify OHLC constraints still hold
            assert!(h >= o && h >= c && h >= l);
            assert!(l <= o && l <= c && l <= h);

            // At least one of high/low should cross zero
            assert!(h >= 0.0 || l <= 0.0);
        }
    }

    #[test]
    fn test_realistic_btc_premium_scenarios() {
        let btc_scenarios = vec![
            (
                "0.000075",
                "0.000125",
                "0.000050",
                "0.000100",
                "Normal BTC premium",
            ),
            (
                "0.000200",
                "0.000350",
                "0.000150",
                "0.000300",
                "High demand period",
            ),
            (
                "-0.000100",
                "-0.000050",
                "-0.000150",
                "-0.000075",
                "Bearish sentiment",
            ),
            (
                "0.000025",
                "0.000075",
                "-0.000025",
                "0.000050",
                "Volatile period",
            ),
        ];

        for (open, high, low, close, _description) in btc_scenarios {
            let json = format!(
                r#"{{
                "t": 1640995200,
                "c": "{}",
                "h": "{}",
                "l": "{}",
                "o": "{}"
            }}"#,
                close, high, low, open
            );

            let premium_index: FuturesPremiumIndex = serde_json::from_str(&json).unwrap();

            let o: f64 = premium_index.o.parse().unwrap();
            let h: f64 = premium_index.h.parse().unwrap();
            let l: f64 = premium_index.l.parse().unwrap();
            let c: f64 = premium_index.c.parse().unwrap();

            // BTC premium should be within reasonable bounds
            assert!(o.abs() < 0.001); // < 0.1%
            assert!(h.abs() < 0.001);
            assert!(l.abs() < 0.001);
            assert!(c.abs() < 0.001);
        }
    }

    #[test]
    fn test_realistic_eth_premium_scenarios() {
        let eth_scenarios = vec![
            (
                "0.000120",
                "0.000180",
                "0.000080",
                "0.000150",
                "Normal ETH premium",
            ),
            (
                "0.000300",
                "0.000450",
                "0.000250",
                "0.000400",
                "High activity",
            ),
            (
                "-0.000150",
                "-0.000080",
                "-0.000200",
                "-0.000120",
                "Discount period",
            ),
            (
                "0.000050",
                "0.000120",
                "-0.000030",
                "0.000080",
                "Mixed sentiment",
            ),
        ];

        for (open, high, low, close, _description) in eth_scenarios {
            let json = format!(
                r#"{{
                "t": 1640995200,
                "c": "{}",
                "h": "{}",
                "l": "{}",
                "o": "{}"
            }}"#,
                close, high, low, open
            );

            let premium_index: FuturesPremiumIndex = serde_json::from_str(&json).unwrap();

            let o: f64 = premium_index.o.parse().unwrap();
            let h: f64 = premium_index.h.parse().unwrap();
            let l: f64 = premium_index.l.parse().unwrap();
            let c: f64 = premium_index.c.parse().unwrap();

            // ETH premium might be slightly higher than BTC
            assert!(o.abs() < 0.002); // < 0.2%
            assert!(h.abs() < 0.002);
            assert!(l.abs() < 0.002);
            assert!(c.abs() < 0.002);
        }
    }

    #[test]
    fn test_high_precision_premium_values() {
        let json = r#"{
            "t": 1640995200,
            "c": "0.000123456789",
            "h": "0.000234567890",
            "l": "0.000012345678",
            "o": "0.000098765432"
        }"#;

        let premium_index: FuturesPremiumIndex = serde_json::from_str(json).unwrap();

        // Verify precision is maintained
        assert_eq!(premium_index.c, "0.000123456789");
        assert_eq!(premium_index.h, "0.000234567890");
        assert_eq!(premium_index.l, "0.000012345678");
        assert_eq!(premium_index.o, "0.000098765432");
    }

    #[test]
    fn test_premium_index_time_series() {
        // Simulate premium index evolution over time
        let time_series = vec![
            (1640995200, "0.000100", "0.000120", "0.000080", "0.000110"),
            (1640995260, "0.000110", "0.000140", "0.000090", "0.000130"),
            (1640995320, "0.000130", "0.000160", "0.000120", "0.000150"),
            (1640995380, "0.000150", "0.000170", "0.000130", "0.000140"),
            (1640995440, "0.000140", "0.000155", "0.000125", "0.000145"),
        ];

        let mut prev_timestamp = 0;
        for (timestamp, open, high, low, close) in time_series {
            let json = format!(
                r#"{{
                "t": {},
                "c": "{}",
                "h": "{}",
                "l": "{}",
                "o": "{}"
            }}"#,
                timestamp, close, high, low, open
            );

            let premium_index: FuturesPremiumIndex = serde_json::from_str(&json).unwrap();

            // Verify timestamps are in ascending order
            assert!(premium_index.t > prev_timestamp);
            prev_timestamp = premium_index.t;

            // Verify OHLC constraints
            let o: f64 = premium_index.o.parse().unwrap();
            let h: f64 = premium_index.h.parse().unwrap();
            let l: f64 = premium_index.l.parse().unwrap();
            let c: f64 = premium_index.c.parse().unwrap();

            assert!(h >= o && h >= c && h >= l);
            assert!(l <= o && l <= c && l <= h);
        }
    }

    #[test]
    fn test_extreme_premium_scenarios() {
        // Test extreme but possible premium scenarios
        let extreme_scenarios = vec![
            (
                "0.001000",
                "0.001500",
                "0.000800",
                "0.001200",
                "Very high premium",
            ),
            (
                "-0.001200",
                "-0.000800",
                "-0.001500",
                "-0.001000",
                "Very high discount",
            ),
            (
                "0.000001",
                "0.000002",
                "0.000001",
                "0.000001",
                "Minimal premium",
            ),
            (
                "-0.000001",
                "-0.000001",
                "-0.000002",
                "-0.000001",
                "Minimal discount",
            ),
        ];

        for (open, high, low, close, _description) in extreme_scenarios {
            let json = format!(
                r#"{{
                "t": 1640995200,
                "c": "{}",
                "h": "{}",
                "l": "{}",
                "o": "{}"
            }}"#,
                close, high, low, open
            );

            let premium_index: FuturesPremiumIndex = serde_json::from_str(&json).unwrap();

            let o: f64 = premium_index.o.parse().unwrap();
            let h: f64 = premium_index.h.parse().unwrap();
            let l: f64 = premium_index.l.parse().unwrap();
            let c: f64 = premium_index.c.parse().unwrap();

            // Even extreme values should follow OHLC rules
            assert!(h >= o && h >= c && h >= l);
            assert!(l <= o && l <= c && l <= h);
        }
    }

    #[test]
    fn test_zero_premium_scenarios() {
        // When mark price equals index price
        let zero_scenarios = vec![
            (
                "0.000000",
                "0.000000",
                "0.000000",
                "0.000000",
                "Perfect zero",
            ),
            (
                "-0.000001",
                "0.000001",
                "-0.000001",
                "0.000000",
                "Hovering around zero",
            ),
            (
                "0.000000",
                "0.000005",
                "-0.000005",
                "0.000000",
                "Small fluctuation",
            ),
        ];

        for (open, high, low, close, _description) in zero_scenarios {
            let json = format!(
                r#"{{
                "t": 1640995200,
                "c": "{}",
                "h": "{}",
                "l": "{}",
                "o": "{}"
            }}"#,
                close, high, low, open
            );

            let premium_index: FuturesPremiumIndex = serde_json::from_str(&json).unwrap();

            let o: f64 = premium_index.o.parse().unwrap();
            let h: f64 = premium_index.h.parse().unwrap();
            let l: f64 = premium_index.l.parse().unwrap();
            let c: f64 = premium_index.c.parse().unwrap();

            // Values should be very close to zero
            assert!(o.abs() < 0.00001);
            assert!(h.abs() < 0.00001);
            assert!(l.abs() < 0.00001);
            assert!(c.abs() < 0.00001);
        }
    }

    #[test]
    fn test_premium_index_volatility_patterns() {
        // Test different volatility patterns in premium index
        let volatility_patterns = vec![
            (
                "0.000100",
                "0.000105",
                "0.000095",
                "0.000102",
                "Low volatility",
            ),
            (
                "0.000100",
                "0.000150",
                "0.000050",
                "0.000120",
                "Medium volatility",
            ),
            (
                "0.000100",
                "0.000300",
                "-0.000100",
                "0.000200",
                "High volatility",
            ),
            (
                "-0.000100",
                "0.000200",
                "-0.000300",
                "0.000050",
                "Extreme volatility",
            ),
        ];

        for (open, high, low, close, _description) in volatility_patterns {
            let json = format!(
                r#"{{
                "t": 1640995200,
                "c": "{}",
                "h": "{}",
                "l": "{}",
                "o": "{}"
            }}"#,
                close, high, low, open
            );

            let premium_index: FuturesPremiumIndex = serde_json::from_str(&json).unwrap();

            let _o: f64 = premium_index.o.parse().unwrap();
            let h: f64 = premium_index.h.parse().unwrap();
            let l: f64 = premium_index.l.parse().unwrap();
            let _c: f64 = premium_index.c.parse().unwrap();

            // Calculate volatility as range relative to midpoint
            let range = h - l;
            let midpoint = (h + l) / 2.0;

            assert!(range >= 0.0);
            if midpoint.abs() > 0.0001 {
                let volatility = range / midpoint.abs();
                assert!(volatility >= 0.0);
            }
        }
    }

    #[test]
    fn test_premium_index_market_phases() {
        // Test premium index behavior in different market phases
        let market_phases = vec![
            (
                "0.000200",
                "0.000250",
                "0.000180",
                "0.000230",
                "Bull market premium",
            ),
            (
                "-0.000150",
                "-0.000100",
                "-0.000180",
                "-0.000120",
                "Bear market discount",
            ),
            (
                "0.000050",
                "0.000080",
                "0.000020",
                "0.000060",
                "Sideways market",
            ),
            (
                "0.000100",
                "0.000300",
                "-0.000100",
                "0.000200",
                "Volatile/uncertain",
            ),
        ];

        for (open, high, low, close, _description) in market_phases {
            let json = format!(
                r#"{{
                "t": 1640995200,
                "c": "{}",
                "h": "{}",
                "l": "{}",
                "o": "{}"
            }}"#,
                close, high, low, open
            );

            let premium_index: FuturesPremiumIndex = serde_json::from_str(&json).unwrap();

            let o: f64 = premium_index.o.parse().unwrap();
            let h: f64 = premium_index.h.parse().unwrap();
            let l: f64 = premium_index.l.parse().unwrap();
            let c: f64 = premium_index.c.parse().unwrap();

            // Verify all values are within reasonable bounds
            assert!(o.abs() < 0.005); // < 0.5%
            assert!(h.abs() < 0.005);
            assert!(l.abs() < 0.005);
            assert!(c.abs() < 0.005);
        }
    }

    #[test]
    fn test_timestamp_scenarios() {
        let timestamps = vec![
            (1640995200, "Recent timestamp"),
            (1577836800, "Year 2020"),
            (1735689600, "Future timestamp"),
            (1609459200, "Year 2021"),
        ];

        for (timestamp, _description) in timestamps {
            let json = format!(
                r#"{{
                "t": {},
                "c": "0.000125",
                "h": "0.000150",
                "l": "0.000100",
                "o": "0.000110"
            }}"#,
                timestamp
            );

            let premium_index: FuturesPremiumIndex = serde_json::from_str(&json).unwrap();
            assert_eq!(premium_index.t, timestamp);
        }
    }

    #[test]
    fn test_clone_behavior() {
        let request = FuturesPremiumIndexRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            from: Some(1640995200),
            to: Some(1640998800),
            interval: Some(CandlestickInterval::Hours1),
            limit: Some(100),
        };

        let cloned = request.clone();
        assert_eq!(cloned.settle, request.settle);
        assert_eq!(cloned.contract, request.contract);
        assert_eq!(cloned.from, request.from);
        assert_eq!(cloned.to, request.to);
        assert_eq!(cloned.interval, request.interval);
        assert_eq!(cloned.limit, request.limit);
    }

    #[test]
    fn test_debug_output() {
        let premium_index = FuturesPremiumIndex {
            t: 1640995200,
            c: "0.000125".to_string(),
            h: "0.000150".to_string(),
            l: "0.000100".to_string(),
            o: "0.000110".to_string(),
        };

        let debug_str = format!("{:?}", premium_index);
        assert!(debug_str.contains("FuturesPremiumIndex"));
        assert!(debug_str.contains("1640995200"));
        assert!(debug_str.contains("0.000125"));
    }

    #[test]
    fn test_serialization_round_trip() {
        let premium_index = FuturesPremiumIndex {
            t: 1640995200,
            c: "0.000125".to_string(),
            h: "0.000150".to_string(),
            l: "0.000100".to_string(),
            o: "0.000110".to_string(),
        };

        let json = serde_json::to_string(&premium_index).unwrap();
        let deserialized: FuturesPremiumIndex = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.t, premium_index.t);
        assert_eq!(deserialized.c, premium_index.c);
        assert_eq!(deserialized.h, premium_index.h);
        assert_eq!(deserialized.l, premium_index.l);
        assert_eq!(deserialized.o, premium_index.o);
    }
}
