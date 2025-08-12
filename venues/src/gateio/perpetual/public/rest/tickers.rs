use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for futures tickers
#[derive(Debug, Clone, Serialize, Default)]
pub struct FuturesTickersRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract name (optional - if not provided, returns all contracts)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,
}

/// Futures ticker information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesTicker {
    /// Contract name
    pub contract: String,

    /// Last trading price
    pub last: String,

    /// Recent lowest ask
    pub lowest_ask: String,

    /// Recent highest bid
    pub highest_bid: String,

    /// Change percentage (24h)
    pub change_percentage: String,

    /// Change amount (24h)
    pub change_utc0: Option<String>,

    /// Change amount (UTC 8)
    pub change_utc8: Option<String>,

    /// Total trading volume (24h)
    pub total_size: Option<String>,

    /// Trading volume (24h in quote currency)
    pub volume_24h: Option<String>,

    /// Trading volume (24h in base currency)
    pub volume_24h_btc: Option<String>,

    /// Trading volume (24h in quote currency)
    pub volume_24h_usd: Option<String>,

    /// Trading volume (24h in settlement currency)
    pub volume_24h_base: Option<String>,

    /// Trading volume (24h in quote currency)
    pub volume_24h_quote: Option<String>,

    /// Trading volume (24h in settlement currency, BTC denominated)
    pub volume_24h_settle: Option<String>,

    /// Mark price
    pub mark_price: Option<String>,

    /// Funding rate
    pub funding_rate: String,

    /// Predicted funding rate  
    pub funding_rate_indicative: String,

    /// Index price
    pub index_price: Option<String>,

    /// Trading enabled
    pub quanto_base_rate: Option<String>,

    /// Next funding time
    pub funding_next_apply: Option<i64>,

    /// Basis rate
    pub basis_rate: Option<String>,

    /// Basis value
    pub basis_value: Option<String>,

    /// Premium index
    pub premium_index: Option<String>,
}

impl RestClient {
    /// List futures tickers
    ///
    /// Retrieves ticker information for futures contracts.
    /// If contract is not specified, returns tickers for all contracts in the settlement currency.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#list-futures-tickers>
    pub async fn get_futures_tickers(
        &self,
        params: FuturesTickersRequest,
    ) -> crate::gateio::perpetual::RestResult<Vec<FuturesTicker>> {
        let endpoint = format!("/futures/{}/tickers", params.settle);
        self.get_with_query(&endpoint, Some(&params)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_futures_tickers_request_all_contracts() {
        let request = FuturesTickersRequest {
            settle: "USDT".to_string(),
            contract: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 1); // Only settle
        assert!(!obj.contains_key("contract"));
    }

    #[test]
    fn test_futures_tickers_request_specific_contract() {
        let request = FuturesTickersRequest {
            settle: "USDT".to_string(),
            contract: Some("BTC_USDT".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "BTC_USDT");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 2);
    }

    #[test]
    fn test_futures_ticker_complete_deserialization() {
        let json = r#"{
            "contract": "BTC_USDT",
            "last": "43250.8",
            "lowest_ask": "43251.0",
            "highest_bid": "43250.5",
            "change_percentage": "2.5",
            "change_utc0": "1050.5",
            "change_utc8": "1075.2",
            "total_size": "15000",
            "volume_24h": "125000000",
            "volume_24h_btc": "2890.5",
            "volume_24h_usd": "125000000",
            "volume_24h_base": "2890.5",
            "volume_24h_quote": "125000000",
            "volume_24h_settle": "2890.5",
            "mark_price": "43251.2",
            "funding_rate": "0.000075",
            "funding_rate_indicative": "0.000082",
            "index_price": "43252.0",
            "quanto_base_rate": "1.0",
            "funding_next_apply": 1640995200,
            "basis_rate": "0.000025",
            "basis_value": "1.2",
            "premium_index": "0.000002"
        }"#;

        let ticker: FuturesTicker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.contract, "BTC_USDT");
        assert_eq!(ticker.last, "43250.8");
        assert_eq!(ticker.lowest_ask, "43251.0");
        assert_eq!(ticker.highest_bid, "43250.5");
        assert_eq!(ticker.change_percentage, "2.5");
        assert_eq!(ticker.change_utc0.as_ref().unwrap(), "1050.5");
        assert_eq!(ticker.change_utc8.as_ref().unwrap(), "1075.2");
        assert_eq!(ticker.total_size.as_ref().unwrap(), "15000");
        assert_eq!(ticker.volume_24h.as_ref().unwrap(), "125000000");
        assert_eq!(ticker.volume_24h_btc.as_ref().unwrap(), "2890.5");
        assert_eq!(ticker.volume_24h_usd.as_ref().unwrap(), "125000000");
        assert_eq!(ticker.volume_24h_base.as_ref().unwrap(), "2890.5");
        assert_eq!(ticker.volume_24h_quote.as_ref().unwrap(), "125000000");
        assert_eq!(ticker.volume_24h_settle.as_ref().unwrap(), "2890.5");
        assert_eq!(ticker.mark_price.as_ref().unwrap(), "43251.2");
        assert_eq!(ticker.funding_rate, "0.000075");
        assert_eq!(ticker.funding_rate_indicative, "0.000082");
        assert_eq!(ticker.index_price.as_ref().unwrap(), "43252.0");
        assert_eq!(ticker.quanto_base_rate.as_ref().unwrap(), "1.0");
        assert_eq!(ticker.funding_next_apply.unwrap(), 1640995200);
        assert_eq!(ticker.basis_rate.as_ref().unwrap(), "0.000025");
        assert_eq!(ticker.basis_value.as_ref().unwrap(), "1.2");
        assert_eq!(ticker.premium_index.as_ref().unwrap(), "0.000002");
    }

    #[test]
    fn test_futures_ticker_minimal_deserialization() {
        let json = r#"{
            "contract": "ETH_USDT",
            "last": "2650.45",
            "lowest_ask": "2650.50",
            "highest_bid": "2650.40",
            "change_percentage": "-1.2",
            "funding_rate": "-0.000034",
            "funding_rate_indicative": "-0.000028"
        }"#;

        let ticker: FuturesTicker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.contract, "ETH_USDT");
        assert_eq!(ticker.last, "2650.45");
        assert_eq!(ticker.lowest_ask, "2650.50");
        assert_eq!(ticker.highest_bid, "2650.40");
        assert_eq!(ticker.change_percentage, "-1.2");
        assert_eq!(ticker.funding_rate, "-0.000034");
        assert_eq!(ticker.funding_rate_indicative, "-0.000028");

        // All optional fields should be None
        assert!(ticker.change_utc0.is_none());
        assert!(ticker.change_utc8.is_none());
        assert!(ticker.total_size.is_none());
        assert!(ticker.volume_24h.is_none());
        assert!(ticker.volume_24h_btc.is_none());
        assert!(ticker.volume_24h_usd.is_none());
        assert!(ticker.volume_24h_base.is_none());
        assert!(ticker.volume_24h_quote.is_none());
        assert!(ticker.volume_24h_settle.is_none());
        assert!(ticker.mark_price.is_none());
        assert!(ticker.index_price.is_none());
        assert!(ticker.quanto_base_rate.is_none());
        assert!(ticker.funding_next_apply.is_none());
        assert!(ticker.basis_rate.is_none());
        assert!(ticker.basis_value.is_none());
        assert!(ticker.premium_index.is_none());
    }

    #[test]
    fn test_bid_ask_spread_validation() {
        let json = r#"{
            "contract": "BTC_USDT",
            "last": "43250.8",
            "lowest_ask": "43251.0",
            "highest_bid": "43250.5",
            "change_percentage": "0.5",
            "funding_rate": "0.000075",
            "funding_rate_indicative": "0.000082"
        }"#;

        let ticker: FuturesTicker = serde_json::from_str(json).unwrap();

        let last: f64 = ticker.last.parse().unwrap();
        let ask: f64 = ticker.lowest_ask.parse().unwrap();
        let bid: f64 = ticker.highest_bid.parse().unwrap();

        // Verify price relationships
        assert!(ask > bid); // Ask should be higher than bid
        assert!(last >= bid && last <= ask); // Last should be between bid and ask

        let spread = ask - bid;
        assert!(spread > 0.0);
        assert!(spread < last * 0.01); // Spread should be reasonable (< 1% of price)
    }

    #[test]
    fn test_realistic_btc_ticker_scenario() {
        let json = r#"{
            "contract": "BTC_USDT",
            "last": "43250.8",
            "lowest_ask": "43251.0",
            "highest_bid": "43250.5",
            "change_percentage": "2.5",
            "change_utc0": "1050.5",
            "change_utc8": "1075.2",
            "total_size": "15000",
            "volume_24h": "125000000",
            "volume_24h_btc": "2890.5",
            "volume_24h_usd": "125000000",
            "mark_price": "43251.2",
            "funding_rate": "0.000075",
            "funding_rate_indicative": "0.000082",
            "index_price": "43252.0",
            "funding_next_apply": 1640995200
        }"#;

        let ticker: FuturesTicker = serde_json::from_str(json).unwrap();

        // Verify BTC price is reasonable
        let last_price: f64 = ticker.last.parse().unwrap();
        assert!(last_price > 20000.0 && last_price < 100000.0);

        // Verify positive change
        let change: f64 = ticker.change_percentage.parse().unwrap();
        assert!(change > 0.0);

        // Verify funding rate is reasonable
        let funding: f64 = ticker.funding_rate.parse().unwrap();
        assert!(funding > 0.0 && funding < 0.001);

        // Verify volumes are positive
        let volume: f64 = ticker.volume_24h.as_ref().unwrap().parse().unwrap();
        assert!(volume > 0.0);
    }

    #[test]
    fn test_realistic_eth_ticker_scenario() {
        let json = r#"{
            "contract": "ETH_USDT",
            "last": "2650.45",
            "lowest_ask": "2650.50",
            "highest_bid": "2650.40",
            "change_percentage": "-1.2",
            "volume_24h": "85000000",
            "volume_24h_btc": "32000.5",
            "mark_price": "2650.48",
            "funding_rate": "-0.000034",
            "funding_rate_indicative": "-0.000028",
            "index_price": "2650.52"
        }"#;

        let ticker: FuturesTicker = serde_json::from_str(json).unwrap();

        // Verify ETH price is reasonable
        let last_price: f64 = ticker.last.parse().unwrap();
        assert!(last_price > 1000.0 && last_price < 10000.0);

        // Verify negative change
        let change: f64 = ticker.change_percentage.parse().unwrap();
        assert!(change < 0.0);

        // Verify negative funding rate
        let funding: f64 = ticker.funding_rate.parse().unwrap();
        assert!(funding < 0.0);
    }

    #[test]
    fn test_different_contract_types() {
        let contracts = vec![
            ("BTC_USDT", "43250.8"),
            ("ETH_USDT", "2650.45"),
            ("ADA_USDT", "0.485"),
            ("SOL_USDT", "98.25"),
            ("MATIC_USDT", "0.825"),
        ];

        for (contract, price) in contracts {
            let json = format!(
                r#"{{
                "contract": "{}",
                "last": "{}",
                "lowest_ask": "{}",
                "highest_bid": "{}",
                "change_percentage": "1.5",
                "funding_rate": "0.000075",
                "funding_rate_indicative": "0.000082"
            }}"#,
                contract, price, price, price
            );

            let ticker: FuturesTicker = serde_json::from_str(&json).unwrap();
            assert_eq!(ticker.contract, contract);
            assert_eq!(ticker.last, price);
        }
    }

    #[test]
    fn test_positive_negative_changes() {
        let change_scenarios = vec![
            ("5.25", "Bull market"),
            ("-3.75", "Bear market"),
            ("0.0", "No change"),
            ("15.5", "Strong rally"),
            ("-12.8", "Strong decline"),
        ];

        for (change, _scenario) in change_scenarios {
            let json = format!(
                r#"{{
                "contract": "BTC_USDT",
                "last": "43250.8",
                "lowest_ask": "43251.0",
                "highest_bid": "43250.5",
                "change_percentage": "{}",
                "funding_rate": "0.000075",
                "funding_rate_indicative": "0.000082"
            }}"#,
                change
            );

            let ticker: FuturesTicker = serde_json::from_str(&json).unwrap();
            assert_eq!(ticker.change_percentage, change);

            let change_val: f64 = ticker.change_percentage.parse().unwrap();
            assert!((-20.0..=20.0).contains(&change_val)); // Reasonable daily change
        }
    }

    #[test]
    fn test_funding_rate_scenarios() {
        let funding_scenarios = vec![
            ("0.000075", "0.000082", "Positive funding"),
            ("-0.000034", "-0.000028", "Negative funding"),
            ("0.0", "0.000001", "Zero current funding"),
            ("0.000375", "0.000375", "Maximum funding"),
            ("-0.000375", "-0.000375", "Minimum funding"),
        ];

        for (current, indicative, _scenario) in funding_scenarios {
            let json = format!(
                r#"{{
                "contract": "BTC_USDT",
                "last": "43250.8",
                "lowest_ask": "43251.0",
                "highest_bid": "43250.5",
                "change_percentage": "1.5",
                "funding_rate": "{}",
                "funding_rate_indicative": "{}"
            }}"#,
                current, indicative
            );

            let ticker: FuturesTicker = serde_json::from_str(&json).unwrap();
            assert_eq!(ticker.funding_rate, current);
            assert_eq!(ticker.funding_rate_indicative, indicative);

            let current_rate: f64 = ticker.funding_rate.parse().unwrap();
            let indicative_rate: f64 = ticker.funding_rate_indicative.parse().unwrap();

            assert!((-0.375..=0.375).contains(&current_rate));
            assert!((-0.375..=0.375).contains(&indicative_rate));
        }
    }

    #[test]
    fn test_volume_metrics_consistency() {
        let json = r#"{
            "contract": "BTC_USDT",
            "last": "43250.8",
            "lowest_ask": "43251.0",
            "highest_bid": "43250.5",
            "change_percentage": "2.5",
            "total_size": "15000",
            "volume_24h": "125000000",
            "volume_24h_btc": "2890.5",
            "volume_24h_usd": "125000000",
            "volume_24h_base": "2890.5",
            "volume_24h_quote": "125000000",
            "funding_rate": "0.000075",
            "funding_rate_indicative": "0.000082"
        }"#;

        let ticker: FuturesTicker = serde_json::from_str(json).unwrap();

        // Check volume consistency
        let volume_24h: f64 = ticker.volume_24h.as_ref().unwrap().parse().unwrap();
        let volume_24h_usd: f64 = ticker.volume_24h_usd.as_ref().unwrap().parse().unwrap();
        let volume_24h_quote: f64 = ticker.volume_24h_quote.as_ref().unwrap().parse().unwrap();

        // These should often be the same for USDT pairs
        assert_eq!(volume_24h, volume_24h_usd);
        assert_eq!(volume_24h, volume_24h_quote);

        // Base volume should be smaller (BTC amount vs USDT amount)
        let volume_24h_base: f64 = ticker.volume_24h_base.as_ref().unwrap().parse().unwrap();
        assert!(volume_24h_base < volume_24h);
    }

    #[test]
    fn test_mark_index_price_relationships() {
        let json = r#"{
            "contract": "BTC_USDT",
            "last": "43250.8",
            "lowest_ask": "43251.0",
            "highest_bid": "43250.5",
            "change_percentage": "2.5",
            "mark_price": "43251.2",
            "index_price": "43252.0",
            "funding_rate": "0.000075",
            "funding_rate_indicative": "0.000082"
        }"#;

        let ticker: FuturesTicker = serde_json::from_str(json).unwrap();

        let last: f64 = ticker.last.parse().unwrap();
        let mark: f64 = ticker.mark_price.as_ref().unwrap().parse().unwrap();
        let index: f64 = ticker.index_price.as_ref().unwrap().parse().unwrap();

        // All prices should be close to each other
        let last_mark_diff = (last - mark).abs();
        let mark_index_diff = (mark - index).abs();

        assert!(last_mark_diff < last * 0.01); // < 1% difference
        assert!(mark_index_diff < mark * 0.01); // < 1% difference
    }

    #[test]
    fn test_high_precision_values() {
        let json = r#"{
            "contract": "BTC_USDT",
            "last": "43250.123456789",
            "lowest_ask": "43251.987654321",
            "highest_bid": "43250.555555555",
            "change_percentage": "2.123456789",
            "volume_24h": "125000000.123456789",
            "mark_price": "43251.234567890",
            "funding_rate": "0.000075123456",
            "funding_rate_indicative": "0.000082987654",
            "basis_rate": "0.000025123456",
            "premium_index": "0.000002987654"
        }"#;

        let ticker: FuturesTicker = serde_json::from_str(json).unwrap();

        // Verify precision is maintained
        assert_eq!(ticker.last, "43250.123456789");
        assert_eq!(ticker.funding_rate, "0.000075123456");
        assert_eq!(ticker.basis_rate.as_ref().unwrap(), "0.000025123456");
        assert_eq!(ticker.premium_index.as_ref().unwrap(), "0.000002987654");
    }

    #[test]
    fn test_large_volume_scenarios() {
        let json = r#"{
            "contract": "BTC_USDT",
            "last": "43250.8",
            "lowest_ask": "43251.0",
            "highest_bid": "43250.5",
            "change_percentage": "2.5",
            "total_size": "999999999",
            "volume_24h": "9999999999999",
            "volume_24h_btc": "231000.123",
            "volume_24h_usd": "9999999999999",
            "funding_rate": "0.000075",
            "funding_rate_indicative": "0.000082"
        }"#;

        let ticker: FuturesTicker = serde_json::from_str(json).unwrap();

        let total_size: i64 = ticker.total_size.as_ref().unwrap().parse().unwrap();
        let volume_24h: f64 = ticker.volume_24h.as_ref().unwrap().parse().unwrap();

        assert_eq!(total_size, 999999999);
        assert!(volume_24h > 1e12); // Very large volume
    }

    #[test]
    fn test_different_settlement_currencies() {
        let settlements = vec!["USDT", "BTC", "USD"];

        for settle in settlements {
            let request = FuturesTickersRequest {
                settle: settle.to_string(),
                contract: Some("BTC_USDT".to_string()),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["settle"], settle);
        }
    }

    #[test]
    fn test_funding_next_apply_scenarios() {
        let timestamps = vec![
            1640995200,         // Fixed timestamp
            1640995200 + 28800, // 8 hours later
            1640995200 + 86400, // 24 hours later
            1735689600,         // Future timestamp
        ];

        for timestamp in timestamps {
            let json = format!(
                r#"{{
                "contract": "BTC_USDT",
                "last": "43250.8",
                "lowest_ask": "43251.0",
                "highest_bid": "43250.5",
                "change_percentage": "2.5",
                "funding_rate": "0.000075",
                "funding_rate_indicative": "0.000082",
                "funding_next_apply": {}
            }}"#,
                timestamp
            );

            let ticker: FuturesTicker = serde_json::from_str(&json).unwrap();
            assert_eq!(ticker.funding_next_apply.unwrap(), timestamp);
        }
    }

    #[test]
    fn test_clone_behavior() {
        let request = FuturesTickersRequest {
            settle: "USDT".to_string(),
            contract: Some("BTC_USDT".to_string()),
        };

        let cloned = request.clone();
        assert_eq!(cloned.settle, request.settle);
        assert_eq!(cloned.contract, request.contract);
    }

    #[test]
    fn test_debug_output() {
        let ticker = FuturesTicker {
            contract: "BTC_USDT".to_string(),
            last: "43250.8".to_string(),
            lowest_ask: "43251.0".to_string(),
            highest_bid: "43250.5".to_string(),
            change_percentage: "2.5".to_string(),
            change_utc0: None,
            change_utc8: None,
            total_size: None,
            volume_24h: None,
            volume_24h_btc: None,
            volume_24h_usd: None,
            volume_24h_base: None,
            volume_24h_quote: None,
            volume_24h_settle: None,
            mark_price: None,
            funding_rate: "0.000075".to_string(),
            funding_rate_indicative: "0.000082".to_string(),
            index_price: None,
            quanto_base_rate: None,
            funding_next_apply: None,
            basis_rate: None,
            basis_value: None,
            premium_index: None,
        };

        let debug_str = format!("{:?}", ticker);
        assert!(debug_str.contains("FuturesTicker"));
        assert!(debug_str.contains("BTC_USDT"));
        assert!(debug_str.contains("43250.8"));
    }

    #[test]
    fn test_serialization_round_trip() {
        let ticker = FuturesTicker {
            contract: "BTC_USDT".to_string(),
            last: "43250.8".to_string(),
            lowest_ask: "43251.0".to_string(),
            highest_bid: "43250.5".to_string(),
            change_percentage: "2.5".to_string(),
            change_utc0: Some("1050.5".to_string()),
            change_utc8: Some("1075.2".to_string()),
            total_size: Some("15000".to_string()),
            volume_24h: Some("125000000".to_string()),
            volume_24h_btc: Some("2890.5".to_string()),
            volume_24h_usd: Some("125000000".to_string()),
            volume_24h_base: Some("2890.5".to_string()),
            volume_24h_quote: Some("125000000".to_string()),
            volume_24h_settle: Some("2890.5".to_string()),
            mark_price: Some("43251.2".to_string()),
            funding_rate: "0.000075".to_string(),
            funding_rate_indicative: "0.000082".to_string(),
            index_price: Some("43252.0".to_string()),
            quanto_base_rate: Some("1.0".to_string()),
            funding_next_apply: Some(1640995200),
            basis_rate: Some("0.000025".to_string()),
            basis_value: Some("1.2".to_string()),
            premium_index: Some("0.000002".to_string()),
        };

        let json = serde_json::to_string(&ticker).unwrap();
        let deserialized: FuturesTicker = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.contract, ticker.contract);
        assert_eq!(deserialized.last, ticker.last);
        assert_eq!(deserialized.funding_rate, ticker.funding_rate);
        assert_eq!(deserialized.mark_price, ticker.mark_price);
        assert_eq!(deserialized.funding_next_apply, ticker.funding_next_apply);
    }
}
