use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::options::RestResult;

const DEPTH_ENDPOINT: &str = "/eapi/v1/depth";

/// Request parameters for order book
#[derive(Debug, Clone, Serialize)]
pub struct OrderBookRequest {
    /// Option trading pair, e.g BTC-200730-9000-C
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Default: 100, Max: 1000, Optional values: [10, 20, 50, 100, 500, 1000]
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Order book response
#[derive(Debug, Clone, Deserialize)]
pub struct OrderBookResponse {
    /// Transaction time
    #[serde(rename = "T")]
    pub transaction_time: u64,

    /// Update ID
    #[serde(rename = "u")]
    pub update_id: u64,

    /// Buy orders
    #[serde(rename = "bids")]
    pub bids: Vec<OrderBookLevel>,

    /// Sell orders
    #[serde(rename = "asks")]
    pub asks: Vec<OrderBookLevel>,
}

/// Order book level (price and quantity)
#[derive(Debug, Clone, Deserialize)]
pub struct OrderBookLevel(pub Decimal, pub Decimal);

impl RestClient {
    /// Get order book
    ///
    /// Returns orderbook depth for the specified symbol.
    ///
    /// [docs]: (https://developers.binance.com/docs/derivatives/option/market-data/Order-Book)
    /// Method: GET /eapi/v1/depth
    /// Weight: 2 (for 5,10,20,50), 5 (for 100), 10 (for 500), 20 (for 1000)
    /// Security: None
    pub async fn get_order_book(&self, params: OrderBookRequest) -> RestResult<OrderBookResponse> {
        let weight = match params.limit.unwrap_or(100) {
            5 | 10 | 20 | 50 => 2,
            100 => 5,
            500 => 10,
            1000 => 20,
            _ => 5, // Default to 100 weight
        };

        self.send_public_request(DEPTH_ENDPOINT, reqwest::Method::GET, Some(params), weight)
            .await
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_order_book_request_serialization_with_symbol_only() {
        let request = OrderBookRequest {
            symbol: "BTC-240329-70000-C".to_string(),
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTC-240329-70000-C"));
        assert!(!serialized.contains("limit"));
    }

    #[test]
    fn test_order_book_request_serialization_with_symbol_and_limit() {
        let request = OrderBookRequest {
            symbol: "BTC-240329-70000-C".to_string(),
            limit: Some(100),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTC-240329-70000-C"));
        assert!(serialized.contains("limit=100"));
    }

    #[test]
    fn test_order_book_request_serialization_various_limits() {
        let valid_limits = vec![5, 10, 20, 50, 100, 500, 1000];

        for limit in valid_limits {
            let request = OrderBookRequest {
                symbol: "BTC-240329-70000-C".to_string(),
                limit: Some(limit),
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains("symbol=BTC-240329-70000-C"));
            assert!(serialized.contains(&format!("limit={}", limit)));
        }
    }

    #[test]
    fn test_order_book_request_serialization_various_symbols() {
        let symbols = vec![
            "BTC-240329-70000-C",
            "BTC-240329-70000-P",
            "ETH-240329-3000-C",
            "ETH-240329-3000-P",
            "BNB-240329-500-C",
            "BNB-240329-500-P",
        ];

        for symbol in symbols {
            let request = OrderBookRequest {
                symbol: symbol.to_string(),
                limit: Some(100),
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("symbol={}", symbol)));
            assert!(serialized.contains("limit=100"));
        }
    }

    #[test]
    fn test_order_book_request_serialization_edge_case_limits() {
        // Test with invalid limit (should still serialize)
        let request = OrderBookRequest {
            symbol: "BTC-240329-70000-C".to_string(),
            limit: Some(999),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTC-240329-70000-C"));
        assert!(serialized.contains("limit=999"));
    }

    #[test]
    fn test_order_book_request_serialization_zero_limit() {
        let request = OrderBookRequest {
            symbol: "BTC-240329-70000-C".to_string(),
            limit: Some(0),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTC-240329-70000-C"));
        assert!(serialized.contains("limit=0"));
    }

    #[test]
    fn test_order_book_request_serialization_max_limit() {
        let request = OrderBookRequest {
            symbol: "BTC-240329-70000-C".to_string(),
            limit: Some(u32::MAX),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTC-240329-70000-C"));
        assert!(serialized.contains(&format!("limit={}", u32::MAX)));
    }

    #[test]
    fn test_order_book_request_clone() {
        let request = OrderBookRequest {
            symbol: "BTC-240329-70000-C".to_string(),
            limit: Some(100),
        };

        let cloned = request.clone();
        assert_eq!(request.symbol, cloned.symbol);
        assert_eq!(request.limit, cloned.limit);
    }

    #[test]
    fn test_order_book_request_debug() {
        let request = OrderBookRequest {
            symbol: "BTC-240329-70000-C".to_string(),
            limit: Some(100),
        };

        let debug_output = format!("{:?}", request);
        assert!(debug_output.contains("OrderBookRequest"));
        assert!(debug_output.contains("BTC-240329-70000-C"));
        assert!(debug_output.contains("100"));
    }

    #[test]
    fn test_order_book_level_deserialization() {
        let json = r#"["1000.50", "0.25"]"#;
        let level: OrderBookLevel = serde_json::from_str(json).unwrap();
        assert_eq!(level.0, dec!(1000.50));
        assert_eq!(level.1, dec!(0.25));
    }

    #[test]
    fn test_order_book_level_deserialization_high_precision() {
        let json = r#"["1000.12345678", "0.87654321"]"#;
        let level: OrderBookLevel = serde_json::from_str(json).unwrap();
        assert_eq!(level.0.to_string(), "1000.12345678");
        assert_eq!(level.1.to_string(), "0.87654321");
    }

    #[test]
    fn test_order_book_level_deserialization_zero_values() {
        let json = r#"["0.00000000", "0.00000000"]"#;
        let level: OrderBookLevel = serde_json::from_str(json).unwrap();
        assert_eq!(level.0, dec!(0.00000000));
        assert_eq!(level.1, dec!(0.00000000));
    }

    #[test]
    fn test_order_book_level_deserialization_large_values() {
        let json = r#"["99999.99999999", "999.99999999"]"#;
        let level: OrderBookLevel = serde_json::from_str(json).unwrap();
        assert_eq!(level.0.to_string(), "99999.99999999");
        assert_eq!(level.1.to_string(), "999.99999999");
    }

    #[test]
    fn test_order_book_level_clone() {
        let level = OrderBookLevel(dec!(1000.50), dec!(0.25));
        let cloned = level.clone();
        assert_eq!(level.0, cloned.0);
        assert_eq!(level.1, cloned.1);
    }

    #[test]
    fn test_order_book_level_debug() {
        let level = OrderBookLevel(dec!(1000.50), dec!(0.25));
        let debug_output = format!("{:?}", level);
        assert!(debug_output.contains("OrderBookLevel"));
        assert!(debug_output.contains("1000.50"));
        assert!(debug_output.contains("0.25"));
    }

    #[test]
    fn test_order_book_response_deserialization() {
        let json = r#"{
            "T": 1625097600000,
            "u": 12345,
            "bids": [
                ["1000.50", "0.25"],
                ["1000.25", "0.50"],
                ["1000.00", "1.00"]
            ],
            "asks": [
                ["1001.00", "0.75"],
                ["1001.25", "0.50"],
                ["1001.50", "0.25"]
            ]
        }"#;

        let response: OrderBookResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.transaction_time, 1625097600000);
        assert_eq!(response.update_id, 12345);
        assert_eq!(response.bids.len(), 3);
        assert_eq!(response.asks.len(), 3);

        // Verify first bid
        assert_eq!(response.bids[0].0, dec!(1000.50));
        assert_eq!(response.bids[0].1, dec!(0.25));

        // Verify first ask
        assert_eq!(response.asks[0].0, dec!(1001.00));
        assert_eq!(response.asks[0].1, dec!(0.75));
    }

    #[test]
    fn test_order_book_response_deserialization_empty_order_book() {
        let json = r#"{
            "T": 1625097600000,
            "u": 12345,
            "bids": [],
            "asks": []
        }"#;

        let response: OrderBookResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.transaction_time, 1625097600000);
        assert_eq!(response.update_id, 12345);
        assert_eq!(response.bids.len(), 0);
        assert_eq!(response.asks.len(), 0);
    }

    #[test]
    fn test_order_book_response_deserialization_only_bids() {
        let json = r#"{
            "T": 1625097600000,
            "u": 12345,
            "bids": [
                ["1000.50", "0.25"],
                ["1000.25", "0.50"]
            ],
            "asks": []
        }"#;

        let response: OrderBookResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.transaction_time, 1625097600000);
        assert_eq!(response.update_id, 12345);
        assert_eq!(response.bids.len(), 2);
        assert_eq!(response.asks.len(), 0);
    }

    #[test]
    fn test_order_book_response_deserialization_only_asks() {
        let json = r#"{
            "T": 1625097600000,
            "u": 12345,
            "bids": [],
            "asks": [
                ["1001.00", "0.75"],
                ["1001.25", "0.50"]
            ]
        }"#;

        let response: OrderBookResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.transaction_time, 1625097600000);
        assert_eq!(response.update_id, 12345);
        assert_eq!(response.bids.len(), 0);
        assert_eq!(response.asks.len(), 2);
    }

    #[test]
    fn test_order_book_response_deserialization_large_order_book() {
        let mut bids = Vec::new();
        let mut asks = Vec::new();

        // Generate 1000 levels for each side
        for i in 0..1000 {
            bids.push(format!(
                r#"["{:.2}", "{:.2}"]"#,
                1000.0 - i as f64 * 0.01,
                0.1 + i as f64 * 0.001
            ));
            asks.push(format!(
                r#"["{:.2}", "{:.2}"]"#,
                1001.0 + i as f64 * 0.01,
                0.1 + i as f64 * 0.001
            ));
        }

        let json = format!(
            r#"{{
                "T": 1625097600000,
                "u": 12345,
                "bids": [{}],
                "asks": [{}]
            }}"#,
            bids.join(","),
            asks.join(",")
        );

        let response: OrderBookResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(response.transaction_time, 1625097600000);
        assert_eq!(response.update_id, 12345);
        assert_eq!(response.bids.len(), 1000);
        assert_eq!(response.asks.len(), 1000);

        // Verify first and last entries
        assert_eq!(response.bids[0].0, dec!(1000.00));
        assert_eq!(response.bids[999].0, dec!(990.01));
        assert_eq!(response.asks[0].0, dec!(1001.00));
        assert_eq!(response.asks[999].0, dec!(1010.99));
    }

    #[test]
    fn test_order_book_response_deserialization_high_precision() {
        let json = r#"{
            "T": 1625097600000,
            "u": 12345,
            "bids": [
                ["1000.12345678", "0.87654321"],
                ["999.87654321", "1.23456789"]
            ],
            "asks": [
                ["1001.98765432", "0.11111111"],
                ["1002.11111111", "2.22222222"]
            ]
        }"#;

        let response: OrderBookResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.bids[0].0.to_string(), "1000.12345678");
        assert_eq!(response.bids[0].1.to_string(), "0.87654321");
        assert_eq!(response.bids[1].0.to_string(), "999.87654321");
        assert_eq!(response.bids[1].1.to_string(), "1.23456789");
        assert_eq!(response.asks[0].0.to_string(), "1001.98765432");
        assert_eq!(response.asks[0].1.to_string(), "0.11111111");
        assert_eq!(response.asks[1].0.to_string(), "1002.11111111");
        assert_eq!(response.asks[1].1.to_string(), "2.22222222");
    }

    #[test]
    fn test_order_book_response_deserialization_zero_quantities() {
        let json = r#"{
            "T": 1625097600000,
            "u": 12345,
            "bids": [
                ["1000.50", "0.00000000"],
                ["1000.25", "0.50"]
            ],
            "asks": [
                ["1001.00", "0.00000000"],
                ["1001.25", "0.75"]
            ]
        }"#;

        let response: OrderBookResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.bids[0].1, dec!(0.00000000));
        assert_eq!(response.bids[1].1, dec!(0.50));
        assert_eq!(response.asks[0].1, dec!(0.00000000));
        assert_eq!(response.asks[1].1, dec!(0.75));
    }

    #[test]
    fn test_order_book_response_deserialization_large_update_id() {
        let json = r#"{
            "T": 1625097600000,
            "u": 18446744073709551615,
            "bids": [
                ["1000.50", "0.25"]
            ],
            "asks": [
                ["1001.00", "0.75"]
            ]
        }"#;

        let response: OrderBookResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.update_id, u64::MAX);
    }

    #[test]
    fn test_order_book_response_deserialization_large_transaction_time() {
        let json = r#"{
            "T": 18446744073709551615,
            "u": 12345,
            "bids": [
                ["1000.50", "0.25"]
            ],
            "asks": [
                ["1001.00", "0.75"]
            ]
        }"#;

        let response: OrderBookResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.transaction_time, u64::MAX);
    }

    #[test]
    fn test_order_book_response_clone() {
        let json = r#"{
            "T": 1625097600000,
            "u": 12345,
            "bids": [
                ["1000.50", "0.25"]
            ],
            "asks": [
                ["1001.00", "0.75"]
            ]
        }"#;

        let response: OrderBookResponse = serde_json::from_str(json).unwrap();
        let cloned = response.clone();

        assert_eq!(response.transaction_time, cloned.transaction_time);
        assert_eq!(response.update_id, cloned.update_id);
        assert_eq!(response.bids.len(), cloned.bids.len());
        assert_eq!(response.asks.len(), cloned.asks.len());
        assert_eq!(response.bids[0].0, cloned.bids[0].0);
        assert_eq!(response.bids[0].1, cloned.bids[0].1);
        assert_eq!(response.asks[0].0, cloned.asks[0].0);
        assert_eq!(response.asks[0].1, cloned.asks[0].1);
    }

    #[test]
    fn test_order_book_response_debug() {
        let json = r#"{
            "T": 1625097600000,
            "u": 12345,
            "bids": [
                ["1000.50", "0.25"]
            ],
            "asks": [
                ["1001.00", "0.75"]
            ]
        }"#;

        let response: OrderBookResponse = serde_json::from_str(json).unwrap();
        let debug_output = format!("{:?}", response);

        assert!(debug_output.contains("OrderBookResponse"));
        assert!(debug_output.contains("1625097600000"));
        assert!(debug_output.contains("12345"));
        assert!(debug_output.contains("1000.50"));
        assert!(debug_output.contains("1001.00"));
    }

    #[test]
    fn test_order_book_response_price_ordering() {
        let json = r#"{
            "T": 1625097600000,
            "u": 12345,
            "bids": [
                ["1000.50", "0.25"],
                ["1000.25", "0.50"],
                ["1000.00", "1.00"]
            ],
            "asks": [
                ["1001.00", "0.75"],
                ["1001.25", "0.50"],
                ["1001.50", "0.25"]
            ]
        }"#;

        let response: OrderBookResponse = serde_json::from_str(json).unwrap();

        // Verify bids are in descending order (highest price first)
        assert!(response.bids[0].0 > response.bids[1].0);
        assert!(response.bids[1].0 > response.bids[2].0);

        // Verify asks are in ascending order (lowest price first)
        assert!(response.asks[0].0 < response.asks[1].0);
        assert!(response.asks[1].0 < response.asks[2].0);

        // Verify spread (best ask > best bid)
        assert!(response.asks[0].0 > response.bids[0].0);
    }

    #[test]
    fn test_order_book_response_spread_calculation() {
        let json = r#"{
            "T": 1625097600000,
            "u": 12345,
            "bids": [
                ["1000.50", "0.25"],
                ["1000.25", "0.50"]
            ],
            "asks": [
                ["1001.00", "0.75"],
                ["1001.25", "0.50"]
            ]
        }"#;

        let response: OrderBookResponse = serde_json::from_str(json).unwrap();

        let best_bid = response.bids[0].0;
        let best_ask = response.asks[0].0;
        let spread = best_ask - best_bid;

        assert_eq!(best_bid, dec!(1000.50));
        assert_eq!(best_ask, dec!(1001.00));
        assert_eq!(spread, dec!(0.50));
    }

    #[test]
    fn test_order_book_response_total_volume_calculation() {
        let json = r#"{
            "T": 1625097600000,
            "u": 12345,
            "bids": [
                ["1000.50", "0.25"],
                ["1000.25", "0.50"],
                ["1000.00", "1.00"]
            ],
            "asks": [
                ["1001.00", "0.75"],
                ["1001.25", "0.50"],
                ["1001.50", "0.25"]
            ]
        }"#;

        let response: OrderBookResponse = serde_json::from_str(json).unwrap();

        let total_bid_volume: rust_decimal::Decimal =
            response.bids.iter().map(|level| level.1).sum();
        let total_ask_volume: rust_decimal::Decimal =
            response.asks.iter().map(|level| level.1).sum();

        assert_eq!(total_bid_volume, dec!(1.75)); // 0.25 + 0.50 + 1.00
        assert_eq!(total_ask_volume, dec!(1.50)); // 0.75 + 0.50 + 0.25
    }

    #[test]
    fn test_order_book_response_consistency_checks() {
        let json = r#"{
            "T": 1625097600000,
            "u": 12345,
            "bids": [
                ["1000.50", "0.25"],
                ["1000.25", "0.50"],
                ["1000.00", "1.00"]
            ],
            "asks": [
                ["1001.00", "0.75"],
                ["1001.25", "0.50"],
                ["1001.50", "0.25"]
            ]
        }"#;

        let response: OrderBookResponse = serde_json::from_str(json).unwrap();

        // All quantities should be non-negative
        for bid in &response.bids {
            assert!(bid.1 >= dec!(0));
        }
        for ask in &response.asks {
            assert!(ask.1 >= dec!(0));
        }

        // All prices should be positive
        for bid in &response.bids {
            assert!(bid.0 > dec!(0));
        }
        for ask in &response.asks {
            assert!(ask.0 > dec!(0));
        }

        // Transaction time should be reasonable (not zero)
        assert!(response.transaction_time > 0);

        // Update ID should be reasonable
        assert!(response.update_id > 0);
    }

    #[test]
    fn test_order_book_response_market_depth_analysis() {
        let json = r#"{
            "T": 1625097600000,
            "u": 12345,
            "bids": [
                ["1000.50", "0.25"],
                ["1000.25", "0.50"],
                ["1000.00", "1.00"],
                ["999.75", "2.00"],
                ["999.50", "3.00"]
            ],
            "asks": [
                ["1001.00", "0.75"],
                ["1001.25", "0.50"],
                ["1001.50", "0.25"],
                ["1001.75", "1.50"],
                ["1002.00", "2.50"]
            ]
        }"#;

        let response: OrderBookResponse = serde_json::from_str(json).unwrap();

        // Calculate depth at different price levels
        let depth_5_levels = response.bids.len().min(5);
        let depth_10_levels = response.bids.len().min(10);

        assert_eq!(depth_5_levels, 5);
        assert_eq!(depth_10_levels, 5); // Only 5 levels available

        // Calculate cumulative volumes
        let mut cumulative_bid_volume = dec!(0);
        let mut cumulative_ask_volume = dec!(0);

        for i in 0..depth_5_levels {
            cumulative_bid_volume += response.bids[i].1;
            cumulative_ask_volume += response.asks[i].1;
        }

        assert_eq!(cumulative_bid_volume, dec!(6.75)); // 0.25 + 0.50 + 1.00 + 2.00 + 3.00
        assert_eq!(cumulative_ask_volume, dec!(5.50)); // 0.75 + 0.50 + 0.25 + 1.50 + 2.50
    }

    #[test]
    fn test_order_book_response_different_data_types() {
        // Test with string numbers that might have trailing zeros
        let json = r#"{
            "T": 1625097600000,
            "u": 12345,
            "bids": [
                ["1000.5000", "0.2500"],
                ["1000.2500", "0.5000"]
            ],
            "asks": [
                ["1001.0000", "0.7500"],
                ["1001.2500", "0.5000"]
            ]
        }"#;

        let response: OrderBookResponse = serde_json::from_str(json).unwrap();

        // Verify proper decimal parsing
        assert_eq!(response.bids[0].0, dec!(1000.5000));
        assert_eq!(response.bids[0].1, dec!(0.2500));
        assert_eq!(response.asks[0].0, dec!(1001.0000));
        assert_eq!(response.asks[0].1, dec!(0.7500));
    }

    #[test]
    fn test_order_book_response_edge_case_very_small_quantities() {
        let json = r#"{
            "T": 1625097600000,
            "u": 12345,
            "bids": [
                ["1000.50", "0.00000001"],
                ["1000.25", "0.00000002"]
            ],
            "asks": [
                ["1001.00", "0.00000001"],
                ["1001.25", "0.00000002"]
            ]
        }"#;

        let response: OrderBookResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.bids[0].1.to_string(), "0.00000001");
        assert_eq!(response.bids[1].1.to_string(), "0.00000002");
        assert_eq!(response.asks[0].1.to_string(), "0.00000001");
        assert_eq!(response.asks[1].1.to_string(), "0.00000002");
    }

    #[test]
    fn test_order_book_response_edge_case_very_large_prices() {
        let json = r#"{
            "T": 1625097600000,
            "u": 12345,
            "bids": [
                ["999999.99999999", "0.00000001"],
                ["999999.99999998", "0.00000002"]
            ],
            "asks": [
                ["1000000.00000000", "0.00000001"],
                ["1000000.00000001", "0.00000002"]
            ]
        }"#;

        let response: OrderBookResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.bids[0].0.to_string(), "999999.99999999");
        assert_eq!(response.bids[1].0.to_string(), "999999.99999998");
        assert_eq!(response.asks[0].0.to_string(), "1000000.00000000");
        assert_eq!(response.asks[1].0.to_string(), "1000000.00000001");
    }
}
