use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::gateio::RestResult;

const DELIVERY_INSURANCE_ENDPOINT: &str = "/delivery/{}/insurance";

/// Request parameters for delivery insurance
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeliveryInsuranceRequest {
    /// Settlement currency
    pub settle: String,

    /// Maximum number of records to return (1-1000, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Delivery insurance balance history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryInsurance {
    /// Timestamp
    pub t: i64,

    /// Insurance balance
    pub b: f64,
}

impl RestClient {
    /// Get delivery insurance balance history
    ///
    /// Retrieves historical insurance fund balance for the specified settlement currency.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#get-delivery-insurance-balance-history)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `params` - The delivery insurance request parameters
    ///
    /// # Returns
    /// Historical insurance fund balance entries
    pub async fn get_delivery_insurance(
        &self,
        params: DeliveryInsuranceRequest,
    ) -> RestResult<Vec<DeliveryInsurance>> {
        let endpoint = DELIVERY_INSURANCE_ENDPOINT.replace("{}", &params.settle);
        self.get_with_query(&endpoint, Some(&params)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delivery_insurance_request_minimal() {
        let request = DeliveryInsuranceRequest {
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
    fn test_delivery_insurance_request_with_limit() {
        let request = DeliveryInsuranceRequest {
            settle: "USDT".to_string(),
            limit: Some(50),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["limit"], 50);
    }

    #[test]
    fn test_limit_values() {
        let limits = vec![1, 10, 50, 100, 500, 1000];

        for limit in limits {
            let request = DeliveryInsuranceRequest {
                settle: "USDT".to_string(),
                limit: Some(limit),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["limit"], limit);
            assert!((1..=1000).contains(&limit));
        }
    }

    #[test]
    fn test_different_settlement_currencies() {
        let currencies = vec!["USDT", "BTC"];

        for settle in currencies {
            let request = DeliveryInsuranceRequest {
                settle: settle.to_string(),
                limit: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["settle"], settle);
        }
    }

    #[test]
    fn test_delivery_insurance_deserialization() {
        let json = r#"{
            "t": 1641024000,
            "b": 5000000.50
        }"#;

        let insurance: DeliveryInsurance = serde_json::from_str(json).unwrap();
        assert_eq!(insurance.t, 1641024000);
        assert_eq!(insurance.b, 5000000.50);
    }

    #[test]
    fn test_insurance_fund_growth_scenario() {
        let entries = [
            r#"{"t": 1641024000, "b": 5000000.0}"#,
            r#"{"t": 1641027600, "b": 5050000.0}"#,
            r#"{"t": 1641031200, "b": 5100000.0}"#,
            r#"{"t": 1641034800, "b": 5150000.0}"#,
        ];

        let mut previous_balance = 0.0;
        for (i, json_str) in entries.iter().enumerate() {
            let insurance: DeliveryInsurance = serde_json::from_str(json_str).unwrap();

            if i > 0 {
                // Insurance fund should be growing
                assert!(insurance.b > previous_balance);
            }
            previous_balance = insurance.b;

            // Verify reasonable insurance fund size
            assert!(insurance.b >= 5000000.0 && insurance.b <= 10000000.0);
        }
    }

    #[test]
    fn test_insurance_fund_drawdown_scenario() {
        let entries = [
            r#"{"t": 1641024000, "b": 10000000.0}"#,
            r#"{"t": 1641027600, "b": 9500000.0}"#,
            r#"{"t": 1641031200, "b": 9000000.0}"#,
            r#"{"t": 1641034800, "b": 8500000.0}"#,
        ];

        let mut previous_balance = f64::MAX;
        for (i, json_str) in entries.iter().enumerate() {
            let insurance: DeliveryInsurance = serde_json::from_str(json_str).unwrap();

            if i > 0 {
                // Insurance fund is decreasing (liquidations)
                assert!(insurance.b < previous_balance);
            }
            previous_balance = insurance.b;

            // Fund should still be substantial
            assert!(insurance.b >= 8000000.0);
        }
    }

    #[test]
    fn test_insurance_fund_stable_scenario() {
        let entries = vec![
            r#"{"t": 1641024000, "b": 25000000.0}"#,
            r#"{"t": 1641027600, "b": 25000500.0}"#,
            r#"{"t": 1641031200, "b": 24999800.0}"#,
            r#"{"t": 1641034800, "b": 25001000.0}"#,
        ];

        let base_balance = 25000000.0;
        for json_str in entries {
            let insurance: DeliveryInsurance = serde_json::from_str(json_str).unwrap();

            // Fund is relatively stable (within 0.1%)
            let variance = ((insurance.b - base_balance) / base_balance).abs();
            assert!(variance < 0.001); // Less than 0.1% variance
        }
    }

    #[test]
    fn test_btc_settlement_insurance() {
        let json = r#"{
            "t": 1641024000,
            "b": 150.75
        }"#;

        let insurance: DeliveryInsurance = serde_json::from_str(json).unwrap();
        assert_eq!(insurance.t, 1641024000);
        assert_eq!(insurance.b, 150.75);

        // BTC insurance fund should be reasonable
        assert!(insurance.b > 10.0 && insurance.b < 1000.0);
    }

    #[test]
    fn test_large_insurance_fund() {
        let json = r#"{
            "t": 1641024000,
            "b": 100000000.0
        }"#;

        let insurance: DeliveryInsurance = serde_json::from_str(json).unwrap();
        assert_eq!(insurance.b, 100000000.0); // 100 million

        // Very large insurance fund (major exchange)
        assert!(insurance.b >= 100000000.0);
    }

    #[test]
    fn test_small_insurance_fund() {
        let json = r#"{
            "t": 1641024000,
            "b": 500000.0
        }"#;

        let insurance: DeliveryInsurance = serde_json::from_str(json).unwrap();
        assert_eq!(insurance.b, 500000.0);

        // Smaller but still substantial fund
        assert!(insurance.b >= 100000.0 && insurance.b < 1000000.0);
    }

    #[test]
    fn test_hourly_snapshots() {
        let hourly_entries = [
            (1641024000, 5000000.0, "00:00"),
            (1641027600, 5010000.0, "01:00"),
            (1641031200, 5020000.0, "02:00"),
            (1641034800, 5030000.0, "03:00"),
            (1641038400, 5040000.0, "04:00"),
        ];

        for (i, (timestamp, balance, _time_desc)) in hourly_entries.iter().enumerate() {
            let insurance = DeliveryInsurance {
                t: *timestamp,
                b: *balance,
            };

            // Verify hourly intervals (3600 seconds)
            if i > 0 {
                let prev_timestamp = hourly_entries[i - 1].0;
                assert_eq!(insurance.t - prev_timestamp, 3600);
            }

            // Balance should be increasing slightly each hour
            assert!(insurance.b >= 5000000.0);
        }
    }

    #[test]
    fn test_daily_snapshots() {
        let daily_entries = [
            (1640995200, 20000000.0, "2022-01-01"),
            (1641081600, 20100000.0, "2022-01-02"),
            (1641168000, 20200000.0, "2022-01-03"),
            (1641254400, 20300000.0, "2022-01-04"),
            (1641340800, 20400000.0, "2022-01-05"),
        ];

        for (i, (timestamp, balance, _date)) in daily_entries.iter().enumerate() {
            let insurance = DeliveryInsurance {
                t: *timestamp,
                b: *balance,
            };

            // Verify daily intervals (86400 seconds)
            if i > 0 {
                let prev_timestamp = daily_entries[i - 1].0;
                assert_eq!(insurance.t - prev_timestamp, 86400);
            }

            assert!(insurance.b >= 20000000.0);
        }
    }

    #[test]
    fn test_market_crash_scenario() {
        // Simulating insurance fund during market crash with liquidations
        let crash_entries = vec![
            r#"{"t": 1641024000, "b": 50000000.0}"#, // Before crash
            r#"{"t": 1641024300, "b": 48000000.0}"#, // First wave of liquidations
            r#"{"t": 1641024600, "b": 45000000.0}"#, // Second wave
            r#"{"t": 1641024900, "b": 42000000.0}"#, // Third wave
            r#"{"t": 1641025200, "b": 40000000.0}"#, // Stabilizing
        ];

        let mut max_drawdown = 0.0;
        let initial_balance = 50000000.0;

        for json_str in crash_entries {
            let insurance: DeliveryInsurance = serde_json::from_str(json_str).unwrap();

            let drawdown = (initial_balance - insurance.b) / initial_balance;
            if drawdown > max_drawdown {
                max_drawdown = drawdown;
            }

            // Fund should maintain minimum threshold
            assert!(insurance.b >= 40000000.0);
        }

        // Maximum drawdown during crash
        assert!(max_drawdown <= 0.20); // 20% max drawdown
    }

    #[test]
    fn test_fund_recovery_scenario() {
        // Insurance fund recovery after market event
        let recovery_entries = [
            r#"{"t": 1641024000, "b": 40000000.0}"#, // Post-crash low
            r#"{"t": 1641110400, "b": 41000000.0}"#, // Day 2
            r#"{"t": 1641196800, "b": 42500000.0}"#, // Day 3
            r#"{"t": 1641283200, "b": 44000000.0}"#, // Day 4
            r#"{"t": 1641369600, "b": 46000000.0}"#, // Day 5
        ];

        let mut previous_balance = 0.0;
        for (i, json_str) in recovery_entries.iter().enumerate() {
            let insurance: DeliveryInsurance = serde_json::from_str(json_str).unwrap();

            if i > 0 {
                // Fund should be recovering
                assert!(insurance.b > previous_balance);
            }
            previous_balance = insurance.b;
        }
    }

    #[test]
    fn test_multiple_currency_funds() {
        let currency_funds = [
            ("USDT", 50000000.0, "USDT fund"),
            ("BTC", 500.0, "BTC fund"),
        ];

        for (currency, balance, _description) in currency_funds {
            let _request = DeliveryInsuranceRequest {
                settle: currency.to_string(),
                limit: Some(10),
            };

            let insurance = DeliveryInsurance {
                t: 1641024000,
                b: balance,
            };

            // Verify reasonable fund sizes for each currency
            if currency == "USDT" {
                assert!(insurance.b >= 1000000.0); // At least 1M USDT
            } else if currency == "BTC" {
                assert!(insurance.b >= 10.0 && insurance.b <= 10000.0); // Reasonable BTC amount
            }
        }
    }

    #[test]
    fn test_high_precision_balance() {
        let json = r#"{
            "t": 1641024000,
            "b": 12345678.123456789
        }"#;

        let insurance: DeliveryInsurance = serde_json::from_str(json).unwrap();
        assert_eq!(insurance.t, 1641024000);
        assert!((insurance.b - 12_345_678.123_456_79).abs() < 0.000001);
    }

    #[test]
    fn test_zero_balance_edge_case() {
        let json = r#"{
            "t": 1641024000,
            "b": 0.0
        }"#;

        let insurance: DeliveryInsurance = serde_json::from_str(json).unwrap();
        assert_eq!(insurance.b, 0.0);

        // Zero balance would be concerning but technically valid
    }

    #[test]
    fn test_timestamp_ordering() {
        let entries = [
            DeliveryInsurance {
                t: 1641024000,
                b: 5000000.0,
            },
            DeliveryInsurance {
                t: 1641027600,
                b: 5100000.0,
            },
            DeliveryInsurance {
                t: 1641031200,
                b: 5200000.0,
            },
        ];

        for i in 1..entries.len() {
            // Timestamps should be increasing
            assert!(entries[i].t > entries[i - 1].t);
        }
    }

    #[test]
    fn test_clone_behavior() {
        let request = DeliveryInsuranceRequest {
            settle: "USDT".to_string(),
            limit: Some(100),
        };

        let cloned = request.clone();
        assert_eq!(cloned.settle, request.settle);
        assert_eq!(cloned.limit, request.limit);
    }

    #[test]
    fn test_debug_output() {
        let insurance = DeliveryInsurance {
            t: 1641024000,
            b: 5000000.0,
        };

        let debug_str = format!("{:?}", insurance);
        assert!(debug_str.contains("DeliveryInsurance"));
        assert!(debug_str.contains("1641024000"));
        assert!(debug_str.contains("5000000"));
    }

    #[test]
    fn test_serialization_round_trip() {
        let insurance = DeliveryInsurance {
            t: 1641024000,
            b: 25000000.50,
        };

        let json = serde_json::to_string(&insurance).unwrap();
        let deserialized: DeliveryInsurance = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.t, insurance.t);
        assert_eq!(deserialized.b, insurance.b);
    }

    #[test]
    fn test_vector_of_entries() {
        let json = r#"[
            {"t": 1641024000, "b": 5000000.0},
            {"t": 1641027600, "b": 5050000.0},
            {"t": 1641031200, "b": 5100000.0}
        ]"#;

        let entries: Vec<DeliveryInsurance> = serde_json::from_str(json).unwrap();
        assert_eq!(entries.len(), 3);

        // Verify chronological order and growth
        for i in 1..entries.len() {
            assert!(entries[i].t > entries[i - 1].t);
            assert!(entries[i].b >= entries[i - 1].b);
        }
    }
}
