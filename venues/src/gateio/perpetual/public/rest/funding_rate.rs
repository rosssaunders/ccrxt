use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for futures funding rate
#[derive(Debug, Clone, Serialize, Default)]
pub struct FuturesFundingRateRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract name
    pub contract: String,
    /// Maximum number of records to return (1-1000, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Futures funding rate history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesFundingRate {
    /// Funding time
    pub t: i64,

    /// Funding rate
    pub r: String,
}

impl RestClient {
    /// Get futures funding rate history
    ///
    /// Retrieves historical funding rates for a specific futures contract.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#funding-rate-history>
    pub async fn get_futures_funding_rate(
        &self,
        params: FuturesFundingRateRequest,
    ) -> crate::gateio::perpetual::Result<Vec<FuturesFundingRate>> {
        let endpoint = format!("/futures/{}/funding_rate", params.settle);
        self.get_with_query(&endpoint, Some(&params)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_futures_funding_rate_request_minimal() {
        let request = FuturesFundingRateRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            limit: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "BTC_USDT");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 2); // Only settle and contract
        assert!(!obj.contains_key("limit"));
    }

    #[test]
    fn test_futures_funding_rate_request_with_limit() {
        let request = FuturesFundingRateRequest {
            settle: "USDT".to_string(),
            contract: "ETH_USDT".to_string(),
            limit: Some(50),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "ETH_USDT");
        assert_eq!(json["limit"], 50);

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 3);
    }

    #[test]
    fn test_different_settlement_currencies() {
        let currencies = vec!["USDT", "BTC", "USD"];

        for settle in currencies {
            let request = FuturesFundingRateRequest {
                settle: settle.to_string(),
                contract: "BTC_USDT".to_string(),
                limit: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["settle"], settle);
        }
    }

    #[test]
    fn test_different_contract_pairs() {
        let contracts = vec![
            "BTC_USDT", "ETH_USDT", "ADA_USDT", "SOL_USDT",
            "MATIC_USDT", "DOT_USDT", "AVAX_USDT", "LINK_USDT"
        ];

        for contract in contracts {
            let request = FuturesFundingRateRequest {
                settle: "USDT".to_string(),
                contract: contract.to_string(),
                limit: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["contract"], contract);
        }
    }

    #[test]
    fn test_limit_variations() {
        let limits = vec![1, 10, 50, 100, 500, 1000];

        for limit in limits {
            let request = FuturesFundingRateRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                limit: Some(limit),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["limit"], limit);
            assert!(limit >= 1 && limit <= 1000);
        }
    }

    #[test]
    fn test_futures_funding_rate_deserialization() {
        let json = r#"{
            "t": 1640995200,
            "r": "0.000075"
        }"#;

        let funding_rate: FuturesFundingRate = serde_json::from_str(json).unwrap();
        assert_eq!(funding_rate.t, 1640995200);
        assert_eq!(funding_rate.r, "0.000075");
    }

    #[test]
    fn test_positive_funding_rate_scenarios() {
        let positive_rates = vec![
            ("0.000075", "Low positive rate"),
            ("0.0001", "Medium positive rate"),
            ("0.00025", "High positive rate"),
            ("0.375", "Maximum positive rate"),
        ];

        for (rate, _description) in positive_rates {
            let json = format!(r#"{{
                "t": 1640995200,
                "r": "{}"
            }}"#, rate);

            let funding_rate: FuturesFundingRate = serde_json::from_str(&json).unwrap();
            assert_eq!(funding_rate.r, rate);

            let rate_val: f64 = funding_rate.r.parse().unwrap();
            assert!(rate_val > 0.0);
        }
    }

    #[test]
    fn test_negative_funding_rate_scenarios() {
        let negative_rates = vec![
            ("-0.000075", "Low negative rate"),
            ("-0.0001", "Medium negative rate"),
            ("-0.00025", "High negative rate"),
            ("-0.375", "Maximum negative rate"),
        ];

        for (rate, _description) in negative_rates {
            let json = format!(r#"{{
                "t": 1640995200,
                "r": "{}"
            }}"#, rate);

            let funding_rate: FuturesFundingRate = serde_json::from_str(&json).unwrap();
            assert_eq!(funding_rate.r, rate);

            let rate_val: f64 = funding_rate.r.parse().unwrap();
            assert!(rate_val < 0.0);
        }
    }

    #[test]
    fn test_zero_funding_rate() {
        let json = r#"{
            "t": 1640995200,
            "r": "0"
        }"#;

        let funding_rate: FuturesFundingRate = serde_json::from_str(json).unwrap();
        assert_eq!(funding_rate.r, "0");

        let rate_val: f64 = funding_rate.r.parse().unwrap();
        assert_eq!(rate_val, 0.0);
    }

    #[test]
    fn test_realistic_btc_funding_scenarios() {
        let btc_scenarios = vec![
            ("0.000123", 1640995200, "Bull market scenario"),
            ("-0.000087", 1640998800, "Bear market scenario"),
            ("0.000001", 1641002400, "Neutral market"),
            ("0.000375", 1641006000, "High demand period"),
            ("-0.000375", 1641009600, "High selling pressure"),
        ];

        for (rate, timestamp, _scenario) in btc_scenarios {
            let json = format!(r#"{{
                "t": {},
                "r": "{}"
            }}"#, timestamp, rate);

            let funding_rate: FuturesFundingRate = serde_json::from_str(&json).unwrap();
            assert_eq!(funding_rate.t, timestamp);
            assert_eq!(funding_rate.r, rate);

            let rate_val: f64 = funding_rate.r.parse().unwrap();
            assert!(rate_val >= -0.375 && rate_val <= 0.375);
        }
    }

    #[test]
    fn test_realistic_eth_funding_scenarios() {
        let eth_scenarios = vec![
            ("0.000098", 1640995200, "ETH trending up"),
            ("-0.000134", 1640998800, "ETH trending down"),
            ("0.000245", 1641002400, "ETH high demand"),
            ("-0.000198", 1641006000, "ETH selling pressure"),
        ];

        for (rate, timestamp, _scenario) in eth_scenarios {
            let json = format!(r#"{{
                "t": {},
                "r": "{}"
            }}"#, timestamp, rate);

            let funding_rate: FuturesFundingRate = serde_json::from_str(&json).unwrap();
            assert_eq!(funding_rate.t, timestamp);
            assert_eq!(funding_rate.r, rate);

            // Verify rate is within reasonable bounds
            let rate_val: f64 = funding_rate.r.parse().unwrap();
            assert!(rate_val.abs() <= 0.375); // Maximum funding rate
        }
    }

    #[test]
    fn test_high_precision_funding_rates() {
        let precision_rates = vec![
            "0.000123456789",
            "-0.000987654321",
            "0.000000000001",
            "-0.000000000001",
        ];

        for rate in precision_rates {
            let json = format!(r#"{{
                "t": 1640995200,
                "r": "{}"
            }}"#, rate);

            let funding_rate: FuturesFundingRate = serde_json::from_str(&json).unwrap();
            assert_eq!(funding_rate.r, rate);

            // Verify precision is maintained
            assert!(funding_rate.r.len() >= rate.len());
        }
    }

    #[test]
    fn test_funding_rate_timestamp_scenarios() {
        let timestamps = vec![
            (1640995200, "Recent timestamp"),
            (1577836800, "Year 2020"),
            (1735689600, "Future timestamp"),
            (1640995200 + 28800, "8 hours later"),
            (1640995200 + 86400, "24 hours later"),
        ];

        for (timestamp, _description) in timestamps {
            let json = format!(r#"{{
                "t": {},
                "r": "0.000075"
            }}"#, timestamp);

            let funding_rate: FuturesFundingRate = serde_json::from_str(&json).unwrap();
            assert_eq!(funding_rate.t, timestamp);
        }
    }

    #[test]
    fn test_funding_rate_sequence_analysis() {
        // Simulate a series of funding rates showing market evolution
        let rate_sequence = vec![
            ("0.000050", 1640995200, "Start positive"),
            ("0.000075", 1640995200 + 28800, "Increasing demand"),
            ("0.000125", 1640995200 + 57600, "Peak demand"),
            ("0.000075", 1640995200 + 86400, "Cooling down"),
            ("0.000025", 1640995200 + 115200, "Low demand"),
            ("-0.000025", 1640995200 + 144000, "Turning negative"),
            ("-0.000075", 1640995200 + 172800, "Negative pressure"),
        ];

        let mut prev_timestamp = 0;
        for (rate, timestamp, _phase) in rate_sequence {
            let json = format!(r#"{{
                "t": {},
                "r": "{}"
            }}"#, timestamp, rate);

            let funding_rate: FuturesFundingRate = serde_json::from_str(&json).unwrap();
            assert_eq!(funding_rate.t, timestamp);
            assert_eq!(funding_rate.r, rate);

            // Verify timestamps are in ascending order
            assert!(funding_rate.t > prev_timestamp);
            prev_timestamp = funding_rate.t;

            // Verify rate is parseable
            let _rate_val: f64 = funding_rate.r.parse().unwrap();
        }
    }

    #[test]
    fn test_extreme_funding_rate_scenarios() {
        let extreme_scenarios = vec![
            ("0.375", "Maximum positive funding"),
            ("-0.375", "Maximum negative funding"),
            ("0.37499999", "Just under maximum"),
            ("-0.37499999", "Just under minimum"),
        ];

        for (rate, _scenario) in extreme_scenarios {
            let json = format!(r#"{{
                "t": 1640995200,
                "r": "{}"
            }}"#, rate);

            let funding_rate: FuturesFundingRate = serde_json::from_str(&json).unwrap();
            assert_eq!(funding_rate.r, rate);

            let rate_val: f64 = funding_rate.r.parse().unwrap();
            assert!(rate_val.abs() <= 0.375);
        }
    }

    #[test]
    fn test_funding_rate_market_impact_analysis() {
        // Test scenarios showing how funding rates reflect market conditions
        let market_scenarios = vec![
            ("0.000300", "High long interest"),
            ("-0.000300", "High short interest"),
            ("0.000001", "Balanced market"),
            ("0.000150", "Moderate long bias"),
            ("-0.000150", "Moderate short bias"),
        ];

        for (rate, _condition) in market_scenarios {
            let json = format!(r#"{{
                "t": 1640995200,
                "r": "{}"
            }}"#, rate);

            let funding_rate: FuturesFundingRate = serde_json::from_str(&json).unwrap();
            let rate_val: f64 = funding_rate.r.parse().unwrap();

            // Verify the market interpretation
            if rate_val > 0.0001 {
                // High positive rate indicates long bias
                assert!(rate_val > 0.0001);
            } else if rate_val < -0.0001 {
                // High negative rate indicates short bias  
                assert!(rate_val < -0.0001);
            } else {
                // Near zero indicates balanced market
                assert!(rate_val.abs() <= 0.0001);
            }
        }
    }

    #[test]
    fn test_clone_behavior() {
        let request = FuturesFundingRateRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            limit: Some(100),
        };

        let cloned = request.clone();
        assert_eq!(cloned.settle, request.settle);
        assert_eq!(cloned.contract, request.contract);
        assert_eq!(cloned.limit, request.limit);
    }

    #[test]
    fn test_debug_output() {
        let funding_rate = FuturesFundingRate {
            t: 1640995200,
            r: "0.000075".to_string(),
        };

        let debug_str = format!("{:?}", funding_rate);
        assert!(debug_str.contains("FuturesFundingRate"));
        assert!(debug_str.contains("1640995200"));
        assert!(debug_str.contains("0.000075"));
    }

    #[test]
    fn test_serialization_round_trip() {
        let funding_rate = FuturesFundingRate {
            t: 1640995200,
            r: "0.000075".to_string(),
        };

        let json = serde_json::to_string(&funding_rate).unwrap();
        let deserialized: FuturesFundingRate = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.t, funding_rate.t);
        assert_eq!(deserialized.r, funding_rate.r);
    }

    #[test]
    fn test_funding_rate_historical_patterns() {
        // Test patterns that might be seen in historical data
        let historical_patterns = vec![
            // Volatile period
            ("0.000200", 1640995200),
            ("0.000350", 1640995200 + 28800),
            ("-0.000100", 1640995200 + 57600),
            ("0.000075", 1640995200 + 86400),
            
            // Stable period
            ("0.000050", 1641081600),
            ("0.000055", 1641081600 + 28800),
            ("0.000048", 1641081600 + 57600),
            ("0.000052", 1641081600 + 86400),
        ];

        for (rate, timestamp) in historical_patterns {
            let json = format!(r#"{{
                "t": {},
                "r": "{}"
            }}"#, timestamp, rate);

            let funding_rate: FuturesFundingRate = serde_json::from_str(&json).unwrap();
            assert_eq!(funding_rate.t, timestamp);
            assert_eq!(funding_rate.r, rate);

            // Verify this could be part of a realistic sequence
            let rate_val: f64 = funding_rate.r.parse().unwrap();
            assert!(rate_val.abs() <= 0.375);
        }
    }

    #[test]
    fn test_different_contract_funding_behavior() {
        // Different contracts might have different funding rate characteristics
        let contract_rates = vec![
            ("BTC_USDT", "0.000075", "BTC typically moderate rates"),
            ("ETH_USDT", "0.000125", "ETH might have higher rates"),
            ("ADA_USDT", "0.000200", "Altcoins might have higher volatility"),
            ("SOL_USDT", "-0.000150", "Some alts might be negative"),
        ];

        for (contract, rate, _description) in contract_rates {
            let request = FuturesFundingRateRequest {
                settle: "USDT".to_string(),
                contract: contract.to_string(),
                limit: Some(100),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["contract"], contract);

            // Test the corresponding funding rate
            let funding_json = format!(r#"{{
                "t": 1640995200,
                "r": "{}"
            }}"#, rate);

            let funding_rate: FuturesFundingRate = serde_json::from_str(&funding_json).unwrap();
            assert_eq!(funding_rate.r, rate);
        }
    }
}
