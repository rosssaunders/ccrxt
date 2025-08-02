use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for futures order book
#[derive(Debug, Clone, Serialize, Default)]
pub struct FuturesOrderBookRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract name
    pub contract: String,
    /// Order book level (1-100, default 10)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// Order book depth limit (1-100, default 10)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Request UTC timestamp in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_id: Option<bool>,
}

/// Order book entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookEntry {
    /// Price
    pub p: String,
    /// Size
    pub s: i64,
}

/// Futures order book
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesOrderBook {
    /// Order book ID
    pub id: Option<i64>,

    /// Current timestamp
    pub current: f64,

    /// Last update timestamp  
    pub update: f64,

    /// Asks (selling orders)
    pub asks: Vec<OrderBookEntry>,

    /// Bids (buying orders)
    pub bids: Vec<OrderBookEntry>,
}

impl RestClient {
    /// Get futures order book
    ///
    /// Retrieves the order book for a specific futures contract.
    /// Bids are sorted by price high to low, asks are sorted by price low to high.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#futures-order-book>
    pub async fn get_futures_order_book(
        &self,
        params: FuturesOrderBookRequest,
    ) -> crate::gateio::perpetual::Result<FuturesOrderBook> {
        let endpoint = format!("/futures/{}/order_book", params.settle);
        self.get_with_query(&endpoint, Some(&params)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_futures_order_book_request_minimal() {
        let request = FuturesOrderBookRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            interval: None,
            limit: None,
            with_id: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "BTC_USDT");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 2); // Only settle and contract
        assert!(!obj.contains_key("interval"));
        assert!(!obj.contains_key("limit"));
        assert!(!obj.contains_key("with_id"));
    }

    #[test]
    fn test_futures_order_book_request_full() {
        let request = FuturesOrderBookRequest {
            settle: "USDT".to_string(),
            contract: "ETH_USDT".to_string(),
            interval: Some("0.01".to_string()),
            limit: Some(50),
            with_id: Some(true),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "ETH_USDT");
        assert_eq!(json["interval"], "0.01");
        assert_eq!(json["limit"], 50);
        assert_eq!(json["with_id"], true);

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 5);
    }

    #[test]
    fn test_different_settlement_currencies() {
        let currencies = vec!["USDT", "BTC", "USD"];

        for settle in currencies {
            let request = FuturesOrderBookRequest {
                settle: settle.to_string(),
                contract: "BTC_USDT".to_string(),
                interval: None,
                limit: None,
                with_id: None,
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
            let request = FuturesOrderBookRequest {
                settle: "USDT".to_string(),
                contract: contract.to_string(),
                interval: None,
                limit: None,
                with_id: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["contract"], contract);
        }
    }

    #[test]
    fn test_different_intervals() {
        let intervals = vec!["0", "0.01", "0.1", "1", "10"];

        for interval in intervals {
            let request = FuturesOrderBookRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                interval: Some(interval.to_string()),
                limit: None,
                with_id: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["interval"], interval);
        }
    }

    #[test]
    fn test_limit_variations() {
        let limits = vec![1, 5, 10, 20, 50, 100];

        for limit in limits {
            let request = FuturesOrderBookRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                interval: None,
                limit: Some(limit),
                with_id: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["limit"], limit);
            assert!(limit >= 1 && limit <= 100);
        }
    }

    #[test]
    fn test_with_id_options() {
        let options = vec![true, false];

        for with_id in options {
            let request = FuturesOrderBookRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                interval: None,
                limit: None,
                with_id: Some(with_id),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["with_id"], with_id);
        }
    }

    #[test]
    fn test_order_book_entry_deserialization() {
        let json = r#"{
            "p": "43250.8",
            "s": 1500
        }"#;

        let entry: OrderBookEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.p, "43250.8");
        assert_eq!(entry.s, 1500);
    }

    #[test]
    fn test_futures_order_book_deserialization() {
        let json = r#"{
            "id": 1234567890,
            "current": 1640995200.123,
            "update": 1640995200.456,
            "asks": [
                {"p": "43251.0", "s": 1500},
                {"p": "43252.0", "s": 2000},
                {"p": "43253.0", "s": 1000}
            ],
            "bids": [
                {"p": "43250.0", "s": 2500},
                {"p": "43249.0", "s": 3000},
                {"p": "43248.0", "s": 1500}
            ]
        }"#;

        let order_book: FuturesOrderBook = serde_json::from_str(json).unwrap();
        assert_eq!(order_book.id.unwrap(), 1234567890);
        assert_eq!(order_book.current, 1640995200.123);
        assert_eq!(order_book.update, 1640995200.456);
        assert_eq!(order_book.asks.len(), 3);
        assert_eq!(order_book.bids.len(), 3);

        // Verify first ask
        assert_eq!(order_book.asks[0].p, "43251.0");
        assert_eq!(order_book.asks[0].s, 1500);

        // Verify first bid
        assert_eq!(order_book.bids[0].p, "43250.0");
        assert_eq!(order_book.bids[0].s, 2500);
    }

    #[test]
    fn test_futures_order_book_without_id() {
        let json = r#"{
            "current": 1640995200.123,
            "update": 1640995200.456,
            "asks": [
                {"p": "2651.0", "s": 500},
                {"p": "2652.0", "s": 750}
            ],
            "bids": [
                {"p": "2650.0", "s": 800},
                {"p": "2649.0", "s": 600}
            ]
        }"#;

        let order_book: FuturesOrderBook = serde_json::from_str(json).unwrap();
        assert!(order_book.id.is_none());
        assert_eq!(order_book.current, 1640995200.123);
        assert_eq!(order_book.update, 1640995200.456);
        assert_eq!(order_book.asks.len(), 2);
        assert_eq!(order_book.bids.len(), 2);
    }

    #[test]
    fn test_order_book_price_sorting_validation() {
        let json = r#"{
            "current": 1640995200.123,
            "update": 1640995200.456,
            "asks": [
                {"p": "43251.0", "s": 1500},
                {"p": "43252.0", "s": 2000},
                {"p": "43253.0", "s": 1000},
                {"p": "43254.0", "s": 800}
            ],
            "bids": [
                {"p": "43250.0", "s": 2500},
                {"p": "43249.0", "s": 3000},
                {"p": "43248.0", "s": 1500},
                {"p": "43247.0", "s": 2000}
            ]
        }"#;

        let order_book: FuturesOrderBook = serde_json::from_str(json).unwrap();

        // Verify asks are sorted low to high (ascending)
        for i in 1..order_book.asks.len() {
            let prev_price: f64 = order_book.asks[i - 1].p.parse().unwrap();
            let curr_price: f64 = order_book.asks[i].p.parse().unwrap();
            assert!(curr_price >= prev_price, "Asks should be sorted ascending");
        }

        // Verify bids are sorted high to low (descending)
        for i in 1..order_book.bids.len() {
            let prev_price: f64 = order_book.bids[i - 1].p.parse().unwrap();
            let curr_price: f64 = order_book.bids[i].p.parse().unwrap();
            assert!(curr_price <= prev_price, "Bids should be sorted descending");
        }
    }

    #[test]
    fn test_bid_ask_spread_validation() {
        let json = r#"{
            "current": 1640995200.123,
            "update": 1640995200.456,
            "asks": [
                {"p": "43251.0", "s": 1500}
            ],
            "bids": [
                {"p": "43250.0", "s": 2500}
            ]
        }"#;

        let order_book: FuturesOrderBook = serde_json::from_str(json).unwrap();

        let best_ask: f64 = order_book.asks[0].p.parse().unwrap();
        let best_bid: f64 = order_book.bids[0].p.parse().unwrap();

        // Best ask should be higher than best bid
        assert!(best_ask > best_bid);

        let spread = best_ask - best_bid;
        assert!(spread > 0.0);
        assert!(spread < best_bid * 0.01); // Spread should be reasonable (< 1% of price)
    }

    #[test]
    fn test_realistic_btc_order_book_scenario() {
        let json = r#"{
            "id": 1234567890,
            "current": 1640995200.123,
            "update": 1640995200.456,
            "asks": [
                {"p": "43251.5", "s": 1250},
                {"p": "43252.0", "s": 2100},
                {"p": "43252.5", "s": 875},
                {"p": "43253.0", "s": 1650},
                {"p": "43254.0", "s": 920}
            ],
            "bids": [
                {"p": "43251.0", "s": 1850},
                {"p": "43250.5", "s": 2750},
                {"p": "43250.0", "s": 1450},
                {"p": "43249.5", "s": 3200},
                {"p": "43249.0", "s": 1100}
            ]
        }"#;

        let order_book: FuturesOrderBook = serde_json::from_str(json).unwrap();

        // Verify BTC price range is reasonable
        let best_ask: f64 = order_book.asks[0].p.parse().unwrap();
        let best_bid: f64 = order_book.bids[0].p.parse().unwrap();

        assert!(best_bid > 40000.0 && best_bid < 50000.0);
        assert!(best_ask > 40000.0 && best_ask < 50000.0);

        // Verify tight spread for BTC
        let spread = best_ask - best_bid;
        assert!(spread > 0.0 && spread < 10.0); // Reasonable BTC spread

        // Verify order sizes are realistic
        for ask in &order_book.asks {
            assert!(ask.s > 0);
            assert!(ask.s < 10000); // Reasonable order size
        }

        for bid in &order_book.bids {
            assert!(bid.s > 0);
            assert!(bid.s < 10000); // Reasonable order size
        }
    }

    #[test]
    fn test_realistic_eth_order_book_scenario() {
        let json = r#"{
            "current": 1640995200.123,
            "update": 1640995200.456,
            "asks": [
                {"p": "2651.25", "s": 850},
                {"p": "2651.50", "s": 1200},
                {"p": "2651.75", "s": 950},
                {"p": "2652.00", "s": 1500}
            ],
            "bids": [
                {"p": "2651.00", "s": 1100},
                {"p": "2650.75", "s": 1650},
                {"p": "2650.50", "s": 800},
                {"p": "2650.25", "s": 2000}
            ]
        }"#;

        let order_book: FuturesOrderBook = serde_json::from_str(json).unwrap();

        // Verify ETH price range is reasonable
        let best_ask: f64 = order_book.asks[0].p.parse().unwrap();
        let best_bid: f64 = order_book.bids[0].p.parse().unwrap();

        assert!(best_bid > 2000.0 && best_bid < 3000.0);
        assert!(best_ask > 2000.0 && best_ask < 3000.0);

        // Verify tight spread for ETH
        let spread = best_ask - best_bid;
        assert!(spread > 0.0 && spread < 5.0); // Reasonable ETH spread
    }

    #[test]
    fn test_deep_order_book() {
        let mut asks = Vec::new();
        let mut bids = Vec::new();

        // Generate asks from 43251.0 to 43300.0
        for i in 0..50 {
            asks.push(OrderBookEntry {
                p: format!("{:.1}", 43251.0 + i as f64 * 1.0),
                s: 1000 + i * 10,
            });
        }

        // Generate bids from 43250.0 to 43201.0
        for i in 0..50 {
            bids.push(OrderBookEntry {
                p: format!("{:.1}", 43250.0 - i as f64 * 1.0),
                s: 1500 + i * 15,
            });
        }

        let order_book = FuturesOrderBook {
            id: Some(9876543210),
            current: 1640995200.123,
            update: 1640995200.456,
            asks,
            bids,
        };

        // Verify deep order book structure
        assert_eq!(order_book.asks.len(), 50);
        assert_eq!(order_book.bids.len(), 50);

        // Verify price progression
        let first_ask: f64 = order_book.asks[0].p.parse().unwrap();
        let last_ask: f64 = order_book.asks[49].p.parse().unwrap();
        assert!(last_ask > first_ask);

        let first_bid: f64 = order_book.bids[0].p.parse().unwrap();
        let last_bid: f64 = order_book.bids[49].p.parse().unwrap();
        assert!(last_bid < first_bid);
    }

    #[test]
    fn test_empty_order_book() {
        let json = r#"{
            "current": 1640995200.123,
            "update": 1640995200.456,
            "asks": [],
            "bids": []
        }"#;

        let order_book: FuturesOrderBook = serde_json::from_str(json).unwrap();
        assert_eq!(order_book.asks.len(), 0);
        assert_eq!(order_book.bids.len(), 0);
        assert!(order_book.id.is_none());
    }

    #[test]
    fn test_asymmetric_order_book() {
        let json = r#"{
            "current": 1640995200.123,
            "update": 1640995200.456,
            "asks": [
                {"p": "43251.0", "s": 1500},
                {"p": "43252.0", "s": 2000},
                {"p": "43253.0", "s": 1000},
                {"p": "43254.0", "s": 800},
                {"p": "43255.0", "s": 1200}
            ],
            "bids": [
                {"p": "43250.0", "s": 2500},
                {"p": "43249.0", "s": 3000}
            ]
        }"#;

        let order_book: FuturesOrderBook = serde_json::from_str(json).unwrap();

        // More asks than bids (bearish sentiment)
        assert!(order_book.asks.len() > order_book.bids.len());
        assert_eq!(order_book.asks.len(), 5);
        assert_eq!(order_book.bids.len(), 2);
    }

    #[test]
    fn test_high_precision_prices() {
        let json = r#"{
            "current": 1640995200.123456789,
            "update": 1640995200.987654321,
            "asks": [
                {"p": "43251.123456789", "s": 1500},
                {"p": "43251.987654321", "s": 2000}
            ],
            "bids": [
                {"p": "43250.555555555", "s": 2500},
                {"p": "43250.111111111", "s": 3000}
            ]
        }"#;

        let order_book: FuturesOrderBook = serde_json::from_str(json).unwrap();

        // Verify precision is maintained
        assert_eq!(order_book.asks[0].p, "43251.123456789");
        assert_eq!(order_book.asks[1].p, "43251.987654321");
        assert_eq!(order_book.bids[0].p, "43250.555555555");
        assert_eq!(order_book.bids[1].p, "43250.111111111");

        // For timestamps (f64), use epsilon comparison due to floating-point precision limits
        // With large numbers like Unix timestamps, we need a larger epsilon
        let epsilon = 1e-6;
        assert!((order_book.current - 1640995200.123456789).abs() < epsilon);
        assert!((order_book.update - 1640995200.987654321).abs() < epsilon);
    }

    #[test]
    fn test_large_order_sizes() {
        let json = r#"{
            "current": 1640995200.123,
            "update": 1640995200.456,
            "asks": [
                {"p": "43251.0", "s": 999999999},
                {"p": "43252.0", "s": 500000000}
            ],
            "bids": [
                {"p": "43250.0", "s": 750000000},
                {"p": "43249.0", "s": 1000000000}
            ]
        }"#;

        let order_book: FuturesOrderBook = serde_json::from_str(json).unwrap();

        // Verify large sizes are handled correctly
        assert_eq!(order_book.asks[0].s, 999999999);
        assert_eq!(order_book.bids[1].s, 1000000000);

        // All sizes should be positive
        for ask in &order_book.asks {
            assert!(ask.s > 0);
        }

        for bid in &order_book.bids {
            assert!(bid.s > 0);
        }
    }

    #[test]
    fn test_timestamp_scenarios() {
        let timestamps = vec![
            (1640995200.0, 1640995200.123, "Recent timestamps"),
            (1577836800.0, 1577836800.456, "Year 2020"),
            (1735689600.0, 1735689600.789, "Future timestamps"),
        ];

        for (current, update, _description) in timestamps {
            let json = format!(
                r#"{{
                "current": {},
                "update": {},
                "asks": [
                    {{"p": "30000.0", "s": 1000}}
                ],
                "bids": [
                    {{"p": "29999.0", "s": 1500}}
                ]
            }}"#,
                current, update
            );

            let order_book: FuturesOrderBook = serde_json::from_str(&json).unwrap();
            assert_eq!(order_book.current, current);
            assert_eq!(order_book.update, update);
        }
    }

    #[test]
    fn test_market_depth_analysis() {
        let json = r#"{
            "current": 1640995200.123,
            "update": 1640995200.456,
            "asks": [
                {"p": "43251.0", "s": 1000},
                {"p": "43252.0", "s": 1500},
                {"p": "43253.0", "s": 2000},
                {"p": "43254.0", "s": 2500},
                {"p": "43255.0", "s": 3000}
            ],
            "bids": [
                {"p": "43250.0", "s": 1200},
                {"p": "43249.0", "s": 1800},
                {"p": "43248.0", "s": 2200},
                {"p": "43247.0", "s": 2800},
                {"p": "43246.0", "s": 3200}
            ]
        }"#;

        let order_book: FuturesOrderBook = serde_json::from_str(json).unwrap();

        // Calculate total ask and bid volumes
        let total_ask_volume: i64 = order_book.asks.iter().map(|a| a.s).sum();
        let total_bid_volume: i64 = order_book.bids.iter().map(|b| b.s).sum();

        assert_eq!(total_ask_volume, 10000); // 1000+1500+2000+2500+3000
        assert_eq!(total_bid_volume, 11200); // 1200+1800+2200+2800+3200

        // Bid volume slightly higher indicates buying pressure
        assert!(total_bid_volume > total_ask_volume);
    }

    #[test]
    fn test_different_order_book_depths() {
        let depths = vec![
            (1, "Minimal depth"),
            (5, "Standard depth"),
            (10, "Medium depth"),
            (20, "Deep depth"),
            (50, "Very deep depth"),
        ];

        for (depth, _description) in depths {
            let request = FuturesOrderBookRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT".to_string(),
                interval: None,
                limit: Some(depth),
                with_id: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["limit"], depth);
            assert!(depth >= 1 && depth <= 100);
        }
    }

    #[test]
    fn test_volatile_market_order_book() {
        let json = r#"{
            "current": 1640995200.123,
            "update": 1640995200.456,
            "asks": [
                {"p": "43500.0", "s": 500},
                {"p": "43600.0", "s": 400},
                {"p": "43700.0", "s": 300}
            ],
            "bids": [
                {"p": "42500.0", "s": 600},
                {"p": "42400.0", "s": 500},
                {"p": "42300.0", "s": 400}
            ]
        }"#;

        let order_book: FuturesOrderBook = serde_json::from_str(json).unwrap();

        let best_ask: f64 = order_book.asks[0].p.parse().unwrap();
        let best_bid: f64 = order_book.bids[0].p.parse().unwrap();

        // Wide spread indicates volatility
        let spread = best_ask - best_bid;
        let mid_price = (best_ask + best_bid) / 2.0;
        let spread_percentage = (spread / mid_price) * 100.0;

        assert!(spread > 500.0); // Large absolute spread
        assert!(spread_percentage > 1.0); // Large percentage spread
    }

    #[test]
    fn test_clone_behavior() {
        let request = FuturesOrderBookRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            interval: Some("0.1".to_string()),
            limit: Some(20),
            with_id: Some(true),
        };

        let cloned = request.clone();
        assert_eq!(cloned.settle, request.settle);
        assert_eq!(cloned.contract, request.contract);
        assert_eq!(cloned.interval, request.interval);
        assert_eq!(cloned.limit, request.limit);
        assert_eq!(cloned.with_id, request.with_id);
    }

    #[test]
    fn test_debug_output() {
        let entry = OrderBookEntry {
            p: "43250.8".to_string(),
            s: 1500,
        };

        let debug_str = format!("{:?}", entry);
        assert!(debug_str.contains("OrderBookEntry"));
        assert!(debug_str.contains("43250.8"));
        assert!(debug_str.contains("1500"));
    }

    #[test]
    fn test_serialization_round_trip() {
        let order_book = FuturesOrderBook {
            id: Some(1234567890),
            current: 1640995200.123,
            update: 1640995200.456,
            asks: vec![
                OrderBookEntry {
                    p: "43251.0".to_string(),
                    s: 1500,
                },
                OrderBookEntry {
                    p: "43252.0".to_string(),
                    s: 2000,
                },
            ],
            bids: vec![
                OrderBookEntry {
                    p: "43250.0".to_string(),
                    s: 2500,
                },
                OrderBookEntry {
                    p: "43249.0".to_string(),
                    s: 3000,
                },
            ],
        };

        let json = serde_json::to_string(&order_book).unwrap();
        let deserialized: FuturesOrderBook = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, order_book.id);
        assert_eq!(deserialized.current, order_book.current);
        assert_eq!(deserialized.update, order_book.update);
        assert_eq!(deserialized.asks.len(), order_book.asks.len());
        assert_eq!(deserialized.bids.len(), order_book.bids.len());
        assert_eq!(deserialized.asks[0].p, order_book.asks[0].p);
        assert_eq!(deserialized.bids[0].s, order_book.bids[0].s);
    }
}
