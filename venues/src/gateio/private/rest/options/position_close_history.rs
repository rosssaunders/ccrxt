use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const OPTIONS_POSITION_CLOSE_ENDPOINT: &str = "/options/position_close";

/// Options position close history entry
#[derive(Debug, Clone, Deserialize)]
pub struct OptionsPositionCloseHistory {
    /// Entry time
    pub time: f64,

    /// Profit and loss
    pub pnl: String,

    /// Position side
    pub side: String,

    /// Contract name
    pub contract: String,

    /// Text description
    pub text: String,
}

/// Request to retrieve options position close history
#[derive(Debug, Clone, Serialize, Default)]
pub struct OptionsPositionCloseHistoryRequest {
    /// Underlying asset name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying: Option<String>,

    /// Contract name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,

    /// Position side
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,

    /// Maximum number of record items to be returned (1-1000)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// You can set this to the last result time to retrieve records after that time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,

    /// Start timestamp (seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End timestamp (seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
}

impl RestClient {
    /// List Position Close History
    ///
    /// This endpoint returns the position close history for options trading.
    ///
    /// [docs](https://www.gate.io/docs/apiv4/en/index.html#list-position-close-history-2)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The position close history request parameters
    ///
    /// # Returns
    /// List of position close history entries
    pub async fn get_options_position_close_history(
        &self,
        request: OptionsPositionCloseHistoryRequest,
    ) -> RestResult<Vec<OptionsPositionCloseHistory>> {
        self.get_with_query(OPTIONS_POSITION_CLOSE_ENDPOINT, &request)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_options_position_close_history_request_minimal_serialization() {
        let request = OptionsPositionCloseHistoryRequest::default();

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_options_position_close_history_request_underlying_filter() {
        let request = OptionsPositionCloseHistoryRequest {
            underlying: Some("BTC_USDT".to_string()),
            contract: None,
            side: None,
            limit: None,
            offset: None,
            from: None,
            to: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "underlying=BTC_USDT");
    }

    #[test]
    fn test_options_position_close_history_request_contract_filter() {
        let request = OptionsPositionCloseHistoryRequest {
            underlying: None,
            contract: Some("BTC-20240101-50000-C".to_string()),
            side: None,
            limit: None,
            offset: None,
            from: None,
            to: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "contract=BTC-20240101-50000-C");
    }

    #[test]
    fn test_options_position_close_history_request_side_filter() {
        let request = OptionsPositionCloseHistoryRequest {
            underlying: None,
            contract: None,
            side: Some("long".to_string()),
            limit: None,
            offset: None,
            from: None,
            to: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "side=long");
    }

    #[test]
    fn test_options_position_close_history_request_pagination() {
        let request = OptionsPositionCloseHistoryRequest {
            underlying: None,
            contract: None,
            side: None,
            limit: Some(100),
            offset: Some(50),
            from: None,
            to: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("limit=100"));
        assert!(serialized.contains("offset=50"));
    }

    #[test]
    fn test_options_position_close_history_request_time_range() {
        let request = OptionsPositionCloseHistoryRequest {
            underlying: None,
            contract: None,
            side: None,
            limit: None,
            offset: None,
            from: Some(1640995200),
            to: Some(1641081600),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("from=1640995200"));
        assert!(serialized.contains("to=1641081600"));
    }

    #[test]
    fn test_options_position_close_history_request_full_parameters() {
        let request = OptionsPositionCloseHistoryRequest {
            underlying: Some("ETH_USDT".to_string()),
            contract: Some("ETH-20240101-3000-P".to_string()),
            side: Some("short".to_string()),
            limit: Some(50),
            offset: Some(25),
            from: Some(1640995200),
            to: Some(1641081600),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("underlying=ETH_USDT"));
        assert!(serialized.contains("contract=ETH-20240101-3000-P"));
        assert!(serialized.contains("side=short"));
        assert!(serialized.contains("limit=50"));
        assert!(serialized.contains("offset=25"));
        assert!(serialized.contains("from=1640995200"));
        assert!(serialized.contains("to=1641081600"));
    }

    #[test]
    fn test_options_position_close_history_request_different_sides() {
        let sides = vec!["long", "short"];

        for side in sides {
            let request = OptionsPositionCloseHistoryRequest {
                underlying: None,
                contract: None,
                side: Some(side.to_string()),
                limit: None,
                offset: None,
                from: None,
                to: None,
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert_eq!(serialized, format!("side={}", side));
        }
    }

    #[test]
    fn test_options_position_close_history_request_negative_values() {
        let request = OptionsPositionCloseHistoryRequest {
            underlying: None,
            contract: None,
            side: None,
            limit: Some(-10),
            offset: Some(-20),
            from: Some(-1640995200),
            to: Some(-1641081600),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("limit=-10"));
        assert!(serialized.contains("offset=-20"));
        assert!(serialized.contains("from=-1640995200"));
        assert!(serialized.contains("to=-1641081600"));
    }

    #[test]
    fn test_options_position_close_history_request_extreme_values() {
        let request = OptionsPositionCloseHistoryRequest {
            underlying: Some("BTC_USDT".to_string()),
            contract: None,
            side: None,
            limit: Some(i32::MAX),
            offset: Some(i32::MAX),
            from: Some(i64::MIN),
            to: Some(i64::MAX),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("underlying=BTC_USDT"));
        assert!(serialized.contains(&format!("limit={}", i32::MAX)));
        assert!(serialized.contains(&format!("offset={}", i32::MAX)));
        assert!(serialized.contains(&format!("from={}", i64::MIN)));
        assert!(serialized.contains(&format!("to={}", i64::MAX)));
    }

    #[test]
    fn test_options_position_close_history_entry_deserialization() {
        let json = r#"{
            "time": 1640995200.123,
            "pnl": "250.75",
            "side": "long",
            "contract": "BTC-20240101-50000-C",
            "text": "Position closed due to expiry"
        }"#;

        let entry: OptionsPositionCloseHistory = serde_json::from_str(json).unwrap();
        assert_eq!(entry.time, 1640995200.123);
        assert_eq!(entry.pnl, "250.75");
        assert_eq!(entry.side, "long");
        assert_eq!(entry.contract, "BTC-20240101-50000-C");
        assert_eq!(entry.text, "Position closed due to expiry");
    }

    #[test]
    fn test_options_position_close_history_entry_negative_pnl() {
        let json = r#"{
            "time": 1640995300.456,
            "pnl": "-125.50",
            "side": "short",
            "contract": "ETH-20240101-3000-P",
            "text": "Position closed manually"
        }"#;

        let entry: OptionsPositionCloseHistory = serde_json::from_str(json).unwrap();
        assert_eq!(entry.time, 1640995300.456);
        assert_eq!(entry.pnl, "-125.50");
        assert_eq!(entry.side, "short");
        assert_eq!(entry.contract, "ETH-20240101-3000-P");
        assert_eq!(entry.text, "Position closed manually");
    }

    #[test]
    fn test_options_position_close_history_entry_zero_pnl() {
        let json = r#"{
            "time": 1640995400.0,
            "pnl": "0.0",
            "side": "long",
            "contract": "BTC-20240215-45000-C",
            "text": "Break-even close"
        }"#;

        let entry: OptionsPositionCloseHistory = serde_json::from_str(json).unwrap();
        assert_eq!(entry.time, 1640995400.0);
        assert_eq!(entry.pnl, "0.0");
        assert_eq!(entry.side, "long");
        assert_eq!(entry.contract, "BTC-20240215-45000-C");
        assert_eq!(entry.text, "Break-even close");
    }

    #[test]
    fn test_options_position_close_history_entry_high_precision() {
        let json = r#"{
            "time": 1640995500.999999,
            "pnl": "1234.567890123",
            "side": "long",
            "contract": "ETH-20240301-2800-P",
            "text": "High precision PnL calculation"
        }"#;

        let entry: OptionsPositionCloseHistory = serde_json::from_str(json).unwrap();
        assert_eq!(entry.time, 1640995500.999999);
        assert_eq!(entry.pnl, "1234.567890123");
        assert_eq!(entry.side, "long");
        assert_eq!(entry.contract, "ETH-20240301-2800-P");
        assert_eq!(entry.text, "High precision PnL calculation");
    }

    #[test]
    fn test_options_position_close_history_entry_different_contracts() {
        let contracts = vec![
            "BTC-20240101-50000-C",
            "ETH-20240215-3000-P",
            "BNB-20240301-400-C",
            "SOL-20240315-150-P",
            "ADA-20240401-1-C",
        ];

        for contract in contracts {
            let json = format!(
                r#"{{
                "time": 1640995200.0,
                "pnl": "100.0",
                "side": "long",
                "contract": "{}",
                "text": "Contract test"
            }}"#,
                contract
            );

            let entry: OptionsPositionCloseHistory = serde_json::from_str(&json).unwrap();
            assert_eq!(entry.contract, contract);
            assert_eq!(entry.pnl, "100.0");
        }
    }

    #[test]
    fn test_options_position_close_history_entry_different_sides() {
        let sides = vec!["long", "short"];

        for side in sides {
            let json = format!(
                r#"{{
                "time": 1640995200.0,
                "pnl": "50.0",
                "side": "{}",
                "contract": "BTC-20240101-50000-C",
                "text": "Side test"
            }}"#,
                side
            );

            let entry: OptionsPositionCloseHistory = serde_json::from_str(&json).unwrap();
            assert_eq!(entry.side, side);
            assert_eq!(entry.time, 1640995200.0);
        }
    }

    #[test]
    fn test_options_position_close_history_entry_various_close_reasons() {
        let texts = vec![
            "Position closed due to expiry",
            "Manual close by user",
            "Liquidation",
            "Exercise",
            "Auto close due to insufficient margin",
            "Stop loss triggered",
            "Take profit triggered",
        ];

        for text in texts {
            let json = format!(
                r#"{{
                "time": 1640995200.0,
                "pnl": "25.0",
                "side": "long",
                "contract": "BTC-20240101-50000-C",
                "text": "{}"
            }}"#,
                text
            );

            let entry: OptionsPositionCloseHistory = serde_json::from_str(&json).unwrap();
            assert_eq!(entry.text, text);
        }
    }

    #[test]
    fn test_options_position_close_history_entry_extreme_pnl_values() {
        // Test very large profit
        let large_profit_json = r#"{
            "time": 1640995200.0,
            "pnl": "999999999.99999999",
            "side": "long",
            "contract": "BTC-20240101-50000-C",
            "text": "Massive profit"
        }"#;

        let large_profit_entry: OptionsPositionCloseHistory =
            serde_json::from_str(large_profit_json).unwrap();
        assert_eq!(large_profit_entry.pnl, "999999999.99999999");

        // Test very large loss
        let large_loss_json = r#"{
            "time": 1640995300.0,
            "pnl": "-999999999.99999999",
            "side": "short",
            "contract": "ETH-20240101-3000-P",
            "text": "Massive loss"
        }"#;

        let large_loss_entry: OptionsPositionCloseHistory =
            serde_json::from_str(large_loss_json).unwrap();
        assert_eq!(large_loss_entry.pnl, "-999999999.99999999");
    }

    #[test]
    fn test_options_position_close_history_entry_small_pnl_values() {
        let json = r#"{
            "time": 1640995200.0,
            "pnl": "0.00000001",
            "side": "long",
            "contract": "BTC-20240101-50000-C",
            "text": "Tiny profit"
        }"#;

        let entry: OptionsPositionCloseHistory = serde_json::from_str(json).unwrap();
        assert_eq!(entry.time, 1640995200.0);
        assert_eq!(entry.pnl, "0.00000001");
        assert_eq!(entry.side, "long");
        assert_eq!(entry.contract, "BTC-20240101-50000-C");
        assert_eq!(entry.text, "Tiny profit");
    }

    #[test]
    fn test_options_position_close_history_array_deserialization() {
        let json = r#"[
            {
                "time": 1640995200.0,
                "pnl": "100.0",
                "side": "long",
                "contract": "BTC-20240101-50000-C",
                "text": "First close"
            },
            {
                "time": 1640995300.0,
                "pnl": "-50.0",
                "side": "short",
                "contract": "ETH-20240101-3000-P",
                "text": "Second close"
            },
            {
                "time": 1640995400.0,
                "pnl": "0.0",
                "side": "long",
                "contract": "BNB-20240201-400-C",
                "text": "Break-even close"
            }
        ]"#;

        let entries: Vec<OptionsPositionCloseHistory> = serde_json::from_str(json).unwrap();
        assert_eq!(entries.len(), 3);

        assert_eq!(entries[0].time, 1640995200.0);
        assert_eq!(entries[0].pnl, "100.0");
        assert_eq!(entries[0].side, "long");
        assert_eq!(entries[0].contract, "BTC-20240101-50000-C");

        assert_eq!(entries[1].time, 1640995300.0);
        assert_eq!(entries[1].pnl, "-50.0");
        assert_eq!(entries[1].side, "short");
        assert_eq!(entries[1].contract, "ETH-20240101-3000-P");

        assert_eq!(entries[2].time, 1640995400.0);
        assert_eq!(entries[2].pnl, "0.0");
        assert_eq!(entries[2].side, "long");
        assert_eq!(entries[2].contract, "BNB-20240201-400-C");
    }

    #[test]
    fn test_options_position_close_history_empty_array_deserialization() {
        let json = r#"[]"#;
        let entries: Vec<OptionsPositionCloseHistory> = serde_json::from_str(json).unwrap();
        assert_eq!(entries.len(), 0);
    }

    #[test]
    fn test_options_position_close_history_entry_special_characters_in_text() {
        let json = r#"{
            "time": 1640995200.0,
            "pnl": "75.5",
            "side": "long",
            "contract": "BTC-20240101-50000-C",
            "text": "Position closed due to user action: \"Stop Loss\" @50000"
        }"#;

        let entry: OptionsPositionCloseHistory = serde_json::from_str(json).unwrap();
        assert_eq!(entry.time, 1640995200.0);
        assert_eq!(entry.pnl, "75.5");
        assert_eq!(entry.side, "long");
        assert_eq!(entry.contract, "BTC-20240101-50000-C");
        assert_eq!(
            entry.text,
            "Position closed due to user action: \"Stop Loss\" @50000"
        );
    }

    #[test]
    fn test_options_position_close_history_entry_realistic_scenarios() {
        // Profitable call option close
        let profitable_call_json = r#"{
            "time": 1640995200.0,
            "pnl": "500.75",
            "side": "long",
            "contract": "BTC-20240101-45000-C",
            "text": "Call option closed in-the-money at expiry"
        }"#;

        let profitable_call: OptionsPositionCloseHistory =
            serde_json::from_str(profitable_call_json).unwrap();
        assert_eq!(profitable_call.pnl, "500.75");
        assert_eq!(profitable_call.side, "long");
        assert!(profitable_call.contract.ends_with("-C"));

        // Loss on put option
        let loss_put_json = r#"{
            "time": 1640995300.0,
            "pnl": "-200.25",
            "side": "long",
            "contract": "ETH-20240101-3500-P",
            "text": "Put option expired out-of-the-money"
        }"#;

        let loss_put: OptionsPositionCloseHistory = serde_json::from_str(loss_put_json).unwrap();
        assert_eq!(loss_put.pnl, "-200.25");
        assert_eq!(loss_put.side, "long");
        assert!(loss_put.contract.ends_with("-P"));

        // Short option profit
        let short_profit_json = r#"{
            "time": 1640995400.0,
            "pnl": "150.0",
            "side": "short",
            "contract": "BTC-20240215-60000-C",
            "text": "Short call option expired worthless"
        }"#;

        let short_profit: OptionsPositionCloseHistory =
            serde_json::from_str(short_profit_json).unwrap();
        assert_eq!(short_profit.pnl, "150.0");
        assert_eq!(short_profit.side, "short");
        assert!(short_profit.contract.contains("-60000-"));
    }

    #[test]
    fn test_options_position_close_history_entry_time_precision() {
        let json = r#"{
            "time": 1640995200.123456789,
            "pnl": "100.0",
            "side": "long",
            "contract": "BTC-20240101-50000-C",
            "text": "High precision timestamp"
        }"#;

        let entry: OptionsPositionCloseHistory = serde_json::from_str(json).unwrap();
        assert_eq!(entry.time, 1_640_995_200.123_456_7);
        assert_eq!(entry.pnl, "100.0");
        assert_eq!(entry.side, "long");
        assert_eq!(entry.contract, "BTC-20240101-50000-C");
        assert_eq!(entry.text, "High precision timestamp");
    }

    #[test]
    fn test_options_position_close_history_entry_edge_case_timestamps() {
        // Test very old timestamp
        let old_timestamp_json = r#"{
            "time": 0.0,
            "pnl": "50.0",
            "side": "long",
            "contract": "BTC-20240101-50000-C",
            "text": "Old timestamp"
        }"#;

        let old_entry: OptionsPositionCloseHistory =
            serde_json::from_str(old_timestamp_json).unwrap();
        assert_eq!(old_entry.time, 0.0);

        // Test future timestamp
        let future_timestamp_json = r#"{
            "time": 9999999999.999,
            "pnl": "75.0",
            "side": "short",
            "contract": "ETH-20240101-3000-P",
            "text": "Future timestamp"
        }"#;

        let future_entry: OptionsPositionCloseHistory =
            serde_json::from_str(future_timestamp_json).unwrap();
        assert_eq!(future_entry.time, 9999999999.999);
    }
}
