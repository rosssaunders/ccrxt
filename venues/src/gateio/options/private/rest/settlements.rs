use serde::{Deserialize, Serialize};

use super::RestClient;

/// Options settlement record
#[derive(Debug, Clone, Deserialize)]
pub struct OptionsSettlement {
    /// Settlement time
    pub time: i64,

    /// Underlying asset
    pub underlying: String,

    /// Contract
    pub contract: String,

    /// Settlement profit
    pub profit: String,

    /// Fee
    pub fee: String,

    /// Settlement price
    pub settle_price: String,

    /// Strike price
    pub strike_price: String,

    /// Size
    pub size: i64,
}

/// Request to retrieve options settlements
#[derive(Debug, Clone, Serialize, Default)]
pub struct OptionsSettlementsRequest {
    /// Underlying asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying: Option<String>,

    /// Contract name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,

    /// Page number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,

    /// Number of results per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Start time (seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End time (seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
}

impl RestClient {
    /// Get options settlements
    ///
    /// This endpoint returns a list of options settlement records.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The settlement request parameters
    ///
    /// # Returns
    /// List of settlement records
    pub async fn get_options_settlements(
        &self,
        request: OptionsSettlementsRequest,
    ) -> crate::gateio::options::Result<Vec<OptionsSettlement>> {
        self.get_with_query("/options/settlements", &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_options_settlements_request_minimal_serialization() {
        let request = OptionsSettlementsRequest::default();

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_options_settlements_request_underlying_filter() {
        let request = OptionsSettlementsRequest {
            underlying: Some("BTC_USDT".to_string()),
            contract: None,
            page: None,
            limit: None,
            from: None,
            to: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "underlying=BTC_USDT");
    }

    #[test]
    fn test_options_settlements_request_contract_filter() {
        let request = OptionsSettlementsRequest {
            underlying: None,
            contract: Some("BTC-20240101-50000-C".to_string()),
            page: None,
            limit: None,
            from: None,
            to: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "contract=BTC-20240101-50000-C");
    }

    #[test]
    fn test_options_settlements_request_pagination() {
        let request = OptionsSettlementsRequest {
            underlying: None,
            contract: None,
            page: Some(2),
            limit: Some(50),
            from: None,
            to: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("page=2"));
        assert!(serialized.contains("limit=50"));
    }

    #[test]
    fn test_options_settlements_request_time_range() {
        let request = OptionsSettlementsRequest {
            underlying: None,
            contract: None,
            page: None,
            limit: None,
            from: Some(1640995200),
            to: Some(1641081600),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("from=1640995200"));
        assert!(serialized.contains("to=1641081600"));
    }

    #[test]
    fn test_options_settlements_request_full_parameters() {
        let request = OptionsSettlementsRequest {
            underlying: Some("ETH_USDT".to_string()),
            contract: Some("ETH-20240101-3000-P".to_string()),
            page: Some(1),
            limit: Some(25),
            from: Some(1640995200),
            to: Some(1641081600),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("underlying=ETH_USDT"));
        assert!(serialized.contains("contract=ETH-20240101-3000-P"));
        assert!(serialized.contains("page=1"));
        assert!(serialized.contains("limit=25"));
        assert!(serialized.contains("from=1640995200"));
        assert!(serialized.contains("to=1641081600"));
    }

    #[test]
    fn test_options_settlements_request_negative_values() {
        let request = OptionsSettlementsRequest {
            underlying: None,
            contract: None,
            page: Some(-1),
            limit: Some(-10),
            from: Some(-1640995200),
            to: Some(-1641081600),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("page=-1"));
        assert!(serialized.contains("limit=-10"));
        assert!(serialized.contains("from=-1640995200"));
        assert!(serialized.contains("to=-1641081600"));
    }

    #[test]
    fn test_options_settlements_request_extreme_values() {
        let request = OptionsSettlementsRequest {
            underlying: Some("BTC_USDT".to_string()),
            contract: None,
            page: Some(i32::MAX),
            limit: Some(i32::MAX),
            from: Some(i64::MIN),
            to: Some(i64::MAX),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("underlying=BTC_USDT"));
        assert!(serialized.contains(&format!("page={}", i32::MAX)));
        assert!(serialized.contains(&format!("limit={}", i32::MAX)));
        assert!(serialized.contains(&format!("from={}", i64::MIN)));
        assert!(serialized.contains(&format!("to={}", i64::MAX)));
    }

    #[test]
    fn test_options_settlement_deserialization() {
        let json = r#"{
            "time": 1640995200,
            "underlying": "BTC_USDT",
            "contract": "BTC-20240101-50000-C",
            "profit": "250.75",
            "fee": "2.5",
            "settle_price": "52000.00",
            "strike_price": "50000.00",
            "size": 10
        }"#;

        let settlement: OptionsSettlement = serde_json::from_str(json).unwrap();
        assert_eq!(settlement.time, 1640995200);
        assert_eq!(settlement.underlying, "BTC_USDT");
        assert_eq!(settlement.contract, "BTC-20240101-50000-C");
        assert_eq!(settlement.profit, "250.75");
        assert_eq!(settlement.fee, "2.5");
        assert_eq!(settlement.settle_price, "52000.00");
        assert_eq!(settlement.strike_price, "50000.00");
        assert_eq!(settlement.size, 10);
    }

    #[test]
    fn test_options_settlement_negative_profit_deserialization() {
        let json = r#"{
            "time": 1640995300,
            "underlying": "ETH_USDT",
            "contract": "ETH-20240101-3500-P",
            "profit": "-125.50",
            "fee": "1.25",
            "settle_price": "3200.00",
            "strike_price": "3500.00",
            "size": -5
        }"#;

        let settlement: OptionsSettlement = serde_json::from_str(json).unwrap();
        assert_eq!(settlement.time, 1640995300);
        assert_eq!(settlement.underlying, "ETH_USDT");
        assert_eq!(settlement.contract, "ETH-20240101-3500-P");
        assert_eq!(settlement.profit, "-125.50");
        assert_eq!(settlement.fee, "1.25");
        assert_eq!(settlement.settle_price, "3200.00");
        assert_eq!(settlement.strike_price, "3500.00");
        assert_eq!(settlement.size, -5);
    }

    #[test]
    fn test_options_settlement_zero_profit_deserialization() {
        let json = r#"{
            "time": 1640995400,
            "underlying": "BTC_USDT",
            "contract": "BTC-20240101-50000-C",
            "profit": "0.0",
            "fee": "0.5",
            "settle_price": "50000.00",
            "strike_price": "50000.00",
            "size": 1
        }"#;

        let settlement: OptionsSettlement = serde_json::from_str(json).unwrap();
        assert_eq!(settlement.time, 1640995400);
        assert_eq!(settlement.underlying, "BTC_USDT");
        assert_eq!(settlement.contract, "BTC-20240101-50000-C");
        assert_eq!(settlement.profit, "0.0");
        assert_eq!(settlement.fee, "0.5");
        assert_eq!(settlement.settle_price, "50000.00");
        assert_eq!(settlement.strike_price, "50000.00");
        assert_eq!(settlement.size, 1);
    }

    #[test]
    fn test_options_settlement_high_precision_deserialization() {
        let json = r#"{
            "time": 1640995500,
            "underlying": "ETH_USDT",
            "contract": "ETH-20240301-2800-P",
            "profit": "1234.567890123",
            "fee": "12.34567890",
            "settle_price": "2999.123456789",
            "strike_price": "2800.000000000",
            "size": 100
        }"#;

        let settlement: OptionsSettlement = serde_json::from_str(json).unwrap();
        assert_eq!(settlement.time, 1640995500);
        assert_eq!(settlement.underlying, "ETH_USDT");
        assert_eq!(settlement.contract, "ETH-20240301-2800-P");
        assert_eq!(settlement.profit, "1234.567890123");
        assert_eq!(settlement.fee, "12.34567890");
        assert_eq!(settlement.settle_price, "2999.123456789");
        assert_eq!(settlement.strike_price, "2800.000000000");
        assert_eq!(settlement.size, 100);
    }

    #[test]
    fn test_options_settlement_different_contracts() {
        let contracts = vec![
            "BTC-20240101-50000-C",
            "ETH-20240215-3000-P",
            "BNB-20240301-400-C",
            "SOL-20240315-150-P",
            "ADA-20240401-1-C"
        ];
        
        for contract in contracts {
            let json = format!(r#"{{
                "time": 1640995200,
                "underlying": "BTC_USDT",
                "contract": "{}",
                "profit": "100.0",
                "fee": "1.0",
                "settle_price": "50000.00",
                "strike_price": "45000.00",
                "size": 1
            }}"#, contract);

            let settlement: OptionsSettlement = serde_json::from_str(&json).unwrap();
            assert_eq!(settlement.contract, contract);
            assert_eq!(settlement.profit, "100.0");
        }
    }

    #[test]
    fn test_options_settlement_different_underlyings() {
        let underlyings = vec!["BTC_USDT", "ETH_USDT", "BNB_USDT", "SOL_USDT", "ADA_USDT"];
        
        for underlying in underlyings {
            let json = format!(r#"{{
                "time": 1640995200,
                "underlying": "{}",
                "contract": "BTC-20240101-50000-C",
                "profit": "50.0",
                "fee": "0.5",
                "settle_price": "52000.00",
                "strike_price": "50000.00",
                "size": 1
            }}"#, underlying);

            let settlement: OptionsSettlement = serde_json::from_str(&json).unwrap();
            assert_eq!(settlement.underlying, underlying);
            assert_eq!(settlement.time, 1640995200);
        }
    }

    #[test]
    fn test_options_settlement_call_option_itm_scenario() {
        // Call option in-the-money: settle_price > strike_price
        let json = r#"{
            "time": 1640995200,
            "underlying": "BTC_USDT",
            "contract": "BTC-20240101-45000-C",
            "profit": "5000.0",
            "fee": "50.0",
            "settle_price": "50000.00",
            "strike_price": "45000.00",
            "size": 1
        }"#;

        let settlement: OptionsSettlement = serde_json::from_str(json).unwrap();
        assert_eq!(settlement.contract, "BTC-20240101-45000-C");
        assert!(settlement.contract.ends_with("-C")); // Call option
        assert_eq!(settlement.profit, "5000.0"); // Positive profit for ITM call
        assert_eq!(settlement.settle_price, "50000.00");
        assert_eq!(settlement.strike_price, "45000.00");
        // settle_price (50000) > strike_price (45000) = ITM call
    }

    #[test]
    fn test_options_settlement_put_option_itm_scenario() {
        // Put option in-the-money: settle_price < strike_price
        let json = r#"{
            "time": 1640995300,
            "underlying": "ETH_USDT",
            "contract": "ETH-20240101-3500-P",
            "profit": "300.0",
            "fee": "3.0",
            "settle_price": "3200.00",
            "strike_price": "3500.00",
            "size": 1
        }"#;

        let settlement: OptionsSettlement = serde_json::from_str(json).unwrap();
        assert_eq!(settlement.contract, "ETH-20240101-3500-P");
        assert!(settlement.contract.ends_with("-P")); // Put option
        assert_eq!(settlement.profit, "300.0"); // Positive profit for ITM put
        assert_eq!(settlement.settle_price, "3200.00");
        assert_eq!(settlement.strike_price, "3500.00");
        // settle_price (3200) < strike_price (3500) = ITM put
    }

    #[test]
    fn test_options_settlement_otm_expiry_scenario() {
        // Out-of-the-money option expiring worthless
        let json = r#"{
            "time": 1640995400,
            "underlying": "BTC_USDT",
            "contract": "BTC-20240101-60000-C",
            "profit": "-500.0",
            "fee": "5.0",
            "settle_price": "50000.00",
            "strike_price": "60000.00",
            "size": 1
        }"#;

        let settlement: OptionsSettlement = serde_json::from_str(json).unwrap();
        assert_eq!(settlement.contract, "BTC-20240101-60000-C");
        assert!(settlement.contract.ends_with("-C")); // Call option
        assert_eq!(settlement.profit, "-500.0"); // Loss for OTM call
        assert_eq!(settlement.settle_price, "50000.00");
        assert_eq!(settlement.strike_price, "60000.00");
        // settle_price (50000) < strike_price (60000) = OTM call (expires worthless)
    }

    #[test]
    fn test_options_settlement_short_position_scenario() {
        // Short position settlement
        let json = r#"{
            "time": 1640995500,
            "underlying": "ETH_USDT",
            "contract": "ETH-20240101-4000-C",
            "profit": "200.0",
            "fee": "2.0",
            "settle_price": "3500.00",
            "strike_price": "4000.00",
            "size": -2
        }"#;

        let settlement: OptionsSettlement = serde_json::from_str(json).unwrap();
        assert_eq!(settlement.contract, "ETH-20240101-4000-C");
        assert_eq!(settlement.profit, "200.0"); // Profit from short position
        assert_eq!(settlement.size, -2); // Negative size indicates short position
        assert_eq!(settlement.settle_price, "3500.00");
        assert_eq!(settlement.strike_price, "4000.00");
        // settle_price < strike_price = OTM call expires worthless (profit for short seller)
    }

    #[test]
    fn test_options_settlement_extreme_timestamps() {
        // Test extreme timestamp values
        let json = r#"{
            "time": 9223372036854775807,
            "underlying": "BTC_USDT",
            "contract": "BTC-20240101-50000-C",
            "profit": "100.0",
            "fee": "1.0",
            "settle_price": "52000.00",
            "strike_price": "50000.00",
            "size": 1
        }"#;

        let settlement: OptionsSettlement = serde_json::from_str(json).unwrap();
        assert_eq!(settlement.time, 9223372036854775807); // i64::MAX
        assert_eq!(settlement.underlying, "BTC_USDT");
    }

    #[test]
    fn test_options_settlement_extreme_size_values() {
        let json = r#"{
            "time": 1640995200,
            "underlying": "BTC_USDT",
            "contract": "BTC-20240101-50000-C",
            "profit": "1000000.0",
            "fee": "10000.0",
            "settle_price": "52000.00",
            "strike_price": "50000.00",
            "size": 9223372036854775807
        }"#;

        let settlement: OptionsSettlement = serde_json::from_str(json).unwrap();
        assert_eq!(settlement.size, 9223372036854775807); // i64::MAX
        assert_eq!(settlement.profit, "1000000.0");
    }

    #[test]
    fn test_options_settlement_array_deserialization() {
        let json = r#"[
            {
                "time": 1640995200,
                "underlying": "BTC_USDT",
                "contract": "BTC-20240101-50000-C",
                "profit": "500.0",
                "fee": "5.0",
                "settle_price": "52000.00",
                "strike_price": "50000.00",
                "size": 1
            },
            {
                "time": 1640995300,
                "underlying": "ETH_USDT",
                "contract": "ETH-20240101-3000-P",
                "profit": "-200.0",
                "fee": "2.0",
                "settle_price": "3200.00",
                "strike_price": "3000.00",
                "size": -1
            },
            {
                "time": 1640995400,
                "underlying": "BNB_USDT",
                "contract": "BNB-20240201-400-C",
                "profit": "0.0",
                "fee": "1.0",
                "settle_price": "400.00",
                "strike_price": "400.00",
                "size": 5
            }
        ]"#;

        let settlements: Vec<OptionsSettlement> = serde_json::from_str(json).unwrap();
        assert_eq!(settlements.len(), 3);
        
        assert_eq!(settlements[0].time, 1640995200);
        assert_eq!(settlements[0].underlying, "BTC_USDT");
        assert_eq!(settlements[0].profit, "500.0");
        assert_eq!(settlements[0].size, 1);
        
        assert_eq!(settlements[1].time, 1640995300);
        assert_eq!(settlements[1].underlying, "ETH_USDT");
        assert_eq!(settlements[1].profit, "-200.0");
        assert_eq!(settlements[1].size, -1);
        
        assert_eq!(settlements[2].time, 1640995400);
        assert_eq!(settlements[2].underlying, "BNB_USDT");
        assert_eq!(settlements[2].profit, "0.0");
        assert_eq!(settlements[2].size, 5);
    }

    #[test]
    fn test_options_settlement_empty_array_deserialization() {
        let json = r#"[]"#;
        let settlements: Vec<OptionsSettlement> = serde_json::from_str(json).unwrap();
        assert_eq!(settlements.len(), 0);
    }

    #[test]
    fn test_options_settlement_large_numbers() {
        let json = r#"{
            "time": 1640995200,
            "underlying": "BTC_USDT",
            "contract": "BTC-20240101-50000-C",
            "profit": "999999999.99999999",
            "fee": "99999.99999999",
            "settle_price": "999999.99999999",
            "strike_price": "888888.88888888",
            "size": 1000000
        }"#;

        let settlement: OptionsSettlement = serde_json::from_str(json).unwrap();
        assert_eq!(settlement.time, 1640995200);
        assert_eq!(settlement.underlying, "BTC_USDT");
        assert_eq!(settlement.contract, "BTC-20240101-50000-C");
        assert_eq!(settlement.profit, "999999999.99999999");
        assert_eq!(settlement.fee, "99999.99999999");
        assert_eq!(settlement.settle_price, "999999.99999999");
        assert_eq!(settlement.strike_price, "888888.88888888");
        assert_eq!(settlement.size, 1000000);
    }

    #[test]
    fn test_options_settlement_small_numbers() {
        let json = r#"{
            "time": 1640995200,
            "underlying": "ADA_USDT",
            "contract": "ADA-20240101-1-C",
            "profit": "0.00000001",
            "fee": "0.00000001",
            "settle_price": "1.00000001",
            "strike_price": "1.00000000",
            "size": 1
        }"#;

        let settlement: OptionsSettlement = serde_json::from_str(json).unwrap();
        assert_eq!(settlement.time, 1640995200);
        assert_eq!(settlement.underlying, "ADA_USDT");
        assert_eq!(settlement.contract, "ADA-20240101-1-C");
        assert_eq!(settlement.profit, "0.00000001");
        assert_eq!(settlement.fee, "0.00000001");
        assert_eq!(settlement.settle_price, "1.00000001");
        assert_eq!(settlement.strike_price, "1.00000000");
        assert_eq!(settlement.size, 1);
    }

    #[test]
    fn test_options_settlement_realistic_scenarios() {
        // Test a realistic Bitcoin call option settlement scenario
        let btc_scenario = r#"{
            "time": 1704067200,
            "underlying": "BTC_USDT",
            "contract": "BTC-20240101-42000-C",
            "profit": "3500.0",
            "fee": "35.0",
            "settle_price": "45500.00",
            "strike_price": "42000.00",
            "size": 1
        }"#;

        let btc_settlement: OptionsSettlement = serde_json::from_str(btc_scenario).unwrap();
        assert_eq!(btc_settlement.underlying, "BTC_USDT");
        assert!(btc_settlement.contract.contains("-42000-C"));
        assert_eq!(btc_settlement.profit, "3500.0");
        assert_eq!(btc_settlement.settle_price, "45500.00");
        assert_eq!(btc_settlement.strike_price, "42000.00");
        // Profit = (45500 - 42000) * size - fee = 3500 - 35 = 3465 (approximately matches recorded profit)

        // Test a realistic Ethereum put option settlement scenario
        let eth_scenario = r#"{
            "time": 1704067200,
            "underlying": "ETH_USDT",
            "contract": "ETH-20240101-2800-P",
            "profit": "300.0",
            "fee": "3.0",
            "settle_price": "2500.00",
            "strike_price": "2800.00",
            "size": 1
        }"#;

        let eth_settlement: OptionsSettlement = serde_json::from_str(eth_scenario).unwrap();
        assert_eq!(eth_settlement.underlying, "ETH_USDT");
        assert!(eth_settlement.contract.contains("-2800-P"));
        assert_eq!(eth_settlement.profit, "300.0");
        assert_eq!(eth_settlement.settle_price, "2500.00");
        assert_eq!(eth_settlement.strike_price, "2800.00");
        // Put ITM: strike_price (2800) > settle_price (2500), profit = 300 - 3 = 297 (approximately matches)
    }
}
