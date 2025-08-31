use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::gateio::RestResult;

const DELIVERY_ORDER_BOOK_ENDPOINT: &str = "/delivery/{}/order_book";

/// Request parameters for delivery order book
#[derive(Debug, Clone, Serialize, Default)]
pub struct DeliveryOrderBookRequest {
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
pub struct DeliveryOrderBookEntry {
    /// Price
    pub p: String,

    /// Size
    pub s: i64,
}

/// Delivery order book
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryOrderBook {
    /// Order book ID
    pub id: Option<i64>,

    /// Current timestamp
    pub current: f64,

    /// Last update timestamp  
    pub update: f64,

    /// Asks (selling orders)
    pub asks: Vec<DeliveryOrderBookEntry>,

    /// Bids (buying orders)
    pub bids: Vec<DeliveryOrderBookEntry>,
}

impl RestClient {
    /// Get delivery order book
    ///
    /// Retrieves the order book for a specific delivery contract.
    /// Bids are sorted by price high to low, asks are sorted by price low to high.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#get-delivery-order-book)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `params` - The delivery order book request parameters
    ///
    /// # Returns
    /// Delivery contract order book
    pub async fn get_delivery_order_book(
        &self,
        params: DeliveryOrderBookRequest,
    ) -> RestResult<DeliveryOrderBook> {
        let endpoint = DELIVERY_ORDER_BOOK_ENDPOINT.replace("{}", &params.settle);
        self.get_with_query(&endpoint, Some(&params)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delivery_order_book_request_minimal() {
        let request = DeliveryOrderBookRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT_20241227".to_string(),
            interval: None,
            limit: None,
            with_id: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "BTC_USDT_20241227");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 2); // Only settle and contract
        assert!(!obj.contains_key("interval"));
        assert!(!obj.contains_key("limit"));
        assert!(!obj.contains_key("with_id"));
    }

    #[test]
    fn test_delivery_order_book_request_full() {
        let request = DeliveryOrderBookRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT_20241227".to_string(),
            interval: Some("0".to_string()),
            limit: Some(50),
            with_id: Some(true),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["contract"], "BTC_USDT_20241227");
        assert_eq!(json["interval"], "0");
        assert_eq!(json["limit"], 50);
        assert!(json["with_id"].as_bool().unwrap_or(false));
    }

    #[test]
    fn test_interval_values() {
        let intervals = vec!["0", "0.1", "0.01"];

        for interval in intervals {
            let request = DeliveryOrderBookRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT_20241227".to_string(),
                interval: Some(interval.to_string()),
                limit: None,
                with_id: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["interval"], interval);
        }
    }

    #[test]
    fn test_limit_values() {
        let limits = vec![1, 5, 10, 20, 50, 100];

        for limit in limits {
            let request = DeliveryOrderBookRequest {
                settle: "USDT".to_string(),
                contract: "BTC_USDT_20241227".to_string(),
                interval: None,
                limit: Some(limit),
                with_id: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["limit"], limit);
            assert!((1..=100).contains(&limit));
        }
    }

    #[test]
    fn test_delivery_order_book_entry_deserialization() {
        let json = r#"{
            "p": "43250.5",
            "s": 1000
        }"#;

        let entry: DeliveryOrderBookEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.p, "43250.5");
        assert_eq!(entry.s, 1000);
    }

    #[test]
    fn test_delivery_order_book_deserialization() {
        let json = r#"{
            "id": 123456789,
            "current": 1641024000.123,
            "update": 1641024000.456,
            "asks": [
                {"p": "43300.0", "s": 500},
                {"p": "43300.5", "s": 1000},
                {"p": "43301.0", "s": 1500}
            ],
            "bids": [
                {"p": "43299.5", "s": 600},
                {"p": "43299.0", "s": 1200},
                {"p": "43298.5", "s": 800}
            ]
        }"#;

        let order_book: DeliveryOrderBook = serde_json::from_str(json).unwrap();
        assert_eq!(order_book.id, Some(123456789));
        assert_eq!(order_book.current, 1641024000.123);
        assert_eq!(order_book.update, 1641024000.456);
        assert_eq!(order_book.asks.len(), 3);
        assert_eq!(order_book.bids.len(), 3);

        // Verify asks are sorted low to high
        assert_eq!(order_book.asks[0].p, "43300.0");
        assert_eq!(order_book.asks[1].p, "43300.5");
        assert_eq!(order_book.asks[2].p, "43301.0");

        // Verify bids are sorted high to low
        assert_eq!(order_book.bids[0].p, "43299.5");
        assert_eq!(order_book.bids[1].p, "43299.0");
        assert_eq!(order_book.bids[2].p, "43298.5");
    }

    #[test]
    fn test_order_book_without_id() {
        let json = r#"{
            "current": 1641024000.123,
            "update": 1641024000.456,
            "asks": [{"p": "43300.0", "s": 500}],
            "bids": [{"p": "43299.5", "s": 600}]
        }"#;

        let order_book: DeliveryOrderBook = serde_json::from_str(json).unwrap();
        assert!(order_book.id.is_none());
    }

    #[test]
    fn test_balanced_order_book() {
        let json = r#"{
            "id": 123456789,
            "current": 1641024000.123,
            "update": 1641024000.456,
            "asks": [
                {"p": "43300.0", "s": 1000},
                {"p": "43300.5", "s": 1500},
                {"p": "43301.0", "s": 2000}
            ],
            "bids": [
                {"p": "43299.5", "s": 1100},
                {"p": "43299.0", "s": 1400},
                {"p": "43298.5", "s": 1900}
            ]
        }"#;

        let order_book: DeliveryOrderBook = serde_json::from_str(json).unwrap();

        // Calculate total sizes
        let total_ask_size: i64 = order_book.asks.iter().map(|e| e.s).sum();
        let total_bid_size: i64 = order_book.bids.iter().map(|e| e.s).sum();

        assert_eq!(total_ask_size, 4500);
        assert_eq!(total_bid_size, 4400);

        // Roughly balanced book
        let imbalance = ((total_ask_size - total_bid_size).abs() as f64) / total_ask_size as f64;
        assert!(imbalance < 0.1); // Less than 10% imbalance
    }

    #[test]
    fn test_bid_heavy_order_book() {
        let json = r#"{
            "id": 123456789,
            "current": 1641024000.123,
            "update": 1641024000.456,
            "asks": [
                {"p": "43300.0", "s": 500},
                {"p": "43300.5", "s": 700},
                {"p": "43301.0", "s": 300}
            ],
            "bids": [
                {"p": "43299.5", "s": 2000},
                {"p": "43299.0", "s": 3000},
                {"p": "43298.5", "s": 2500}
            ]
        }"#;

        let order_book: DeliveryOrderBook = serde_json::from_str(json).unwrap();

        let total_ask_size: i64 = order_book.asks.iter().map(|e| e.s).sum();
        let total_bid_size: i64 = order_book.bids.iter().map(|e| e.s).sum();

        assert_eq!(total_ask_size, 1500);
        assert_eq!(total_bid_size, 7500);
        assert!(total_bid_size > total_ask_size * 4); // Heavily bid-sided
    }

    #[test]
    fn test_ask_heavy_order_book() {
        let json = r#"{
            "id": 123456789,
            "current": 1641024000.123,
            "update": 1641024000.456,
            "asks": [
                {"p": "43300.0", "s": 3000},
                {"p": "43300.5", "s": 2500},
                {"p": "43301.0", "s": 2000}
            ],
            "bids": [
                {"p": "43299.5", "s": 300},
                {"p": "43299.0", "s": 500},
                {"p": "43298.5", "s": 700}
            ]
        }"#;

        let order_book: DeliveryOrderBook = serde_json::from_str(json).unwrap();

        let total_ask_size: i64 = order_book.asks.iter().map(|e| e.s).sum();
        let total_bid_size: i64 = order_book.bids.iter().map(|e| e.s).sum();

        assert_eq!(total_ask_size, 7500);
        assert_eq!(total_bid_size, 1500);
        assert!(total_ask_size > total_bid_size * 4); // Heavily ask-sided
    }

    #[test]
    fn test_tight_spread() {
        let json = r#"{
            "id": 123456789,
            "current": 1641024000.123,
            "update": 1641024000.456,
            "asks": [{"p": "43300.0", "s": 1000}],
            "bids": [{"p": "43299.5", "s": 1000}]
        }"#;

        let order_book: DeliveryOrderBook = serde_json::from_str(json).unwrap();

        let best_ask: f64 = order_book.asks[0].p.parse().unwrap();
        let best_bid: f64 = order_book.bids[0].p.parse().unwrap();
        let spread = best_ask - best_bid;
        let spread_bps = (spread / best_bid) * 10000.0;

        assert_eq!(spread, 0.5);
        assert!(spread_bps < 2.0); // Less than 2 basis points
    }

    #[test]
    fn test_wide_spread() {
        let json = r#"{
            "id": 123456789,
            "current": 1641024000.123,
            "update": 1641024000.456,
            "asks": [{"p": "43350.0", "s": 1000}],
            "bids": [{"p": "43250.0", "s": 1000}]
        }"#;

        let order_book: DeliveryOrderBook = serde_json::from_str(json).unwrap();

        let best_ask: f64 = order_book.asks[0].p.parse().unwrap();
        let best_bid: f64 = order_book.bids[0].p.parse().unwrap();
        let spread = best_ask - best_bid;
        let spread_bps = (spread / best_bid) * 10000.0;

        assert_eq!(spread, 100.0);
        assert!(spread_bps > 20.0); // More than 20 basis points
    }

    #[test]
    fn test_deep_order_book() {
        let mut asks = Vec::new();
        let mut bids = Vec::new();

        // Create 50 levels
        for i in 0..50 {
            asks.push(DeliveryOrderBookEntry {
                p: format!("{}", 43300.0 + i as f64 * 0.5),
                s: 1000 + i * 100,
            });
            bids.push(DeliveryOrderBookEntry {
                p: format!("{}", 43299.5 - i as f64 * 0.5),
                s: 1000 + i * 100,
            });
        }

        let order_book = DeliveryOrderBook {
            id: Some(123456789),
            current: 1641024000.123,
            update: 1641024000.456,
            asks,
            bids,
        };

        assert_eq!(order_book.asks.len(), 50);
        assert_eq!(order_book.bids.len(), 50);

        // Verify price ordering
        for i in 1..50 {
            let prev_ask: f64 = order_book.asks[i - 1].p.parse().unwrap();
            let curr_ask: f64 = order_book.asks[i].p.parse().unwrap();
            assert!(curr_ask > prev_ask);

            let prev_bid: f64 = order_book.bids[i - 1].p.parse().unwrap();
            let curr_bid: f64 = order_book.bids[i].p.parse().unwrap();
            assert!(curr_bid < prev_bid);
        }
    }

    #[test]
    fn test_altcoin_order_book() {
        let altcoin_books = vec![
            ("ETH_USDT_20241227", "2650.0", "2649.5"),
            ("ADA_USDT_20241227", "0.485", "0.484"),
            ("SOL_USDT_20241227", "98.50", "98.45"),
            ("MATIC_USDT_20241227", "0.850", "0.849"),
        ];

        for (_contract, ask_price, bid_price) in altcoin_books {
            let json = format!(
                r#"{{
                "id": 123456789,
                "current": 1641024000.123,
                "update": 1641024000.456,
                "asks": [{{"p": "{}", "s": 1000}}],
                "bids": [{{"p": "{}", "s": 1000}}]
            }}"#,
                ask_price, bid_price
            );

            let order_book: DeliveryOrderBook = serde_json::from_str(&json).unwrap();
            assert_eq!(order_book.asks[0].p, ask_price);
            assert_eq!(order_book.bids[0].p, bid_price);

            // Verify reasonable spread
            let ask: f64 = ask_price.parse().unwrap();
            let bid: f64 = bid_price.parse().unwrap();
            assert!(ask > bid);
        }
    }

    #[test]
    fn test_large_order_sizes() {
        let json = r#"{
            "id": 123456789,
            "current": 1641024000.123,
            "update": 1641024000.456,
            "asks": [
                {"p": "43300.0", "s": 50000},
                {"p": "43300.5", "s": 100000},
                {"p": "43301.0", "s": 75000}
            ],
            "bids": [
                {"p": "43299.5", "s": 60000},
                {"p": "43299.0", "s": 80000},
                {"p": "43298.5", "s": 90000}
            ]
        }"#;

        let order_book: DeliveryOrderBook = serde_json::from_str(json).unwrap();

        // Verify large institutional-sized orders
        for ask in &order_book.asks {
            assert!(ask.s >= 50000);
        }
        for bid in &order_book.bids {
            assert!(bid.s >= 60000);
        }
    }

    #[test]
    fn test_wall_detection() {
        let json = r#"{
            "id": 123456789,
            "current": 1641024000.123,
            "update": 1641024000.456,
            "asks": [
                {"p": "43300.0", "s": 500},
                {"p": "43300.5", "s": 600},
                {"p": "43301.0", "s": 10000}
            ],
            "bids": [
                {"p": "43299.5", "s": 400},
                {"p": "43299.0", "s": 500},
                {"p": "43298.5", "s": 12000}
            ]
        }"#;

        let order_book: DeliveryOrderBook = serde_json::from_str(json).unwrap();

        // Simplified wall detection: just pick the max size entries
        let ask_wall = order_book.asks.iter().max_by_key(|e| e.s).unwrap();
        assert_eq!(ask_wall.p, "43301.0");

        let bid_wall = order_book.bids.iter().max_by_key(|e| e.s).unwrap();
        assert_eq!(bid_wall.p, "43298.5");
    }

    #[test]
    fn test_timestamp_consistency() {
        let json = r#"{
            "id": 123456789,
            "current": 1641024000.789,
            "update": 1641024000.456,
            "asks": [{"p": "43300.0", "s": 1000}],
            "bids": [{"p": "43299.5", "s": 1000}]
        }"#;

        let order_book: DeliveryOrderBook = serde_json::from_str(json).unwrap();

        // Current time should be >= update time
        assert!(order_book.current >= order_book.update);

        // Timestamps should be reasonable
        assert!(order_book.current > 1600000000.0); // After Sept 2020
        assert!(order_book.update > 1600000000.0);
    }

    #[test]
    fn test_different_contracts() {
        let contracts = vec![
            "BTC_USDT_20241227",
            "ETH_USDT_20241227",
            "BTC_USDT_20250103",
            "BTC_USDT_20250328",
        ];

        for contract in contracts {
            let request = DeliveryOrderBookRequest {
                settle: "USDT".to_string(),
                contract: contract.to_string(),
                interval: None,
                limit: Some(20),
                with_id: Some(true),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["contract"], contract);
        }
    }

    #[test]
    fn test_empty_order_book() {
        let json = r#"{
            "id": 123456789,
            "current": 1641024000.123,
            "update": 1641024000.456,
            "asks": [],
            "bids": []
        }"#;

        let order_book: DeliveryOrderBook = serde_json::from_str(json).unwrap();
        assert!(order_book.asks.is_empty());
        assert!(order_book.bids.is_empty());
    }

    #[test]
    fn test_one_sided_book() {
        // Only asks
        let json = r#"{
            "id": 123456789,
            "current": 1641024000.123,
            "update": 1641024000.456,
            "asks": [{"p": "43300.0", "s": 1000}],
            "bids": []
        }"#;

        let order_book: DeliveryOrderBook = serde_json::from_str(json).unwrap();
        assert_eq!(order_book.asks.len(), 1);
        assert!(order_book.bids.is_empty());
    }

    #[test]
    fn test_price_precision() {
        let json = r#"{
            "id": 123456789,
            "current": 1641024000.123,
            "update": 1641024000.456,
            "asks": [
                {"p": "43300.0", "s": 1000},
                {"p": "43300.1", "s": 1000},
                {"p": "43300.01", "s": 1000},
                {"p": "43300.001", "s": 1000}
            ],
            "bids": [{"p": "43299.999", "s": 1000}]
        }"#;

        let order_book: DeliveryOrderBook = serde_json::from_str(json).unwrap();

        // Different precision levels
        assert_eq!(order_book.asks[0].p, "43300.0");
        assert_eq!(order_book.asks[1].p, "43300.1");
        assert_eq!(order_book.asks[2].p, "43300.01");
        assert_eq!(order_book.asks[3].p, "43300.001");
        assert_eq!(order_book.bids[0].p, "43299.999");
    }

    #[test]
    fn test_clone_behavior() {
        let request = DeliveryOrderBookRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT_20241227".to_string(),
            interval: Some("0".to_string()),
            limit: Some(50),
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
        let entry = DeliveryOrderBookEntry {
            p: "43300.0".to_string(),
            s: 1000,
        };

        let order_book = DeliveryOrderBook {
            id: Some(123456789),
            current: 1641024000.123,
            update: 1641024000.456,
            asks: vec![entry.clone()],
            bids: vec![],
        };

        let debug_str = format!("{:?}", order_book);
        assert!(debug_str.contains("DeliveryOrderBook"));
        assert!(debug_str.contains("123456789"));
        assert!(debug_str.contains("43300.0"));
    }

    #[test]
    fn test_serialization_round_trip() {
        let order_book = DeliveryOrderBook {
            id: Some(123456789),
            current: 1641024000.123,
            update: 1641024000.456,
            asks: vec![
                DeliveryOrderBookEntry {
                    p: "43300.0".to_string(),
                    s: 1000,
                },
                DeliveryOrderBookEntry {
                    p: "43300.5".to_string(),
                    s: 1500,
                },
            ],
            bids: vec![
                DeliveryOrderBookEntry {
                    p: "43299.5".to_string(),
                    s: 1100,
                },
                DeliveryOrderBookEntry {
                    p: "43299.0".to_string(),
                    s: 1600,
                },
            ],
        };

        let json = serde_json::to_string(&order_book).unwrap();
        let deserialized: DeliveryOrderBook = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, order_book.id);
        assert_eq!(deserialized.current, order_book.current);
        assert_eq!(deserialized.update, order_book.update);
        assert_eq!(deserialized.asks.len(), order_book.asks.len());
        assert_eq!(deserialized.bids.len(), order_book.bids.len());
    }
}
