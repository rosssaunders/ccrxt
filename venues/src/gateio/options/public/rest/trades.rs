use serde::{Deserialize, Serialize};

use super::RestClient;

const ENDPOINT_OPTIONS_TRADES: &str = "/options/trades";

/// Request parameters for options trades
#[derive(Debug, Clone, Serialize, Default)]
pub struct OptionsTradesRequest {
    /// Contract name
    pub contract: String,

    /// Filter trades after this ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,

    /// Maximum number of records to return (1-1000, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Options trade information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsTrade {
    /// Trade ID
    pub id: i64,

    /// Trade timestamp
    pub create_time: f64,

    /// Contract name
    pub contract: String,

    /// Trade size
    pub size: String,

    /// Trading price (quote currency)
    pub price: String,

    /// Whether internal trade
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_internal: Option<bool>,
}

impl RestClient {
    /// Options trade history
    ///
    /// Retrieves recent trade history for a specific options contract.
    ///
    /// [docs](https://www.gate.com/docs/developers/apiv4/#options-trade-history)
    pub async fn get_options_trades(
        &self,
        params: OptionsTradesRequest,
    ) -> crate::gateio::options::RestResult<Vec<OptionsTrade>> {
        self.get_with_query(ENDPOINT_OPTIONS_TRADES, Some(&params))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_options_trades_request_minimal_serialization() {
        let request = OptionsTradesRequest {
            contract: "BTC-20240101-50000-C".to_string(),
            last_id: None,
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "contract=BTC-20240101-50000-C");
    }

    #[test]
    fn test_options_trades_request_with_last_id() {
        let request = OptionsTradesRequest {
            contract: "ETH-20240101-3000-P".to_string(),
            last_id: Some("12345".to_string()),
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("contract=ETH-20240101-3000-P"));
        assert!(serialized.contains("last_id=12345"));
    }

    #[test]
    fn test_options_trades_request_with_limit() {
        let request = OptionsTradesRequest {
            contract: "BNB-20240201-400-C".to_string(),
            last_id: None,
            limit: Some(50),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("contract=BNB-20240201-400-C"));
        assert!(serialized.contains("limit=50"));
    }

    #[test]
    fn test_options_trades_request_full_parameters() {
        let request = OptionsTradesRequest {
            contract: "SOL-20240215-150-P".to_string(),
            last_id: Some("67890".to_string()),
            limit: Some(100),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("contract=SOL-20240215-150-P"));
        assert!(serialized.contains("last_id=67890"));
        assert!(serialized.contains("limit=100"));
    }

    #[test]
    fn test_options_trades_request_limit_ranges() {
        let limits = vec![1, 100, 500, 1000];

        for limit in limits {
            let request = OptionsTradesRequest {
                contract: "BTC-20240101-50000-C".to_string(),
                last_id: None,
                limit: Some(limit),
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("limit={}", limit)));
        }
    }

    #[test]
    fn test_options_trades_request_negative_limit() {
        let request = OptionsTradesRequest {
            contract: "BTC-20240101-50000-C".to_string(),
            last_id: None,
            limit: Some(-10),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("limit=-10"));
    }

    #[test]
    fn test_options_trades_request_extreme_limit() {
        let request = OptionsTradesRequest {
            contract: "BTC-20240101-50000-C".to_string(),
            last_id: None,
            limit: Some(i32::MAX),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains(&format!("limit={}", i32::MAX)));
    }

    #[test]
    fn test_options_trades_request_different_contract_formats() {
        let contracts = vec![
            "BTC-20240101-50000-C",
            "ETH-20240215-3000-P",
            "BNB-20240301-400-C",
            "SOL-20240315-150-P",
            "ADA-20240401-1-C",
        ];

        for contract in contracts {
            let request = OptionsTradesRequest {
                contract: contract.to_string(),
                last_id: None,
                limit: None,
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert_eq!(serialized, format!("contract={}", contract));
        }
    }

    #[test]
    fn test_options_trades_request_different_last_id_formats() {
        let last_ids = vec!["123", "abc123", "trade_456", "0000001", "999999999"];

        for last_id in last_ids {
            let request = OptionsTradesRequest {
                contract: "BTC-20240101-50000-C".to_string(),
                last_id: Some(last_id.to_string()),
                limit: None,
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("last_id={}", last_id)));
        }
    }

    #[test]
    fn test_options_trade_deserialization() {
        let json = r#"{
            "id": 123456789,
            "create_time": 1640995200.123,
            "contract": "BTC-20240101-50000-C",
            "size": "1.5",
            "price": "0.08",
            "is_internal": false
        }"#;

        let trade: OptionsTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, 123456789);
        assert_eq!(trade.create_time, 1640995200.123);
        assert_eq!(trade.contract, "BTC-20240101-50000-C");
        assert_eq!(trade.size, "1.5");
        assert_eq!(trade.price, "0.08");
        assert_eq!(trade.is_internal, Some(false));
    }

    #[test]
    fn test_options_trade_without_internal_flag() {
        let json = r#"{
            "id": 987654321,
            "create_time": 1640995300.456,
            "contract": "ETH-20240101-3000-P",
            "size": "2.0",
            "price": "0.05"
        }"#;

        let trade: OptionsTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, 987654321);
        assert_eq!(trade.create_time, 1640995300.456);
        assert_eq!(trade.contract, "ETH-20240101-3000-P");
        assert_eq!(trade.size, "2.0");
        assert_eq!(trade.price, "0.05");
        assert_eq!(trade.is_internal, None);
    }

    #[test]
    fn test_options_trade_internal_trade() {
        let json = r#"{
            "id": 555666777,
            "create_time": 1640995400.789,
            "contract": "BNB-20240201-400-C",
            "size": "0.5",
            "price": "0.02",
            "is_internal": true
        }"#;

        let trade: OptionsTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, 555666777);
        assert_eq!(trade.create_time, 1640995400.789);
        assert_eq!(trade.contract, "BNB-20240201-400-C");
        assert_eq!(trade.size, "0.5");
        assert_eq!(trade.price, "0.02");
        assert_eq!(trade.is_internal, Some(true));
    }

    #[test]
    fn test_options_trade_negative_id() {
        let json = r#"{
            "id": -123456789,
            "create_time": 1640995500.0,
            "contract": "SOL-20240215-150-P",
            "size": "3.0",
            "price": "0.01"
        }"#;

        let trade: OptionsTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, -123456789);
        assert_eq!(trade.create_time, 1640995500.0);
        assert_eq!(trade.contract, "SOL-20240215-150-P");
        assert_eq!(trade.size, "3.0");
        assert_eq!(trade.price, "0.01");
        assert_eq!(trade.is_internal, None);
    }

    #[test]
    fn test_options_trade_high_precision_values() {
        let json = r#"{
            "id": 999999999,
            "create_time": 1640995600.999999,
            "contract": "ADA-20240301-1-C",
            "size": "100.123456789",
            "price": "0.001234567",
            "is_internal": false
        }"#;

        let trade: OptionsTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, 999999999);
        assert_eq!(trade.create_time, 1640995600.999999);
        assert_eq!(trade.contract, "ADA-20240301-1-C");
        assert_eq!(trade.size, "100.123456789");
        assert_eq!(trade.price, "0.001234567");
        assert_eq!(trade.is_internal, Some(false));
    }

    #[test]
    fn test_options_trade_zero_values() {
        let json = r#"{
            "id": 0,
            "create_time": 0.0,
            "contract": "BTC-20240101-50000-C",
            "size": "0",
            "price": "0"
        }"#;

        let trade: OptionsTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, 0);
        assert_eq!(trade.create_time, 0.0);
        assert_eq!(trade.contract, "BTC-20240101-50000-C");
        assert_eq!(trade.size, "0");
        assert_eq!(trade.price, "0");
        assert_eq!(trade.is_internal, None);
    }

    #[test]
    fn test_options_trade_extreme_values() {
        let json = format!(
            r#"{{
            "id": {},
            "create_time": 9999999999.999,
            "contract": "BTC-20240101-50000-C",
            "size": "999999999.999999999",
            "price": "999999.999999999"
        }}"#,
            i64::MAX
        );

        let trade: OptionsTrade = serde_json::from_str(&json).unwrap();
        assert_eq!(trade.id, i64::MAX);
        assert_eq!(trade.create_time, 9999999999.999);
        assert_eq!(trade.contract, "BTC-20240101-50000-C");
        assert_eq!(trade.size, "999999999.999999999");
        assert_eq!(trade.price, "999999.999999999");
    }

    #[test]
    fn test_options_trade_different_contract_types() {
        let contracts = vec![
            ("BTC-20240101-50000-C", "call"),
            ("ETH-20240215-3000-P", "put"),
            ("BNB-20240301-400-C", "call"),
            ("SOL-20240315-150-P", "put"),
            ("ADA-20240401-1-C", "call"),
        ];

        for (contract, option_type) in contracts {
            let json = format!(
                r#"{{
                "id": 12345,
                "create_time": 1640995200.0,
                "contract": "{}",
                "size": "1.0",
                "price": "0.1"
            }}"#,
                contract
            );

            let trade: OptionsTrade = serde_json::from_str(&json).unwrap();
            assert_eq!(trade.contract, contract);

            if option_type == "call" {
                assert!(trade.contract.ends_with("-C"));
            } else {
                assert!(trade.contract.ends_with("-P"));
            }
        }
    }

    #[test]
    fn test_options_trade_array_deserialization() {
        let json = r#"[
            {
                "id": 1,
                "create_time": 1640995200.0,
                "contract": "BTC-20240101-50000-C",
                "size": "1.0",
                "price": "0.08",
                "is_internal": false
            },
            {
                "id": 2,
                "create_time": 1640995300.0,
                "contract": "ETH-20240101-3000-P",
                "size": "2.0",
                "price": "0.05",
                "is_internal": true
            },
            {
                "id": 3,
                "create_time": 1640995400.0,
                "contract": "BNB-20240201-400-C",
                "size": "0.5",
                "price": "0.02"
            }
        ]"#;

        let trades: Vec<OptionsTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 3);

        assert_eq!(trades[0].id, 1);
        assert_eq!(trades[0].contract, "BTC-20240101-50000-C");
        assert_eq!(trades[0].is_internal, Some(false));

        assert_eq!(trades[1].id, 2);
        assert_eq!(trades[1].contract, "ETH-20240101-3000-P");
        assert_eq!(trades[1].is_internal, Some(true));

        assert_eq!(trades[2].id, 3);
        assert_eq!(trades[2].contract, "BNB-20240201-400-C");
        assert_eq!(trades[2].is_internal, None);
    }

    #[test]
    fn test_options_trade_empty_array_deserialization() {
        let json = r#"[]"#;
        let trades: Vec<OptionsTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 0);
    }

    #[test]
    fn test_options_trade_serialization() {
        let trade = OptionsTrade {
            id: 123456789,
            create_time: 1640995200.123,
            contract: "BTC-20240101-50000-C".to_string(),
            size: "1.5".to_string(),
            price: "0.08".to_string(),
            is_internal: Some(false),
        };

        let json = serde_json::to_value(&trade).unwrap();
        assert_eq!(json["id"], 123456789);
        assert_eq!(json["create_time"], 1640995200.123);
        assert_eq!(json["contract"], "BTC-20240101-50000-C");
        assert_eq!(json["size"], "1.5");
        assert_eq!(json["price"], "0.08");
        assert_eq!(json["is_internal"], false);
    }

    #[test]
    fn test_options_trade_serialization_without_internal() {
        let trade = OptionsTrade {
            id: 987654321,
            create_time: 1640995300.456,
            contract: "ETH-20240101-3000-P".to_string(),
            size: "2.0".to_string(),
            price: "0.05".to_string(),
            is_internal: None,
        };

        let json = serde_json::to_value(&trade).unwrap();
        assert_eq!(json["id"], 987654321);
        assert_eq!(json["create_time"], 1640995300.456);
        assert_eq!(json["contract"], "ETH-20240101-3000-P");
        assert_eq!(json["size"], "2.0");
        assert_eq!(json["price"], "0.05");
        assert!(json.get("is_internal").is_none());
    }

    #[test]
    fn test_options_trade_serialization_round_trip() {
        let original = OptionsTrade {
            id: 555666777,
            create_time: 1640995400.789,
            contract: "BNB-20240201-400-C".to_string(),
            size: "0.5".to_string(),
            price: "0.02".to_string(),
            is_internal: Some(true),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: OptionsTrade = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, original.id);
        assert_eq!(deserialized.create_time, original.create_time);
        assert_eq!(deserialized.contract, original.contract);
        assert_eq!(deserialized.size, original.size);
        assert_eq!(deserialized.price, original.price);
        assert_eq!(deserialized.is_internal, original.is_internal);
    }

    #[test]
    fn test_options_trade_realistic_scenarios() {
        // Large call option trade
        let large_call_json = r#"{
            "id": 111222333,
            "create_time": 1640995200.123,
            "contract": "BTC-20240301-60000-C",
            "size": "50.0",
            "price": "0.15",
            "is_internal": false
        }"#;

        let large_call: OptionsTrade = serde_json::from_str(large_call_json).unwrap();
        assert_eq!(large_call.contract, "BTC-20240301-60000-C");
        assert!(large_call.contract.ends_with("-C"));
        assert_eq!(large_call.size, "50.0");
        assert_eq!(large_call.price, "0.15");

        // Small put option trade
        let small_put_json = r#"{
            "id": 444555666,
            "create_time": 1640995300.456,
            "contract": "ADA-20240401-1.5-P",
            "size": "0.1",
            "price": "0.001",
            "is_internal": true
        }"#;

        let small_put: OptionsTrade = serde_json::from_str(small_put_json).unwrap();
        assert_eq!(small_put.contract, "ADA-20240401-1.5-P");
        assert!(small_put.contract.ends_with("-P"));
        assert_eq!(small_put.size, "0.1");
        assert_eq!(small_put.price, "0.001");
        assert_eq!(small_put.is_internal, Some(true));
    }

    #[test]
    fn test_options_trade_timestamp_precision() {
        let timestamps = vec![
            1640995200.0,
            1640995200.1,
            1640995200.123,
            1640995200.123456,
            1640995200.999999,
        ];

        for expected_ts in timestamps {
            let json = format!(
                r#"{{
                "id": 12345,
                "create_time": {},
                "contract": "BTC-20240101-50000-C",
                "size": "1.0",
                "price": "0.1"
            }}"#,
                expected_ts
            );

            let trade: OptionsTrade = serde_json::from_str(&json).unwrap();
            assert_eq!(trade.create_time, expected_ts);
        }
    }
}
