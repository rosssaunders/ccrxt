use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for futures contract stats
#[derive(Debug, Clone, Serialize, Default)]
pub struct FuturesStatsRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract name
    pub contract: String,
    /// Start time in Unix seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    /// Interval time between data points (default 5m)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// Maximum number of records to return (1-200, default 30)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Futures contract statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesStats {
    /// Statistical timestamp
    pub time: i64,

    /// Long/short account number ratio
    pub lsr_taker: f64,

    /// Long/short position ratio  
    pub lsr_account: f64,

    /// Long liquidation size
    pub long_liq_size: serde_json::Value,

    /// Long liquidation amount
    pub long_liq_amount: serde_json::Value,

    /// Long liquidation volume (in USD)
    pub long_liq_usd: serde_json::Value,

    /// Short liquidation size
    pub short_liq_size: serde_json::Value,

    /// Short liquidation amount
    pub short_liq_amount: serde_json::Value,

    /// Short liquidation volume (in USD)
    pub short_liq_usd: serde_json::Value,

    /// Open interest
    pub open_interest: serde_json::Value,

    /// Mark price
    pub mark_price: serde_json::Value,

    /// Top trader long/short position ratio
    pub top_lsr_account: f64,

    /// Top trader long/short size ratio
    pub top_lsr_size: f64,
}

impl RestClient {
    /// Get futures contract stats
    ///
    /// Retrieves statistical data for a specific futures contract including
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#futures-stats>
    /// liquidation data, position ratios, and open interest.
    pub async fn get_futures_stats(
        &self,
        params: FuturesStatsRequest,
    ) -> crate::gateio::perpetual::Result<Vec<FuturesStats>> {
        let endpoint = format!("/futures/{}/contract_stats", params.settle);
        self.get_with_query(&endpoint, Some(&params)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_futures_stats_request_minimal() {
        let request = FuturesStatsRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            from: None,
            interval: None,
            limit: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "BTC_USDT");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 2); // Only settle and contract
        assert!(!obj.contains_key("from"));
        assert!(!obj.contains_key("interval"));
        assert!(!obj.contains_key("limit"));
    }

    #[test]
    fn test_futures_stats_request_full() {
        let request = FuturesStatsRequest {
            settle: "USDT".to_string(),
            contract: "ETH_USDT".to_string(),
            from: Some(1640995200),
            interval: Some("1h".to_string()),
            limit: Some(100),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "ETH_USDT");
        assert_eq!(json["from"], 1640995200);
        assert_eq!(json["interval"], "1h");
        assert_eq!(json["limit"], 100);

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 5);
    }

    #[test]
    fn test_different_settlement_currencies() {
        let currencies = vec!["USDT", "BTC", "USD"];

        for settle in currencies {
            let request = FuturesStatsRequest {
                settle: settle.to_string(),
                contract: "BTC_USDT".to_string(),
                from: None,
                interval: None,
                limit: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["settle"], settle);
        }
    }

    #[test]
    fn test_different_contract_pairs() {
        let contracts = vec![
            "BTC_USDT",
            "ETH_USDT",
            "ADA_USDT",
            "SOL_USDT",
            "MATIC_USDT",
            "DOT_USDT",
            "AVAX_USDT",
            "LINK_USDT",
        ];

        for contract in contracts {
            let request = FuturesStatsRequest {
                settle: "USDT".to_string(),
                contract: contract.to_string(),
                from: None,
                interval: None,
                limit: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["contract"], contract);
        }
    }

    #[test]
    fn test_different_intervals() {
        let intervals = vec!["5m", "15m", "30m", "1h", "4h", "1d"];

        for interval in intervals {
            let request = FuturesStatsRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                from: None,
                interval: Some(interval.to_string()),
                limit: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["interval"], interval);
        }
    }

    #[test]
    fn test_limit_variations() {
        let limits = vec![1, 10, 30, 50, 100, 200];

        for limit in limits {
            let request = FuturesStatsRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                from: None,
                interval: None,
                limit: Some(limit),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["limit"], limit);
            assert!(limit >= 1 && limit <= 200);
        }
    }

    #[test]
    fn test_from_timestamp_scenarios() {
        let timestamps = vec![
            1640995200, // Recent timestamp
            1577836800, // Year 2020
            1609459200, // Year 2021
        ];

        for timestamp in timestamps {
            let request = FuturesStatsRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                from: Some(timestamp),
                interval: None,
                limit: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["from"], timestamp);
        }
    }

    #[test]
    fn test_futures_stats_deserialization() {
        let json = r#"{
            "time": 1640995200,
            "lsr_taker": 1.25,
            "lsr_account": 0.85,
            "long_liq_size": "150000",
            "long_liq_amount": "6450000",
            "long_liq_usd": "6450000",
            "short_liq_size": "200000",
            "short_liq_amount": "8600000",
            "short_liq_usd": "8600000",
            "open_interest": "45000000",
            "mark_price": "43250.8",
            "top_lsr_account": 1.15,
            "top_lsr_size": 1.35
        }"#;

        let stats: FuturesStats = serde_json::from_str(json).unwrap();
        assert_eq!(stats.time, 1640995200);
        assert_eq!(stats.lsr_taker, 1.25);
        assert_eq!(stats.lsr_account, 0.85);
        assert_eq!(stats.top_lsr_account, 1.15);
        assert_eq!(stats.top_lsr_size, 1.35);

        // Verify JSON values can be extracted
        assert!(stats.long_liq_size.is_string());
        assert!(stats.open_interest.is_string());
        assert!(stats.mark_price.is_string());
    }

    #[test]
    fn test_bullish_market_stats() {
        let json = r#"{
            "time": 1640995200,
            "lsr_taker": 1.45,
            "lsr_account": 1.25,
            "long_liq_size": "50000",
            "long_liq_amount": "2150000",
            "long_liq_usd": "2150000",
            "short_liq_size": "250000",
            "short_liq_amount": "10750000",
            "short_liq_usd": "10750000",
            "open_interest": "55000000",
            "mark_price": "43850.5",
            "top_lsr_account": 1.65,
            "top_lsr_size": 1.80
        }"#;

        let stats: FuturesStats = serde_json::from_str(json).unwrap();

        // In bullish markets, long ratios should be higher
        assert!(stats.lsr_taker > 1.0); // More longs than shorts
        assert!(stats.lsr_account > 1.0); // More long accounts
        assert!(stats.top_lsr_account > 1.0); // Top traders are long-biased
        assert!(stats.top_lsr_size > 1.0); // Top traders have larger long positions

        // More short liquidations than long in bull market
        let long_liq: f64 = stats.long_liq_usd.as_str().unwrap().parse().unwrap();
        let short_liq: f64 = stats.short_liq_usd.as_str().unwrap().parse().unwrap();
        assert!(short_liq > long_liq);
    }

    #[test]
    fn test_bearish_market_stats() {
        let json = r#"{
            "time": 1640995200,
            "lsr_taker": 0.65,
            "lsr_account": 0.75,
            "long_liq_size": "300000",
            "long_liq_amount": "12900000",
            "long_liq_usd": "12900000",
            "short_liq_size": "100000",
            "short_liq_amount": "4300000",
            "short_liq_usd": "4300000",
            "open_interest": "42000000",
            "mark_price": "41250.2",
            "top_lsr_account": 0.75,
            "top_lsr_size": 0.60
        }"#;

        let stats: FuturesStats = serde_json::from_str(json).unwrap();

        // In bearish markets, short ratios should be higher (ratios < 1.0)
        assert!(stats.lsr_taker < 1.0); // More shorts than longs
        assert!(stats.lsr_account < 1.0); // More short accounts
        assert!(stats.top_lsr_account < 1.0); // Top traders are short-biased
        assert!(stats.top_lsr_size < 1.0); // Top traders have larger short positions

        // More long liquidations than short in bear market
        let long_liq: f64 = stats.long_liq_usd.as_str().unwrap().parse().unwrap();
        let short_liq: f64 = stats.short_liq_usd.as_str().unwrap().parse().unwrap();
        assert!(long_liq > short_liq);
    }

    #[test]
    fn test_balanced_market_stats() {
        let json = r#"{
            "time": 1640995200,
            "lsr_taker": 0.98,
            "lsr_account": 1.02,
            "long_liq_size": "120000",
            "long_liq_amount": "5160000",
            "long_liq_usd": "5160000",
            "short_liq_size": "125000",
            "short_liq_amount": "5375000",
            "short_liq_usd": "5375000",
            "open_interest": "48000000",
            "mark_price": "43000.0",
            "top_lsr_account": 1.05,
            "top_lsr_size": 0.95
        }"#;

        let stats: FuturesStats = serde_json::from_str(json).unwrap();

        // In balanced markets, ratios should be close to 1.0
        assert!(stats.lsr_taker > 0.9 && stats.lsr_taker < 1.1);
        assert!(stats.lsr_account > 0.9 && stats.lsr_account < 1.1);
        assert!(stats.top_lsr_account > 0.9 && stats.top_lsr_account < 1.1);
        assert!(stats.top_lsr_size > 0.9 && stats.top_lsr_size < 1.1);

        // Liquidations should be relatively balanced
        let long_liq: f64 = stats.long_liq_usd.as_str().unwrap().parse().unwrap();
        let short_liq: f64 = stats.short_liq_usd.as_str().unwrap().parse().unwrap();
        let liq_ratio = long_liq / short_liq;
        assert!(liq_ratio > 0.8 && liq_ratio < 1.2);
    }

    #[test]
    fn test_high_liquidation_scenario() {
        let json = r#"{
            "time": 1640995200,
            "lsr_taker": 0.45,
            "lsr_account": 0.55,
            "long_liq_size": "1500000",
            "long_liq_amount": "64500000",
            "long_liq_usd": "64500000",
            "short_liq_size": "800000",
            "short_liq_amount": "34400000",
            "short_liq_usd": "34400000",
            "open_interest": "35000000",
            "mark_price": "40150.8",
            "top_lsr_account": 0.35,
            "top_lsr_size": 0.25
        }"#;

        let stats: FuturesStats = serde_json::from_str(json).unwrap();

        // High liquidation scenario (market crash)
        let long_liq: f64 = stats.long_liq_usd.as_str().unwrap().parse().unwrap();
        let short_liq: f64 = stats.short_liq_usd.as_str().unwrap().parse().unwrap();
        let open_interest: f64 = stats.open_interest.as_str().unwrap().parse().unwrap();

        // Total liquidations should be significant relative to open interest
        let total_liq = long_liq + short_liq;
        let liq_percentage = total_liq / open_interest;
        assert!(liq_percentage > 2.0); // > 200% of open interest liquidated

        // Long liquidations should dominate in crash scenario
        assert!(long_liq > short_liq * 1.5);
    }

    #[test]
    fn test_low_liquidation_scenario() {
        let json = r#"{
            "time": 1640995200,
            "lsr_taker": 1.05,
            "lsr_account": 0.95,
            "long_liq_size": "25000",
            "long_liq_amount": "1075000",
            "long_liq_usd": "1075000",
            "short_liq_size": "30000",
            "short_liq_amount": "1290000",
            "short_liq_usd": "1290000",
            "open_interest": "62000000",
            "mark_price": "43100.5",
            "top_lsr_account": 1.10,
            "top_lsr_size": 1.15
        }"#;

        let stats: FuturesStats = serde_json::from_str(json).unwrap();

        // Low liquidation scenario (stable market)
        let long_liq: f64 = stats.long_liq_usd.as_str().unwrap().parse().unwrap();
        let short_liq: f64 = stats.short_liq_usd.as_str().unwrap().parse().unwrap();
        let open_interest: f64 = stats.open_interest.as_str().unwrap().parse().unwrap();

        // Total liquidations should be small relative to open interest
        let total_liq = long_liq + short_liq;
        let liq_percentage = total_liq / open_interest;
        assert!(liq_percentage < 0.1); // < 10% of open interest liquidated

        // Liquidations should be relatively balanced
        let liq_ratio = long_liq / short_liq;
        assert!(liq_ratio > 0.6 && liq_ratio < 1.6);
    }

    #[test]
    fn test_open_interest_scenarios() {
        let oi_scenarios = vec![
            ("15000000", "Low OI"),
            ("45000000", "Medium OI"),
            ("85000000", "High OI"),
            ("150000000", "Very high OI"),
        ];

        for (oi, _description) in oi_scenarios {
            let json = format!(
                r#"{{
                "time": 1640995200,
                "lsr_taker": 1.05,
                "lsr_account": 0.95,
                "long_liq_size": "50000",
                "long_liq_amount": "2150000",
                "long_liq_usd": "2150000",
                "short_liq_size": "60000",
                "short_liq_amount": "2580000",
                "short_liq_usd": "2580000",
                "open_interest": "{}",
                "mark_price": "43000.0",
                "top_lsr_account": 1.00,
                "top_lsr_size": 1.05
            }}"#,
                oi
            );

            let stats: FuturesStats = serde_json::from_str(&json).unwrap();
            let open_interest: f64 = stats.open_interest.as_str().unwrap().parse().unwrap();
            assert!(open_interest > 0.0);
            assert_eq!(stats.open_interest.as_str().unwrap(), oi);
        }
    }

    #[test]
    fn test_mark_price_scenarios() {
        let price_scenarios = vec![
            ("35000.5", "Lower BTC price"),
            ("43250.8", "Mid BTC price"),
            ("58750.2", "Higher BTC price"),
            ("2650.45", "ETH price"),
        ];

        for (price, _description) in price_scenarios {
            let json = format!(
                r#"{{
                "time": 1640995200,
                "lsr_taker": 1.05,
                "lsr_account": 0.95,
                "long_liq_size": "50000",
                "long_liq_amount": "2150000",
                "long_liq_usd": "2150000",
                "short_liq_size": "60000",
                "short_liq_amount": "2580000",
                "short_liq_usd": "2580000",
                "open_interest": "45000000",
                "mark_price": "{}",
                "top_lsr_account": 1.00,
                "top_lsr_size": 1.05
            }}"#,
                price
            );

            let stats: FuturesStats = serde_json::from_str(&json).unwrap();
            let mark_price: f64 = stats.mark_price.as_str().unwrap().parse().unwrap();
            assert!(mark_price > 0.0);
            assert_eq!(stats.mark_price.as_str().unwrap(), price);
        }
    }

    #[test]
    fn test_extreme_ratio_scenarios() {
        let extreme_ratios = vec![
            (0.1, 0.15, 0.05, 0.08, "Extremely bearish"),
            (5.0, 4.8, 6.2, 5.5, "Extremely bullish"),
            (0.01, 0.02, 0.01, 0.015, "Nearly all shorts"),
            (50.0, 45.0, 55.0, 52.0, "Nearly all longs"),
        ];

        for (lsr_taker, lsr_account, top_lsr_account, top_lsr_size, _description) in extreme_ratios
        {
            let json = format!(
                r#"{{
                "time": 1640995200,
                "lsr_taker": {},
                "lsr_account": {},
                "long_liq_size": "50000",
                "long_liq_amount": "2150000",
                "long_liq_usd": "2150000",
                "short_liq_size": "60000",
                "short_liq_amount": "2580000",
                "short_liq_usd": "2580000",
                "open_interest": "45000000",
                "mark_price": "43000.0",
                "top_lsr_account": {},
                "top_lsr_size": {}
            }}"#,
                lsr_taker, lsr_account, top_lsr_account, top_lsr_size
            );

            let stats: FuturesStats = serde_json::from_str(&json).unwrap();
            assert_eq!(stats.lsr_taker, lsr_taker);
            assert_eq!(stats.lsr_account, lsr_account);
            assert_eq!(stats.top_lsr_account, top_lsr_account);
            assert_eq!(stats.top_lsr_size, top_lsr_size);

            // All ratios should be positive
            assert!(stats.lsr_taker > 0.0);
            assert!(stats.lsr_account > 0.0);
            assert!(stats.top_lsr_account > 0.0);
            assert!(stats.top_lsr_size > 0.0);
        }
    }

    #[test]
    fn test_zero_liquidation_scenario() {
        let json = r#"{
            "time": 1640995200,
            "lsr_taker": 1.02,
            "lsr_account": 0.98,
            "long_liq_size": "0",
            "long_liq_amount": "0",
            "long_liq_usd": "0",
            "short_liq_size": "0",
            "short_liq_amount": "0",
            "short_liq_usd": "0",
            "open_interest": "50000000",
            "mark_price": "43200.0",
            "top_lsr_account": 1.05,
            "top_lsr_size": 0.95
        }"#;

        let stats: FuturesStats = serde_json::from_str(json).unwrap();

        // Verify zero liquidations
        assert_eq!(stats.long_liq_size.as_str().unwrap(), "0");
        assert_eq!(stats.long_liq_amount.as_str().unwrap(), "0");
        assert_eq!(stats.long_liq_usd.as_str().unwrap(), "0");
        assert_eq!(stats.short_liq_size.as_str().unwrap(), "0");
        assert_eq!(stats.short_liq_amount.as_str().unwrap(), "0");
        assert_eq!(stats.short_liq_usd.as_str().unwrap(), "0");

        // Open interest should still be positive
        let oi: f64 = stats.open_interest.as_str().unwrap().parse().unwrap();
        assert!(oi > 0.0);
    }

    #[test]
    fn test_liquidation_size_amount_consistency() {
        let json = r#"{
            "time": 1640995200,
            "lsr_taker": 1.25,
            "lsr_account": 0.85,
            "long_liq_size": "1000",
            "long_liq_amount": "43250000",
            "long_liq_usd": "43250000",
            "short_liq_size": "500",
            "short_liq_amount": "21625000",
            "short_liq_usd": "21625000",
            "open_interest": "45000000",
            "mark_price": "43250.0",
            "top_lsr_account": 1.15,
            "top_lsr_size": 1.35
        }"#;

        let stats: FuturesStats = serde_json::from_str(json).unwrap();

        // Check consistency between size and amount
        let long_size: f64 = stats.long_liq_size.as_str().unwrap().parse().unwrap();
        let long_amount: f64 = stats.long_liq_amount.as_str().unwrap().parse().unwrap();
        let mark_price: f64 = stats.mark_price.as_str().unwrap().parse().unwrap();

        // Amount should approximately equal size * price
        let expected_amount = long_size * mark_price;
        let difference = (long_amount - expected_amount).abs();
        let tolerance = expected_amount * 0.01; // 1% tolerance
        assert!(
            difference <= tolerance,
            "Long liquidation amount inconsistent with size * price"
        );

        // Same check for shorts
        let short_size: f64 = stats.short_liq_size.as_str().unwrap().parse().unwrap();
        let short_amount: f64 = stats.short_liq_amount.as_str().unwrap().parse().unwrap();

        let expected_short_amount = short_size * mark_price;
        let short_difference = (short_amount - expected_short_amount).abs();
        let short_tolerance = expected_short_amount * 0.01;
        assert!(
            short_difference <= short_tolerance,
            "Short liquidation amount inconsistent with size * price"
        );
    }

    #[test]
    fn test_stats_time_series() {
        // Simulate stats evolution over time showing market transition
        let time_series = vec![
            (1640995200, 1.5, 1.3, "2150000", "5375000", "Bull phase"),
            (1640995500, 1.2, 1.1, "3225000", "4300000", "Cooling down"),
            (
                1640995800,
                0.9,
                0.8,
                "5375000",
                "3225000",
                "Turning bearish",
            ),
            (1640996100, 0.6, 0.5, "8600000", "2150000", "Bear phase"),
            (1640996400, 1.0, 0.9, "4300000", "4515000", "Stabilizing"),
        ];

        let mut prev_time = 0;
        for (time, lsr_taker, lsr_account, long_liq, short_liq, _phase) in time_series {
            let json = format!(
                r#"{{
                "time": {},
                "lsr_taker": {},
                "lsr_account": {},
                "long_liq_size": "100000",
                "long_liq_amount": "{}",
                "long_liq_usd": "{}",
                "short_liq_size": "125000",
                "short_liq_amount": "{}",
                "short_liq_usd": "{}",
                "open_interest": "45000000",
                "mark_price": "43000.0",
                "top_lsr_account": {},
                "top_lsr_size": {}
            }}"#,
                time,
                lsr_taker,
                lsr_account,
                long_liq,
                long_liq,
                short_liq,
                short_liq,
                lsr_account,
                lsr_taker
            );

            let stats: FuturesStats = serde_json::from_str(&json).unwrap();

            // Verify time progression
            assert!(stats.time > prev_time);
            prev_time = stats.time;

            // Verify ratios are consistent with market phase
            if lsr_taker > 1.0 {
                // Bullish phase - more long liquidations expected to be lower
                let long_liq_val: f64 = stats.long_liq_usd.as_str().unwrap().parse().unwrap();
                let short_liq_val: f64 = stats.short_liq_usd.as_str().unwrap().parse().unwrap();
                // In bull market, shorts get liquidated more
                if lsr_taker > 1.2 {
                    assert!(short_liq_val >= long_liq_val);
                }
            }
        }
    }

    #[test]
    fn test_top_trader_analysis() {
        let json = r#"{
            "time": 1640995200,
            "lsr_taker": 1.05,
            "lsr_account": 0.95,
            "long_liq_size": "50000",
            "long_liq_amount": "2150000",
            "long_liq_usd": "2150000",
            "short_liq_size": "60000",
            "short_liq_amount": "2580000",
            "short_liq_usd": "2580000",
            "open_interest": "45000000",
            "mark_price": "43000.0",
            "top_lsr_account": 1.85,
            "top_lsr_size": 2.25
        }"#;

        let stats: FuturesStats = serde_json::from_str(json).unwrap();

        // Top traders are significantly more bullish than overall market
        assert!(stats.top_lsr_account > stats.lsr_account);
        assert!(stats.top_lsr_size > stats.lsr_taker);

        // Top traders have larger positions relative to their account bias
        assert!(stats.top_lsr_size > stats.top_lsr_account);

        // This suggests top traders are not only more bullish but also
        // sizing their positions accordingly
    }

    #[test]
    fn test_market_maker_vs_taker_analysis() {
        let json = r#"{
            "time": 1640995200,
            "lsr_taker": 0.75,
            "lsr_account": 1.15,
            "long_liq_size": "150000",
            "long_liq_amount": "6450000",
            "long_liq_usd": "6450000",
            "short_liq_size": "200000",
            "short_liq_amount": "8600000",
            "short_liq_usd": "8600000",
            "open_interest": "45000000",
            "mark_price": "43000.0",
            "top_lsr_account": 1.25,
            "top_lsr_size": 0.85
        }"#;

        let stats: FuturesStats = serde_json::from_str(json).unwrap();

        // Scenario where accounts are bullish but takers are bearish
        // This might suggest market makers are providing short liquidity
        // while retail (takers) are selling
        assert!(stats.lsr_account > 1.0); // More long accounts
        assert!(stats.lsr_taker < 1.0); // But more short takers

        // Divergence suggests different behavior between market participants
        let divergence = (stats.lsr_account - stats.lsr_taker).abs();
        assert!(divergence > 0.3); // Significant divergence
    }

    #[test]
    fn test_json_value_handling() {
        let json = r#"{
            "time": 1640995200,
            "lsr_taker": 1.05,
            "lsr_account": 0.95,
            "long_liq_size": null,
            "long_liq_amount": "2150000",
            "long_liq_usd": 2150000,
            "short_liq_size": "60000",
            "short_liq_amount": null,
            "short_liq_usd": "2580000",
            "open_interest": 45000000,
            "mark_price": "43000.0",
            "top_lsr_account": 1.00,
            "top_lsr_size": 1.05
        }"#;

        let stats: FuturesStats = serde_json::from_str(json).unwrap();

        // Should handle null values
        assert!(stats.long_liq_size.is_null());
        assert!(stats.short_liq_amount.is_null());

        // Should handle string values
        assert!(stats.long_liq_amount.is_string());
        assert!(stats.short_liq_usd.is_string());
        assert!(stats.mark_price.is_string());

        // Should handle number values
        assert!(stats.long_liq_usd.is_number());
        assert!(stats.open_interest.is_number());
    }

    #[test]
    fn test_clone_behavior() {
        let request = FuturesStatsRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            from: Some(1640995200),
            interval: Some("1h".to_string()),
            limit: Some(50),
        };

        let cloned = request.clone();
        assert_eq!(cloned.settle, request.settle);
        assert_eq!(cloned.contract, request.contract);
        assert_eq!(cloned.from, request.from);
        assert_eq!(cloned.interval, request.interval);
        assert_eq!(cloned.limit, request.limit);
    }

    #[test]
    fn test_debug_output() {
        let stats = FuturesStats {
            time: 1640995200,
            lsr_taker: 1.25,
            lsr_account: 0.85,
            long_liq_size: serde_json::Value::String("150000".to_string()),
            long_liq_amount: serde_json::Value::String("6450000".to_string()),
            long_liq_usd: serde_json::Value::String("6450000".to_string()),
            short_liq_size: serde_json::Value::String("200000".to_string()),
            short_liq_amount: serde_json::Value::String("8600000".to_string()),
            short_liq_usd: serde_json::Value::String("8600000".to_string()),
            open_interest: serde_json::Value::String("45000000".to_string()),
            mark_price: serde_json::Value::String("43250.8".to_string()),
            top_lsr_account: 1.15,
            top_lsr_size: 1.35,
        };

        let debug_str = format!("{:?}", stats);
        assert!(debug_str.contains("FuturesStats"));
        assert!(debug_str.contains("1640995200"));
        assert!(debug_str.contains("1.25"));
    }

    #[test]
    fn test_serialization_round_trip() {
        let stats = FuturesStats {
            time: 1640995200,
            lsr_taker: 1.25,
            lsr_account: 0.85,
            long_liq_size: serde_json::Value::String("150000".to_string()),
            long_liq_amount: serde_json::Value::String("6450000".to_string()),
            long_liq_usd: serde_json::Value::String("6450000".to_string()),
            short_liq_size: serde_json::Value::String("200000".to_string()),
            short_liq_amount: serde_json::Value::String("8600000".to_string()),
            short_liq_usd: serde_json::Value::String("8600000".to_string()),
            open_interest: serde_json::Value::String("45000000".to_string()),
            mark_price: serde_json::Value::String("43250.8".to_string()),
            top_lsr_account: 1.15,
            top_lsr_size: 1.35,
        };

        let json = serde_json::to_string(&stats).unwrap();
        let deserialized: FuturesStats = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.time, stats.time);
        assert_eq!(deserialized.lsr_taker, stats.lsr_taker);
        assert_eq!(deserialized.lsr_account, stats.lsr_account);
        assert_eq!(deserialized.top_lsr_account, stats.top_lsr_account);
        assert_eq!(deserialized.top_lsr_size, stats.top_lsr_size);
    }
}
