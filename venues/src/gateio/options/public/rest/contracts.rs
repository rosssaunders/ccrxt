use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for options contracts
#[derive(Debug, Clone, Serialize, Default)]
pub struct OptionsContractsRequest {
    /// Underlying asset filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying: Option<String>,

    /// Expiration time filter (Unix timestamp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<i64>,
}

/// Options contract information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsContract {
    /// Contract name
    pub name: String,

    /// Tag for mark price usage
    pub tag: String,

    /// Creation time
    pub create_time: f64,

    /// Expiration time
    pub expiration_time: i64,

    /// Underlying asset
    pub underlying: String,

    /// Underlying price
    pub underlying_price: Option<String>,

    /// Last trading time
    pub last_price: Option<String>,

    /// Mark price
    pub mark_price: Option<String>,

    /// Index price
    pub index_price: Option<String>,

    /// Mark IV (implied volatility)
    pub mark_iv: Option<String>,

    /// Option type (call/put)
    #[serde(rename = "type")]
    pub option_type: Option<String>,

    /// Strike price
    pub strike_price: String,

    /// Is call option
    pub is_call: bool,

    /// Multiplier
    pub multiplier: String,

    /// Current total long position size
    pub position_size: Option<i64>,

    /// Maximum number of open orders
    pub orders_limit: i32,
}

impl RestClient {
    /// List all the contracts with specified underlying and expiration time
    ///
    /// Retrieves options contracts with optional filtering by underlying and expiration.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#list-all-the-contracts-with-specified-underlying-and-expiration-time>
    pub async fn get_options_contracts(
        &self,
        params: OptionsContractsRequest,
    ) -> crate::gateio::options::RestResult<Vec<OptionsContract>> {
        self.get_with_query("/options/contracts", Some(&params))
            .await
    }

    /// Query specified contract detail
    ///
    /// Retrieves detailed information for a specific options contract.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#query-specified-contract-detail>
    pub async fn get_options_contract(
        &self,
        contract: &str,
    ) -> crate::gateio::options::RestResult<OptionsContract> {
        let endpoint = format!("/options/contracts/{}", contract);
        self.get(&endpoint).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_options_contracts_request_minimal_serialization() {
        let request = OptionsContractsRequest::default();

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_options_contracts_request_with_underlying() {
        let request = OptionsContractsRequest {
            underlying: Some("BTC_USDT".to_string()),
            expiration: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "underlying=BTC_USDT");
    }

    #[test]
    fn test_options_contracts_request_with_expiration() {
        let request = OptionsContractsRequest {
            underlying: None,
            expiration: Some(1640995200),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "expiration=1640995200");
    }

    #[test]
    fn test_options_contracts_request_full_parameters() {
        let request = OptionsContractsRequest {
            underlying: Some("ETH_USDT".to_string()),
            expiration: Some(1641081600),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("underlying=ETH_USDT"));
        assert!(serialized.contains("expiration=1641081600"));
    }

    #[test]
    fn test_options_contracts_request_different_underlyings() {
        let underlyings = vec!["BTC_USDT", "ETH_USDT", "BNB_USDT", "SOL_USDT", "ADA_USDT"];

        for underlying in underlyings {
            let request = OptionsContractsRequest {
                underlying: Some(underlying.to_string()),
                expiration: None,
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert_eq!(serialized, format!("underlying={}", underlying));
        }
    }

    #[test]
    fn test_options_contracts_request_negative_expiration() {
        let request = OptionsContractsRequest {
            underlying: None,
            expiration: Some(-1640995200),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "expiration=-1640995200");
    }

    #[test]
    fn test_options_contracts_request_extreme_expiration() {
        let request = OptionsContractsRequest {
            underlying: None,
            expiration: Some(i64::MAX),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, format!("expiration={}", i64::MAX));
    }

    #[test]
    fn test_options_contract_call_option_deserialization() {
        let json = r#"{
            "name": "BTC-20240101-50000-C",
            "tag": "BTC_USDT",
            "create_time": 1640995200.123,
            "expiration_time": 1704067200,
            "underlying": "BTC_USDT",
            "underlying_price": "42000.50",
            "last_price": "0.08",
            "mark_price": "0.085",
            "index_price": "42000.50",
            "mark_iv": "0.25",
            "type": "call",
            "strike_price": "50000",
            "is_call": true,
            "multiplier": "0.0001",
            "position_size": 1000,
            "orders_limit": 200
        }"#;

        let contract: OptionsContract = serde_json::from_str(json).unwrap();
        assert_eq!(contract.name, "BTC-20240101-50000-C");
        assert_eq!(contract.tag, "BTC_USDT");
        assert_eq!(contract.create_time, 1640995200.123);
        assert_eq!(contract.expiration_time, 1704067200);
        assert_eq!(contract.underlying, "BTC_USDT");
        assert_eq!(contract.underlying_price, Some("42000.50".to_string()));
        assert_eq!(contract.last_price, Some("0.08".to_string()));
        assert_eq!(contract.mark_price, Some("0.085".to_string()));
        assert_eq!(contract.index_price, Some("42000.50".to_string()));
        assert_eq!(contract.mark_iv, Some("0.25".to_string()));
        assert_eq!(contract.option_type, Some("call".to_string()));
        assert_eq!(contract.strike_price, "50000");
        assert_eq!(contract.is_call, true);
        assert_eq!(contract.multiplier, "0.0001");
        assert_eq!(contract.position_size, Some(1000));
        assert_eq!(contract.orders_limit, 200);
    }

    #[test]
    fn test_options_contract_put_option_deserialization() {
        let json = r#"{
            "name": "ETH-20240101-3000-P",
            "tag": "ETH_USDT",
            "create_time": 1640995300.456,
            "expiration_time": 1704067200,
            "underlying": "ETH_USDT",
            "underlying_price": "3200.75",
            "last_price": "0.05",
            "mark_price": "0.055",
            "index_price": "3200.75",
            "mark_iv": "0.35",
            "type": "put",
            "strike_price": "3000",
            "is_call": false,
            "multiplier": "0.001",
            "position_size": 500,
            "orders_limit": 150
        }"#;

        let contract: OptionsContract = serde_json::from_str(json).unwrap();
        assert_eq!(contract.name, "ETH-20240101-3000-P");
        assert_eq!(contract.tag, "ETH_USDT");
        assert_eq!(contract.create_time, 1640995300.456);
        assert_eq!(contract.expiration_time, 1704067200);
        assert_eq!(contract.underlying, "ETH_USDT");
        assert_eq!(contract.underlying_price, Some("3200.75".to_string()));
        assert_eq!(contract.last_price, Some("0.05".to_string()));
        assert_eq!(contract.mark_price, Some("0.055".to_string()));
        assert_eq!(contract.index_price, Some("3200.75".to_string()));
        assert_eq!(contract.mark_iv, Some("0.35".to_string()));
        assert_eq!(contract.option_type, Some("put".to_string()));
        assert_eq!(contract.strike_price, "3000");
        assert_eq!(contract.is_call, false);
        assert_eq!(contract.multiplier, "0.001");
        assert_eq!(contract.position_size, Some(500));
        assert_eq!(contract.orders_limit, 150);
    }

    #[test]
    fn test_options_contract_minimal_deserialization() {
        let json = r#"{
            "name": "BNB-20240201-400-C",
            "tag": "BNB_USDT",
            "create_time": 1640995400.0,
            "expiration_time": 1706745600,
            "underlying": "BNB_USDT",
            "strike_price": "400",
            "is_call": true,
            "multiplier": "0.01",
            "orders_limit": 100
        }"#;

        let contract: OptionsContract = serde_json::from_str(json).unwrap();
        assert_eq!(contract.name, "BNB-20240201-400-C");
        assert_eq!(contract.tag, "BNB_USDT");
        assert_eq!(contract.create_time, 1640995400.0);
        assert_eq!(contract.expiration_time, 1706745600);
        assert_eq!(contract.underlying, "BNB_USDT");
        assert_eq!(contract.underlying_price, None);
        assert_eq!(contract.last_price, None);
        assert_eq!(contract.mark_price, None);
        assert_eq!(contract.index_price, None);
        assert_eq!(contract.mark_iv, None);
        assert_eq!(contract.option_type, None);
        assert_eq!(contract.strike_price, "400");
        assert_eq!(contract.is_call, true);
        assert_eq!(contract.multiplier, "0.01");
        assert_eq!(contract.position_size, None);
        assert_eq!(contract.orders_limit, 100);
    }

    #[test]
    fn test_options_contract_negative_values() {
        let json = r#"{
            "name": "SOL-20240215-150-P",
            "tag": "SOL_USDT",
            "create_time": -1640995200.0,
            "expiration_time": -1704067200,
            "underlying": "SOL_USDT",
            "strike_price": "150",
            "is_call": false,
            "multiplier": "0.1",
            "position_size": -100,
            "orders_limit": -50
        }"#;

        let contract: OptionsContract = serde_json::from_str(json).unwrap();
        assert_eq!(contract.name, "SOL-20240215-150-P");
        assert_eq!(contract.create_time, -1640995200.0);
        assert_eq!(contract.expiration_time, -1704067200);
        assert_eq!(contract.position_size, Some(-100));
        assert_eq!(contract.orders_limit, -50);
    }

    #[test]
    fn test_options_contract_zero_values() {
        let json = r#"{
            "name": "ADA-20240301-1-C",
            "tag": "ADA_USDT",
            "create_time": 0.0,
            "expiration_time": 0,
            "underlying": "ADA_USDT",
            "underlying_price": "0",
            "last_price": "0",
            "mark_price": "0",
            "index_price": "0",
            "mark_iv": "0",
            "strike_price": "0",
            "is_call": true,
            "multiplier": "0",
            "position_size": 0,
            "orders_limit": 0
        }"#;

        let contract: OptionsContract = serde_json::from_str(json).unwrap();
        assert_eq!(contract.name, "ADA-20240301-1-C");
        assert_eq!(contract.create_time, 0.0);
        assert_eq!(contract.expiration_time, 0);
        assert_eq!(contract.underlying_price, Some("0".to_string()));
        assert_eq!(contract.last_price, Some("0".to_string()));
        assert_eq!(contract.mark_price, Some("0".to_string()));
        assert_eq!(contract.index_price, Some("0".to_string()));
        assert_eq!(contract.mark_iv, Some("0".to_string()));
        assert_eq!(contract.strike_price, "0");
        assert_eq!(contract.multiplier, "0");
        assert_eq!(contract.position_size, Some(0));
        assert_eq!(contract.orders_limit, 0);
    }

    #[test]
    fn test_options_contract_high_precision_values() {
        let json = r#"{
            "name": "BTC-20240315-55000-C",
            "tag": "BTC_USDT",
            "create_time": 1640995500.999999,
            "expiration_time": 1710460800,
            "underlying": "BTC_USDT",
            "underlying_price": "42123.123456789",
            "last_price": "0.123456789",
            "mark_price": "0.987654321",
            "index_price": "42123.123456789",
            "mark_iv": "0.555555555",
            "strike_price": "55000.123456789",
            "is_call": true,
            "multiplier": "0.000123456",
            "position_size": 123456789,
            "orders_limit": 999999
        }"#;

        let contract: OptionsContract = serde_json::from_str(json).unwrap();
        assert_eq!(contract.name, "BTC-20240315-55000-C");
        assert_eq!(contract.create_time, 1640995500.999999);
        assert_eq!(
            contract.underlying_price,
            Some("42123.123456789".to_string())
        );
        assert_eq!(contract.last_price, Some("0.123456789".to_string()));
        assert_eq!(contract.mark_price, Some("0.987654321".to_string()));
        assert_eq!(contract.index_price, Some("42123.123456789".to_string()));
        assert_eq!(contract.mark_iv, Some("0.555555555".to_string()));
        assert_eq!(contract.strike_price, "55000.123456789");
        assert_eq!(contract.multiplier, "0.000123456");
        assert_eq!(contract.position_size, Some(123456789));
        assert_eq!(contract.orders_limit, 999999);
    }

    #[test]
    fn test_options_contract_different_contract_names() {
        let contracts = vec![
            "BTC-20240101-50000-C",
            "ETH-20240215-3000-P",
            "BNB-20240301-400-C",
            "SOL-20240315-150-P",
            "ADA-20240401-1-C",
        ];

        for contract_name in contracts {
            let json = format!(
                r#"{{
                "name": "{}",
                "tag": "TEST_USDT",
                "create_time": 1640995200.0,
                "expiration_time": 1704067200,
                "underlying": "TEST_USDT",
                "strike_price": "100",
                "is_call": true,
                "multiplier": "0.01",
                "orders_limit": 100
            }}"#,
                contract_name
            );

            let contract: OptionsContract = serde_json::from_str(&json).unwrap();
            assert_eq!(contract.name, contract_name);
        }
    }

    #[test]
    fn test_options_contract_different_option_types() {
        let option_types = vec![
            ("call", true),
            ("put", false),
            ("CALL", true),
            ("PUT", false),
        ];

        for (type_str, is_call) in option_types {
            let json = format!(
                r#"{{
                "name": "BTC-20240101-50000-{}",
                "tag": "BTC_USDT",
                "create_time": 1640995200.0,
                "expiration_time": 1704067200,
                "underlying": "BTC_USDT",
                "type": "{}",
                "strike_price": "50000",
                "is_call": {},
                "multiplier": "0.0001",
                "orders_limit": 200
            }}"#,
                if is_call { "C" } else { "P" },
                type_str,
                is_call
            );

            let contract: OptionsContract = serde_json::from_str(&json).unwrap();
            assert_eq!(contract.option_type, Some(type_str.to_string()));
            assert_eq!(contract.is_call, is_call);
        }
    }

    #[test]
    fn test_options_contract_array_deserialization() {
        let json = r#"[
            {
                "name": "BTC-20240101-50000-C",
                "tag": "BTC_USDT",
                "create_time": 1640995200.0,
                "expiration_time": 1704067200,
                "underlying": "BTC_USDT",
                "strike_price": "50000",
                "is_call": true,
                "multiplier": "0.0001",
                "orders_limit": 200
            },
            {
                "name": "ETH-20240101-3000-P",
                "tag": "ETH_USDT",
                "create_time": 1640995300.0,
                "expiration_time": 1704067200,
                "underlying": "ETH_USDT",
                "strike_price": "3000",
                "is_call": false,
                "multiplier": "0.001",
                "orders_limit": 150
            }
        ]"#;

        let contracts: Vec<OptionsContract> = serde_json::from_str(json).unwrap();
        assert_eq!(contracts.len(), 2);

        assert_eq!(contracts[0].name, "BTC-20240101-50000-C");
        assert_eq!(contracts[0].is_call, true);
        assert_eq!(contracts[0].strike_price, "50000");

        assert_eq!(contracts[1].name, "ETH-20240101-3000-P");
        assert_eq!(contracts[1].is_call, false);
        assert_eq!(contracts[1].strike_price, "3000");
    }

    #[test]
    fn test_options_contract_empty_array_deserialization() {
        let json = r#"[]"#;
        let contracts: Vec<OptionsContract> = serde_json::from_str(json).unwrap();
        assert_eq!(contracts.len(), 0);
    }

    #[test]
    fn test_options_contract_serialization() {
        let contract = OptionsContract {
            name: "BTC-20240101-50000-C".to_string(),
            tag: "BTC_USDT".to_string(),
            create_time: 1640995200.123,
            expiration_time: 1704067200,
            underlying: "BTC_USDT".to_string(),
            underlying_price: Some("42000.50".to_string()),
            last_price: Some("0.08".to_string()),
            mark_price: Some("0.085".to_string()),
            index_price: Some("42000.50".to_string()),
            mark_iv: Some("0.25".to_string()),
            option_type: Some("call".to_string()),
            strike_price: "50000".to_string(),
            is_call: true,
            multiplier: "0.0001".to_string(),
            position_size: Some(1000),
            orders_limit: 200,
        };

        let json = serde_json::to_value(&contract).unwrap();
        assert_eq!(json["name"], "BTC-20240101-50000-C");
        assert_eq!(json["tag"], "BTC_USDT");
        assert_eq!(json["create_time"], 1640995200.123);
        assert_eq!(json["expiration_time"], 1704067200);
        assert_eq!(json["underlying"], "BTC_USDT");
        assert_eq!(json["underlying_price"], "42000.50");
        assert_eq!(json["last_price"], "0.08");
        assert_eq!(json["mark_price"], "0.085");
        assert_eq!(json["index_price"], "42000.50");
        assert_eq!(json["mark_iv"], "0.25");
        assert_eq!(json["type"], "call");
        assert_eq!(json["strike_price"], "50000");
        assert_eq!(json["is_call"], true);
        assert_eq!(json["multiplier"], "0.0001");
        assert_eq!(json["position_size"], 1000);
        assert_eq!(json["orders_limit"], 200);
    }

    #[test]
    fn test_options_contract_serialization_round_trip() {
        let original = OptionsContract {
            name: "ETH-20240215-2800-P".to_string(),
            tag: "ETH_USDT".to_string(),
            create_time: 1640995300.456,
            expiration_time: 1708041600,
            underlying: "ETH_USDT".to_string(),
            underlying_price: Some("3200.75".to_string()),
            last_price: None,
            mark_price: Some("0.055".to_string()),
            index_price: Some("3200.75".to_string()),
            mark_iv: None,
            option_type: Some("put".to_string()),
            strike_price: "2800".to_string(),
            is_call: false,
            multiplier: "0.001".to_string(),
            position_size: Some(500),
            orders_limit: 150,
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: OptionsContract = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.name, original.name);
        assert_eq!(deserialized.tag, original.tag);
        assert_eq!(deserialized.create_time, original.create_time);
        assert_eq!(deserialized.expiration_time, original.expiration_time);
        assert_eq!(deserialized.underlying, original.underlying);
        assert_eq!(deserialized.underlying_price, original.underlying_price);
        assert_eq!(deserialized.last_price, original.last_price);
        assert_eq!(deserialized.mark_price, original.mark_price);
        assert_eq!(deserialized.index_price, original.index_price);
        assert_eq!(deserialized.mark_iv, original.mark_iv);
        assert_eq!(deserialized.option_type, original.option_type);
        assert_eq!(deserialized.strike_price, original.strike_price);
        assert_eq!(deserialized.is_call, original.is_call);
        assert_eq!(deserialized.multiplier, original.multiplier);
        assert_eq!(deserialized.position_size, original.position_size);
        assert_eq!(deserialized.orders_limit, original.orders_limit);
    }

    #[test]
    fn test_options_contract_realistic_scenarios() {
        // Deep ITM call option
        let itm_call_json = r#"{
            "name": "BTC-20240101-40000-C",
            "tag": "BTC_USDT",
            "create_time": 1640995200.0,
            "expiration_time": 1704067200,
            "underlying": "BTC_USDT",
            "underlying_price": "50000.00",
            "last_price": "10500.00",
            "mark_price": "10000.00",
            "index_price": "50000.00",
            "mark_iv": "0.15",
            "type": "call",
            "strike_price": "40000",
            "is_call": true,
            "multiplier": "0.0001",
            "position_size": 50,
            "orders_limit": 200
        }"#;

        let itm_call: OptionsContract = serde_json::from_str(itm_call_json).unwrap();
        assert_eq!(itm_call.name, "BTC-20240101-40000-C");
        assert!(itm_call.name.ends_with("-C"));
        assert_eq!(itm_call.is_call, true);
        assert_eq!(itm_call.strike_price, "40000");

        // Parse prices to verify ITM status
        let underlying: f64 = itm_call.underlying_price.as_ref().unwrap().parse().unwrap();
        let strike: f64 = itm_call.strike_price.parse().unwrap();
        assert!(underlying > strike); // ITM for call

        // OTM put option
        let otm_put_json = r#"{
            "name": "ETH-20240101-2000-P",
            "tag": "ETH_USDT",
            "create_time": 1640995300.0,
            "expiration_time": 1704067200,
            "underlying": "ETH_USDT",
            "underlying_price": "3000.00",
            "last_price": "5.00",
            "mark_price": "4.50",
            "index_price": "3000.00",
            "mark_iv": "0.40",
            "type": "put",
            "strike_price": "2000",
            "is_call": false,
            "multiplier": "0.001",
            "position_size": 25,
            "orders_limit": 150
        }"#;

        let otm_put: OptionsContract = serde_json::from_str(otm_put_json).unwrap();
        assert_eq!(otm_put.name, "ETH-20240101-2000-P");
        assert!(otm_put.name.ends_with("-P"));
        assert_eq!(otm_put.is_call, false);
        assert_eq!(otm_put.strike_price, "2000");

        // Parse prices to verify OTM status
        let underlying: f64 = otm_put.underlying_price.as_ref().unwrap().parse().unwrap();
        let strike: f64 = otm_put.strike_price.parse().unwrap();
        assert!(underlying > strike); // OTM for put
    }

    #[test]
    fn test_options_contract_expiration_scenarios() {
        // Near-term expiration
        let near_term_json = r#"{
            "name": "BTC-20240105-45000-C",
            "tag": "BTC_USDT",
            "create_time": 1704153600.0,
            "expiration_time": 1704499200,
            "underlying": "BTC_USDT",
            "strike_price": "45000",
            "is_call": true,
            "multiplier": "0.0001",
            "orders_limit": 200
        }"#;

        let near_term: OptionsContract = serde_json::from_str(near_term_json).unwrap();
        assert_eq!(near_term.expiration_time, 1704499200);

        // Verify near-term (less than 1 week from creation)
        let time_to_expiry = near_term.expiration_time as f64 - near_term.create_time;
        assert!(time_to_expiry < 7.0 * 24.0 * 3600.0); // Less than 1 week

        // Long-term expiration
        let long_term_json = r#"{
            "name": "ETH-20241231-3000-P",
            "tag": "ETH_USDT",
            "create_time": 1704153600.0,
            "expiration_time": 1735689600,
            "underlying": "ETH_USDT",
            "strike_price": "3000",
            "is_call": false,
            "multiplier": "0.001",
            "orders_limit": 150
        }"#;

        let long_term: OptionsContract = serde_json::from_str(long_term_json).unwrap();
        assert_eq!(long_term.expiration_time, 1735689600);

        // Verify long-term (more than 6 months from creation)
        let time_to_expiry = long_term.expiration_time as f64 - long_term.create_time;
        assert!(time_to_expiry > 180.0 * 24.0 * 3600.0); // More than 6 months
    }

    #[test]
    fn test_options_contract_edge_cases() {
        // Contract with extreme strike price
        let extreme_strike_json = r#"{
            "name": "BTC-20240101-999999-C",
            "tag": "BTC_USDT",
            "create_time": 1640995200.0,
            "expiration_time": 1704067200,
            "underlying": "BTC_USDT",
            "strike_price": "999999.999999999",
            "is_call": true,
            "multiplier": "0.000000001",
            "orders_limit": 1
        }"#;

        let extreme_contract: OptionsContract = serde_json::from_str(extreme_strike_json).unwrap();
        assert_eq!(extreme_contract.strike_price, "999999.999999999");
        assert_eq!(extreme_contract.multiplier, "0.000000001");
        assert_eq!(extreme_contract.orders_limit, 1);

        // Contract with very large position size
        let large_position_json = format!(
            r#"{{
            "name": "ETH-20240101-3000-P",
            "tag": "ETH_USDT",
            "create_time": 1640995200.0,
            "expiration_time": 1704067200,
            "underlying": "ETH_USDT",
            "strike_price": "3000",
            "is_call": false,
            "multiplier": "0.001",
            "position_size": {},
            "orders_limit": {}
        }}"#,
            i64::MAX,
            i32::MAX
        );

        let large_contract: OptionsContract = serde_json::from_str(&large_position_json).unwrap();
        assert_eq!(large_contract.position_size, Some(i64::MAX));
        assert_eq!(large_contract.orders_limit, i32::MAX);
    }
}
