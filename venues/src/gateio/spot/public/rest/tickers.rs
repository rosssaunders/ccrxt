use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for retrieving tickers
#[derive(Debug, Clone, Serialize, Default)]
pub struct TickersRequest {
    /// Currency pair to query ticker for (if omitted, returns all tickers)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,

    /// Timezone for the response (e.g., "utc0", "utc8")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

/// Ticker information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticker {
    /// Currency pair
    pub currency_pair: String,

    /// Last trading price
    pub last: String,

    /// Lowest ask price
    pub lowest_ask: String,

    /// Highest bid price
    pub highest_bid: String,

    /// Change percentage in the last 24h
    pub change_percentage: String,

    /// Change amount in the last 24h
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_utc0: Option<String>,

    /// Change amount in the last 24h in given timezone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_utc8: Option<String>,

    /// Base currency traded volume in the last 24h
    pub base_volume: String,

    /// Quote currency traded volume in the last 24h
    pub quote_volume: String,

    /// Highest price in the last 24h
    pub high_24h: String,

    /// Lowest price in the last 24h
    pub low_24h: String,

    /// ETF net value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etf_net_value: Option<String>,

    /// ETF previous close
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etf_pre_net_value: Option<String>,

    /// ETF rebalance time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etf_pre_timestamp: Option<i64>,

    /// ETF leverage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etf_leverage: Option<String>,
}

impl RestClient {
    /// Get tickers for all or specific currency pairs
    ///
    /// This endpoint returns ticker information including 24h price changes,
    /// volumes, and current bid/ask prices. You can get all tickers or filter
    /// by a specific currency pair and timezone.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#retrieve-ticker-information>
    pub async fn get_tickers(
        &self,
        params: TickersRequest,
    ) -> crate::gateio::spot::Result<Vec<Ticker>> {
        self.get_with_query("/spot/tickers", Some(&params)).await
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_ticker_request_serialization() {
        let request = TickersRequest {
            currency_pair: Some("BTC_USDT".to_string()),
            timezone: Some("utc0".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("BTC_USDT"));
        assert!(serialized.contains("utc0"));

        let default_request = TickersRequest::default();
        let default_serialized = serde_json::to_string(&default_request).unwrap();
        assert_eq!(default_serialized, "{}"); // Empty object for defaults
    }

    #[test]
    fn test_tickers_request_minimal_serialization() {
        let request = TickersRequest {
            currency_pair: None,
            timezone: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_tickers_request_with_currency_pair() {
        let request = TickersRequest {
            currency_pair: Some("BTC_USDT".to_string()),
            timezone: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "currency_pair=BTC_USDT");
    }

    #[test]
    fn test_tickers_request_with_timezone() {
        let request = TickersRequest {
            currency_pair: None,
            timezone: Some("utc8".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "timezone=utc8");
    }

    #[test]
    fn test_tickers_request_full_parameters() {
        let request = TickersRequest {
            currency_pair: Some("ETH_USDT".to_string()),
            timezone: Some("utc0".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("currency_pair=ETH_USDT"));
        assert!(serialized.contains("timezone=utc0"));
    }

    #[test]
    fn test_tickers_request_different_pairs() {
        let pairs = vec!["BTC_USDT", "ETH_BTC", "BNB_USDT", "SOL_USDC", "ADA_USDT"];

        for pair in pairs {
            let request = TickersRequest {
                currency_pair: Some(pair.to_string()),
                timezone: None,
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert_eq!(serialized, format!("currency_pair={}", pair));
        }
    }

    #[test]
    fn test_tickers_request_different_timezones() {
        let timezones = vec!["utc0", "utc8", "utc-5", "local"];

        for timezone in timezones {
            let request = TickersRequest {
                currency_pair: None,
                timezone: Some(timezone.to_string()),
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert_eq!(serialized, format!("timezone={}", timezone));
        }
    }

    #[test]
    fn test_tickers_request_timezone_with_plus() {
        let request = TickersRequest {
            currency_pair: None,
            timezone: Some("utc+1".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "timezone=utc%2B1"); // + gets URL encoded to %2B
    }

    #[test]
    fn test_tickers_request_default() {
        let request = TickersRequest::default();
        assert_eq!(request.currency_pair, None);
        assert_eq!(request.timezone, None);
    }

    #[test]
    fn test_ticker_deserialization() {
        let json = r#"{
            "currency_pair": "BTC_USDT",
            "last": "30000.5",
            "lowest_ask": "30001.0",
            "highest_bid": "29999.5",
            "change_percentage": "2.5",
            "change_utc0": "735.2",
            "change_utc8": "750.1",
            "base_volume": "1234.567",
            "quote_volume": "37037037.5",
            "high_24h": "30500.0",
            "low_24h": "29000.0"
        }"#;

        let ticker: Ticker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.currency_pair, "BTC_USDT");
        assert_eq!(ticker.last, "30000.5");
        assert_eq!(ticker.lowest_ask, "30001.0");
        assert_eq!(ticker.highest_bid, "29999.5");
        assert_eq!(ticker.change_percentage, "2.5");
        assert_eq!(ticker.change_utc0, Some("735.2".to_string()));
        assert_eq!(ticker.change_utc8, Some("750.1".to_string()));
        assert_eq!(ticker.base_volume, "1234.567");
        assert_eq!(ticker.quote_volume, "37037037.5");
        assert_eq!(ticker.high_24h, "30500.0");
        assert_eq!(ticker.low_24h, "29000.0");
    }

    #[test]
    fn test_ticker_minimal_deserialization() {
        let json = r#"{
            "currency_pair": "ETH_USDT",
            "last": "2500.0",
            "lowest_ask": "2501.0",
            "highest_bid": "2499.0",
            "change_percentage": "-1.2",
            "base_volume": "5000.0",
            "quote_volume": "12500000.0",
            "high_24h": "2600.0",
            "low_24h": "2400.0"
        }"#;

        let ticker: Ticker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.currency_pair, "ETH_USDT");
        assert_eq!(ticker.last, "2500.0");
        assert_eq!(ticker.change_percentage, "-1.2");
        assert_eq!(ticker.change_utc0, None);
        assert_eq!(ticker.change_utc8, None);
        assert_eq!(ticker.etf_net_value, None);
        assert_eq!(ticker.etf_pre_net_value, None);
        assert_eq!(ticker.etf_pre_timestamp, None);
        assert_eq!(ticker.etf_leverage, None);
    }

    #[test]
    fn test_ticker_with_etf_fields() {
        let json = r#"{
            "currency_pair": "BTC3L_USDT",
            "last": "150.25",
            "lowest_ask": "150.30",
            "highest_bid": "150.20",
            "change_percentage": "5.2",
            "base_volume": "1000.0",
            "quote_volume": "150250.0",
            "high_24h": "155.0",
            "low_24h": "140.0",
            "etf_net_value": "150.123456",
            "etf_pre_net_value": "148.987654",
            "etf_pre_timestamp": 1640995200,
            "etf_leverage": "3"
        }"#;

        let ticker: Ticker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.currency_pair, "BTC3L_USDT");
        assert_eq!(ticker.etf_net_value, Some("150.123456".to_string()));
        assert_eq!(ticker.etf_pre_net_value, Some("148.987654".to_string()));
        assert_eq!(ticker.etf_pre_timestamp, Some(1640995200));
        assert_eq!(ticker.etf_leverage, Some("3".to_string()));
    }

    #[test]
    fn test_ticker_negative_change() {
        let json = r#"{
            "currency_pair": "BEAR_USDT",
            "last": "0.85",
            "lowest_ask": "0.86",
            "highest_bid": "0.84",
            "change_percentage": "-15.0",
            "change_utc0": "-0.15",
            "base_volume": "10000.0",
            "quote_volume": "8500.0",
            "high_24h": "1.0",
            "low_24h": "0.80"
        }"#;

        let ticker: Ticker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.change_percentage, "-15.0");
        assert_eq!(ticker.change_utc0, Some("-0.15".to_string()));

        // Verify negative change
        let change_pct: f64 = ticker.change_percentage.parse().unwrap();
        assert!(change_pct < 0.0);
    }

    #[test]
    fn test_ticker_zero_volume() {
        let json = r#"{
            "currency_pair": "NEW_USDT",
            "last": "1.0",
            "lowest_ask": "1.01",
            "highest_bid": "0.99",
            "change_percentage": "0.0",
            "base_volume": "0",
            "quote_volume": "0",
            "high_24h": "1.0",
            "low_24h": "1.0"
        }"#;

        let ticker: Ticker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.base_volume, "0");
        assert_eq!(ticker.quote_volume, "0");
        assert_eq!(ticker.change_percentage, "0.0");
        assert_eq!(ticker.high_24h, ticker.low_24h);
    }

    #[test]
    fn test_ticker_high_precision() {
        let json = r#"{
            "currency_pair": "SHIB_USDT",
            "last": "0.00001234",
            "lowest_ask": "0.00001235",
            "highest_bid": "0.00001233",
            "change_percentage": "1.23456789",
            "base_volume": "12345678901234.567890",
            "quote_volume": "152.41937452",
            "high_24h": "0.00001250",
            "low_24h": "0.00001200"
        }"#;

        let ticker: Ticker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.last, "0.00001234");
        assert_eq!(ticker.change_percentage, "1.23456789");
        assert_eq!(ticker.base_volume, "12345678901234.567890");
    }

    #[test]
    fn test_ticker_stablecoin_pair() {
        let json = r#"{
            "currency_pair": "USDC_USDT",
            "last": "1.0001",
            "lowest_ask": "1.0002",
            "highest_bid": "1.0000",
            "change_percentage": "0.01",
            "base_volume": "1000000.0",
            "quote_volume": "1000100.0",
            "high_24h": "1.0005",
            "low_24h": "0.9998"
        }"#;

        let ticker: Ticker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.currency_pair, "USDC_USDT");

        // Verify tight spread for stablecoin pair
        let bid: f64 = ticker.highest_bid.parse().unwrap();
        let ask: f64 = ticker.lowest_ask.parse().unwrap();
        let spread = ask - bid;
        assert!(spread < 0.01); // Tight spread for stablecoins
    }

    #[test]
    fn test_ticker_array_deserialization() {
        let json = r#"[
            {
                "currency_pair": "BTC_USDT",
                "last": "30000.0",
                "lowest_ask": "30001.0",
                "highest_bid": "29999.0",
                "change_percentage": "2.0",
                "base_volume": "1000.0",
                "quote_volume": "30000000.0",
                "high_24h": "30500.0",
                "low_24h": "29000.0"
            },
            {
                "currency_pair": "ETH_USDT",
                "last": "2500.0",
                "lowest_ask": "2501.0",
                "highest_bid": "2499.0",
                "change_percentage": "-1.0",
                "base_volume": "5000.0",
                "quote_volume": "12500000.0",
                "high_24h": "2600.0",
                "low_24h": "2400.0"
            }
        ]"#;

        let tickers: Vec<Ticker> = serde_json::from_str(json).unwrap();
        assert_eq!(tickers.len(), 2);

        assert_eq!(tickers[0].currency_pair, "BTC_USDT");
        assert_eq!(tickers[0].last, "30000.0");
        assert_eq!(tickers[0].change_percentage, "2.0");

        assert_eq!(tickers[1].currency_pair, "ETH_USDT");
        assert_eq!(tickers[1].last, "2500.0");
        assert_eq!(tickers[1].change_percentage, "-1.0");
    }

    #[test]
    fn test_ticker_empty_array() {
        let json = r#"[]"#;
        let tickers: Vec<Ticker> = serde_json::from_str(json).unwrap();
        assert_eq!(tickers.len(), 0);
    }

    #[test]
    fn test_ticker_serialization() {
        let ticker = Ticker {
            currency_pair: "BTC_USDT".to_string(),
            last: "30000.0".to_string(),
            lowest_ask: "30001.0".to_string(),
            highest_bid: "29999.0".to_string(),
            change_percentage: "2.0".to_string(),
            change_utc0: Some("600.0".to_string()),
            change_utc8: None,
            base_volume: "1000.0".to_string(),
            quote_volume: "30000000.0".to_string(),
            high_24h: "30500.0".to_string(),
            low_24h: "29000.0".to_string(),
            etf_net_value: None,
            etf_pre_net_value: None,
            etf_pre_timestamp: None,
            etf_leverage: None,
        };

        let json = serde_json::to_value(&ticker).unwrap();
        assert_eq!(json["currency_pair"], "BTC_USDT");
        assert_eq!(json["last"], "30000.0");
        assert_eq!(json["change_percentage"], "2.0");
        assert_eq!(json["change_utc0"], "600.0");
        assert!(!json.as_object().unwrap().contains_key("change_utc8"));
    }

    #[test]
    fn test_ticker_round_trip() {
        let original = Ticker {
            currency_pair: "ETH_BTC".to_string(),
            last: "0.075".to_string(),
            lowest_ask: "0.0751".to_string(),
            highest_bid: "0.0749".to_string(),
            change_percentage: "1.5".to_string(),
            change_utc0: Some("0.001125".to_string()),
            change_utc8: Some("0.001200".to_string()),
            base_volume: "2000.0".to_string(),
            quote_volume: "150.0".to_string(),
            high_24h: "0.076".to_string(),
            low_24h: "0.074".to_string(),
            etf_net_value: None,
            etf_pre_net_value: None,
            etf_pre_timestamp: None,
            etf_leverage: None,
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Ticker = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.currency_pair, original.currency_pair);
        assert_eq!(deserialized.last, original.last);
        assert_eq!(deserialized.change_percentage, original.change_percentage);
        assert_eq!(deserialized.change_utc0, original.change_utc0);
        assert_eq!(deserialized.change_utc8, original.change_utc8);
        assert_eq!(deserialized.base_volume, original.base_volume);
        assert_eq!(deserialized.quote_volume, original.quote_volume);
    }

    #[test]
    fn test_ticker_realistic_market_scenarios() {
        // Bull market scenario
        let bull_json = r#"{
            "currency_pair": "BTC_USDT",
            "last": "45000.0",
            "lowest_ask": "45050.0",
            "highest_bid": "44950.0",
            "change_percentage": "8.5",
            "base_volume": "2500.0",
            "quote_volume": "112500000.0",
            "high_24h": "45200.0",
            "low_24h": "41000.0"
        }"#;

        let bull_ticker: Ticker = serde_json::from_str(bull_json).unwrap();
        let change: f64 = bull_ticker.change_percentage.parse().unwrap();
        assert!(change > 5.0); // Strong positive movement

        // Bear market scenario
        let bear_json = r#"{
            "currency_pair": "BTC_USDT",
            "last": "25000.0",
            "lowest_ask": "25100.0",
            "highest_bid": "24900.0",
            "change_percentage": "-12.5",
            "base_volume": "5000.0",
            "quote_volume": "125000000.0",
            "high_24h": "29000.0",
            "low_24h": "24500.0"
        }"#;

        let bear_ticker: Ticker = serde_json::from_str(bear_json).unwrap();
        let bear_change: f64 = bear_ticker.change_percentage.parse().unwrap();
        assert!(bear_change < -10.0); // Strong negative movement
    }

    #[test]
    fn test_ticker_different_currencies() {
        let currencies = vec![
            "BTC_USDT",
            "ETH_BTC",
            "BNB_USDT",
            "SOL_USDC",
            "ADA_USDT",
            "DOT_USDT",
            "MATIC_USDT",
            "LINK_USDT",
        ];

        for currency_pair in currencies {
            let json = format!(
                r#"{{
                "currency_pair": "{}",
                "last": "100.0",
                "lowest_ask": "100.1",
                "highest_bid": "99.9",
                "change_percentage": "1.0",
                "base_volume": "1000.0",
                "quote_volume": "100000.0",
                "high_24h": "105.0",
                "low_24h": "95.0"
            }}"#,
                currency_pair
            );

            let ticker: Ticker = serde_json::from_str(&json).unwrap();
            assert_eq!(ticker.currency_pair, currency_pair);
        }
    }

    #[test]
    fn test_ticker_clone() {
        let original = Ticker {
            currency_pair: "BTC_USDT".to_string(),
            last: "30000.0".to_string(),
            lowest_ask: "30001.0".to_string(),
            highest_bid: "29999.0".to_string(),
            change_percentage: "2.0".to_string(),
            change_utc0: Some("600.0".to_string()),
            change_utc8: None,
            base_volume: "1000.0".to_string(),
            quote_volume: "30000000.0".to_string(),
            high_24h: "30500.0".to_string(),
            low_24h: "29000.0".to_string(),
            etf_net_value: None,
            etf_pre_net_value: None,
            etf_pre_timestamp: None,
            etf_leverage: None,
        };

        let cloned = original.clone();
        assert_eq!(cloned.currency_pair, original.currency_pair);
        assert_eq!(cloned.last, original.last);
        assert_eq!(cloned.change_percentage, original.change_percentage);
        assert_eq!(cloned.change_utc0, original.change_utc0);
        assert_eq!(cloned.base_volume, original.base_volume);
    }

    #[test]
    fn test_ticker_debug() {
        let ticker = Ticker {
            currency_pair: "BTC_USDT".to_string(),
            last: "30000.0".to_string(),
            lowest_ask: "30001.0".to_string(),
            highest_bid: "29999.0".to_string(),
            change_percentage: "2.0".to_string(),
            change_utc0: None,
            change_utc8: None,
            base_volume: "1000.0".to_string(),
            quote_volume: "30000000.0".to_string(),
            high_24h: "30500.0".to_string(),
            low_24h: "29000.0".to_string(),
            etf_net_value: None,
            etf_pre_net_value: None,
            etf_pre_timestamp: None,
            etf_leverage: None,
        };

        let debug_str = format!("{:?}", ticker);
        assert!(debug_str.contains("Ticker"));
        assert!(debug_str.contains("BTC_USDT"));
        assert!(debug_str.contains("30000.0"));
    }
}
