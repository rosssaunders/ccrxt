use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for retrieving order book
#[derive(Debug, Clone, Serialize, Default)]
pub struct OrderBookRequest {
    /// Currency pair to query order book for
    pub currency_pair: String,

    /// Maximum number of order book levels to return (default: 10, max: 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Whether to include order IDs in the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_id: Option<bool>,
}

/// Order book entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookEntry {
    /// Price
    #[serde(rename = "0")]
    pub price: String,

    /// Amount
    #[serde(rename = "1")]
    pub amount: String,
}

/// Order book response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    /// Current order book ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// Last updated timestamp
    pub current: i64,

    /// Last update timestamp (in milliseconds)
    pub update: i64,

    /// Ask orders (price, amount)
    pub asks: Vec<Vec<String>>,

    /// Bid orders (price, amount)
    pub bids: Vec<Vec<String>>,
}

impl RestClient {
    /// Get order book for a currency pair
    ///
    /// This endpoint returns the current order book (bid/ask) for the specified currency pair.
    /// You can optionally include order IDs and limit the number of levels returned.
    ///
    /// [docs](https://www.gate.com/docs/developers/apiv4/#retrieve-order-book)
    pub async fn get_order_book(
        &self,
        params: OrderBookRequest,
    ) -> crate::gateio::spot::RestResult<OrderBook> {
        self.get_with_query("/spot/order_book", Some(&params)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_book_request_minimal_serialization() {
        let request = OrderBookRequest {
            currency_pair: "BTC_USDT".to_string(),
            limit: None,
            with_id: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "currency_pair=BTC_USDT");
    }

    #[test]
    fn test_order_book_request_with_limit() {
        let request = OrderBookRequest {
            currency_pair: "ETH_USDT".to_string(),
            limit: Some(20),
            with_id: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("currency_pair=ETH_USDT"));
        assert!(serialized.contains("limit=20"));
        assert!(!serialized.contains("with_id"));
    }

    #[test]
    fn test_order_book_request_with_id() {
        let request = OrderBookRequest {
            currency_pair: "BNB_USDT".to_string(),
            limit: None,
            with_id: Some(true),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("currency_pair=BNB_USDT"));
        assert!(serialized.contains("with_id=true"));
    }

    #[test]
    fn test_order_book_request_full_parameters() {
        let request = OrderBookRequest {
            currency_pair: "SOL_USDT".to_string(),
            limit: Some(50),
            with_id: Some(false),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("currency_pair=SOL_USDT"));
        assert!(serialized.contains("limit=50"));
        assert!(serialized.contains("with_id=false"));
    }

    #[test]
    fn test_order_book_request_limit_edge_cases() {
        let limits = vec![1, 10, 50, 100];

        for limit in limits {
            let request = OrderBookRequest {
                currency_pair: "BTC_USDT".to_string(),
                limit: Some(limit),
                with_id: None,
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("limit={}", limit)));
        }
    }

    #[test]
    fn test_order_book_request_max_limit() {
        let request = OrderBookRequest {
            currency_pair: "BTC_USDT".to_string(),
            limit: Some(u32::MAX),
            with_id: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains(&format!("limit={}", u32::MAX)));
    }

    #[test]
    fn test_order_book_request_different_pairs() {
        let pairs = vec![
            "BTC_USDT",
            "ETH_USDT",
            "BNB_USDT",
            "SOL_USDT",
            "ADA_USDT",
            "DOT_USDT",
            "MATIC_USDT",
        ];

        for pair in pairs {
            let request = OrderBookRequest {
                currency_pair: pair.to_string(),
                limit: None,
                with_id: None,
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert_eq!(serialized, format!("currency_pair={}", pair));
        }
    }

    #[test]
    fn test_order_book_request_default() {
        let request = OrderBookRequest::default();
        assert_eq!(request.currency_pair, "");
        assert_eq!(request.limit, None);
        assert_eq!(request.with_id, None);
    }

    #[test]
    fn test_order_book_entry_deserialization() {
        let json = r#"{
            "0": "48500.50",
            "1": "0.5"
        }"#;

        let entry: OrderBookEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.price, "48500.50");
        assert_eq!(entry.amount, "0.5");
    }

    #[test]
    fn test_order_book_entry_serialization() {
        let entry = OrderBookEntry {
            price: "48500.50".to_string(),
            amount: "0.5".to_string(),
        };

        let json = serde_json::to_value(&entry).unwrap();
        assert_eq!(json["0"], "48500.50");
        assert_eq!(json["1"], "0.5");
    }

    #[test]
    fn test_order_book_with_id_deserialization() {
        let json = r#"{
            "id": 123456789,
            "current": 1640995200,
            "update": 1640995200000,
            "asks": [
                ["48500.50", "0.5"],
                ["48501.00", "1.0"],
                ["48502.00", "1.5"]
            ],
            "bids": [
                ["48499.50", "0.5"],
                ["48499.00", "1.0"],
                ["48498.00", "1.5"]
            ]
        }"#;

        let order_book: OrderBook = serde_json::from_str(json).unwrap();
        assert_eq!(order_book.id, Some(123456789));
        assert_eq!(order_book.current, 1640995200);
        assert_eq!(order_book.update, 1640995200000);
        assert_eq!(order_book.asks.len(), 3);
        assert_eq!(order_book.bids.len(), 3);

        // Verify first ask
        assert_eq!(order_book.asks[0][0], "48500.50");
        assert_eq!(order_book.asks[0][1], "0.5");

        // Verify first bid
        assert_eq!(order_book.bids[0][0], "48499.50");
        assert_eq!(order_book.bids[0][1], "0.5");
    }

    #[test]
    fn test_order_book_without_id_deserialization() {
        let json = r#"{
            "current": 1640995300,
            "update": 1640995300000,
            "asks": [
                ["3000.00", "2.0"],
                ["3000.50", "3.0"]
            ],
            "bids": [
                ["2999.50", "2.0"],
                ["2999.00", "3.0"]
            ]
        }"#;

        let order_book: OrderBook = serde_json::from_str(json).unwrap();
        assert_eq!(order_book.id, None);
        assert_eq!(order_book.current, 1640995300);
        assert_eq!(order_book.update, 1640995300000);
        assert_eq!(order_book.asks.len(), 2);
        assert_eq!(order_book.bids.len(), 2);
    }

    #[test]
    fn test_order_book_empty_levels() {
        let json = r#"{
            "current": 1640995400,
            "update": 1640995400000,
            "asks": [],
            "bids": []
        }"#;

        let order_book: OrderBook = serde_json::from_str(json).unwrap();
        assert_eq!(order_book.asks.len(), 0);
        assert_eq!(order_book.bids.len(), 0);
    }

    #[test]
    fn test_order_book_single_level() {
        let json = r#"{
            "current": 1640995500,
            "update": 1640995500000,
            "asks": [["100.00", "10.0"]],
            "bids": [["99.00", "10.0"]]
        }"#;

        let order_book: OrderBook = serde_json::from_str(json).unwrap();
        assert_eq!(order_book.asks.len(), 1);
        assert_eq!(order_book.bids.len(), 1);
        assert_eq!(order_book.asks[0][0], "100.00");
        assert_eq!(order_book.asks[0][1], "10.0");
        assert_eq!(order_book.bids[0][0], "99.00");
        assert_eq!(order_book.bids[0][1], "10.0");
    }

    #[test]
    fn test_order_book_large_amounts() {
        let json = r#"{
            "current": 1640995600,
            "update": 1640995600000,
            "asks": [
                ["48500.00", "999999.99999999"],
                ["48501.00", "0.00000001"]
            ],
            "bids": [
                ["48499.00", "999999.99999999"],
                ["48498.00", "0.00000001"]
            ]
        }"#;

        let order_book: OrderBook = serde_json::from_str(json).unwrap();
        assert_eq!(order_book.asks[0][1], "999999.99999999");
        assert_eq!(order_book.asks[1][1], "0.00000001");
        assert_eq!(order_book.bids[0][1], "999999.99999999");
        assert_eq!(order_book.bids[1][1], "0.00000001");
    }

    #[test]
    fn test_order_book_extreme_prices() {
        let json = r#"{
            "current": 1640995700,
            "update": 1640995700000,
            "asks": [
                ["999999.99999999", "1.0"],
                ["0.00000001", "1000000.0"]
            ],
            "bids": [
                ["999999.99999998", "1.0"],
                ["0.000000009", "1000000.0"]
            ]
        }"#;

        let order_book: OrderBook = serde_json::from_str(json).unwrap();
        assert_eq!(order_book.asks[0][0], "999999.99999999");
        assert_eq!(order_book.asks[1][0], "0.00000001");
        assert_eq!(order_book.bids[0][0], "999999.99999998");
        assert_eq!(order_book.bids[1][0], "0.000000009");
    }

    #[test]
    fn test_order_book_negative_id() {
        let json = r#"{
            "id": -123456789,
            "current": 1640995800,
            "update": 1640995800000,
            "asks": [["100.00", "1.0"]],
            "bids": [["99.00", "1.0"]]
        }"#;

        let order_book: OrderBook = serde_json::from_str(json).unwrap();
        assert_eq!(order_book.id, Some(-123456789));
    }

    #[test]
    fn test_order_book_max_id() {
        let json = format!(
            r#"{{
            "id": {},
            "current": 1640995900,
            "update": 1640995900000,
            "asks": [["100.00", "1.0"]],
            "bids": [["99.00", "1.0"]]
        }}"#,
            i64::MAX
        );

        let order_book: OrderBook = serde_json::from_str(&json).unwrap();
        assert_eq!(order_book.id, Some(i64::MAX));
    }

    #[test]
    fn test_order_book_zero_timestamps() {
        let json = r#"{
            "current": 0,
            "update": 0,
            "asks": [],
            "bids": []
        }"#;

        let order_book: OrderBook = serde_json::from_str(json).unwrap();
        assert_eq!(order_book.current, 0);
        assert_eq!(order_book.update, 0);
    }

    #[test]
    fn test_order_book_serialization() {
        let order_book = OrderBook {
            id: Some(123456789),
            current: 1640995200,
            update: 1640995200000,
            asks: vec![
                vec!["48500.50".to_string(), "0.5".to_string()],
                vec!["48501.00".to_string(), "1.0".to_string()],
            ],
            bids: vec![
                vec!["48499.50".to_string(), "0.5".to_string()],
                vec!["48499.00".to_string(), "1.0".to_string()],
            ],
        };

        let json = serde_json::to_value(&order_book).unwrap();
        assert_eq!(json["id"], 123456789);
        assert_eq!(json["current"], 1640995200);
        assert_eq!(json["update"], 1640995200000i64);
        assert_eq!(json["asks"][0][0], "48500.50");
        assert_eq!(json["asks"][0][1], "0.5");
        assert_eq!(json["bids"][0][0], "48499.50");
        assert_eq!(json["bids"][0][1], "0.5");
    }

    #[test]
    fn test_order_book_serialization_without_id() {
        let order_book = OrderBook {
            id: None,
            current: 1640995200,
            update: 1640995200000,
            asks: vec![vec!["100.00".to_string(), "1.0".to_string()]],
            bids: vec![vec!["99.00".to_string(), "1.0".to_string()]],
        };

        let json = serde_json::to_string(&order_book).unwrap();
        assert!(!json.contains("\"id\":"));
    }

    #[test]
    fn test_order_book_round_trip() {
        let original = OrderBook {
            id: Some(987654321),
            current: 1640995200,
            update: 1640995200000,
            asks: vec![
                vec!["3000.00".to_string(), "2.0".to_string()],
                vec!["3000.50".to_string(), "3.0".to_string()],
                vec!["3001.00".to_string(), "4.0".to_string()],
            ],
            bids: vec![
                vec!["2999.50".to_string(), "2.0".to_string()],
                vec!["2999.00".to_string(), "3.0".to_string()],
                vec!["2998.50".to_string(), "4.0".to_string()],
            ],
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: OrderBook = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, original.id);
        assert_eq!(deserialized.current, original.current);
        assert_eq!(deserialized.update, original.update);
        assert_eq!(deserialized.asks.len(), original.asks.len());
        assert_eq!(deserialized.bids.len(), original.bids.len());

        for i in 0..original.asks.len() {
            assert_eq!(deserialized.asks[i], original.asks[i]);
        }

        for i in 0..original.bids.len() {
            assert_eq!(deserialized.bids[i], original.bids[i]);
        }
    }

    #[test]
    fn test_order_book_realistic_btc_data() {
        let json = r#"{
            "id": 123456789,
            "current": 1640995200,
            "update": 1640995200123,
            "asks": [
                ["47234.56", "0.12345678"],
                ["47234.57", "0.23456789"],
                ["47234.58", "0.34567890"],
                ["47234.59", "0.45678901"],
                ["47234.60", "0.56789012"]
            ],
            "bids": [
                ["47234.55", "0.12345678"],
                ["47234.54", "0.23456789"],
                ["47234.53", "0.34567890"],
                ["47234.52", "0.45678901"],
                ["47234.51", "0.56789012"]
            ]
        }"#;

        let order_book: OrderBook = serde_json::from_str(json).unwrap();
        assert_eq!(order_book.id, Some(123456789));
        assert_eq!(order_book.asks.len(), 5);
        assert_eq!(order_book.bids.len(), 5);

        // Verify asks are in ascending order
        let ask_prices: Vec<f64> = order_book
            .asks
            .iter()
            .map(|level| level[0].parse::<f64>().unwrap())
            .collect();
        for i in 1..ask_prices.len() {
            assert!(ask_prices[i] > ask_prices[i - 1]);
        }

        // Verify bids are in descending order
        let bid_prices: Vec<f64> = order_book
            .bids
            .iter()
            .map(|level| level[0].parse::<f64>().unwrap())
            .collect();
        for i in 1..bid_prices.len() {
            assert!(bid_prices[i] < bid_prices[i - 1]);
        }

        // Verify spread exists (best ask > best bid)
        assert!(ask_prices[0] > bid_prices[0]);
    }

    #[test]
    fn test_order_book_stablecoin_tight_spread() {
        let json = r#"{
            "current": 1640995200,
            "update": 1640995200000,
            "asks": [
                ["1.0001", "100000.0"],
                ["1.0002", "200000.0"],
                ["1.0003", "300000.0"]
            ],
            "bids": [
                ["1.0000", "100000.0"],
                ["0.9999", "200000.0"],
                ["0.9998", "300000.0"]
            ]
        }"#;

        let order_book: OrderBook = serde_json::from_str(json).unwrap();

        // Calculate spread
        let best_ask: f64 = order_book.asks[0][0].parse().unwrap();
        let best_bid: f64 = order_book.bids[0][0].parse().unwrap();
        let spread = best_ask - best_bid;

        // Verify tight spread for stablecoin pair
        assert!(spread < 0.001);
        assert!(spread > 0.0);
    }

    #[test]
    fn test_order_book_many_levels() {
        let mut asks = Vec::new();
        let mut bids = Vec::new();

        // Generate 100 levels
        for i in 0..100 {
            asks.push(vec![
                format!("{:.2}", 50000.0 + i as f64 * 0.01),
                format!("{:.8}", 0.1 + i as f64 * 0.01),
            ]);
            bids.push(vec![
                format!("{:.2}", 49999.99 - i as f64 * 0.01),
                format!("{:.8}", 0.1 + i as f64 * 0.01),
            ]);
        }

        let order_book = OrderBook {
            id: None,
            current: 1640995200,
            update: 1640995200000,
            asks,
            bids,
        };

        assert_eq!(order_book.asks.len(), 100);
        assert_eq!(order_book.bids.len(), 100);
    }

    #[test]
    fn test_order_book_clone() {
        let original = OrderBook {
            id: Some(123456789),
            current: 1640995200,
            update: 1640995200000,
            asks: vec![vec!["100.00".to_string(), "1.0".to_string()]],
            bids: vec![vec!["99.00".to_string(), "1.0".to_string()]],
        };

        let cloned = original.clone();
        assert_eq!(cloned.id, original.id);
        assert_eq!(cloned.current, original.current);
        assert_eq!(cloned.update, original.update);
        assert_eq!(cloned.asks, original.asks);
        assert_eq!(cloned.bids, original.bids);
    }

    #[test]
    fn test_order_book_debug() {
        let order_book = OrderBook {
            id: Some(123456789),
            current: 1640995200,
            update: 1640995200000,
            asks: vec![vec!["100.00".to_string(), "1.0".to_string()]],
            bids: vec![vec!["99.00".to_string(), "1.0".to_string()]],
        };

        let debug_str = format!("{:?}", order_book);
        assert!(debug_str.contains("OrderBook"));
        assert!(debug_str.contains("123456789"));
        assert!(debug_str.contains("100.00"));
    }
}
