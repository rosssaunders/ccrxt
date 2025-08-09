use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::options::RestResult;

const TRADES_ENDPOINT: &str = "/eapi/v1/trades";

/// Request parameters for recent trades
#[derive(Debug, Clone, Serialize)]
pub struct RecentTradesRequest {
    /// Option trading pair, e.g BTC-200730-9000-C
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Number of records (Default: 100, Max: 500)
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Recent trade information
#[derive(Debug, Clone, Deserialize)]
pub struct RecentTrade {
    /// Trade ID
    #[serde(rename = "id")]
    pub id: String,

    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Completed trade price
    #[serde(rename = "price")]
    pub price: Decimal,

    /// Completed trade quantity
    #[serde(rename = "qty")]
    pub qty: Decimal,

    /// Completed trade amount
    #[serde(rename = "quoteQty")]
    pub quote_qty: Decimal,

    /// Completed trade direction (-1 Sell, 1 Buy)
    #[serde(rename = "side")]
    pub side: i32,

    /// Time
    #[serde(rename = "time")]
    pub time: u64,
}

impl RestClient {
    /// Get recent trades list
    ///
    /// Returns recent market trades.
    ///
    /// [docs]: (https://developers.binance.com/docs/derivatives/option/market-data/Recent-Trades-List)
    /// Method: GET /eapi/v1/trades
    /// Weight: 5
    /// Security: None
    pub async fn get_recent_trades(
        &self,
        params: RecentTradesRequest,
    ) -> RestResult<Vec<RecentTrade>> {
        self.send_get_request(TRADES_ENDPOINT, Some(params), 5)
            .await
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_recent_trades_request_serialization_basic() {
        let request = RecentTradesRequest {
            symbol: "BTC-240329-70000-C".to_string(),
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTC-240329-70000-C"));
        assert!(!serialized.contains("limit"));
    }

    #[test]
    fn test_recent_trades_request_serialization_with_limit() {
        let request = RecentTradesRequest {
            symbol: "BTC-240329-70000-C".to_string(),
            limit: Some(100),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTC-240329-70000-C"));
        assert!(serialized.contains("limit=100"));
    }

    #[test]
    fn test_recent_trades_request_serialization_with_max_limit() {
        let request = RecentTradesRequest {
            symbol: "ETH-240329-3000-P".to_string(),
            limit: Some(500),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=ETH-240329-3000-P"));
        assert!(serialized.contains("limit=500"));
    }

    #[test]
    fn test_recent_trades_request_serialization_with_min_limit() {
        let request = RecentTradesRequest {
            symbol: "BTC-240329-70000-C".to_string(),
            limit: Some(1),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTC-240329-70000-C"));
        assert!(serialized.contains("limit=1"));
    }

    #[test]
    fn test_recent_trades_request_serialization_various_symbols() {
        let symbols = vec![
            "BTC-240329-70000-C",
            "BTC-240329-70000-P",
            "ETH-240329-3000-C",
            "ETH-240329-3000-P",
            "BNB-240329-500-C",
            "BNB-240329-500-P",
        ];

        for symbol in symbols {
            let request = RecentTradesRequest {
                symbol: symbol.to_string(),
                limit: Some(100),
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("symbol={}", symbol)));
            assert!(serialized.contains("limit=100"));
        }
    }

    #[test]
    fn test_recent_trades_request_serialization_different_limits() {
        let limits = vec![1, 10, 50, 100, 250, 500];

        for limit in limits {
            let request = RecentTradesRequest {
                symbol: "BTC-240329-70000-C".to_string(),
                limit: Some(limit),
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("limit={}", limit)));
        }
    }

    #[test]
    fn test_recent_trades_request_clone() {
        let request = RecentTradesRequest {
            symbol: "BTC-240329-70000-C".to_string(),
            limit: Some(100),
        };

        let cloned = request.clone();
        assert_eq!(request.symbol, cloned.symbol);
        assert_eq!(request.limit, cloned.limit);
    }

    #[test]
    fn test_recent_trades_request_debug() {
        let request = RecentTradesRequest {
            symbol: "BTC-240329-70000-C".to_string(),
            limit: Some(100),
        };

        let debug_output = format!("{:?}", request);
        assert!(debug_output.contains("RecentTradesRequest"));
        assert!(debug_output.contains("BTC-240329-70000-C"));
        assert!(debug_output.contains("100"));
    }

    #[test]
    fn test_recent_trade_deserialization() {
        let json = r#"{
            "id": "12345",
            "symbol": "BTC-240329-70000-C",
            "price": "7150.50",
            "qty": "0.10",
            "quoteQty": "715.05",
            "side": 1,
            "time": 1625097600000
        }"#;

        let trade: RecentTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, "12345");
        assert_eq!(trade.symbol, "BTC-240329-70000-C");
        assert_eq!(trade.price, dec!(7150.50));
        assert_eq!(trade.qty, dec!(0.10));
        assert_eq!(trade.quote_qty, dec!(715.05));
        assert_eq!(trade.side, 1);
        assert_eq!(trade.time, 1625097600000);
    }

    #[test]
    fn test_recent_trade_deserialization_sell_side() {
        let json = r#"{
            "id": "67890",
            "symbol": "ETH-240329-3000-P",
            "price": "2950.25",
            "qty": "0.25",
            "quoteQty": "737.56",
            "side": -1,
            "time": 1625097700000
        }"#;

        let trade: RecentTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, "67890");
        assert_eq!(trade.symbol, "ETH-240329-3000-P");
        assert_eq!(trade.price, dec!(2950.25));
        assert_eq!(trade.qty, dec!(0.25));
        assert_eq!(trade.quote_qty, dec!(737.56));
        assert_eq!(trade.side, -1);
        assert_eq!(trade.time, 1625097700000);
    }

    #[test]
    fn test_recent_trade_deserialization_high_precision() {
        let json = r#"{
            "id": "999999",
            "symbol": "BTC-240329-70000-C",
            "price": "7150.12345678",
            "qty": "0.12345678",
            "quoteQty": "883.12345678",
            "side": 1,
            "time": 1625097800000
        }"#;

        let trade: RecentTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.price.to_string(), "7150.12345678");
        assert_eq!(trade.qty.to_string(), "0.12345678");
        assert_eq!(trade.quote_qty.to_string(), "883.12345678");
    }

    #[test]
    fn test_recent_trade_deserialization_zero_values() {
        let json = r#"{
            "id": "0",
            "symbol": "BTC-240329-70000-C",
            "price": "0.00000000",
            "qty": "0.00000000",
            "quoteQty": "0.00000000",
            "side": 0,
            "time": 0
        }"#;

        let trade: RecentTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, "0");
        assert_eq!(trade.price, dec!(0.00000000));
        assert_eq!(trade.qty, dec!(0.00000000));
        assert_eq!(trade.quote_qty, dec!(0.00000000));
        assert_eq!(trade.side, 0);
        assert_eq!(trade.time, 0);
    }

    #[test]
    fn test_recent_trade_deserialization_large_values() {
        let json = r#"{
            "id": "999999999",
            "symbol": "BTC-240329-100000-C",
            "price": "99999.99999999",
            "qty": "999.99999999",
            "quoteQty": "99999999.99999999",
            "side": 1,
            "time": 9999999999999
        }"#;

        let trade: RecentTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, "999999999");
        assert_eq!(trade.price.to_string(), "99999.99999999");
        assert_eq!(trade.qty.to_string(), "999.99999999");
        assert_eq!(trade.quote_qty.to_string(), "99999999.99999999");
        assert_eq!(trade.side, 1);
        assert_eq!(trade.time, 9999999999999);
    }

    #[test]
    fn test_recent_trade_deserialization_different_symbols() {
        let symbols = vec![
            "BTC-240329-70000-C",
            "BTC-240329-70000-P",
            "ETH-240329-3000-C",
            "ETH-240329-3000-P",
            "BNB-240329-500-C",
            "BNB-240329-500-P",
        ];

        for symbol in symbols {
            let json = format!(
                r#"{{
                    "id": "12345",
                    "symbol": "{}",
                    "price": "1000.00",
                    "qty": "0.10",
                    "quoteQty": "100.00",
                    "side": 1,
                    "time": 1625097600000
                }}"#,
                symbol
            );

            let trade: RecentTrade = serde_json::from_str(&json).unwrap();
            assert_eq!(trade.symbol, symbol);
        }
    }

    #[test]
    fn test_recent_trade_deserialization_side_values() {
        let sides = vec![-1, 0, 1];

        for side in sides {
            let json = format!(
                r#"{{
                    "id": "12345",
                    "symbol": "BTC-240329-70000-C",
                    "price": "7150.50",
                    "qty": "0.10",
                    "quoteQty": "715.05",
                    "side": {},
                    "time": 1625097600000
                }}"#,
                side
            );

            let trade: RecentTrade = serde_json::from_str(&json).unwrap();
            assert_eq!(trade.side, side);
        }
    }

    #[test]
    fn test_recent_trades_array_deserialization() {
        let json = r#"[
            {
                "id": "12345",
                "symbol": "BTC-240329-70000-C",
                "price": "7150.50",
                "qty": "0.10",
                "quoteQty": "715.05",
                "side": 1,
                "time": 1625097600000
            },
            {
                "id": "67890",
                "symbol": "BTC-240329-70000-C",
                "price": "7155.25",
                "qty": "0.05",
                "quoteQty": "357.76",
                "side": -1,
                "time": 1625097700000
            }
        ]"#;

        let trades: Vec<RecentTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 2);

        // First trade (buy)
        assert_eq!(trades[0].id, "12345");
        assert_eq!(trades[0].symbol, "BTC-240329-70000-C");
        assert_eq!(trades[0].price, dec!(7150.50));
        assert_eq!(trades[0].qty, dec!(0.10));
        assert_eq!(trades[0].quote_qty, dec!(715.05));
        assert_eq!(trades[0].side, 1);
        assert_eq!(trades[0].time, 1625097600000);

        // Second trade (sell)
        assert_eq!(trades[1].id, "67890");
        assert_eq!(trades[1].symbol, "BTC-240329-70000-C");
        assert_eq!(trades[1].price, dec!(7155.25));
        assert_eq!(trades[1].qty, dec!(0.05));
        assert_eq!(trades[1].quote_qty, dec!(357.76));
        assert_eq!(trades[1].side, -1);
        assert_eq!(trades[1].time, 1625097700000);
    }

    #[test]
    fn test_recent_trades_empty_array_deserialization() {
        let json = r#"[]"#;
        let trades: Vec<RecentTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 0);
    }

    #[test]
    fn test_recent_trade_clone() {
        let json = r#"{
            "id": "12345",
            "symbol": "BTC-240329-70000-C",
            "price": "7150.50",
            "qty": "0.10",
            "quoteQty": "715.05",
            "side": 1,
            "time": 1625097600000
        }"#;

        let trade: RecentTrade = serde_json::from_str(json).unwrap();
        let cloned = trade.clone();

        assert_eq!(trade.id, cloned.id);
        assert_eq!(trade.symbol, cloned.symbol);
        assert_eq!(trade.price, cloned.price);
        assert_eq!(trade.qty, cloned.qty);
        assert_eq!(trade.quote_qty, cloned.quote_qty);
        assert_eq!(trade.side, cloned.side);
        assert_eq!(trade.time, cloned.time);
    }

    #[test]
    fn test_recent_trade_debug() {
        let json = r#"{
            "id": "12345",
            "symbol": "BTC-240329-70000-C",
            "price": "7150.50",
            "qty": "0.10",
            "quoteQty": "715.05",
            "side": 1,
            "time": 1625097600000
        }"#;

        let trade: RecentTrade = serde_json::from_str(json).unwrap();
        let debug_output = format!("{:?}", trade);

        assert!(debug_output.contains("RecentTrade"));
        assert!(debug_output.contains("12345"));
        assert!(debug_output.contains("BTC-240329-70000-C"));
        assert!(debug_output.contains("7150.50"));
    }

    #[test]
    fn test_recent_trade_consistency_checks() {
        let json = r#"{
            "id": "12345",
            "symbol": "BTC-240329-70000-C",
            "price": "7150.50",
            "qty": "0.10",
            "quoteQty": "715.05",
            "side": 1,
            "time": 1625097600000
        }"#;

        let trade: RecentTrade = serde_json::from_str(json).unwrap();

        // Verify quote_qty is consistent with price * qty
        let expected_quote_qty = trade.price * trade.qty;
        assert_eq!(trade.quote_qty, expected_quote_qty);

        // Verify side is valid (-1 for sell, 1 for buy)
        assert!(trade.side == -1 || trade.side == 1);

        // Verify positive values for price and qty
        assert!(trade.price > dec!(0));
        assert!(trade.qty > dec!(0));
        assert!(trade.quote_qty > dec!(0));

        // Verify time is reasonable (should be a timestamp)
        assert!(trade.time > 0);
    }

    #[test]
    fn test_recent_trade_buy_sell_scenarios() {
        // Test BUY trade
        let buy_json = r#"{
            "id": "12345",
            "symbol": "BTC-240329-70000-C",
            "price": "7150.50",
            "qty": "0.10",
            "quoteQty": "715.05",
            "side": 1,
            "time": 1625097600000
        }"#;

        let buy_trade: RecentTrade = serde_json::from_str(buy_json).unwrap();
        assert_eq!(buy_trade.side, 1);
        assert!(buy_trade.price > dec!(0));
        assert!(buy_trade.qty > dec!(0));

        // Test SELL trade
        let sell_json = r#"{
            "id": "67890",
            "symbol": "BTC-240329-70000-C",
            "price": "7140.25",
            "qty": "0.05",
            "quoteQty": "357.01",
            "side": -1,
            "time": 1625097700000
        }"#;

        let sell_trade: RecentTrade = serde_json::from_str(sell_json).unwrap();
        assert_eq!(sell_trade.side, -1);
        assert!(sell_trade.price > dec!(0));
        assert!(sell_trade.qty > dec!(0));

        // Both trades should have same symbol but different sides
        assert_eq!(buy_trade.symbol, sell_trade.symbol);
        assert_ne!(buy_trade.side, sell_trade.side);
    }

    #[test]
    fn test_recent_trade_call_put_scenarios() {
        // Test CALL option trade
        let call_json = r#"{
            "id": "12345",
            "symbol": "BTC-240329-70000-C",
            "price": "7150.50",
            "qty": "0.10",
            "quoteQty": "715.05",
            "side": 1,
            "time": 1625097600000
        }"#;

        let call_trade: RecentTrade = serde_json::from_str(call_json).unwrap();
        assert!(call_trade.symbol.contains("-C"));

        // Test PUT option trade
        let put_json = r#"{
            "id": "67890",
            "symbol": "BTC-240329-70000-P",
            "price": "2950.25",
            "qty": "0.25",
            "quoteQty": "737.56",
            "side": -1,
            "time": 1625097700000
        }"#;

        let put_trade: RecentTrade = serde_json::from_str(put_json).unwrap();
        assert!(put_trade.symbol.contains("-P"));

        // Both should have same underlying and strike but different option type
        assert!(call_trade.symbol.contains("BTC-240329-70000"));
        assert!(put_trade.symbol.contains("BTC-240329-70000"));
    }

    #[test]
    fn test_recent_trade_time_ordering() {
        let json = r#"[
            {
                "id": "12345",
                "symbol": "BTC-240329-70000-C",
                "price": "7150.50",
                "qty": "0.10",
                "quoteQty": "715.05",
                "side": 1,
                "time": 1625097600000
            },
            {
                "id": "12346",
                "symbol": "BTC-240329-70000-C",
                "price": "7155.25",
                "qty": "0.05",
                "quoteQty": "357.76",
                "side": -1,
                "time": 1625097700000
            },
            {
                "id": "12347",
                "symbol": "BTC-240329-70000-C",
                "price": "7160.00",
                "qty": "0.15",
                "quoteQty": "1074.00",
                "side": 1,
                "time": 1625097800000
            }
        ]"#;

        let trades: Vec<RecentTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 3);

        // Verify trades are in time order (recent trades should be chronological)
        assert!(trades[0].time <= trades[1].time);
        assert!(trades[1].time <= trades[2].time);

        // Verify IDs are in order
        assert_eq!(trades[0].id, "12345");
        assert_eq!(trades[1].id, "12346");
        assert_eq!(trades[2].id, "12347");
    }

    #[test]
    fn test_recent_trade_volume_calculation() {
        let json = r#"{
            "id": "12345",
            "symbol": "BTC-240329-70000-C",
            "price": "7150.50",
            "qty": "0.12345678",
            "quoteQty": "882.7777053900",
            "side": 1,
            "time": 1625097600000
        }"#;

        let trade: RecentTrade = serde_json::from_str(json).unwrap();

        // Verify precise calculation: price * qty = quoteQty
        let calculated_quote_qty = trade.price * trade.qty;
        assert_eq!(trade.quote_qty, calculated_quote_qty);

        // Verify high precision is maintained
        assert_eq!(trade.price.to_string(), "7150.50");
        assert_eq!(trade.qty.to_string(), "0.12345678");
        assert_eq!(trade.quote_qty.to_string(), "882.7777053900");
    }

    #[test]
    fn test_recent_trade_string_id_variations() {
        let ids = vec!["0", "1", "12345", "999999999", "abc123", "trade_001"];

        for id in ids {
            let json = format!(
                r#"{{
                    "id": "{}",
                    "symbol": "BTC-240329-70000-C",
                    "price": "7150.50",
                    "qty": "0.10",
                    "quoteQty": "715.05",
                    "side": 1,
                    "time": 1625097600000
                }}"#,
                id
            );

            let trade: RecentTrade = serde_json::from_str(&json).unwrap();
            assert_eq!(trade.id, id);
        }
    }

    #[test]
    fn test_recent_trades_multiple_symbols() {
        let json = r#"[
            {
                "id": "12345",
                "symbol": "BTC-240329-70000-C",
                "price": "7150.50",
                "qty": "0.10",
                "quoteQty": "715.05",
                "side": 1,
                "time": 1625097600000
            },
            {
                "id": "67890",
                "symbol": "ETH-240329-3000-P",
                "price": "2950.25",
                "qty": "0.25",
                "quoteQty": "737.56",
                "side": -1,
                "time": 1625097700000
            },
            {
                "id": "11111",
                "symbol": "BNB-240329-500-C",
                "price": "450.00",
                "qty": "1.00",
                "quoteQty": "450.00",
                "side": 1,
                "time": 1625097800000
            }
        ]"#;

        let trades: Vec<RecentTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 3);

        // Verify different symbols
        assert_eq!(trades[0].symbol, "BTC-240329-70000-C");
        assert_eq!(trades[1].symbol, "ETH-240329-3000-P");
        assert_eq!(trades[2].symbol, "BNB-240329-500-C");

        // Verify different sides
        assert_eq!(trades[0].side, 1);
        assert_eq!(trades[1].side, -1);
        assert_eq!(trades[2].side, 1);
    }
}
