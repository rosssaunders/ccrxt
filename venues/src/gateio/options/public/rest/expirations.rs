use serde::Serialize;

use super::RestClient;

const OPTIONS_EXPIRATIONS_ENDPOINT: &str = "/options/expirations";

/// Request parameters for options expirations
#[derive(Debug, Clone, Serialize, Default)]
pub struct OptionsExpirationsRequest {
    /// Underlying asset
    pub underlying: String,
}

impl RestClient {
    /// List all expiration times
    ///
    /// Retrieves all available expiration times for options contracts.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#list-all-expiration-times>
    /// Returns Unix timestamps.
    pub async fn get_options_expirations(
        &self,
        params: OptionsExpirationsRequest,
    ) -> crate::gateio::options::Result<Vec<i64>> {
        self.get_with_query(OPTIONS_EXPIRATIONS_ENDPOINT, Some(&params))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_options_expirations_request_serialization() {
        let request = OptionsExpirationsRequest {
            underlying: "BTC_USDT".to_string(),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "underlying=BTC_USDT");
    }

    #[test]
    fn test_options_expirations_request_different_underlyings() {
        let underlyings = vec!["BTC_USDT", "ETH_USDT", "BNB_USDT", "SOL_USDT", "ADA_USDT"];

        for underlying in underlyings {
            let request = OptionsExpirationsRequest {
                underlying: underlying.to_string(),
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert_eq!(serialized, format!("underlying={}", underlying));
        }
    }

    #[test]
    fn test_options_expirations_request_special_characters() {
        let request = OptionsExpirationsRequest {
            underlying: "TEST-ASSET_USDT".to_string(),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "underlying=TEST-ASSET_USDT");
    }

    #[test]
    fn test_options_expirations_request_empty_underlying() {
        let request = OptionsExpirationsRequest {
            underlying: "".to_string(),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "underlying=");
    }

    #[test]
    fn test_options_expirations_request_long_underlying() {
        let request = OptionsExpirationsRequest {
            underlying: "VERY_LONG_ASSET_NAME_WITH_MANY_CHARACTERS_USDT".to_string(),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(
            serialized,
            "underlying=VERY_LONG_ASSET_NAME_WITH_MANY_CHARACTERS_USDT"
        );
    }

    #[test]
    fn test_options_expirations_request_case_variations() {
        let case_variations = vec!["BTC_USDT", "btc_usdt", "BTC_usdt", "btc_USDT", "Btc_Usdt"];

        for underlying in case_variations {
            let request = OptionsExpirationsRequest {
                underlying: underlying.to_string(),
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert_eq!(serialized, format!("underlying={}", underlying));
        }
    }

    #[test]
    fn test_options_expirations_response_empty_array() {
        let json = r#"[]"#;
        let expirations: Vec<i64> = serde_json::from_str(json).unwrap();
        assert_eq!(expirations.len(), 0);
    }

    #[test]
    fn test_options_expirations_response_single_expiration() {
        let json = r#"[1640995200]"#;
        let expirations: Vec<i64> = serde_json::from_str(json).unwrap();
        assert_eq!(expirations.len(), 1);
        assert_eq!(expirations[0], 1640995200);
    }

    #[test]
    fn test_options_expirations_response_multiple_expirations() {
        let json = r#"[1640995200, 1641081600, 1641168000, 1641254400]"#;
        let expirations: Vec<i64> = serde_json::from_str(json).unwrap();
        assert_eq!(expirations.len(), 4);
        assert_eq!(expirations[0], 1640995200); // 2022-01-01 00:00:00
        assert_eq!(expirations[1], 1641081600); // 2022-01-02 00:00:00
        assert_eq!(expirations[2], 1641168000); // 2022-01-03 00:00:00
        assert_eq!(expirations[3], 1641254400); // 2022-01-04 00:00:00
    }

    #[test]
    fn test_options_expirations_response_sorted_order() {
        let json = r#"[1640995200, 1641081600, 1641168000, 1641254400, 1641340800]"#;
        let expirations: Vec<i64> = serde_json::from_str(json).unwrap();
        assert_eq!(expirations.len(), 5);

        // Verify timestamps are in ascending order
        for i in 1..expirations.len() {
            assert!(expirations[i] > expirations[i - 1]);
        }
    }

    #[test]
    fn test_options_expirations_response_realistic_timestamps() {
        // Realistic expiration timestamps for options (end of week/month)
        let json = r#"[
            1704067200,
            1704153600,
            1704240000,
            1704326400,
            1705881600
        ]"#;

        let expirations: Vec<i64> = serde_json::from_str(json).unwrap();
        assert_eq!(expirations.len(), 5);

        // Verify all timestamps are in 2024 (reasonable for options expirations)
        for timestamp in &expirations {
            assert!(*timestamp >= 1704067200); // 2024-01-01
            assert!(*timestamp <= 1735689600); // 2025-01-01
        }
    }

    #[test]
    fn test_options_expirations_response_weekly_expirations() {
        // Weekly options expirations (every Friday)
        let json = r#"[
            1704499200,
            1705104000,
            1705708800,
            1706313600,
            1706918400
        ]"#;

        let expirations: Vec<i64> = serde_json::from_str(json).unwrap();
        assert_eq!(expirations.len(), 5);

        // Verify weekly intervals (approximately 7 days = 604800 seconds)
        for i in 1..expirations.len() {
            let interval = expirations[i] - expirations[i - 1];
            assert!(interval >= 604800 - 3600); // Allow 1 hour tolerance
            assert!(interval <= 604800 + 3600);
        }
    }

    #[test]
    fn test_options_expirations_response_monthly_expirations() {
        // Monthly options expirations (end of month)
        let json = r#"[
            1706745600,
            1709251200,
            1711929600,
            1714521600,
            1717200000
        ]"#;

        let expirations: Vec<i64> = serde_json::from_str(json).unwrap();
        assert_eq!(expirations.len(), 5);

        // Verify monthly-like intervals (28-31 days)
        for i in 1..expirations.len() {
            let interval = expirations[i] - expirations[i - 1];
            assert!(interval >= 28 * 24 * 3600); // At least 28 days
            assert!(interval <= 31 * 24 * 3600); // At most 31 days
        }
    }

    #[test]
    fn test_options_expirations_response_negative_timestamp() {
        let json = r#"[-1640995200]"#;
        let expirations: Vec<i64> = serde_json::from_str(json).unwrap();
        assert_eq!(expirations.len(), 1);
        assert_eq!(expirations[0], -1640995200);
    }

    #[test]
    fn test_options_expirations_response_zero_timestamp() {
        let json = r#"[0]"#;
        let expirations: Vec<i64> = serde_json::from_str(json).unwrap();
        assert_eq!(expirations.len(), 1);
        assert_eq!(expirations[0], 0);
    }

    #[test]
    fn test_options_expirations_response_large_timestamp() {
        let json = format!(r#"[{}]"#, i64::MAX);
        let expirations: Vec<i64> = serde_json::from_str(&json).unwrap();
        assert_eq!(expirations.len(), 1);
        assert_eq!(expirations[0], i64::MAX);
    }

    #[test]
    fn test_options_expirations_response_small_timestamp() {
        let json = format!(r#"[{}]"#, i64::MIN);
        let expirations: Vec<i64> = serde_json::from_str(&json).unwrap();
        assert_eq!(expirations.len(), 1);
        assert_eq!(expirations[0], i64::MIN);
    }

    #[test]
    fn test_options_expirations_response_mixed_timestamps() {
        let json = r#"[
            0,
            1640995200,
            -1000000,
            9223372036854775807,
            1700000000
        ]"#;

        let expirations: Vec<i64> = serde_json::from_str(json).unwrap();
        assert_eq!(expirations.len(), 5);
        assert_eq!(expirations[0], 0);
        assert_eq!(expirations[1], 1640995200);
        assert_eq!(expirations[2], -1000000);
        assert_eq!(expirations[3], 9223372036854775807);
        assert_eq!(expirations[4], 1700000000);
    }

    #[test]
    fn test_options_expirations_response_duplicate_timestamps() {
        let json = r#"[1640995200, 1640995200, 1641081600, 1641081600]"#;
        let expirations: Vec<i64> = serde_json::from_str(json).unwrap();
        assert_eq!(expirations.len(), 4);
        assert_eq!(expirations[0], 1640995200);
        assert_eq!(expirations[1], 1640995200);
        assert_eq!(expirations[2], 1641081600);
        assert_eq!(expirations[3], 1641081600);
    }

    #[test]
    fn test_options_expirations_response_large_array() {
        // Generate a large array of timestamps (52 weeks of weekly expirations)
        let mut timestamps = Vec::new();
        let base_timestamp = 1704499200; // Start date
        for i in 0..52 {
            timestamps.push(base_timestamp + i * 604800); // Add 1 week each time
        }

        let json = format!(
            "[{}]",
            timestamps
                .iter()
                .map(|t| t.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );
        let expirations: Vec<i64> = serde_json::from_str(&json).unwrap();

        assert_eq!(expirations.len(), 52);
        assert_eq!(expirations[0], base_timestamp);
        assert_eq!(expirations[51], base_timestamp + 51 * 604800);

        // Verify all are weekly intervals
        for i in 1..expirations.len() {
            assert_eq!(expirations[i] - expirations[i - 1], 604800);
        }
    }

    #[test]
    fn test_options_expirations_response_btc_realistic_scenario() {
        // Realistic BTC options expirations including weeklies and monthlies
        let json = r#"[
            1704499200,
            1705104000,
            1705708800,
            1706313600,
            1706745600,
            1706918400,
            1707523200,
            1708128000,
            1708732800
        ]"#;

        let expirations: Vec<i64> = serde_json::from_str(json).unwrap();
        assert_eq!(expirations.len(), 9);

        // Should be sorted
        for i in 1..expirations.len() {
            assert!(expirations[i] > expirations[i - 1]);
        }

        // All should be reasonable future dates
        for timestamp in &expirations {
            assert!(*timestamp > 1700000000); // After Nov 2023
            assert!(*timestamp < 1800000000); // Before Jan 2027
        }
    }

    #[test]
    fn test_options_expirations_response_eth_realistic_scenario() {
        // Realistic ETH options expirations
        let json = r#"[
            1704153600,
            1704499200,
            1705104000,
            1705708800,
            1706313600
        ]"#;

        let expirations: Vec<i64> = serde_json::from_str(json).unwrap();
        assert_eq!(expirations.len(), 5);

        // Verify chronological order
        for i in 1..expirations.len() {
            assert!(expirations[i] > expirations[i - 1]);
        }
    }

    #[test]
    fn test_options_expirations_response_edge_case_scenarios() {
        // Single timestamp at epoch
        let epoch_json = r#"[1]"#;
        let epoch_expirations: Vec<i64> = serde_json::from_str(epoch_json).unwrap();
        assert_eq!(epoch_expirations.len(), 1);
        assert_eq!(epoch_expirations[0], 1);

        // Very recent timestamp
        let recent_json = r#"[1700000000]"#;
        let recent_expirations: Vec<i64> = serde_json::from_str(recent_json).unwrap();
        assert_eq!(recent_expirations.len(), 1);
        assert_eq!(recent_expirations[0], 1700000000);

        // Far future timestamp
        let future_json = r#"[2000000000]"#;
        let future_expirations: Vec<i64> = serde_json::from_str(future_json).unwrap();
        assert_eq!(future_expirations.len(), 1);
        assert_eq!(future_expirations[0], 2000000000);
    }

    #[test]
    fn test_options_expirations_serialization_round_trip() {
        let original_timestamps = vec![1640995200, 1641081600, 1641168000];

        let json = serde_json::to_string(&original_timestamps).unwrap();
        let deserialized: Vec<i64> = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.len(), original_timestamps.len());
        for (i, timestamp) in original_timestamps.iter().enumerate() {
            assert_eq!(deserialized[i], *timestamp);
        }
    }

    #[test]
    fn test_options_expirations_request_default() {
        let request = OptionsExpirationsRequest::default();
        assert_eq!(request.underlying, "");
    }

    #[test]
    fn test_options_expirations_request_clone() {
        let original = OptionsExpirationsRequest {
            underlying: "BTC_USDT".to_string(),
        };

        let cloned = original.clone();
        assert_eq!(cloned.underlying, original.underlying);
    }

    #[test]
    fn test_options_expirations_request_debug_format() {
        let request = OptionsExpirationsRequest {
            underlying: "ETH_USDT".to_string(),
        };

        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("ETH_USDT"));
        assert!(debug_str.contains("OptionsExpirationsRequest"));
    }
}
