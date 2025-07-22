use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for options tickers
#[derive(Debug, Clone, Serialize, Default)]
pub struct OptionsTickersRequest {
    /// Underlying asset filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying: Option<String>,
}

/// Options ticker information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsTicker {
    /// Contract name
    pub name: String,

    /// Last trading price
    pub last: Option<String>,

    /// Change percentage (24h)
    pub change_percentage: Option<String>,

    /// Total size (24h)
    pub total_size: Option<String>,

    /// Lowest ask
    pub lowest_ask: Option<String>,

    /// Highest bid
    pub highest_bid: Option<String>,

    /// Mark price
    pub mark_price: Option<String>,

    /// Mark IV (implied volatility)
    pub mark_iv: Option<String>,

    /// Index price
    pub index_price: Option<String>,

    /// Bid IV
    pub bid_iv: Option<String>,

    /// Ask IV
    pub ask_iv: Option<String>,

    /// Position size
    pub position_size: Option<i64>,

    /// Delta
    pub delta: Option<String>,

    /// Gamma
    pub gamma: Option<String>,

    /// Vega
    pub vega: Option<String>,

    /// Theta
    pub theta: Option<String>,

    /// Rho
    pub rho: Option<String>,
}

/// Underlying ticker information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnderlyingTicker {
    /// Trading enabled
    pub trade_enabled: Option<bool>,

    /// Index price (quote currency)
    pub index_price: String,

    /// Total put options trades amount in last 24h
    pub trade_put: i64,

    /// Total call options trades amount in last 24h
    pub trade_call: i64,
}

impl RestClient {
    /// List tickers of options contracts
    ///
    /// Retrieves ticker information for options contracts.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#list-tickers-of-options-contracts>
    pub async fn get_options_tickers(
        &self,
        params: OptionsTickersRequest,
    ) -> crate::gateio::options::Result<Vec<OptionsTicker>> {
        self.get_with_query("/options/tickers", Some(&params)).await
    }

    /// Get underlying ticker
    ///
    /// Retrieves ticker information for a specific underlying asset.
    pub async fn get_underlying_ticker(
        &self,
        underlying: &str,
    ) -> crate::gateio::options::Result<UnderlyingTicker> {
        let endpoint = format!("/options/underlying/tickers/{}", underlying);
        self.get(&endpoint).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_options_tickers_request_minimal_serialization() {
        let request = OptionsTickersRequest::default();

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_options_tickers_request_with_underlying() {
        let request = OptionsTickersRequest {
            underlying: Some("BTC_USDT".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "underlying=BTC_USDT");
    }

    #[test]
    fn test_options_tickers_request_different_underlyings() {
        let underlyings = vec!["BTC_USDT", "ETH_USDT", "BNB_USDT", "SOL_USDT", "ADA_USDT"];
        
        for underlying in underlyings {
            let request = OptionsTickersRequest {
                underlying: Some(underlying.to_string()),
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert_eq!(serialized, format!("underlying={}", underlying));
        }
    }

    #[test]
    fn test_options_ticker_full_deserialization() {
        let json = r#"{
            "name": "BTC-20240101-50000-C",
            "last": "0.08",
            "change_percentage": "5.25",
            "total_size": "100",
            "lowest_ask": "0.085",
            "highest_bid": "0.075",
            "mark_price": "0.08",
            "mark_iv": "0.25",
            "index_price": "42000.50",
            "bid_iv": "0.24",
            "ask_iv": "0.26",
            "position_size": 50,
            "delta": "0.65",
            "gamma": "0.015",
            "vega": "0.08",
            "theta": "-0.002",
            "rho": "0.012"
        }"#;

        let ticker: OptionsTicker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.name, "BTC-20240101-50000-C");
        assert_eq!(ticker.last, Some("0.08".to_string()));
        assert_eq!(ticker.change_percentage, Some("5.25".to_string()));
        assert_eq!(ticker.total_size, Some("100".to_string()));
        assert_eq!(ticker.lowest_ask, Some("0.085".to_string()));
        assert_eq!(ticker.highest_bid, Some("0.075".to_string()));
        assert_eq!(ticker.mark_price, Some("0.08".to_string()));
        assert_eq!(ticker.mark_iv, Some("0.25".to_string()));
        assert_eq!(ticker.index_price, Some("42000.50".to_string()));
        assert_eq!(ticker.bid_iv, Some("0.24".to_string()));
        assert_eq!(ticker.ask_iv, Some("0.26".to_string()));
        assert_eq!(ticker.position_size, Some(50));
        assert_eq!(ticker.delta, Some("0.65".to_string()));
        assert_eq!(ticker.gamma, Some("0.015".to_string()));
        assert_eq!(ticker.vega, Some("0.08".to_string()));
        assert_eq!(ticker.theta, Some("-0.002".to_string()));
        assert_eq!(ticker.rho, Some("0.012".to_string()));
    }

    #[test]
    fn test_options_ticker_minimal_deserialization() {
        let json = r#"{
            "name": "ETH-20240101-3000-P"
        }"#;

        let ticker: OptionsTicker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.name, "ETH-20240101-3000-P");
        assert_eq!(ticker.last, None);
        assert_eq!(ticker.change_percentage, None);
        assert_eq!(ticker.total_size, None);
        assert_eq!(ticker.lowest_ask, None);
        assert_eq!(ticker.highest_bid, None);
        assert_eq!(ticker.mark_price, None);
        assert_eq!(ticker.mark_iv, None);
        assert_eq!(ticker.index_price, None);
        assert_eq!(ticker.bid_iv, None);
        assert_eq!(ticker.ask_iv, None);
        assert_eq!(ticker.position_size, None);
        assert_eq!(ticker.delta, None);
        assert_eq!(ticker.gamma, None);
        assert_eq!(ticker.vega, None);
        assert_eq!(ticker.theta, None);
        assert_eq!(ticker.rho, None);
    }

    #[test]
    fn test_options_ticker_partial_deserialization() {
        let json = r#"{
            "name": "BNB-20240201-400-C",
            "last": "0.02",
            "mark_price": "0.025",
            "delta": "0.45",
            "gamma": "0.01"
        }"#;

        let ticker: OptionsTicker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.name, "BNB-20240201-400-C");
        assert_eq!(ticker.last, Some("0.02".to_string()));
        assert_eq!(ticker.mark_price, Some("0.025".to_string()));
        assert_eq!(ticker.delta, Some("0.45".to_string()));
        assert_eq!(ticker.gamma, Some("0.01".to_string()));
        assert_eq!(ticker.change_percentage, None);
        assert_eq!(ticker.vega, None);
        assert_eq!(ticker.theta, None);
    }

    #[test]
    fn test_options_ticker_call_option_greeks() {
        let json = r#"{
            "name": "BTC-20240101-45000-C",
            "last": "0.12",
            "mark_price": "0.11",
            "delta": "0.75",
            "gamma": "0.02",
            "vega": "0.15",
            "theta": "-0.005",
            "rho": "0.018"
        }"#;

        let ticker: OptionsTicker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.name, "BTC-20240101-45000-C");
        assert!(ticker.name.ends_with("-C"));
        assert_eq!(ticker.delta, Some("0.75".to_string())); // Positive delta for call
        assert_eq!(ticker.gamma, Some("0.02".to_string())); // Positive gamma
        assert_eq!(ticker.vega, Some("0.15".to_string())); // Positive vega
        assert_eq!(ticker.theta, Some("-0.005".to_string())); // Negative theta (time decay)
        assert_eq!(ticker.rho, Some("0.018".to_string())); // Positive rho for call
    }

    #[test]
    fn test_options_ticker_put_option_greeks() {
        let json = r#"{
            "name": "ETH-20240101-3000-P",
            "last": "0.06",
            "mark_price": "0.065",
            "delta": "-0.35",
            "gamma": "0.015",
            "vega": "0.12",
            "theta": "-0.003",
            "rho": "-0.012"
        }"#;

        let ticker: OptionsTicker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.name, "ETH-20240101-3000-P");
        assert!(ticker.name.ends_with("-P"));
        assert_eq!(ticker.delta, Some("-0.35".to_string())); // Negative delta for put
        assert_eq!(ticker.gamma, Some("0.015".to_string())); // Positive gamma
        assert_eq!(ticker.vega, Some("0.12".to_string())); // Positive vega
        assert_eq!(ticker.theta, Some("-0.003".to_string())); // Negative theta (time decay)
        assert_eq!(ticker.rho, Some("-0.012".to_string())); // Negative rho for put
    }

    #[test]
    fn test_options_ticker_negative_values() {
        let json = r#"{
            "name": "SOL-20240215-150-P",
            "last": "0.001",
            "change_percentage": "-12.5",
            "total_size": "0",
            "position_size": -25,
            "delta": "-0.05",
            "theta": "-0.01"
        }"#;

        let ticker: OptionsTicker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.name, "SOL-20240215-150-P");
        assert_eq!(ticker.change_percentage, Some("-12.5".to_string()));
        assert_eq!(ticker.position_size, Some(-25));
        assert_eq!(ticker.delta, Some("-0.05".to_string()));
        assert_eq!(ticker.theta, Some("-0.01".to_string()));
    }

    #[test]
    fn test_options_ticker_zero_values() {
        let json = r#"{
            "name": "ADA-20240301-1-C",
            "last": "0",
            "change_percentage": "0",
            "total_size": "0",
            "position_size": 0,
            "delta": "0",
            "gamma": "0",
            "vega": "0",
            "theta": "0",
            "rho": "0"
        }"#;

        let ticker: OptionsTicker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.name, "ADA-20240301-1-C");
        assert_eq!(ticker.last, Some("0".to_string()));
        assert_eq!(ticker.change_percentage, Some("0".to_string()));
        assert_eq!(ticker.total_size, Some("0".to_string()));
        assert_eq!(ticker.position_size, Some(0));
        assert_eq!(ticker.delta, Some("0".to_string()));
        assert_eq!(ticker.gamma, Some("0".to_string()));
        assert_eq!(ticker.vega, Some("0".to_string()));
        assert_eq!(ticker.theta, Some("0".to_string()));
        assert_eq!(ticker.rho, Some("0".to_string()));
    }

    #[test]
    fn test_options_ticker_high_precision_values() {
        let json = r#"{
            "name": "BTC-20240315-55000-C",
            "last": "0.123456789",
            "mark_price": "0.987654321",
            "mark_iv": "0.555555555",
            "delta": "0.123456789",
            "gamma": "0.987654321",
            "vega": "0.111111111",
            "theta": "-0.999999999",
            "rho": "0.123123123"
        }"#;

        let ticker: OptionsTicker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.name, "BTC-20240315-55000-C");
        assert_eq!(ticker.last, Some("0.123456789".to_string()));
        assert_eq!(ticker.mark_price, Some("0.987654321".to_string()));
        assert_eq!(ticker.mark_iv, Some("0.555555555".to_string()));
        assert_eq!(ticker.delta, Some("0.123456789".to_string()));
        assert_eq!(ticker.gamma, Some("0.987654321".to_string()));
        assert_eq!(ticker.vega, Some("0.111111111".to_string()));
        assert_eq!(ticker.theta, Some("-0.999999999".to_string()));
        assert_eq!(ticker.rho, Some("0.123123123".to_string()));
    }

    #[test]
    fn test_options_ticker_array_deserialization() {
        let json = r#"[
            {
                "name": "BTC-20240101-50000-C",
                "last": "0.08",
                "delta": "0.65"
            },
            {
                "name": "ETH-20240101-3000-P",
                "last": "0.05",
                "delta": "-0.35"
            },
            {
                "name": "BNB-20240201-400-C",
                "last": "0.02",
                "delta": "0.45"
            }
        ]"#;

        let tickers: Vec<OptionsTicker> = serde_json::from_str(json).unwrap();
        assert_eq!(tickers.len(), 3);
        
        assert_eq!(tickers[0].name, "BTC-20240101-50000-C");
        assert_eq!(tickers[0].last, Some("0.08".to_string()));
        assert_eq!(tickers[0].delta, Some("0.65".to_string()));
        
        assert_eq!(tickers[1].name, "ETH-20240101-3000-P");
        assert_eq!(tickers[1].last, Some("0.05".to_string()));
        assert_eq!(tickers[1].delta, Some("-0.35".to_string()));
        
        assert_eq!(tickers[2].name, "BNB-20240201-400-C");
        assert_eq!(tickers[2].last, Some("0.02".to_string()));
        assert_eq!(tickers[2].delta, Some("0.45".to_string()));
    }

    #[test]
    fn test_options_ticker_empty_array_deserialization() {
        let json = r#"[]"#;
        let tickers: Vec<OptionsTicker> = serde_json::from_str(json).unwrap();
        assert_eq!(tickers.len(), 0);
    }

    #[test]
    fn test_underlying_ticker_deserialization() {
        let json = r#"{
            "trade_enabled": true,
            "index_price": "42000.50",
            "trade_put": 150,
            "trade_call": 250
        }"#;

        let ticker: UnderlyingTicker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.trade_enabled, Some(true));
        assert_eq!(ticker.index_price, "42000.50");
        assert_eq!(ticker.trade_put, 150);
        assert_eq!(ticker.trade_call, 250);
    }

    #[test]
    fn test_underlying_ticker_without_trade_enabled() {
        let json = r#"{
            "index_price": "3000.75",
            "trade_put": 75,
            "trade_call": 125
        }"#;

        let ticker: UnderlyingTicker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.trade_enabled, None);
        assert_eq!(ticker.index_price, "3000.75");
        assert_eq!(ticker.trade_put, 75);
        assert_eq!(ticker.trade_call, 125);
    }

    #[test]
    fn test_underlying_ticker_trade_disabled() {
        let json = r#"{
            "trade_enabled": false,
            "index_price": "400.25",
            "trade_put": 0,
            "trade_call": 0
        }"#;

        let ticker: UnderlyingTicker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.trade_enabled, Some(false));
        assert_eq!(ticker.index_price, "400.25");
        assert_eq!(ticker.trade_put, 0);
        assert_eq!(ticker.trade_call, 0);
    }

    #[test]
    fn test_underlying_ticker_high_volume() {
        let json = r#"{
            "trade_enabled": true,
            "index_price": "45000.123456",
            "trade_put": 9999999,
            "trade_call": 8888888
        }"#;

        let ticker: UnderlyingTicker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.trade_enabled, Some(true));
        assert_eq!(ticker.index_price, "45000.123456");
        assert_eq!(ticker.trade_put, 9999999);
        assert_eq!(ticker.trade_call, 8888888);
    }

    #[test]
    fn test_underlying_ticker_negative_values() {
        let json = r#"{
            "trade_enabled": true,
            "index_price": "100.0",
            "trade_put": -50,
            "trade_call": -25
        }"#;

        let ticker: UnderlyingTicker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.trade_enabled, Some(true));
        assert_eq!(ticker.index_price, "100.0");
        assert_eq!(ticker.trade_put, -50);
        assert_eq!(ticker.trade_call, -25);
    }

    #[test]
    fn test_options_ticker_serialization() {
        let ticker = OptionsTicker {
            name: "BTC-20240101-50000-C".to_string(),
            last: Some("0.08".to_string()),
            change_percentage: Some("5.25".to_string()),
            total_size: Some("100".to_string()),
            lowest_ask: Some("0.085".to_string()),
            highest_bid: Some("0.075".to_string()),
            mark_price: Some("0.08".to_string()),
            mark_iv: Some("0.25".to_string()),
            index_price: Some("42000.50".to_string()),
            bid_iv: Some("0.24".to_string()),
            ask_iv: Some("0.26".to_string()),
            position_size: Some(50),
            delta: Some("0.65".to_string()),
            gamma: Some("0.015".to_string()),
            vega: Some("0.08".to_string()),
            theta: Some("-0.002".to_string()),
            rho: Some("0.012".to_string()),
        };

        let json = serde_json::to_value(&ticker).unwrap();
        assert_eq!(json["name"], "BTC-20240101-50000-C");
        assert_eq!(json["last"], "0.08");
        assert_eq!(json["change_percentage"], "5.25");
        assert_eq!(json["delta"], "0.65");
        assert_eq!(json["position_size"], 50);
    }

    #[test]
    fn test_underlying_ticker_serialization() {
        let ticker = UnderlyingTicker {
            trade_enabled: Some(true),
            index_price: "42000.50".to_string(),
            trade_put: 150,
            trade_call: 250,
        };

        let json = serde_json::to_value(&ticker).unwrap();
        assert_eq!(json["trade_enabled"], true);
        assert_eq!(json["index_price"], "42000.50");
        assert_eq!(json["trade_put"], 150);
        assert_eq!(json["trade_call"], 250);
    }

    #[test]
    fn test_options_ticker_serialization_round_trip() {
        let original = OptionsTicker {
            name: "ETH-20240101-3000-P".to_string(),
            last: Some("0.05".to_string()),
            change_percentage: Some("-2.5".to_string()),
            total_size: Some("75".to_string()),
            lowest_ask: None,
            highest_bid: None,
            mark_price: Some("0.052".to_string()),
            mark_iv: Some("0.35".to_string()),
            index_price: Some("3000.75".to_string()),
            bid_iv: None,
            ask_iv: None,
            position_size: Some(-25),
            delta: Some("-0.35".to_string()),
            gamma: Some("0.015".to_string()),
            vega: Some("0.12".to_string()),
            theta: Some("-0.003".to_string()),
            rho: Some("-0.012".to_string()),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: OptionsTicker = serde_json::from_str(&json).unwrap();
        
        assert_eq!(deserialized.name, original.name);
        assert_eq!(deserialized.last, original.last);
        assert_eq!(deserialized.change_percentage, original.change_percentage);
        assert_eq!(deserialized.position_size, original.position_size);
        assert_eq!(deserialized.delta, original.delta);
        assert_eq!(deserialized.gamma, original.gamma);
        assert_eq!(deserialized.vega, original.vega);
        assert_eq!(deserialized.theta, original.theta);
        assert_eq!(deserialized.rho, original.rho);
    }

    #[test]
    fn test_options_ticker_different_contract_types() {
        let contracts = vec![
            ("BTC-20240101-50000-C", "call"),
            ("ETH-20240215-3000-P", "put"),
            ("BNB-20240301-400-C", "call"),
            ("SOL-20240315-150-P", "put"),
            ("ADA-20240401-1-C", "call")
        ];
        
        for (contract, option_type) in contracts {
            let json = format!(r#"{{
                "name": "{}",
                "last": "0.1",
                "delta": "{}"
            }}"#, contract, if option_type == "call" { "0.5" } else { "-0.3" });

            let ticker: OptionsTicker = serde_json::from_str(&json).unwrap();
            assert_eq!(ticker.name, contract);
            
            if option_type == "call" {
                assert!(ticker.name.ends_with("-C"));
                assert_eq!(ticker.delta, Some("0.5".to_string()));
            } else {
                assert!(ticker.name.ends_with("-P"));
                assert_eq!(ticker.delta, Some("-0.3".to_string()));
            }
        }
    }

    #[test]
    fn test_options_ticker_iv_scenarios() {
        // High volatility scenario
        let high_vol_json = r#"{
            "name": "BTC-20240101-70000-C",
            "mark_iv": "1.50",
            "bid_iv": "1.45",
            "ask_iv": "1.55"
        }"#;

        let high_vol: OptionsTicker = serde_json::from_str(high_vol_json).unwrap();
        assert_eq!(high_vol.mark_iv, Some("1.50".to_string()));
        assert_eq!(high_vol.bid_iv, Some("1.45".to_string()));
        assert_eq!(high_vol.ask_iv, Some("1.55".to_string()));

        // Low volatility scenario
        let low_vol_json = r#"{
            "name": "ETH-20240315-3000-P",
            "mark_iv": "0.15",
            "bid_iv": "0.14",
            "ask_iv": "0.16"
        }"#;

        let low_vol: OptionsTicker = serde_json::from_str(low_vol_json).unwrap();
        assert_eq!(low_vol.mark_iv, Some("0.15".to_string()));
        assert_eq!(low_vol.bid_iv, Some("0.14".to_string()));
        assert_eq!(low_vol.ask_iv, Some("0.16".to_string()));
    }

    #[test]
    fn test_options_ticker_realistic_market_scenarios() {
        // ITM call option
        let itm_call_json = r#"{
            "name": "BTC-20240101-40000-C",
            "last": "2500.0",
            "mark_price": "2450.0",
            "delta": "0.95",
            "gamma": "0.001",
            "vega": "0.05",
            "theta": "-0.1"
        }"#;

        let itm_call: OptionsTicker = serde_json::from_str(itm_call_json).unwrap();
        assert_eq!(itm_call.delta, Some("0.95".to_string())); // High delta for deep ITM
        assert_eq!(itm_call.gamma, Some("0.001".to_string())); // Low gamma for deep ITM

        // OTM put option
        let otm_put_json = r#"{
            "name": "ETH-20240101-2000-P",
            "last": "5.0",
            "mark_price": "4.5",
            "delta": "-0.05",
            "gamma": "0.001",
            "vega": "0.02",
            "theta": "-0.5"
        }"#;

        let otm_put: OptionsTicker = serde_json::from_str(otm_put_json).unwrap();
        assert_eq!(otm_put.delta, Some("-0.05".to_string())); // Low delta for OTM put
        assert_eq!(otm_put.gamma, Some("0.001".to_string())); // Low gamma for OTM
    }
}
