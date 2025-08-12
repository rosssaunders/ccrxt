use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::options::RestResult;

const HISTORICAL_TRADES_ENDPOINT: &str = "/eapi/v1/historicalTrades";

/// Request parameters for historical trades
#[derive(Debug, Clone, Serialize)]
pub struct HistoricalTradesRequest {
    /// Option trading pair, e.g BTC-200730-9000-C
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// The UniqueId ID from which to return. The latest deal record is returned by default
    #[serde(rename = "fromId", skip_serializing_if = "Option::is_none")]
    pub from_id: Option<u64>,

    /// Number of records (Default: 100, Max: 500)
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Historical trade information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalTrade {
    /// UniqueId
    #[serde(rename = "id")]
    pub id: String,

    /// TradeId
    #[serde(rename = "tradeId")]
    pub trade_id: String,

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
    /// Get old trades lookup
    ///
    /// Returns older market historical trades.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/option/market-data/Old-Trades-Lookup)
    /// Method: GET /eapi/v1/historicalTrades
    /// Weight: 20
    /// Security: MARKET_DATA
    pub async fn get_historical_trades(
        &self,
        params: HistoricalTradesRequest,
    ) -> RestResult<Vec<HistoricalTrade>> {
        self.send_get_request(HISTORICAL_TRADES_ENDPOINT, Some(params), 10)
            .await
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_historical_trades_request_serialization_basic() {
        let request = HistoricalTradesRequest {
            symbol: "BTC-240329-70000-C".to_string(),
            from_id: None,
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTC-240329-70000-C"));
        assert!(!serialized.contains("fromId"));
        assert!(!serialized.contains("limit"));
    }

    #[test]
    fn test_historical_trades_request_serialization_with_limit() {
        let request = HistoricalTradesRequest {
            symbol: "BTC-240329-70000-C".to_string(),
            from_id: None,
            limit: Some(100),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTC-240329-70000-C"));
        assert!(!serialized.contains("fromId"));
        assert!(serialized.contains("limit=100"));
    }

    #[test]
    fn test_historical_trades_request_serialization_with_from_id() {
        let request = HistoricalTradesRequest {
            symbol: "BTC-240329-70000-C".to_string(),
            from_id: Some(12345),
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTC-240329-70000-C"));
        assert!(serialized.contains("fromId=12345"));
        assert!(!serialized.contains("limit"));
    }

    #[test]
    fn test_historical_trades_request_serialization_with_all_fields() {
        let request = HistoricalTradesRequest {
            symbol: "BTC-240329-70000-C".to_string(),
            from_id: Some(12345),
            limit: Some(100),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTC-240329-70000-C"));
        assert!(serialized.contains("fromId=12345"));
        assert!(serialized.contains("limit=100"));
    }

    #[test]
    fn test_historical_trades_request_serialization_with_max_limit() {
        let request = HistoricalTradesRequest {
            symbol: "ETH-240329-3000-P".to_string(),
            from_id: None,
            limit: Some(500),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=ETH-240329-3000-P"));
        assert!(serialized.contains("limit=500"));
    }

    #[test]
    fn test_historical_trades_request_serialization_with_min_limit() {
        let request = HistoricalTradesRequest {
            symbol: "BTC-240329-70000-C".to_string(),
            from_id: None,
            limit: Some(1),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTC-240329-70000-C"));
        assert!(serialized.contains("limit=1"));
    }

    #[test]
    fn test_historical_trades_request_serialization_various_symbols() {
        let symbols = vec![
            "BTC-240329-70000-C",
            "BTC-240329-70000-P",
            "ETH-240329-3000-C",
            "ETH-240329-3000-P",
            "BNB-240329-500-C",
            "BNB-240329-500-P",
        ];

        for symbol in symbols {
            let request = HistoricalTradesRequest {
                symbol: symbol.to_string(),
                from_id: Some(12345),
                limit: Some(100),
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("symbol={}", symbol)));
            assert!(serialized.contains("fromId=12345"));
            assert!(serialized.contains("limit=100"));
        }
    }

    #[test]
    fn test_historical_trades_request_serialization_different_limits() {
        let limits = vec![1, 10, 50, 100, 250, 500];

        for limit in limits {
            let request = HistoricalTradesRequest {
                symbol: "BTC-240329-70000-C".to_string(),
                from_id: None,
                limit: Some(limit),
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("limit={}", limit)));
        }
    }

    #[test]
    fn test_historical_trades_request_serialization_different_from_ids() {
        let from_ids = vec![0, 1, 100, 12345, 999999, u64::MAX];

        for from_id in from_ids {
            let request = HistoricalTradesRequest {
                symbol: "BTC-240329-70000-C".to_string(),
                from_id: Some(from_id),
                limit: None,
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("fromId={}", from_id)));
        }
    }

    #[test]
    fn test_historical_trades_request_serialization_edge_cases() {
        // Test with very long symbol names
        let request = HistoricalTradesRequest {
            symbol: "VERY-LONG-SYMBOL-NAME-240329-999999-C".to_string(),
            from_id: Some(u64::MAX),
            limit: Some(500),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=VERY-LONG-SYMBOL-NAME-240329-999999-C"));
        assert!(serialized.contains(&format!("fromId={}", u64::MAX)));
        assert!(serialized.contains("limit=500"));
    }

    #[test]
    fn test_historical_trades_request_serialization_zero_values() {
        let request = HistoricalTradesRequest {
            symbol: "BTC-240329-70000-C".to_string(),
            from_id: Some(0),
            limit: Some(0),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTC-240329-70000-C"));
        assert!(serialized.contains("fromId=0"));
        assert!(serialized.contains("limit=0"));
    }

    #[test]
    fn test_historical_trades_request_clone() {
        let request = HistoricalTradesRequest {
            symbol: "BTC-240329-70000-C".to_string(),
            from_id: Some(12345),
            limit: Some(100),
        };

        let cloned = request.clone();
        assert_eq!(request.symbol, cloned.symbol);
        assert_eq!(request.from_id, cloned.from_id);
        assert_eq!(request.limit, cloned.limit);
    }

    #[test]
    fn test_historical_trades_request_debug() {
        let request = HistoricalTradesRequest {
            symbol: "BTC-240329-70000-C".to_string(),
            from_id: Some(12345),
            limit: Some(100),
        };

        let debug_output = format!("{:?}", request);
        assert!(debug_output.contains("HistoricalTradesRequest"));
        assert!(debug_output.contains("BTC-240329-70000-C"));
        assert!(debug_output.contains("12345"));
        assert!(debug_output.contains("100"));
    }

    #[test]
    fn test_historical_trade_deserialization() {
        let json = r#"{
            "id": "12345",
            "tradeId": "67890",
            "price": "7150.50",
            "qty": "0.10",
            "quoteQty": "715.05",
            "side": 1,
            "time": 1625097600000
        }"#;

        let trade: HistoricalTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, "12345");
        assert_eq!(trade.trade_id, "67890");
        assert_eq!(trade.price, dec!(7150.50));
        assert_eq!(trade.qty, dec!(0.10));
        assert_eq!(trade.quote_qty, dec!(715.05));
        assert_eq!(trade.side, 1);
        assert_eq!(trade.time, 1625097600000);
    }

    #[test]
    fn test_historical_trade_deserialization_sell_side() {
        let json = r#"{
            "id": "99999",
            "tradeId": "11111",
            "price": "2950.25",
            "qty": "0.25",
            "quoteQty": "737.56",
            "side": -1,
            "time": 1625097700000
        }"#;

        let trade: HistoricalTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, "99999");
        assert_eq!(trade.trade_id, "11111");
        assert_eq!(trade.price, dec!(2950.25));
        assert_eq!(trade.qty, dec!(0.25));
        assert_eq!(trade.quote_qty, dec!(737.56));
        assert_eq!(trade.side, -1);
        assert_eq!(trade.time, 1625097700000);
    }

    #[test]
    fn test_historical_trade_deserialization_high_precision() {
        let json = r#"{
            "id": "999999",
            "tradeId": "888888",
            "price": "7150.12345678",
            "qty": "0.12345678",
            "quoteQty": "883.12345678",
            "side": 1,
            "time": 1625097800000
        }"#;

        let trade: HistoricalTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.price.to_string(), "7150.12345678");
        assert_eq!(trade.qty.to_string(), "0.12345678");
        assert_eq!(trade.quote_qty.to_string(), "883.12345678");
    }

    #[test]
    fn test_historical_trade_deserialization_zero_values() {
        let json = r#"{
            "id": "0",
            "tradeId": "0",
            "price": "0.00000000",
            "qty": "0.00000000",
            "quoteQty": "0.00000000",
            "side": 0,
            "time": 0
        }"#;

        let trade: HistoricalTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, "0");
        assert_eq!(trade.trade_id, "0");
        assert_eq!(trade.price, dec!(0.00000000));
        assert_eq!(trade.qty, dec!(0.00000000));
        assert_eq!(trade.quote_qty, dec!(0.00000000));
        assert_eq!(trade.side, 0);
        assert_eq!(trade.time, 0);
    }

    #[test]
    fn test_historical_trade_deserialization_large_values() {
        let json = r#"{
            "id": "999999999999999999",
            "tradeId": "888888888888888888",
            "price": "99999.99999999",
            "qty": "999.99999999",
            "quoteQty": "99999999.99999999",
            "side": 1,
            "time": 9999999999999
        }"#;

        let trade: HistoricalTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, "999999999999999999");
        assert_eq!(trade.trade_id, "888888888888888888");
        assert_eq!(trade.price.to_string(), "99999.99999999");
        assert_eq!(trade.qty.to_string(), "999.99999999");
        assert_eq!(trade.quote_qty.to_string(), "99999999.99999999");
        assert_eq!(trade.side, 1);
        assert_eq!(trade.time, 9999999999999);
    }

    #[test]
    fn test_historical_trade_deserialization_different_id_types() {
        let test_cases = vec![
            ("0", "0"),
            ("1", "1"),
            ("12345", "67890"),
            ("999999999", "888888888"),
            ("trade_001", "trade_002"),
            ("abc123", "def456"),
        ];

        for (id, trade_id) in test_cases {
            let json = format!(
                r#"{{
                    "id": "{}",
                    "tradeId": "{}",
                    "price": "1000.00",
                    "qty": "0.10",
                    "quoteQty": "100.00",
                    "side": 1,
                    "time": 1625097600000
                }}"#,
                id, trade_id
            );

            let trade: HistoricalTrade = serde_json::from_str(&json).unwrap();
            assert_eq!(trade.id, id);
            assert_eq!(trade.trade_id, trade_id);
        }
    }

    #[test]
    fn test_historical_trade_deserialization_side_values() {
        let sides = vec![-1, 0, 1];

        for side in sides {
            let json = format!(
                r#"{{
                    "id": "12345",
                    "tradeId": "67890",
                    "price": "7150.50",
                    "qty": "0.10",
                    "quoteQty": "715.05",
                    "side": {},
                    "time": 1625097600000
                }}"#,
                side
            );

            let trade: HistoricalTrade = serde_json::from_str(&json).unwrap();
            assert_eq!(trade.side, side);
        }
    }

    #[test]
    fn test_historical_trades_array_deserialization() {
        let json = r#"[
            {
                "id": "12345",
                "tradeId": "67890",
                "price": "7150.50",
                "qty": "0.10",
                "quoteQty": "715.05",
                "side": 1,
                "time": 1625097600000
            },
            {
                "id": "12346",
                "tradeId": "67891",
                "price": "7155.25",
                "qty": "0.05",
                "quoteQty": "357.76",
                "side": -1,
                "time": 1625097700000
            }
        ]"#;

        let trades: Vec<HistoricalTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 2);

        // First trade (buy)
        assert_eq!(trades[0].id, "12345");
        assert_eq!(trades[0].trade_id, "67890");
        assert_eq!(trades[0].price, dec!(7150.50));
        assert_eq!(trades[0].qty, dec!(0.10));
        assert_eq!(trades[0].quote_qty, dec!(715.05));
        assert_eq!(trades[0].side, 1);
        assert_eq!(trades[0].time, 1625097600000);

        // Second trade (sell)
        assert_eq!(trades[1].id, "12346");
        assert_eq!(trades[1].trade_id, "67891");
        assert_eq!(trades[1].price, dec!(7155.25));
        assert_eq!(trades[1].qty, dec!(0.05));
        assert_eq!(trades[1].quote_qty, dec!(357.76));
        assert_eq!(trades[1].side, -1);
        assert_eq!(trades[1].time, 1625097700000);
    }

    #[test]
    fn test_historical_trades_empty_array_deserialization() {
        let json = r#"[]"#;
        let trades: Vec<HistoricalTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 0);
    }

    #[test]
    fn test_historical_trades_single_trade_array() {
        let json = r#"[
            {
                "id": "12345",
                "tradeId": "67890",
                "price": "7150.50",
                "qty": "0.10",
                "quoteQty": "715.05",
                "side": 1,
                "time": 1625097600000
            }
        ]"#;

        let trades: Vec<HistoricalTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].id, "12345");
        assert_eq!(trades[0].trade_id, "67890");
        assert_eq!(trades[0].price, dec!(7150.50));
    }

    #[test]
    fn test_historical_trade_clone() {
        let json = r#"{
            "id": "12345",
            "tradeId": "67890",
            "price": "7150.50",
            "qty": "0.10",
            "quoteQty": "715.05",
            "side": 1,
            "time": 1625097600000
        }"#;

        let trade: HistoricalTrade = serde_json::from_str(json).unwrap();
        let cloned = trade.clone();

        assert_eq!(trade.id, cloned.id);
        assert_eq!(trade.trade_id, cloned.trade_id);
        assert_eq!(trade.price, cloned.price);
        assert_eq!(trade.qty, cloned.qty);
        assert_eq!(trade.quote_qty, cloned.quote_qty);
        assert_eq!(trade.side, cloned.side);
        assert_eq!(trade.time, cloned.time);
    }

    #[test]
    fn test_historical_trade_debug() {
        let json = r#"{
            "id": "12345",
            "tradeId": "67890",
            "price": "7150.50",
            "qty": "0.10",
            "quoteQty": "715.05",
            "side": 1,
            "time": 1625097600000
        }"#;

        let trade: HistoricalTrade = serde_json::from_str(json).unwrap();
        let debug_output = format!("{:?}", trade);

        assert!(debug_output.contains("HistoricalTrade"));
        assert!(debug_output.contains("12345"));
        assert!(debug_output.contains("67890"));
        assert!(debug_output.contains("7150.50"));
    }

    #[test]
    fn test_historical_trade_consistency_checks() {
        let json = r#"{
            "id": "12345",
            "tradeId": "67890",
            "price": "7150.50",
            "qty": "0.10",
            "quoteQty": "715.05",
            "side": 1,
            "time": 1625097600000
        }"#;

        let trade: HistoricalTrade = serde_json::from_str(json).unwrap();

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
    fn test_historical_trade_buy_sell_scenarios() {
        // Test BUY trade
        let buy_json = r#"{
            "id": "12345",
            "tradeId": "67890",
            "price": "7150.50",
            "qty": "0.10",
            "quoteQty": "715.05",
            "side": 1,
            "time": 1625097600000
        }"#;

        let buy_trade: HistoricalTrade = serde_json::from_str(buy_json).unwrap();
        assert_eq!(buy_trade.side, 1);
        assert!(buy_trade.price > dec!(0));
        assert!(buy_trade.qty > dec!(0));

        // Test SELL trade
        let sell_json = r#"{
            "id": "12346",
            "tradeId": "67891",
            "price": "7140.25",
            "qty": "0.05",
            "quoteQty": "357.01",
            "side": -1,
            "time": 1625097700000
        }"#;

        let sell_trade: HistoricalTrade = serde_json::from_str(sell_json).unwrap();
        assert_eq!(sell_trade.side, -1);
        assert!(sell_trade.price > dec!(0));
        assert!(sell_trade.qty > dec!(0));

        // Both trades should have different sides
        assert_ne!(buy_trade.side, sell_trade.side);
    }

    #[test]
    fn test_historical_trade_time_ordering() {
        let json = r#"[
            {
                "id": "12345",
                "tradeId": "67890",
                "price": "7150.50",
                "qty": "0.10",
                "quoteQty": "715.05",
                "side": 1,
                "time": 1625097600000
            },
            {
                "id": "12346",
                "tradeId": "67891",
                "price": "7155.25",
                "qty": "0.05",
                "quoteQty": "357.76",
                "side": -1,
                "time": 1625097700000
            },
            {
                "id": "12347",
                "tradeId": "67892",
                "price": "7160.00",
                "qty": "0.15",
                "quoteQty": "1074.00",
                "side": 1,
                "time": 1625097800000
            }
        ]"#;

        let trades: Vec<HistoricalTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 3);

        // Verify trades are in time order (historical trades should be chronological)
        assert!(trades[0].time <= trades[1].time);
        assert!(trades[1].time <= trades[2].time);

        // Verify IDs are in order
        assert_eq!(trades[0].id, "12345");
        assert_eq!(trades[1].id, "12346");
        assert_eq!(trades[2].id, "12347");
    }

    #[test]
    fn test_historical_trade_volume_calculation() {
        let json = r#"{
            "id": "12345",
            "tradeId": "67890",
            "price": "7150.50",
            "qty": "0.12345678",
            "quoteQty": "882.7777053900",
            "side": 1,
            "time": 1625097600000
        }"#;

        let trade: HistoricalTrade = serde_json::from_str(json).unwrap();

        // Verify precise calculation: price * qty = quoteQty
        let calculated_quote_qty = trade.price * trade.qty;
        assert_eq!(trade.quote_qty, calculated_quote_qty);

        // Verify high precision is maintained
        assert_eq!(trade.price.to_string(), "7150.50");
        assert_eq!(trade.qty.to_string(), "0.12345678");
        assert_eq!(trade.quote_qty.to_string(), "882.7777053900");
    }

    #[test]
    fn test_historical_trades_different_options() {
        let json = r#"[
            {
                "id": "12345",
                "tradeId": "67890",
                "price": "7150.50",
                "qty": "0.10",
                "quoteQty": "715.05",
                "side": 1,
                "time": 1625097600000
            },
            {
                "id": "12346",
                "tradeId": "67891",
                "price": "2950.25",
                "qty": "0.25",
                "quoteQty": "737.56",
                "side": -1,
                "time": 1625097700000
            },
            {
                "id": "12347",
                "tradeId": "67892",
                "price": "450.00",
                "qty": "1.00",
                "quoteQty": "450.00",
                "side": 1,
                "time": 1625097800000
            }
        ]"#;

        let trades: Vec<HistoricalTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 3);

        // Verify different prices and quantities
        assert_eq!(trades[0].price, dec!(7150.50));
        assert_eq!(trades[1].price, dec!(2950.25));
        assert_eq!(trades[2].price, dec!(450.00));

        // Verify different sides
        assert_eq!(trades[0].side, 1);
        assert_eq!(trades[1].side, -1);
        assert_eq!(trades[2].side, 1);
    }

    #[test]
    fn test_historical_trade_id_uniqueness() {
        let json = r#"[
            {
                "id": "12345",
                "tradeId": "67890",
                "price": "7150.50",
                "qty": "0.10",
                "quoteQty": "715.05",
                "side": 1,
                "time": 1625097600000
            },
            {
                "id": "12346",
                "tradeId": "67891",
                "price": "7155.25",
                "qty": "0.05",
                "quoteQty": "357.76",
                "side": -1,
                "time": 1625097700000
            }
        ]"#;

        let trades: Vec<HistoricalTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 2);

        // Verify different IDs
        assert_ne!(trades[0].id, trades[1].id);
        assert_ne!(trades[0].trade_id, trades[1].trade_id);

        // Verify unique IDs
        assert_eq!(trades[0].id, "12345");
        assert_eq!(trades[0].trade_id, "67890");
        assert_eq!(trades[1].id, "12346");
        assert_eq!(trades[1].trade_id, "67891");
    }

    #[test]
    fn test_historical_trade_large_dataset() {
        let mut json_array = Vec::new();
        json_array.push("[".to_string());

        for i in 0..100 {
            let trade_json = format!(
                r#"{{
                    "id": "{}",
                    "tradeId": "{}",
                    "price": "7150.50",
                    "qty": "0.10",
                    "quoteQty": "715.05",
                    "side": {},
                    "time": {}
                }}"#,
                i,
                i + 1000000,
                if i % 2 == 0 { 1 } else { -1 },
                1625097600000u64 + i * 1000
            );
            json_array.push(trade_json);
            if i < 99 {
                json_array.push(",".to_string());
            }
        }
        json_array.push("]".to_string());

        let json = json_array.join("\n");
        let trades: Vec<HistoricalTrade> = serde_json::from_str(&json).unwrap();
        assert_eq!(trades.len(), 100);

        // Verify first and last trades
        assert_eq!(trades[0].id, "0");
        assert_eq!(trades[0].trade_id, "1000000");
        assert_eq!(trades[0].side, 1);
        assert_eq!(trades[0].time, 1625097600000);

        assert_eq!(trades[99].id, "99");
        assert_eq!(trades[99].trade_id, "1000099");
        assert_eq!(trades[99].side, -1);
        assert_eq!(trades[99].time, 1625097600000u64 + 99 * 1000);
    }

    #[test]
    fn test_historical_trade_edge_case_precision() {
        let json = r#"{
            "id": "edge_case_001",
            "tradeId": "edge_case_002",
            "price": "0.00000001",
            "qty": "100000000.00000000",
            "quoteQty": "1.00000000",
            "side": 1,
            "time": 1625097600000
        }"#;

        let trade: HistoricalTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.price.to_string(), "0.00000001");
        assert_eq!(trade.qty.to_string(), "100000000.00000000");
        assert_eq!(trade.quote_qty.to_string(), "1.00000000");

        // Verify calculation is consistent
        let calculated_quote_qty = trade.price * trade.qty;
        assert_eq!(trade.quote_qty, calculated_quote_qty);
    }

    #[test]
    fn test_historical_trade_round_trip_serialization() {
        let original_json = r#"{
            "id": "12345",
            "tradeId": "67890",
            "price": "7150.50",
            "qty": "0.10",
            "quoteQty": "715.05",
            "side": 1,
            "time": 1625097600000
        }"#;

        // Deserialize
        let trade: HistoricalTrade = serde_json::from_str(original_json).unwrap();

        // Serialize back
        let serialized = serde_json::to_string(&trade).unwrap();

        // Deserialize again
        let trade2: HistoricalTrade = serde_json::from_str(&serialized).unwrap();

        // Verify they match
        assert_eq!(trade.id, trade2.id);
        assert_eq!(trade.trade_id, trade2.trade_id);
        assert_eq!(trade.price, trade2.price);
        assert_eq!(trade.qty, trade2.qty);
        assert_eq!(trade.quote_qty, trade2.quote_qty);
        assert_eq!(trade.side, trade2.side);
        assert_eq!(trade.time, trade2.time);
    }
}
