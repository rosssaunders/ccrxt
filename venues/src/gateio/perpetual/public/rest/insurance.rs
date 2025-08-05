use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for futures insurance
#[derive(Debug, Clone, Serialize, Default)]
pub struct FuturesInsuranceRequest {
    /// Settlement currency
    pub settle: String,
    /// Maximum number of records to return (1-1000, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Futures insurance balance history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesInsurance {
    /// Timestamp
    pub t: i64,

    /// Insurance balance
    pub b: f64,
}

impl RestClient {
    /// Get futures insurance balance history
    ///
    /// Retrieves historical insurance fund balance for the specified settlement currency.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#futures-insurance-balance-history>
    pub async fn get_futures_insurance(
        &self,
        params: FuturesInsuranceRequest,
    ) -> crate::gateio::perpetual::RestResult<Vec<FuturesInsurance>> {
        let endpoint = format!("/futures/{}/insurance", params.settle);
        self.get_with_query(&endpoint, Some(&params)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_futures_insurance_request_minimal() {
        let request = FuturesInsuranceRequest {
            settle: "USDT".to_string(),
            limit: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 1); // Only settle
        assert!(!obj.contains_key("limit"));
    }

    #[test]
    fn test_futures_insurance_request_with_limit() {
        let request = FuturesInsuranceRequest {
            settle: "BTC".to_string(),
            limit: Some(500),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "BTC");
        assert_eq!(json["limit"], 500);

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 2);
    }

    #[test]
    fn test_different_settlement_currencies() {
        let currencies = vec!["USDT", "BTC", "USD", "ETH"];

        for settle in currencies {
            let request = FuturesInsuranceRequest {
                settle: settle.to_string(),
                limit: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["settle"], settle);
        }
    }

    #[test]
    fn test_limit_variations() {
        let limits = vec![1, 10, 50, 100, 500, 1000];

        for limit in limits {
            let request = FuturesInsuranceRequest {
                settle: "USDT".to_string(),
                limit: Some(limit),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["limit"], limit);
            assert!(limit >= 1 && limit <= 1000);
        }
    }

    #[test]
    fn test_futures_insurance_deserialization() {
        let json = r#"{
            "t": 1640995200,
            "b": 15000000.5
        }"#;

        let insurance: FuturesInsurance = serde_json::from_str(json).unwrap();
        assert_eq!(insurance.t, 1640995200);
        assert_eq!(insurance.b, 15000000.5);
    }

    #[test]
    fn test_realistic_usdt_insurance_fund_scenarios() {
        let usdt_scenarios = vec![
            (1640995200, 15000000.0, "Normal fund level"),
            (1640995260, 15050000.0, "Fund growing"),
            (1640995320, 14800000.0, "Fund after liquidation"),
            (1640995380, 15200000.0, "Fund recovery"),
            (1640995440, 15500000.0, "Fund at high level"),
        ];

        for (timestamp, balance, _description) in usdt_scenarios {
            let json = format!(
                r#"{{
                "t": {},
                "b": {}
            }}"#,
                timestamp, balance
            );

            let insurance: FuturesInsurance = serde_json::from_str(&json).unwrap();
            assert_eq!(insurance.t, timestamp);
            assert_eq!(insurance.b, balance);

            // USDT insurance fund should be substantial
            assert!(insurance.b > 1000000.0); // > 1M USDT
            assert!(insurance.b < 100000000.0); // < 100M USDT
        }
    }

    #[test]
    fn test_realistic_btc_insurance_fund_scenarios() {
        let btc_scenarios = vec![
            (1640995200, 500.25, "Normal BTC fund level"),
            (1640995260, 502.75, "BTC fund growing"),
            (1640995320, 495.50, "BTC fund after liquidation"),
            (1640995380, 508.00, "BTC fund recovery"),
            (1640995440, 515.80, "BTC fund peak"),
        ];

        for (timestamp, balance, _description) in btc_scenarios {
            let json = format!(
                r#"{{
                "t": {},
                "b": {}
            }}"#,
                timestamp, balance
            );

            let insurance: FuturesInsurance = serde_json::from_str(&json).unwrap();
            assert_eq!(insurance.t, timestamp);
            assert_eq!(insurance.b, balance);

            // BTC insurance fund should be reasonable
            assert!(insurance.b > 100.0); // > 100 BTC
            assert!(insurance.b < 10000.0); // < 10K BTC
        }
    }

    #[test]
    fn test_insurance_fund_growth_pattern() {
        // Simulate insurance fund growth over time
        let growth_pattern = vec![
            (1640995200, 10000000.0),
            (1641081600, 10150000.0), // +1.5% after 24h
            (1641168000, 10280000.0), // +2.8% total
            (1641254400, 10420000.0), // +4.2% total
            (1641340800, 10380000.0), // Slight decrease
            (1641427200, 10550000.0), // Recovery
        ];

        let mut prev_timestamp = 0;
        for (timestamp, balance) in growth_pattern {
            let json = format!(
                r#"{{
                "t": {},
                "b": {}
            }}"#,
                timestamp, balance
            );

            let insurance: FuturesInsurance = serde_json::from_str(&json).unwrap();
            assert_eq!(insurance.t, timestamp);
            assert_eq!(insurance.b, balance);

            // Verify timestamps are in ascending order
            assert!(insurance.t > prev_timestamp);
            prev_timestamp = insurance.t;

            // Verify balance is positive
            assert!(insurance.b > 0.0);
        }
    }

    #[test]
    fn test_insurance_fund_liquidation_event() {
        // Simulate a liquidation event affecting insurance fund
        let liquidation_event = vec![
            (1640995200, 12000000.0, "Before liquidation"),
            (1640995260, 12000000.0, "Liquidation starts"),
            (1640995320, 11750000.0, "Fund depleted by 250K"),
            (1640995380, 11500000.0, "Further depletion"),
            (1640995440, 11600000.0, "Slight recovery"),
            (1640995500, 11850000.0, "Fund rebuilding"),
        ];

        let initial_balance = 12000000.0;
        for (timestamp, balance, _phase) in liquidation_event {
            let json = format!(
                r#"{{
                "t": {},
                "b": {}
            }}"#,
                timestamp, balance
            );

            let insurance: FuturesInsurance = serde_json::from_str(&json).unwrap();
            assert_eq!(insurance.t, timestamp);
            assert_eq!(insurance.b, balance);

            // Verify balance doesn't drop too dramatically
            let drop_percentage = (initial_balance - insurance.b) / initial_balance;
            assert!(drop_percentage < 0.2); // < 20% drop
        }
    }

    #[test]
    fn test_high_precision_balances() {
        let json = r#"{
            "t": 1640995200,
            "b": 15234567.123456789
        }"#;

        let insurance: FuturesInsurance = serde_json::from_str(json).unwrap();

        // Verify precision is maintained (within f64 precision limits)
        let expected = 15234567.123456789;
        let epsilon = 1e-7;
        assert!((insurance.b - expected).abs() < epsilon);
        assert_eq!(insurance.t, 1640995200);
    }

    #[test]
    fn test_very_large_insurance_funds() {
        let large_funds = vec![
            (50000000.0, "50M fund"),
            (100000000.0, "100M fund"),
            (500000000.0, "500M fund"),
            (1000000000.0, "1B fund"),
        ];

        for (balance, _description) in large_funds {
            let json = format!(
                r#"{{
                "t": 1640995200,
                "b": {}
            }}"#,
                balance
            );

            let insurance: FuturesInsurance = serde_json::from_str(&json).unwrap();
            assert_eq!(insurance.b, balance);
            assert!(insurance.b >= 50000000.0);
        }
    }

    #[test]
    fn test_small_altcoin_insurance_funds() {
        let small_funds = vec![
            (1000.0, "Small fund"),
            (5000.0, "Growing fund"),
            (10000.0, "Medium fund"),
            (50000.0, "Larger fund"),
        ];

        for (balance, _description) in small_funds {
            let json = format!(
                r#"{{
                "t": 1640995200,
                "b": {}
            }}"#,
                balance
            );

            let insurance: FuturesInsurance = serde_json::from_str(&json).unwrap();
            assert_eq!(insurance.b, balance);
            assert!(insurance.b > 0.0);
        }
    }

    #[test]
    fn test_insurance_fund_volatility_analysis() {
        // Test periods of high volatility affecting insurance funds
        let volatility_periods = vec![
            (1640995200, 15000000.0, "Calm period"),
            (1640995260, 14500000.0, "Market stress begins"),
            (1640995320, 13800000.0, "High volatility"),
            (1640995380, 13200000.0, "Peak stress"),
            (1640995440, 13600000.0, "Stabilizing"),
            (1640995500, 14100000.0, "Recovery"),
            (1640995560, 14800000.0, "Near normal"),
        ];

        for (timestamp, balance, _phase) in volatility_periods {
            let json = format!(
                r#"{{
                "t": {},
                "b": {}
            }}"#,
                timestamp, balance
            );

            let insurance: FuturesInsurance = serde_json::from_str(&json).unwrap();
            assert_eq!(insurance.t, timestamp);
            assert_eq!(insurance.b, balance);

            // Even during stress, fund should remain substantial
            assert!(insurance.b > 10000000.0);
        }
    }

    #[test]
    fn test_cross_settlement_insurance_comparison() {
        let settlements = vec![
            ("USDT", 15000000.0, "USDT fund"),
            ("BTC", 500.0, "BTC fund"),
            ("ETH", 5000.0, "ETH fund"),
            ("USD", 14500000.0, "USD fund"),
        ];

        for (settle, balance, _description) in settlements {
            let request = FuturesInsuranceRequest {
                settle: settle.to_string(),
                limit: Some(100),
            };

            let request_json = serde_json::to_value(&request).unwrap();
            assert_eq!(request_json["settle"], settle);

            let insurance_json = format!(
                r#"{{
                "t": 1640995200,
                "b": {}
            }}"#,
                balance
            );

            let insurance: FuturesInsurance = serde_json::from_str(&insurance_json).unwrap();
            assert_eq!(insurance.b, balance);
            assert!(insurance.b > 0.0);
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
                "b": 15000000.0
            }}"#,
                timestamp
            );

            let insurance: FuturesInsurance = serde_json::from_str(&json).unwrap();
            assert_eq!(insurance.t, timestamp);
        }
    }

    #[test]
    fn test_insurance_fund_daily_snapshots() {
        // Simulate daily insurance fund snapshots
        let daily_snapshots = vec![
            (1640995200, 15000000.0), // Day 1
            (1641081600, 15125000.0), // Day 2
            (1641168000, 15080000.0), // Day 3
            (1641254400, 15250000.0), // Day 4
            (1641340800, 15190000.0), // Day 5
            (1641427200, 15320000.0), // Day 6
            (1641513600, 15400000.0), // Day 7
        ];

        for (timestamp, balance) in daily_snapshots {
            let json = format!(
                r#"{{
                "t": {},
                "b": {}
            }}"#,
                timestamp, balance
            );

            let insurance: FuturesInsurance = serde_json::from_str(&json).unwrap();
            assert_eq!(insurance.t, timestamp);
            assert_eq!(insurance.b, balance);

            // Fund should be stable day-to-day
            assert!(insurance.b > 14000000.0);
            assert!(insurance.b < 16000000.0);
        }
    }

    #[test]
    fn test_insurance_fund_percentage_changes() {
        let base_balance = 10000000.0;
        let percentage_changes = vec![
            (0.0, "No change"),
            (0.01, "1% increase"),
            (0.05, "5% increase"),
            (-0.02, "2% decrease"),
            (-0.08, "8% decrease"),
            (0.12, "12% increase"),
        ];

        for (change, _description) in percentage_changes {
            let new_balance = base_balance * (1.0 + change);
            let json = format!(
                r#"{{
                "t": 1640995200,
                "b": {}
            }}"#,
                new_balance
            );

            let insurance: FuturesInsurance = serde_json::from_str(&json).unwrap();
            assert_eq!(insurance.b, new_balance);

            // Verify the change is within reasonable bounds
            assert!(insurance.b > base_balance * 0.8); // > -20%
            assert!(insurance.b < base_balance * 1.2); // < +20%
        }
    }

    #[test]
    fn test_zero_and_minimal_balances() {
        let minimal_balances = vec![
            (0.0, "Empty fund"),
            (0.1, "Minimal fund"),
            (1.0, "Small fund"),
            (10.0, "Tiny fund"),
        ];

        for (balance, _description) in minimal_balances {
            let json = format!(
                r#"{{
                "t": 1640995200,
                "b": {}
            }}"#,
                balance
            );

            let insurance: FuturesInsurance = serde_json::from_str(&json).unwrap();
            assert_eq!(insurance.b, balance);
            assert!(insurance.b >= 0.0);
        }
    }

    #[test]
    fn test_insurance_fund_seasonal_patterns() {
        // Simulate seasonal variations in insurance fund
        let seasonal_data = vec![
            (1609459200, 12000000.0, "Q1 start"),
            (1617235200, 12800000.0, "Q1 end"),
            (1625097600, 13200000.0, "Q2 end"),
            (1633046400, 13500000.0, "Q3 end"),
            (1640995200, 14000000.0, "Q4 end"),
        ];

        for (timestamp, balance, _quarter) in seasonal_data {
            let json = format!(
                r#"{{
                "t": {},
                "b": {}
            }}"#,
                timestamp, balance
            );

            let insurance: FuturesInsurance = serde_json::from_str(&json).unwrap();
            assert_eq!(insurance.t, timestamp);
            assert_eq!(insurance.b, balance);

            // Fund should show general growth trend
            assert!(insurance.b > 10000000.0);
        }
    }

    #[test]
    fn test_insurance_fund_stress_test_scenarios() {
        // Simulate extreme market conditions
        let stress_scenarios = vec![
            (1640995200, 15000000.0, "Normal conditions"),
            (1640995260, 14200000.0, "Market crash begins"),
            (1640995320, 12800000.0, "Deep crash"),
            (1640995380, 11500000.0, "Maximum stress"),
            (1640995440, 12200000.0, "Dead cat bounce"),
            (1640995500, 13100000.0, "Recovery starts"),
            (1640995560, 14500000.0, "Near recovery"),
        ];

        for (timestamp, balance, _phase) in stress_scenarios {
            let json = format!(
                r#"{{
                "t": {},
                "b": {}
            }}"#,
                timestamp, balance
            );

            let insurance: FuturesInsurance = serde_json::from_str(&json).unwrap();
            assert_eq!(insurance.t, timestamp);
            assert_eq!(insurance.b, balance);

            // Even in stress, fund should not be depleted
            assert!(insurance.b > 5000000.0);
        }
    }

    #[test]
    fn test_multi_currency_fund_correlation() {
        // Test how different currency funds might correlate
        let correlation_data = vec![
            ("USDT", 15000000.0, "Stable fund"),
            ("BTC", 400.0, "BTC correlated"),
            ("ETH", 4500.0, "ETH correlated"),
        ];

        for (currency, balance, _description) in correlation_data {
            let request = FuturesInsuranceRequest {
                settle: currency.to_string(),
                limit: Some(50),
            };

            let request_json = serde_json::to_value(&request).unwrap();
            assert_eq!(request_json["settle"], currency);

            let insurance_json = format!(
                r#"{{
                "t": 1640995200,
                "b": {}
            }}"#,
                balance
            );

            let insurance: FuturesInsurance = serde_json::from_str(&insurance_json).unwrap();
            assert_eq!(insurance.b, balance);
        }
    }

    #[test]
    fn test_clone_behavior() {
        let request = FuturesInsuranceRequest {
            settle: "USDT".to_string(),
            limit: Some(500),
        };

        let cloned = request.clone();
        assert_eq!(cloned.settle, request.settle);
        assert_eq!(cloned.limit, request.limit);
    }

    #[test]
    fn test_debug_output() {
        let insurance = FuturesInsurance {
            t: 1640995200,
            b: 15000000.5,
        };

        let debug_str = format!("{:?}", insurance);
        assert!(debug_str.contains("FuturesInsurance"));
        assert!(debug_str.contains("1640995200"));
        assert!(debug_str.contains("15000000.5"));
    }

    #[test]
    fn test_serialization_round_trip() {
        let insurance = FuturesInsurance {
            t: 1640995200,
            b: 15000000.5,
        };

        let json = serde_json::to_string(&insurance).unwrap();
        let deserialized: FuturesInsurance = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.t, insurance.t);
        assert_eq!(deserialized.b, insurance.b);
    }
}
