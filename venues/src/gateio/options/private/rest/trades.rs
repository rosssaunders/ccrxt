use serde::{Deserialize, Serialize};

use super::RestClient;

const OPTIONS_MY_TRADES_ENDPOINT: &str = "/options/my_trades";

/// Options trade record
#[derive(Debug, Clone, Deserialize)]
pub struct OptionsTrade {
    /// Trade ID
    pub id: String,

    /// Trade creation time
    pub create_time: f64,

    /// Order ID
    pub order_id: String,

    /// Contract name
    pub contract: String,

    /// Trade size
    pub size: i64,

    /// Trade price
    pub price: String,

    /// Underlying asset
    pub underlying: String,

    /// Trade role
    pub role: String,
}

/// Request to retrieve options trades
#[derive(Debug, Clone, Serialize, Default)]
pub struct OptionsTradesRequest {
    /// Underlying asset name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying: Option<String>,

    /// Contract name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,

    /// Order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    /// Maximum number of record items to be returned (1-1000)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// You can set this to the last result ID to retrieve the next page
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
    /// Get options trades
    ///
    /// This endpoint returns a list of options trades.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The trades request parameters
    ///
    /// # Returns
    /// List of trade records
    pub async fn get_options_trades(
        &self,
        request: OptionsTradesRequest,
    ) -> crate::gateio::options::RestResult<Vec<OptionsTrade>> {
        self.get_with_query(OPTIONS_MY_TRADES_ENDPOINT, &request)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_options_trades_request_minimal_serialization() {
        let request = OptionsTradesRequest::default();

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_options_trades_request_underlying_filter() {
        let request = OptionsTradesRequest {
            underlying: Some("BTC_USDT".to_string()),
            contract: None,
            order_id: None,
            limit: None,
            offset: None,
            from: None,
            to: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "underlying=BTC_USDT");
    }

    #[test]
    fn test_options_trades_request_contract_filter() {
        let request = OptionsTradesRequest {
            underlying: None,
            contract: Some("BTC-20240101-50000-C".to_string()),
            order_id: None,
            limit: None,
            offset: None,
            from: None,
            to: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "contract=BTC-20240101-50000-C");
    }

    #[test]
    fn test_options_trades_request_order_id_filter() {
        let request = OptionsTradesRequest {
            underlying: None,
            contract: None,
            order_id: Some("12345678".to_string()),
            limit: None,
            offset: None,
            from: None,
            to: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "order_id=12345678");
    }

    #[test]
    fn test_options_trades_request_pagination() {
        let request = OptionsTradesRequest {
            underlying: None,
            contract: None,
            order_id: None,
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
    fn test_options_trades_request_time_range() {
        let request = OptionsTradesRequest {
            underlying: None,
            contract: None,
            order_id: None,
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
    fn test_options_trades_request_full_parameters() {
        let request = OptionsTradesRequest {
            underlying: Some("ETH_USDT".to_string()),
            contract: Some("ETH-20240101-3000-P".to_string()),
            order_id: Some("987654321".to_string()),
            limit: Some(50),
            offset: Some(25),
            from: Some(1640995200),
            to: Some(1641081600),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("underlying=ETH_USDT"));
        assert!(serialized.contains("contract=ETH-20240101-3000-P"));
        assert!(serialized.contains("order_id=987654321"));
        assert!(serialized.contains("limit=50"));
        assert!(serialized.contains("offset=25"));
        assert!(serialized.contains("from=1640995200"));
        assert!(serialized.contains("to=1641081600"));
    }

    #[test]
    fn test_options_trades_request_negative_values() {
        let request = OptionsTradesRequest {
            underlying: None,
            contract: None,
            order_id: None,
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
    fn test_options_trades_request_extreme_values() {
        let request = OptionsTradesRequest {
            underlying: Some("BTC_USDT".to_string()),
            contract: None,
            order_id: None,
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
    fn test_options_trade_deserialization() {
        let json = r#"{
            "id": "123456789",
            "create_time": 1640995200.123,
            "order_id": "987654321",
            "contract": "BTC-20240101-50000-C",
            "size": 10,
            "price": "0.08",
            "underlying": "BTC_USDT",
            "role": "taker"
        }"#;

        let trade: OptionsTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, "123456789");
        assert_eq!(trade.create_time, 1640995200.123);
        assert_eq!(trade.order_id, "987654321");
        assert_eq!(trade.contract, "BTC-20240101-50000-C");
        assert_eq!(trade.size, 10);
        assert_eq!(trade.price, "0.08");
        assert_eq!(trade.underlying, "BTC_USDT");
        assert_eq!(trade.role, "taker");
    }

    #[test]
    fn test_options_trade_maker_role_deserialization() {
        let json = r#"{
            "id": "abcdefg12345",
            "create_time": 1640995300.456,
            "order_id": "555666777",
            "contract": "ETH-20240101-3000-P",
            "size": -5,
            "price": "0.05",
            "underlying": "ETH_USDT",
            "role": "maker"
        }"#;

        let trade: OptionsTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, "abcdefg12345");
        assert_eq!(trade.create_time, 1640995300.456);
        assert_eq!(trade.order_id, "555666777");
        assert_eq!(trade.contract, "ETH-20240101-3000-P");
        assert_eq!(trade.size, -5); // Short position
        assert_eq!(trade.price, "0.05");
        assert_eq!(trade.underlying, "ETH_USDT");
        assert_eq!(trade.role, "maker");
    }

    #[test]
    fn test_options_trade_zero_size_deserialization() {
        let json = r#"{
            "id": "000000000",
            "create_time": 1640995400.0,
            "order_id": "000000000",
            "contract": "BTC-20240215-45000-C",
            "size": 0,
            "price": "0.0",
            "underlying": "BTC_USDT",
            "role": "taker"
        }"#;

        let trade: OptionsTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, "000000000");
        assert_eq!(trade.create_time, 1640995400.0);
        assert_eq!(trade.order_id, "000000000");
        assert_eq!(trade.contract, "BTC-20240215-45000-C");
        assert_eq!(trade.size, 0);
        assert_eq!(trade.price, "0.0");
        assert_eq!(trade.underlying, "BTC_USDT");
        assert_eq!(trade.role, "taker");
    }

    #[test]
    fn test_options_trade_high_precision_deserialization() {
        let json = r#"{
            "id": "high_precision_123",
            "create_time": 1640995500.999999,
            "order_id": "hp_order_456",
            "contract": "ETH-20240301-2800-P",
            "size": 100,
            "price": "0.123456789",
            "underlying": "ETH_USDT",
            "role": "maker"
        }"#;

        let trade: OptionsTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, "high_precision_123");
        assert_eq!(trade.create_time, 1640995500.999999);
        assert_eq!(trade.order_id, "hp_order_456");
        assert_eq!(trade.contract, "ETH-20240301-2800-P");
        assert_eq!(trade.size, 100);
        assert_eq!(trade.price, "0.123456789");
        assert_eq!(trade.underlying, "ETH_USDT");
        assert_eq!(trade.role, "maker");
    }

    #[test]
    fn test_options_trade_different_contracts() {
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
                "id": "test_id_{}",
                "create_time": 1640995200.0,
                "order_id": "test_order_123",
                "contract": "{}",
                "size": 1,
                "price": "0.1",
                "underlying": "BTC_USDT",
                "role": "taker"
            }}"#,
                contract.replace("-", "_"),
                contract
            );

            let trade: OptionsTrade = serde_json::from_str(&json).unwrap();
            assert_eq!(trade.contract, contract);
            assert_eq!(trade.price, "0.1");
        }
    }

    #[test]
    fn test_options_trade_different_underlyings() {
        let underlyings = vec!["BTC_USDT", "ETH_USDT", "BNB_USDT", "SOL_USDT", "ADA_USDT"];

        for underlying in underlyings {
            let json = format!(
                r#"{{
                "id": "test_{}",
                "create_time": 1640995200.0,
                "order_id": "order_123",
                "contract": "BTC-20240101-50000-C",
                "size": 1,
                "price": "0.05",
                "underlying": "{}",
                "role": "maker"
            }}"#,
                underlying, underlying
            );

            let trade: OptionsTrade = serde_json::from_str(&json).unwrap();
            assert_eq!(trade.underlying, underlying);
            assert_eq!(trade.create_time, 1640995200.0);
        }
    }

    #[test]
    fn test_options_trade_different_roles() {
        let roles = vec!["taker", "maker"];

        for role in roles {
            let json = format!(
                r#"{{
                "id": "role_test_{}",
                "create_time": 1640995200.0,
                "order_id": "role_order_123",
                "contract": "BTC-20240101-50000-C",
                "size": 1,
                "price": "0.1",
                "underlying": "BTC_USDT",
                "role": "{}"
            }}"#,
                role, role
            );

            let trade: OptionsTrade = serde_json::from_str(&json).unwrap();
            assert_eq!(trade.role, role);
            assert_eq!(trade.id, format!("role_test_{}", role));
        }
    }

    #[test]
    fn test_options_trade_extreme_size_values() {
        // Test maximum positive size
        let large_size_json = r#"{
            "id": "large_size_trade",
            "create_time": 1640995200.0,
            "order_id": "large_order",
            "contract": "BTC-20240101-50000-C",
            "size": 9223372036854775807,
            "price": "0.1",
            "underlying": "BTC_USDT",
            "role": "taker"
        }"#;

        let large_trade: OptionsTrade = serde_json::from_str(large_size_json).unwrap();
        assert_eq!(large_trade.size, 9223372036854775807); // i64::MAX

        // Test maximum negative size (short position)
        let negative_size_json = r#"{
            "id": "negative_size_trade",
            "create_time": 1640995300.0,
            "order_id": "negative_order",
            "contract": "ETH-20240101-3000-P",
            "size": -9223372036854775808,
            "price": "0.05",
            "underlying": "ETH_USDT",
            "role": "maker"
        }"#;

        let negative_trade: OptionsTrade = serde_json::from_str(negative_size_json).unwrap();
        assert_eq!(negative_trade.size, -9223372036854775808); // i64::MIN
    }

    #[test]
    fn test_options_trade_string_id_formats() {
        let id_formats = vec![
            "123456789",
            "abc123def456",
            "trade_id_with_underscores",
            "trade-id-with-dashes",
            "TradeIdWithCapitals",
            "trade.id.with.dots",
            "0000000000000001",
        ];

        for id in id_formats {
            let json = format!(
                r#"{{
                "id": "{}",
                "create_time": 1640995200.0,
                "order_id": "order_123",
                "contract": "BTC-20240101-50000-C",
                "size": 1,
                "price": "0.1",
                "underlying": "BTC_USDT",
                "role": "taker"
            }}"#,
                id
            );

            let trade: OptionsTrade = serde_json::from_str(&json).unwrap();
            assert_eq!(trade.id, id);
        }
    }

    #[test]
    fn test_options_trade_time_precision_scenarios() {
        // Test integer timestamp
        let integer_time_json = r#"{
            "id": "integer_time",
            "create_time": 1640995200,
            "order_id": "order_123",
            "contract": "BTC-20240101-50000-C",
            "size": 1,
            "price": "0.1",
            "underlying": "BTC_USDT",
            "role": "taker"
        }"#;

        let integer_trade: OptionsTrade = serde_json::from_str(integer_time_json).unwrap();
        assert_eq!(integer_trade.create_time, 1640995200.0);

        // Test very precise timestamp
        let precise_time_json = r#"{
            "id": "precise_time",
            "create_time": 1640995200.123456789,
            "order_id": "order_456",
            "contract": "ETH-20240101-3000-P",
            "size": 1,
            "price": "0.05",
            "underlying": "ETH_USDT",
            "role": "maker"
        }"#;

        let precise_trade: OptionsTrade = serde_json::from_str(precise_time_json).unwrap();
        assert_eq!(precise_trade.create_time, 1640995200.123456789);
    }

    #[test]
    fn test_options_trade_array_deserialization() {
        let json = r#"[
            {
                "id": "trade_1",
                "create_time": 1640995200.0,
                "order_id": "order_1",
                "contract": "BTC-20240101-50000-C",
                "size": 5,
                "price": "0.08",
                "underlying": "BTC_USDT",
                "role": "taker"
            },
            {
                "id": "trade_2",
                "create_time": 1640995300.0,
                "order_id": "order_2",
                "contract": "ETH-20240101-3000-P",
                "size": -3,
                "price": "0.05",
                "underlying": "ETH_USDT",
                "role": "maker"
            },
            {
                "id": "trade_3",
                "create_time": 1640995400.0,
                "order_id": "order_3",
                "contract": "BNB-20240201-400-C",
                "size": 10,
                "price": "0.02",
                "underlying": "BNB_USDT",
                "role": "taker"
            }
        ]"#;

        let trades: Vec<OptionsTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 3);

        assert_eq!(trades[0].id, "trade_1");
        assert_eq!(trades[0].create_time, 1640995200.0);
        assert_eq!(trades[0].contract, "BTC-20240101-50000-C");
        assert_eq!(trades[0].size, 5);
        assert_eq!(trades[0].role, "taker");

        assert_eq!(trades[1].id, "trade_2");
        assert_eq!(trades[1].create_time, 1640995300.0);
        assert_eq!(trades[1].contract, "ETH-20240101-3000-P");
        assert_eq!(trades[1].size, -3);
        assert_eq!(trades[1].role, "maker");

        assert_eq!(trades[2].id, "trade_3");
        assert_eq!(trades[2].create_time, 1640995400.0);
        assert_eq!(trades[2].contract, "BNB-20240201-400-C");
        assert_eq!(trades[2].size, 10);
        assert_eq!(trades[2].role, "taker");
    }

    #[test]
    fn test_options_trade_empty_array_deserialization() {
        let json = r#"[]"#;
        let trades: Vec<OptionsTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 0);
    }

    #[test]
    fn test_options_trade_price_formats() {
        let price_formats = vec![
            "0.1",
            "0.123456789",
            "1.0",
            "999.999999999",
            "0.00000001",
            "1000000.0",
        ];

        for price in price_formats {
            let json = format!(
                r#"{{
                "id": "price_test_{}",
                "create_time": 1640995200.0,
                "order_id": "order_123",
                "contract": "BTC-20240101-50000-C",
                "size": 1,
                "price": "{}",
                "underlying": "BTC_USDT",
                "role": "taker"
            }}"#,
                price.replace(".", "_"),
                price
            );

            let trade: OptionsTrade = serde_json::from_str(&json).unwrap();
            assert_eq!(trade.price, price);
        }
    }

    #[test]
    fn test_options_trade_realistic_call_scenario() {
        let json = r#"{
            "id": "call_trade_12345",
            "create_time": 1640995200.123,
            "order_id": "buy_call_order_987",
            "contract": "BTC-20240101-45000-C",
            "size": 2,
            "price": "0.12",
            "underlying": "BTC_USDT",
            "role": "taker"
        }"#;

        let trade: OptionsTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, "call_trade_12345");
        assert_eq!(trade.contract, "BTC-20240101-45000-C");
        assert!(trade.contract.ends_with("-C")); // Call option
        assert_eq!(trade.size, 2); // Buying 2 call contracts
        assert_eq!(trade.price, "0.12"); // Premium paid
        assert_eq!(trade.role, "taker"); // Taking liquidity
        assert_eq!(trade.underlying, "BTC_USDT");
    }

    #[test]
    fn test_options_trade_realistic_put_scenario() {
        let json = r#"{
            "id": "put_trade_67890",
            "create_time": 1640995300.456,
            "order_id": "sell_put_order_456",
            "contract": "ETH-20240115-2800-P",
            "size": -1,
            "price": "0.08",
            "underlying": "ETH_USDT",
            "role": "maker"
        }"#;

        let trade: OptionsTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, "put_trade_67890");
        assert_eq!(trade.contract, "ETH-20240115-2800-P");
        assert!(trade.contract.ends_with("-P")); // Put option
        assert_eq!(trade.size, -1); // Selling/shorting 1 put contract
        assert_eq!(trade.price, "0.08"); // Premium received
        assert_eq!(trade.role, "maker"); // Providing liquidity
        assert_eq!(trade.underlying, "ETH_USDT");
    }

    #[test]
    fn test_options_trade_large_transaction_scenario() {
        let json = r#"{
            "id": "institutional_trade_999",
            "create_time": 1640995400.789,
            "order_id": "large_order_institutional",
            "contract": "BTC-20240301-50000-C",
            "size": 1000,
            "price": "0.25",
            "underlying": "BTC_USDT",
            "role": "maker"
        }"#;

        let trade: OptionsTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, "institutional_trade_999");
        assert_eq!(trade.contract, "BTC-20240301-50000-C");
        assert_eq!(trade.size, 1000); // Large institutional order
        assert_eq!(trade.price, "0.25"); // Higher premium for longer expiry
        assert_eq!(trade.role, "maker"); // Providing liquidity for large order
        assert_eq!(trade.underlying, "BTC_USDT");
    }

    #[test]
    fn test_options_trade_micro_transaction_scenario() {
        let json = r#"{
            "id": "micro_trade_abc",
            "create_time": 1640995500.001,
            "order_id": "small_order_xyz",
            "contract": "ADA-20240201-1.5-C",
            "size": 1,
            "price": "0.001",
            "underlying": "ADA_USDT",
            "role": "taker"
        }"#;

        let trade: OptionsTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, "micro_trade_abc");
        assert_eq!(trade.contract, "ADA-20240201-1.5-C");
        assert_eq!(trade.size, 1); // Small retail order
        assert_eq!(trade.price, "0.001"); // Very low premium
        assert_eq!(trade.role, "taker"); // Taking available liquidity
        assert_eq!(trade.underlying, "ADA_USDT");
    }

    #[test]
    fn test_options_trade_partial_fill_scenario() {
        let json = r#"{
            "id": "partial_fill_1_of_3",
            "create_time": 1640995600.0,
            "order_id": "large_order_partial",
            "contract": "ETH-20240215-3500-P",
            "size": 10,
            "price": "0.15",
            "underlying": "ETH_USDT",
            "role": "taker"
        }"#;

        let trade: OptionsTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, "partial_fill_1_of_3");
        assert_eq!(trade.order_id, "large_order_partial");
        assert_eq!(trade.contract, "ETH-20240215-3500-P");
        assert_eq!(trade.size, 10); // Partial fill of larger order
        assert_eq!(trade.price, "0.15");
        assert_eq!(trade.role, "taker");
        assert_eq!(trade.underlying, "ETH_USDT");
    }
}
