use serde::{Deserialize, Serialize};

use super::RestClient;

/// Countdown cancel all request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountdownCancelAllRequest {
    /// Countdown time in seconds (0 to disable)
    pub timeout: u32,

    /// Currency pair (optional, all pairs if not specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,
}

/// Countdown cancel all response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountdownCancelAllResponse {
    /// Trigger time (Unix timestamp)
    pub trigger_time: i64,
}

impl RestClient {
    /// Set up countdown cancel all
    ///
    /// This endpoint sets up an automatic order cancellation after a specified timeout.
    /// Setting timeout to 0 disables the countdown.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#countdown-cancel-orders>
    pub async fn countdown_cancel_all(
        &self,
        request: CountdownCancelAllRequest,
    ) -> crate::gateio::spot::Result<CountdownCancelAllResponse> {
        self.post("/spot/countdown_cancel_all", &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_countdown_cancel_all_request_with_timeout_only() {
        let request = CountdownCancelAllRequest {
            timeout: 300, // 5 minutes
            currency_pair: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["timeout"], 300);

        // currency_pair should be omitted when None
        let obj = json.as_object().unwrap();
        assert!(!obj.contains_key("currency_pair"));
    }

    #[test]
    fn test_countdown_cancel_all_request_with_currency_pair() {
        let request = CountdownCancelAllRequest {
            timeout: 600, // 10 minutes
            currency_pair: Some("BTC_USDT".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["timeout"], 600);
        assert_eq!(json["currency_pair"], "BTC_USDT");
    }

    #[test]
    fn test_countdown_cancel_all_request_disable_countdown() {
        let request = CountdownCancelAllRequest {
            timeout: 0, // Disable countdown
            currency_pair: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["timeout"], 0);
    }

    #[test]
    fn test_countdown_cancel_all_request_different_timeouts() {
        let timeouts = vec![
            30,   // 30 seconds
            60,   // 1 minute
            300,  // 5 minutes
            600,  // 10 minutes
            1800, // 30 minutes
            3600, // 1 hour
        ];

        for timeout in timeouts {
            let request = CountdownCancelAllRequest {
                timeout,
                currency_pair: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["timeout"], timeout);
        }
    }

    #[test]
    fn test_countdown_cancel_all_request_different_currency_pairs() {
        let pairs = vec![
            "BTC_USDT",
            "ETH_USDT",
            "BNB_USDT",
            "SOL_USDC",
            "ETH_BTC",
            "USDC_USDT",
        ];

        for pair in pairs {
            let request = CountdownCancelAllRequest {
                timeout: 300,
                currency_pair: Some(pair.to_string()),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["currency_pair"], pair);
        }
    }

    #[test]
    fn test_countdown_cancel_all_response_deserialization() {
        let json = r#"{
            "trigger_time": 1640995500
        }"#;

        let response: CountdownCancelAllResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.trigger_time, 1640995500);
    }

    #[test]
    fn test_countdown_cancel_all_response_future_timestamp() {
        let json = r#"{
            "trigger_time": 2000000000
        }"#;

        let response: CountdownCancelAllResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.trigger_time, 2000000000);
    }

    #[test]
    fn test_countdown_cancel_all_response_zero_timestamp() {
        // When countdown is disabled, trigger_time might be 0
        let json = r#"{
            "trigger_time": 0
        }"#;

        let response: CountdownCancelAllResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.trigger_time, 0);
    }

    #[test]
    fn test_countdown_cancel_all_request_realistic_emergency_scenario() {
        // Scenario: Emergency stop - cancel all orders in 30 seconds
        let request = CountdownCancelAllRequest {
            timeout: 30,
            currency_pair: None, // All pairs
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["timeout"], 30);

        let obj = json.as_object().unwrap();
        assert!(!obj.contains_key("currency_pair"));
    }

    #[test]
    fn test_countdown_cancel_all_request_realistic_market_closure_scenario() {
        // Scenario: Market closure preparation - cancel all BTC orders in 15 minutes
        let request = CountdownCancelAllRequest {
            timeout: 900, // 15 minutes
            currency_pair: Some("BTC_USDT".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["timeout"], 900);
        assert_eq!(json["currency_pair"], "BTC_USDT");
    }

    #[test]
    fn test_countdown_cancel_all_request_realistic_risk_management_scenario() {
        // Scenario: Risk management - auto-cancel positions before high volatility event
        let request = CountdownCancelAllRequest {
            timeout: 1800,       // 30 minutes before news release
            currency_pair: None, // All positions
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["timeout"], 1800);
    }

    #[test]
    fn test_countdown_cancel_all_request_realistic_maintenance_scenario() {
        // Scenario: System maintenance preparation
        let request = CountdownCancelAllRequest {
            timeout: 3600, // 1 hour
            currency_pair: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["timeout"], 3600);
    }

    #[test]
    fn test_countdown_cancel_all_request_realistic_disable_existing_countdown() {
        // Scenario: Disable existing countdown due to changed market conditions
        let request = CountdownCancelAllRequest {
            timeout: 0,                                  // Disable
            currency_pair: Some("ETH_USDT".to_string()), // Specific pair
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["timeout"], 0);
        assert_eq!(json["currency_pair"], "ETH_USDT");
    }

    #[test]
    fn test_countdown_cancel_all_request_edge_case_maximum_timeout() {
        // Test with maximum reasonable timeout (24 hours)
        let request = CountdownCancelAllRequest {
            timeout: 86400, // 24 hours
            currency_pair: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["timeout"], 86400);
    }

    #[test]
    fn test_countdown_cancel_all_request_edge_case_minimum_timeout() {
        // Test with minimum non-zero timeout
        let request = CountdownCancelAllRequest {
            timeout: 1, // 1 second
            currency_pair: Some("BTC_USDT".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["timeout"], 1);
        assert_eq!(json["currency_pair"], "BTC_USDT");
    }

    #[test]
    fn test_countdown_cancel_all_request_clone() {
        let original = CountdownCancelAllRequest {
            timeout: 300,
            currency_pair: Some("BTC_USDT".to_string()),
        };

        let cloned = original.clone();
        assert_eq!(cloned.timeout, original.timeout);
        assert_eq!(cloned.currency_pair, original.currency_pair);
    }

    #[test]
    fn test_countdown_cancel_all_response_clone() {
        let original = CountdownCancelAllResponse {
            trigger_time: 1640995500,
        };

        let cloned = original.clone();
        assert_eq!(cloned.trigger_time, original.trigger_time);
    }

    #[test]
    fn test_countdown_cancel_all_request_debug() {
        let request = CountdownCancelAllRequest {
            timeout: 300,
            currency_pair: Some("BTC_USDT".to_string()),
        };

        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("CountdownCancelAllRequest"));
        assert!(debug_str.contains("300"));
        assert!(debug_str.contains("BTC_USDT"));
    }

    #[test]
    fn test_countdown_cancel_all_response_debug() {
        let response = CountdownCancelAllResponse {
            trigger_time: 1640995500,
        };

        let debug_str = format!("{:?}", response);
        assert!(debug_str.contains("CountdownCancelAllResponse"));
        assert!(debug_str.contains("1640995500"));
    }

    #[test]
    fn test_countdown_cancel_all_request_serialization() {
        let request = CountdownCancelAllRequest {
            timeout: 300,
            currency_pair: Some("BTC_USDT".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        let obj = json.as_object().unwrap();
        assert!(obj.contains_key("timeout"));
        assert!(obj.contains_key("currency_pair"));
        assert_eq!(obj.len(), 2);
    }

    #[test]
    fn test_countdown_cancel_all_response_serialization() {
        let response = CountdownCancelAllResponse {
            trigger_time: 1640995500,
        };

        let json = serde_json::to_value(&response).unwrap();
        assert_eq!(json["trigger_time"], 1640995500);

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 1); // Only trigger_time field
    }

    #[test]
    fn test_countdown_cancel_all_request_endpoint_validation() {
        let request = CountdownCancelAllRequest {
            timeout: 300,
            currency_pair: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert!(json.as_object().unwrap().contains_key("timeout"));

        // Verify timeout is a number
        assert!(json["timeout"].is_number());
    }

    #[test]
    fn test_countdown_cancel_all_response_round_trip() {
        let original = CountdownCancelAllResponse {
            trigger_time: 1640995500,
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: CountdownCancelAllResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.trigger_time, original.trigger_time);
    }

    #[test]
    fn test_countdown_cancel_all_request_round_trip() {
        let original = CountdownCancelAllRequest {
            timeout: 300,
            currency_pair: Some("BTC_USDT".to_string()),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: CountdownCancelAllRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.timeout, original.timeout);
        assert_eq!(deserialized.currency_pair, original.currency_pair);
    }

    #[test]
    fn test_countdown_cancel_all_response_negative_timestamp() {
        // Test edge case with negative timestamp (shouldn't normally happen)
        let json = r#"{
            "trigger_time": -1
        }"#;

        let response: CountdownCancelAllResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.trigger_time, -1);
    }

    #[test]
    fn test_countdown_cancel_all_request_common_timeout_values() {
        let common_timeouts = vec![
            (0, "Disable countdown"),
            (10, "10 seconds - very short"),
            (30, "30 seconds - emergency"),
            (60, "1 minute - quick response"),
            (300, "5 minutes - standard"),
            (600, "10 minutes - extended"),
            (900, "15 minutes - preparation"),
            (1800, "30 minutes - long term"),
            (3600, "1 hour - maintenance"),
        ];

        for (timeout, description) in common_timeouts {
            let request = CountdownCancelAllRequest {
                timeout,
                currency_pair: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["timeout"], timeout, "Failed for case: {}", description);
        }
    }

    #[test]
    fn test_countdown_cancel_all_request_trading_session_scenarios() {
        let scenarios = vec![
            (
                CountdownCancelAllRequest {
                    timeout: 1800,
                    currency_pair: None,
                },
                "End of trading session - cancel all orders",
            ),
            (
                CountdownCancelAllRequest {
                    timeout: 300,
                    currency_pair: Some("BTC_USDT".to_string()),
                },
                "Major news coming - cancel BTC orders",
            ),
            (
                CountdownCancelAllRequest {
                    timeout: 900,
                    currency_pair: Some("ETH_USDT".to_string()),
                },
                "High volatility expected - cancel ETH orders",
            ),
            (
                CountdownCancelAllRequest {
                    timeout: 0,
                    currency_pair: None,
                },
                "False alarm - disable countdown",
            ),
        ];

        for (request, description) in scenarios {
            let json = serde_json::to_value(&request).unwrap();
            assert!(json["timeout"].is_number(), "Failed for: {}", description);

            if request.currency_pair.is_some() {
                assert!(
                    json["currency_pair"].is_string(),
                    "Failed for: {}",
                    description
                );
            } else {
                let obj = json.as_object().unwrap();
                assert!(
                    !obj.contains_key("currency_pair"),
                    "Failed for: {}",
                    description
                );
            }
        }
    }

    #[test]
    fn test_countdown_cancel_all_response_calculation_verification() {
        // Test that we can calculate time remaining from response
        let current_time = 1640995200; // Mock current time
        let trigger_time = 1640995500; // 5 minutes later

        let json = format!(r#"{{"trigger_time": {}}}"#, trigger_time);
        let response: CountdownCancelAllResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response.trigger_time, trigger_time);

        // Calculate time remaining
        let time_remaining = response.trigger_time - current_time;
        assert_eq!(time_remaining, 300); // 5 minutes
    }

    #[test]
    fn test_countdown_cancel_all_request_field_optional_behavior() {
        // Test that currency_pair is properly optional
        let request_without_pair = CountdownCancelAllRequest {
            timeout: 300,
            currency_pair: None,
        };

        let request_with_pair = CountdownCancelAllRequest {
            timeout: 300,
            currency_pair: Some("BTC_USDT".to_string()),
        };

        let json_without = serde_json::to_value(&request_without_pair).unwrap();
        let json_with = serde_json::to_value(&request_with_pair).unwrap();

        // Without currency_pair - should be omitted
        let obj_without = json_without.as_object().unwrap();
        assert!(!obj_without.contains_key("currency_pair"));
        assert_eq!(obj_without.len(), 1); // Only timeout

        // With currency_pair - should be included
        let obj_with = json_with.as_object().unwrap();
        assert!(obj_with.contains_key("currency_pair"));
        assert_eq!(obj_with.len(), 2); // timeout and currency_pair
    }

    #[test]
    fn test_countdown_cancel_all_response_large_timestamp() {
        // Test with large timestamp (year 2038 problem)
        let large_timestamp = 2147483647_i64; // Max 32-bit signed integer

        let json = format!(r#"{{"trigger_time": {}}}"#, large_timestamp);
        let response: CountdownCancelAllResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response.trigger_time, large_timestamp);
    }

    #[test]
    fn test_countdown_cancel_all_request_maximum_u32_timeout() {
        // Test edge case with maximum u32 value
        let request = CountdownCancelAllRequest {
            timeout: u32::MAX,
            currency_pair: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["timeout"], u32::MAX);
    }
}
