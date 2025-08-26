use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::gateio::RestResult;

/// Request parameters for futures trades
#[derive(Debug, Clone, Serialize, Default)]
pub struct FuturesTradesRequest {
    /// Settlement currency
    pub settle: String,

    /// Contract name
    pub contract: String,

    /// Maximum number of records to return (1-1000, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Specify list offset (default 0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,

    /// Specify the starting point for this list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_id: Option<String>,

    /// Specify starting time in Unix seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// Specify ending time in Unix seconds  
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
}

/// Futures trade entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesTrade {
    /// Trade ID
    pub id: i64,

    /// Trading time
    pub create_time: f64,

    /// Trading contract
    pub contract: String,

    /// Trading size
    pub size: i64,

    /// Trading price
    pub price: String,

    /// Whether internal trade
    pub is_internal: Option<bool>,
}

impl RestClient {
    /// Get futures trading history
    ///
    /// Retrieves recent trades for a specific futures contract.
    ///
    /// # API Documentation
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#futures-trading-history)
    /// Maximum of 1000 records can be returned per request.
    pub async fn get_futures_trades(
        &self,
        params: FuturesTradesRequest,
    ) -> RestResult<Vec<FuturesTrade>> {
        let endpoint = format!("/futures/{}/trades", params.settle);
        self.get_with_query(&endpoint, Some(&params)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_futures_trades_request_minimal() {
        let request = FuturesTradesRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            limit: None,
            offset: None,
            last_id: None,
            from: None,
            to: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "BTC_USDT");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 2); // Only settle and contract
        assert!(!obj.contains_key("limit"));
        assert!(!obj.contains_key("offset"));
        assert!(!obj.contains_key("last_id"));
        assert!(!obj.contains_key("from"));
        assert!(!obj.contains_key("to"));
    }

    #[test]
    fn test_futures_trades_request_full() {
        let request = FuturesTradesRequest {
            settle: "USDT".to_string(),
            contract: "ETH_USDT".to_string(),
            limit: Some(500),
            offset: Some(100),
            last_id: Some("12345678".to_string()),
            from: Some(1640995200),
            to: Some(1640998800),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "ETH_USDT");
        assert_eq!(json["limit"], 500);
        assert_eq!(json["offset"], 100);
        assert_eq!(json["last_id"], "12345678");
        assert_eq!(json["from"], 1640995200);
        assert_eq!(json["to"], 1640998800);

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 7);
    }

    #[test]
    fn test_different_settlement_currencies() {
        let currencies = vec!["USDT", "BTC", "USD"];

        for settle in currencies {
            let request = FuturesTradesRequest {
                settle: settle.to_string(),
                contract: "BTC_USDT".to_string(),
                limit: None,
                offset: None,
                last_id: None,
                from: None,
                to: None,
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
            let request = FuturesTradesRequest {
                settle: "USDT".to_string(),
                contract: contract.to_string(),
                limit: None,
                offset: None,
                last_id: None,
                from: None,
                to: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["contract"], contract);
        }
    }

    #[test]
    fn test_limit_variations() {
        let limits = vec![1, 10, 50, 100, 500, 1000];

        for limit in limits {
            let request = FuturesTradesRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                limit: Some(limit),
                offset: None,
                last_id: None,
                from: None,
                to: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["limit"], limit);
            assert!((1..=1000).contains(&limit));
        }
    }

    #[test]
    fn test_offset_variations() {
        let offsets = vec![0, 10, 50, 100, 500, 1000];

        for offset in offsets {
            let request = FuturesTradesRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                limit: None,
                offset: Some(offset),
                last_id: None,
                from: None,
                to: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["offset"], offset);
            assert!(offset >= 0);
        }
    }

    #[test]
    fn test_pagination_scenarios() {
        let pagination_configs = vec![
            (50, 0, "First page"),
            (50, 50, "Second page"),
            (100, 200, "Third page"),
            (25, 75, "Custom pagination"),
        ];

        for (limit, offset, _description) in pagination_configs {
            let request = FuturesTradesRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                limit: Some(limit),
                offset: Some(offset),
                last_id: None,
                from: None,
                to: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["limit"], limit);
            assert_eq!(json["offset"], offset);
        }
    }

    #[test]
    fn test_last_id_scenarios() {
        let last_ids = vec!["123456789", "987654321", "1", "9999999999"];

        for last_id in last_ids {
            let request = FuturesTradesRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                limit: None,
                offset: None,
                last_id: Some(last_id.to_string()),
                from: None,
                to: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["last_id"], last_id);
        }
    }

    #[test]
    fn test_time_range_scenarios() {
        let time_ranges = vec![
            (1640995200, 1640998800, "1 hour"),
            (1640995200, 1641081600, "24 hours"),
            (1640995200, 1641686400, "1 week"),
            (1640995200, 1643673600, "1 month"),
        ];

        for (from, to, _description) in time_ranges {
            let request = FuturesTradesRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                limit: None,
                offset: None,
                last_id: None,
                from: Some(from),
                to: Some(to),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["from"], from);
            assert_eq!(json["to"], to);
            assert!(to > from);
        }
    }

    #[test]
    fn test_futures_trade_deserialization() {
        let json = r#"{
            "id": 123456789,
            "create_time": 1640995200.123,
            "contract": "BTC_USDT",
            "size": 1500,
            "price": "43250.8",
            "is_internal": false
        }"#;

        let trade: FuturesTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, 123456789);
        assert_eq!(trade.create_time, 1640995200.123);
        assert_eq!(trade.contract, "BTC_USDT");
        assert_eq!(trade.size, 1500);
        assert_eq!(trade.price, "43250.8");
        assert!(!trade.is_internal.unwrap());
    }

    #[test]
    fn test_futures_trade_without_is_internal() {
        let json = r#"{
            "id": 987654321,
            "create_time": 1640995300.456,
            "contract": "ETH_USDT",
            "size": 2500,
            "price": "2650.45"
        }"#;

        let trade: FuturesTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, 987654321);
        assert_eq!(trade.create_time, 1640995300.456);
        assert_eq!(trade.contract, "ETH_USDT");
        assert_eq!(trade.size, 2500);
        assert_eq!(trade.price, "2650.45");
        assert!(trade.is_internal.is_none());
    }

    #[test]
    fn test_realistic_btc_trade_scenarios() {
        let btc_trades = vec![
            (123456789, "43251.5", 1250, "Large BTC trade"),
            (123456790, "43252.0", 500, "Medium BTC trade"),
            (123456791, "43251.8", 2500, "Large BTC trade"),
            (123456792, "43250.9", 100, "Small BTC trade"),
        ];

        for (id, price, size, _description) in btc_trades {
            let json = format!(
                r#"{{
                "id": {},
                "create_time": 1640995200.123,
                "contract": "BTC_USDT",
                "size": {},
                "price": "{}",
                "is_internal": false
            }}"#,
                id, size, price
            );

            let trade: FuturesTrade = serde_json::from_str(&json).unwrap();
            assert_eq!(trade.id, id);
            assert_eq!(trade.size, size);
            assert_eq!(trade.price, price);

            // Verify BTC price is reasonable
            let trade_price: f64 = trade.price.parse().unwrap();
            assert!(trade_price > 40000.0 && trade_price < 50000.0);
        }
    }

    #[test]
    fn test_realistic_eth_trade_scenarios() {
        let eth_trades = vec![
            (987654321, "2651.25", 850, "ETH buy"),
            (987654322, "2650.75", 1200, "ETH sell"),
            (987654323, "2651.00", 600, "ETH trade"),
            (987654324, "2650.50", 1500, "Large ETH trade"),
        ];

        for (id, price, size, _description) in eth_trades {
            let json = format!(
                r#"{{
                "id": {},
                "create_time": 1640995250.789,
                "contract": "ETH_USDT",
                "size": {},
                "price": "{}",
                "is_internal": false
            }}"#,
                id, size, price
            );

            let trade: FuturesTrade = serde_json::from_str(&json).unwrap();
            assert_eq!(trade.id, id);
            assert_eq!(trade.size, size);
            assert_eq!(trade.price, price);

            // Verify ETH price is reasonable
            let trade_price: f64 = trade.price.parse().unwrap();
            assert!(trade_price > 2000.0 && trade_price < 3000.0);
        }
    }

    #[test]
    fn test_internal_vs_external_trades() {
        let trade_types = vec![(true, "Internal trade"), (false, "External trade")];

        for (is_internal, _description) in trade_types {
            let json = format!(
                r#"{{
                "id": 123456789,
                "create_time": 1640995200.123,
                "contract": "BTC_USDT",
                "size": 1000,
                "price": "43250.0",
                "is_internal": {}
            }}"#,
                is_internal
            );

            let trade: FuturesTrade = serde_json::from_str(&json).unwrap();
            assert_eq!(trade.is_internal.unwrap(), is_internal);
        }
    }

    #[test]
    fn test_trade_size_variations() {
        let sizes = vec![1, 10, 100, 1000, 10000, 100000, 1000000];

        for size in sizes {
            let json = format!(
                r#"{{
                "id": 123456789,
                "create_time": 1640995200.123,
                "contract": "BTC_USDT",
                "size": {},
                "price": "43250.0"
            }}"#,
                size
            );

            let trade: FuturesTrade = serde_json::from_str(&json).unwrap();
            assert_eq!(trade.size, size);
            assert!(trade.size > 0);
        }
    }

    #[test]
    fn test_high_precision_prices() {
        let json = r#"{
            "id": 123456789,
            "create_time": 1640995200.123456789,
            "contract": "BTC_USDT",
            "size": 1500,
            "price": "43251.123456789"
        }"#;

        let trade: FuturesTrade = serde_json::from_str(json).unwrap();

        // Verify precision is maintained
        assert_eq!(trade.price, "43251.123456789");
        assert_eq!(trade.create_time, 1_640_995_200.123_456_7);
    }

    #[test]
    fn test_large_trade_ids() {
        let large_ids = vec![9999999999i64, 1234567890123i64, 9876543210987i64];

        for id in large_ids {
            let json = format!(
                r#"{{
                "id": {},
                "create_time": 1640995200.123,
                "contract": "BTC_USDT",
                "size": 1000,
                "price": "43250.0"
            }}"#,
                id
            );

            let trade: FuturesTrade = serde_json::from_str(&json).unwrap();
            assert_eq!(trade.id, id);
        }
    }

    #[test]
    fn test_timestamp_scenarios() {
        let timestamps = vec![
            (1640995200.0, "Recent timestamp"),
            (1577836800.0, "Year 2020"),
            (1735689600.0, "Future timestamp"),
            (1640995200.123456, "High precision"),
        ];

        for (timestamp, _description) in timestamps {
            let json = format!(
                r#"{{
                "id": 123456789,
                "create_time": {},
                "contract": "BTC_USDT",
                "size": 1000,
                "price": "43250.0"
            }}"#,
                timestamp
            );

            let trade: FuturesTrade = serde_json::from_str(&json).unwrap();
            assert_eq!(trade.create_time, timestamp);
        }
    }

    #[test]
    fn test_altcoin_trade_scenarios() {
        let altcoin_trades = vec![
            ("ADA_USDT", "0.485", 20000),
            ("SOL_USDT", "98.25", 500),
            ("MATIC_USDT", "0.825", 15000),
            ("DOT_USDT", "6.75", 2500),
            ("AVAX_USDT", "35.80", 1000),
            ("LINK_USDT", "14.25", 1500),
        ];

        for (contract, price, size) in altcoin_trades {
            let json = format!(
                r#"{{
                "id": 123456789,
                "create_time": 1640995200.123,
                "contract": "{}",
                "size": {},
                "price": "{}"
            }}"#,
                contract, size, price
            );

            let trade: FuturesTrade = serde_json::from_str(&json).unwrap();
            assert_eq!(trade.contract, contract);
            assert_eq!(trade.size, size);
            assert_eq!(trade.price, price);

            // Verify price is positive
            let trade_price: f64 = trade.price.parse().unwrap();
            assert!(trade_price > 0.0);
        }
    }

    #[test]
    fn test_trading_sequence_analysis() {
        // Simulate a sequence of trades showing market activity
        let trade_sequence = vec![
            (123456780, 1640995200.0, "43250.0", 1000),
            (123456781, 1640995201.5, "43251.5", 1500),
            (123456782, 1640995203.2, "43252.0", 800),
            (123456783, 1640995205.8, "43251.0", 2000),
            (123456784, 1640995208.1, "43253.5", 1200),
        ];

        let mut prev_time = 0.0;
        for (id, timestamp, price, size) in trade_sequence {
            let json = format!(
                r#"{{
                "id": {},
                "create_time": {},
                "contract": "BTC_USDT",
                "size": {},
                "price": "{}"
            }}"#,
                id, timestamp, size, price
            );

            let trade: FuturesTrade = serde_json::from_str(&json).unwrap();
            assert_eq!(trade.id, id);
            assert_eq!(trade.create_time, timestamp);

            // Verify timestamps are in ascending order
            assert!(trade.create_time > prev_time);
            prev_time = trade.create_time;

            // Verify trade data
            assert!(trade.size > 0);
            let trade_price: f64 = trade.price.parse().unwrap();
            assert!(trade_price > 0.0);
        }
    }

    #[test]
    fn test_volatile_price_movements() {
        let volatile_trades = vec![
            ("43000.0", "Start of volatility"),
            ("43500.0", "Price spike up"),
            ("42800.0", "Sharp drop"),
            ("43200.0", "Recovery"),
            ("43800.0", "New high"),
            ("43100.0", "Settling down"),
        ];

        for (price, _description) in volatile_trades {
            let json = format!(
                r#"{{
                "id": 123456789,
                "create_time": 1640995200.123,
                "contract": "BTC_USDT",
                "size": 1000,
                "price": "{}"
            }}"#,
                price
            );

            let trade: FuturesTrade = serde_json::from_str(&json).unwrap();
            assert_eq!(trade.price, price);

            let trade_price: f64 = trade.price.parse().unwrap();
            assert!(trade_price > 42000.0 && trade_price < 44000.0);
        }
    }

    #[test]
    fn test_different_trade_volumes() {
        let volume_scenarios = vec![
            (1, "Minimum trade"),
            (100, "Small retail trade"),
            (1000, "Medium trade"),
            (10000, "Large trade"),
            (100000, "Institutional trade"),
            (1000000, "Whale trade"),
        ];

        for (size, _description) in volume_scenarios {
            let json = format!(
                r#"{{
                "id": 123456789,
                "create_time": 1640995200.123,
                "contract": "BTC_USDT",
                "size": {},
                "price": "43250.0"
            }}"#,
                size
            );

            let trade: FuturesTrade = serde_json::from_str(&json).unwrap();
            assert_eq!(trade.size, size);
            assert!(trade.size > 0);
        }
    }

    #[test]
    fn test_market_impact_analysis() {
        // Large trades that might impact market
        let market_impact_trades = vec![
            (500000, "43250.0", "Large buy order"),
            (750000, "43249.5", "Massive sell pressure"),
            (1000000, "43251.0", "Whale transaction"),
        ];

        for (size, price, _description) in market_impact_trades {
            let json = format!(
                r#"{{
                "id": 123456789,
                "create_time": 1640995200.123,
                "contract": "BTC_USDT",
                "size": {},
                "price": "{}",
                "is_internal": false
            }}"#,
                size, price
            );

            let trade: FuturesTrade = serde_json::from_str(&json).unwrap();
            assert_eq!(trade.size, size);
            assert_eq!(trade.price, price);

            // Large trades should be marked as external
            assert!(!trade.is_internal.unwrap());
        }
    }

    #[test]
    fn test_cross_exchange_arbitrage_detection() {
        // Trades that might indicate arbitrage opportunities
        let arbitrage_trades = vec![
            (123456789, "43250.0", false, "Regular trade"),
            (123456790, "43250.0", true, "Internal arbitrage"),
            (123456791, "43251.5", false, "Price difference"),
            (123456792, "43251.5", true, "Internal correction"),
        ];

        for (id, price, is_internal, _description) in arbitrage_trades {
            let json = format!(
                r#"{{
                "id": {},
                "create_time": 1640995200.123,
                "contract": "BTC_USDT",
                "size": 1000,
                "price": "{}",
                "is_internal": {}
            }}"#,
                id, price, is_internal
            );

            let trade: FuturesTrade = serde_json::from_str(&json).unwrap();
            assert_eq!(trade.id, id);
            assert_eq!(trade.price, price);
            assert_eq!(trade.is_internal.unwrap(), is_internal);
        }
    }

    #[test]
    fn test_clone_behavior() {
        let request = FuturesTradesRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            limit: Some(100),
            offset: Some(50),
            last_id: Some("123456789".to_string()),
            from: Some(1640995200),
            to: Some(1640998800),
        };

        let cloned = request.clone();
        assert_eq!(cloned.settle, request.settle);
        assert_eq!(cloned.contract, request.contract);
        assert_eq!(cloned.limit, request.limit);
        assert_eq!(cloned.offset, request.offset);
        assert_eq!(cloned.last_id, request.last_id);
        assert_eq!(cloned.from, request.from);
        assert_eq!(cloned.to, request.to);
    }

    #[test]
    fn test_debug_output() {
        let trade = FuturesTrade {
            id: 123456789,
            create_time: 1640995200.123,
            contract: "BTC_USDT".to_string(),
            size: 1500,
            price: "43250.8".to_string(),
            is_internal: Some(false),
        };

        let debug_str = format!("{:?}", trade);
        assert!(debug_str.contains("FuturesTrade"));
        assert!(debug_str.contains("123456789"));
        assert!(debug_str.contains("BTC_USDT"));
        assert!(debug_str.contains("43250.8"));
    }

    #[test]
    fn test_serialization_round_trip() {
        let trade = FuturesTrade {
            id: 123456789,
            create_time: 1640995200.123,
            contract: "BTC_USDT".to_string(),
            size: 1500,
            price: "43250.8".to_string(),
            is_internal: Some(false),
        };

        let json = serde_json::to_string(&trade).unwrap();
        let deserialized: FuturesTrade = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, trade.id);
        assert_eq!(deserialized.create_time, trade.create_time);
        assert_eq!(deserialized.contract, trade.contract);
        assert_eq!(deserialized.size, trade.size);
        assert_eq!(deserialized.price, trade.price);
        assert_eq!(deserialized.is_internal, trade.is_internal);
    }
}
