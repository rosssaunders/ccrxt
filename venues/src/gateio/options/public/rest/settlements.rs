use serde::{Deserialize, Serialize};

use super::RestClient;

const ENDPOINT_OPTIONS_SETTLEMENTS: &str = "/options/settlements";

/// Request parameters for options settlements
#[derive(Debug, Clone, Serialize, Default)]
pub struct OptionsSettlementsRequest {
    /// Underlying asset filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying: Option<String>,

    /// Maximum number of records to return (1-1000, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Options settlement information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsSettlement {
    /// Settlement time
    pub time: i64,

    /// Contract name
    pub contract: String,

    /// Underlying asset
    pub underlying: Option<String>,

    /// Strike price (quote currency)
    pub strike_price: String,

    /// Settlement price (quote currency)
    pub settle_price: String,
}

impl RestClient {
    /// List settlement history
    ///
    /// Retrieves settlement history for options contracts.
    ///
    /// [docs](https://www.gate.com/docs/developers/apiv4/#list-settlement-history)
    pub async fn get_options_settlements(
        &self,
        params: OptionsSettlementsRequest,
    ) -> crate::gateio::options::RestResult<Vec<OptionsSettlement>> {
        self.get_with_query(ENDPOINT_OPTIONS_SETTLEMENTS, Some(&params))
            .await
    }

    /// Get specified contract's settlement
    ///
    /// Retrieves settlement information for a specific options contract.
    ///
    /// [docs](https://www.gate.com/docs/developers/apiv4/#get-specified-contract-s-settlement)
    pub async fn get_options_contract_settlement(
        &self,
        contract: &str,
    ) -> crate::gateio::options::RestResult<OptionsSettlement> {
        let endpoint = format!("{}/{}", ENDPOINT_OPTIONS_SETTLEMENTS, contract);
        self.get(&endpoint).await
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
    fn test_options_settlements_request_with_underlying() {
        let request = OptionsSettlementsRequest {
            underlying: Some("BTC_USDT".to_string()),
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "underlying=BTC_USDT");
    }

    #[test]
    fn test_options_settlements_request_with_limit() {
        let request = OptionsSettlementsRequest {
            underlying: None,
            limit: Some(50),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "limit=50");
    }

    #[test]
    fn test_options_settlements_request_full_parameters() {
        let request = OptionsSettlementsRequest {
            underlying: Some("ETH_USDT".to_string()),
            limit: Some(100),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("underlying=ETH_USDT"));
        assert!(serialized.contains("limit=100"));
    }

    #[test]
    fn test_options_settlements_request_different_underlyings() {
        let underlyings = vec!["BTC_USDT", "ETH_USDT", "BNB_USDT", "SOL_USDT", "ADA_USDT"];

        for underlying in underlyings {
            let request = OptionsSettlementsRequest {
                underlying: Some(underlying.to_string()),
                limit: None,
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert_eq!(serialized, format!("underlying={}", underlying));
        }
    }

    #[test]
    fn test_options_settlements_request_limit_ranges() {
        let limits = vec![1, 50, 100, 500, 1000];

        for limit in limits {
            let request = OptionsSettlementsRequest {
                underlying: None,
                limit: Some(limit),
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert_eq!(serialized, format!("limit={}", limit));
        }
    }

    #[test]
    fn test_options_settlements_request_negative_limit() {
        let request = OptionsSettlementsRequest {
            underlying: None,
            limit: Some(-10),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "limit=-10");
    }

    #[test]
    fn test_options_settlements_request_extreme_limit() {
        let request = OptionsSettlementsRequest {
            underlying: None,
            limit: Some(i32::MAX),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, format!("limit={}", i32::MAX));
    }

    #[test]
    fn test_options_settlement_call_option_deserialization() {
        let json = r#"{
            "time": 1640995200,
            "contract": "BTC-20240101-50000-C",
            "underlying": "BTC_USDT",
            "strike_price": "50000.00",
            "settle_price": "45000.00"
        }"#;

        let settlement: OptionsSettlement = serde_json::from_str(json).unwrap();
        assert_eq!(settlement.time, 1640995200);
        assert_eq!(settlement.contract, "BTC-20240101-50000-C");
        assert_eq!(settlement.underlying, Some("BTC_USDT".to_string()));
        assert_eq!(settlement.strike_price, "50000.00");
        assert_eq!(settlement.settle_price, "45000.00");
    }

    #[test]
    fn test_options_settlement_put_option_deserialization() {
        let json = r#"{
            "time": 1640995300,
            "contract": "ETH-20240101-3000-P",
            "underlying": "ETH_USDT",
            "strike_price": "3000.00",
            "settle_price": "3200.50"
        }"#;

        let settlement: OptionsSettlement = serde_json::from_str(json).unwrap();
        assert_eq!(settlement.time, 1640995300);
        assert_eq!(settlement.contract, "ETH-20240101-3000-P");
        assert_eq!(settlement.underlying, Some("ETH_USDT".to_string()));
        assert_eq!(settlement.strike_price, "3000.00");
        assert_eq!(settlement.settle_price, "3200.50");
    }

    #[test]
    fn test_options_settlement_without_underlying() {
        let json = r#"{
            "time": 1640995400,
            "contract": "BNB-20240201-400-C",
            "strike_price": "400.00",
            "settle_price": "420.75"
        }"#;

        let settlement: OptionsSettlement = serde_json::from_str(json).unwrap();
        assert_eq!(settlement.time, 1640995400);
        assert_eq!(settlement.contract, "BNB-20240201-400-C");
        assert_eq!(settlement.underlying, None);
        assert_eq!(settlement.strike_price, "400.00");
        assert_eq!(settlement.settle_price, "420.75");
    }

    #[test]
    fn test_options_settlement_high_precision_prices() {
        let json = r#"{
            "time": 1640995500,
            "contract": "SOL-20240215-150-P",
            "underlying": "SOL_USDT",
            "strike_price": "150.123456789",
            "settle_price": "145.987654321"
        }"#;

        let settlement: OptionsSettlement = serde_json::from_str(json).unwrap();
        assert_eq!(settlement.time, 1640995500);
        assert_eq!(settlement.contract, "SOL-20240215-150-P");
        assert_eq!(settlement.underlying, Some("SOL_USDT".to_string()));
        assert_eq!(settlement.strike_price, "150.123456789");
        assert_eq!(settlement.settle_price, "145.987654321");
    }

    #[test]
    fn test_options_settlement_zero_prices() {
        let json = r#"{
            "time": 1640995600,
            "contract": "ADA-20240301-1-C",
            "underlying": "ADA_USDT",
            "strike_price": "0",
            "settle_price": "0"
        }"#;

        let settlement: OptionsSettlement = serde_json::from_str(json).unwrap();
        assert_eq!(settlement.time, 1640995600);
        assert_eq!(settlement.contract, "ADA-20240301-1-C");
        assert_eq!(settlement.underlying, Some("ADA_USDT".to_string()));
        assert_eq!(settlement.strike_price, "0");
        assert_eq!(settlement.settle_price, "0");
    }

    #[test]
    fn test_options_settlement_negative_timestamp() {
        let json = r#"{
            "time": -1640995200,
            "contract": "BTC-20240101-50000-C",
            "strike_price": "50000.00",
            "settle_price": "45000.00"
        }"#;

        let settlement: OptionsSettlement = serde_json::from_str(json).unwrap();
        assert_eq!(settlement.time, -1640995200);
        assert_eq!(settlement.contract, "BTC-20240101-50000-C");
        assert_eq!(settlement.strike_price, "50000.00");
        assert_eq!(settlement.settle_price, "45000.00");
    }

    #[test]
    fn test_options_settlement_large_timestamp() {
        let json = format!(
            r#"{{
            "time": {},
            "contract": "ETH-20240315-3500-P",
            "strike_price": "3500.00",
            "settle_price": "3400.00"
        }}"#,
            i64::MAX
        );

        let settlement: OptionsSettlement = serde_json::from_str(&json).unwrap();
        assert_eq!(settlement.time, i64::MAX);
        assert_eq!(settlement.contract, "ETH-20240315-3500-P");
        assert_eq!(settlement.strike_price, "3500.00");
        assert_eq!(settlement.settle_price, "3400.00");
    }

    #[test]
    fn test_options_settlement_array_deserialization() {
        let json = r#"[
            {
                "time": 1640995200,
                "contract": "BTC-20240101-50000-C",
                "underlying": "BTC_USDT",
                "strike_price": "50000.00",
                "settle_price": "45000.00"
            },
            {
                "time": 1640995300,
                "contract": "ETH-20240101-3000-P",
                "underlying": "ETH_USDT",
                "strike_price": "3000.00",
                "settle_price": "3200.50"
            },
            {
                "time": 1640995400,
                "contract": "BNB-20240201-400-C",
                "strike_price": "400.00",
                "settle_price": "420.75"
            }
        ]"#;

        let settlements: Vec<OptionsSettlement> = serde_json::from_str(json).unwrap();
        assert_eq!(settlements.len(), 3);

        assert_eq!(settlements[0].time, 1640995200);
        assert_eq!(settlements[0].contract, "BTC-20240101-50000-C");
        assert_eq!(settlements[0].underlying, Some("BTC_USDT".to_string()));
        assert_eq!(settlements[0].strike_price, "50000.00");
        assert_eq!(settlements[0].settle_price, "45000.00");

        assert_eq!(settlements[1].time, 1640995300);
        assert_eq!(settlements[1].contract, "ETH-20240101-3000-P");
        assert_eq!(settlements[1].underlying, Some("ETH_USDT".to_string()));
        assert_eq!(settlements[1].strike_price, "3000.00");
        assert_eq!(settlements[1].settle_price, "3200.50");

        assert_eq!(settlements[2].time, 1640995400);
        assert_eq!(settlements[2].contract, "BNB-20240201-400-C");
        assert_eq!(settlements[2].underlying, None);
        assert_eq!(settlements[2].strike_price, "400.00");
        assert_eq!(settlements[2].settle_price, "420.75");
    }

    #[test]
    fn test_options_settlement_empty_array_deserialization() {
        let json = r#"[]"#;
        let settlements: Vec<OptionsSettlement> = serde_json::from_str(json).unwrap();
        assert_eq!(settlements.len(), 0);
    }

    #[test]
    fn test_options_settlement_serialization() {
        let settlement = OptionsSettlement {
            time: 1640995200,
            contract: "BTC-20240101-50000-C".to_string(),
            underlying: Some("BTC_USDT".to_string()),
            strike_price: "50000.00".to_string(),
            settle_price: "45000.00".to_string(),
        };

        let json = serde_json::to_value(&settlement).unwrap();
        assert_eq!(json["time"], 1640995200);
        assert_eq!(json["contract"], "BTC-20240101-50000-C");
        assert_eq!(json["underlying"], "BTC_USDT");
        assert_eq!(json["strike_price"], "50000.00");
        assert_eq!(json["settle_price"], "45000.00");
    }

    #[test]
    fn test_options_settlement_serialization_round_trip() {
        let original = OptionsSettlement {
            time: 1640995300,
            contract: "ETH-20240101-3000-P".to_string(),
            underlying: Some("ETH_USDT".to_string()),
            strike_price: "3000.123456".to_string(),
            settle_price: "3200.987654".to_string(),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: OptionsSettlement = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.time, original.time);
        assert_eq!(deserialized.contract, original.contract);
        assert_eq!(deserialized.underlying, original.underlying);
        assert_eq!(deserialized.strike_price, original.strike_price);
        assert_eq!(deserialized.settle_price, original.settle_price);
    }

    #[test]
    fn test_options_settlement_itm_call_scenario() {
        // Call option that expires in-the-money (settle > strike)
        let json = r#"{
            "time": 1704067200,
            "contract": "BTC-20240101-40000-C",
            "underlying": "BTC_USDT",
            "strike_price": "40000.00",
            "settle_price": "45000.00"
        }"#;

        let settlement: OptionsSettlement = serde_json::from_str(json).unwrap();
        assert_eq!(settlement.contract, "BTC-20240101-40000-C");
        assert!(settlement.contract.ends_with("-C"));

        // Parse prices to verify ITM status
        let strike: f64 = settlement.strike_price.parse().unwrap();
        let settle: f64 = settlement.settle_price.parse().unwrap();
        assert!(settle > strike); // ITM for call
    }

    #[test]
    fn test_options_settlement_itm_put_scenario() {
        // Put option that expires in-the-money (settle < strike)
        let json = r#"{
            "time": 1704067200,
            "contract": "ETH-20240101-3500-P",
            "underlying": "ETH_USDT",
            "strike_price": "3500.00",
            "settle_price": "3200.00"
        }"#;

        let settlement: OptionsSettlement = serde_json::from_str(json).unwrap();
        assert_eq!(settlement.contract, "ETH-20240101-3500-P");
        assert!(settlement.contract.ends_with("-P"));

        // Parse prices to verify ITM status
        let strike: f64 = settlement.strike_price.parse().unwrap();
        let settle: f64 = settlement.settle_price.parse().unwrap();
        assert!(settle < strike); // ITM for put
    }

    #[test]
    fn test_options_settlement_otm_scenarios() {
        // OTM call option (settle < strike)
        let otm_call_json = r#"{
            "time": 1704067200,
            "contract": "BTC-20240101-60000-C",
            "underlying": "BTC_USDT",
            "strike_price": "60000.00",
            "settle_price": "45000.00"
        }"#;

        let otm_call: OptionsSettlement = serde_json::from_str(otm_call_json).unwrap();
        let call_strike: f64 = otm_call.strike_price.parse().unwrap();
        let call_settle: f64 = otm_call.settle_price.parse().unwrap();
        assert!(call_settle < call_strike); // OTM for call

        // OTM put option (settle > strike)
        let otm_put_json = r#"{
            "time": 1704067200,
            "contract": "ETH-20240101-2800-P",
            "underlying": "ETH_USDT",
            "strike_price": "2800.00",
            "settle_price": "3200.00"
        }"#;

        let otm_put: OptionsSettlement = serde_json::from_str(otm_put_json).unwrap();
        let put_strike: f64 = otm_put.strike_price.parse().unwrap();
        let put_settle: f64 = otm_put.settle_price.parse().unwrap();
        assert!(put_settle > put_strike); // OTM for put
    }

    #[test]
    fn test_options_settlement_atm_scenarios() {
        // At-the-money call option (settle = strike)
        let atm_call_json = r#"{
            "time": 1704067200,
            "contract": "BTC-20240101-45000-C",
            "underlying": "BTC_USDT",
            "strike_price": "45000.00",
            "settle_price": "45000.00"
        }"#;

        let atm_call: OptionsSettlement = serde_json::from_str(atm_call_json).unwrap();
        let call_strike: f64 = atm_call.strike_price.parse().unwrap();
        let call_settle: f64 = atm_call.settle_price.parse().unwrap();
        assert_eq!(call_settle, call_strike); // ATM

        // At-the-money put option (settle = strike)
        let atm_put_json = r#"{
            "time": 1704067200,
            "contract": "ETH-20240101-3200-P",
            "underlying": "ETH_USDT",
            "strike_price": "3200.00",
            "settle_price": "3200.00"
        }"#;

        let atm_put: OptionsSettlement = serde_json::from_str(atm_put_json).unwrap();
        let put_strike: f64 = atm_put.strike_price.parse().unwrap();
        let put_settle: f64 = atm_put.settle_price.parse().unwrap();
        assert_eq!(put_settle, put_strike); // ATM
    }

    #[test]
    fn test_options_settlement_different_contract_types() {
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
                "time": 1704067200,
                "contract": "{}",
                "underlying": "TEST_USDT",
                "strike_price": "100.00",
                "settle_price": "105.00"
            }}"#,
                contract
            );

            let settlement: OptionsSettlement = serde_json::from_str(&json).unwrap();
            assert_eq!(settlement.contract, contract);

            if option_type == "call" {
                assert!(settlement.contract.ends_with("-C"));
            } else {
                assert!(settlement.contract.ends_with("-P"));
            }
        }
    }

    #[test]
    fn test_options_settlement_expiration_times() {
        let expiration_times = vec![
            (1704067200, "2024-01-01"), // Monday
            (1704499200, "2024-01-05"), // Friday
            (1706745600, "2024-01-31"), // End of month
            (1711929600, "2024-03-31"), // Quarterly
            (1719792000, "2024-06-30"), // Half-yearly
        ];

        for (timestamp, _description) in expiration_times {
            let json = format!(
                r#"{{
                "time": {},
                "contract": "BTC-20240101-50000-C",
                "underlying": "BTC_USDT",
                "strike_price": "50000.00",
                "settle_price": "45000.00"
            }}"#,
                timestamp
            );

            let settlement: OptionsSettlement = serde_json::from_str(&json).unwrap();
            assert_eq!(settlement.time, timestamp);

            // Verify timestamp is reasonable (after 2020 and before 2030)
            assert!(timestamp > 1577836800); // 2020-01-01
            assert!(timestamp < 1893456000); // 2030-01-01
        }
    }

    #[test]
    fn test_options_settlement_edge_cases() {
        // Extreme strike and settle prices
        let extreme_json = r#"{
            "time": 1640995200,
            "contract": "EXTREME-20240101-999999-C",
            "strike_price": "999999.999999999",
            "settle_price": "0.000000001"
        }"#;

        let extreme: OptionsSettlement = serde_json::from_str(extreme_json).unwrap();
        assert_eq!(extreme.strike_price, "999999.999999999");
        assert_eq!(extreme.settle_price, "0.000000001");

        // Very small decimal prices
        let small_json = r#"{
            "time": 1640995200,
            "contract": "SMALL-20240101-0.001-P",
            "strike_price": "0.000000001",
            "settle_price": "0.000000002"
        }"#;

        let small: OptionsSettlement = serde_json::from_str(small_json).unwrap();
        assert_eq!(small.strike_price, "0.000000001");
        assert_eq!(small.settle_price, "0.000000002");
    }
}
