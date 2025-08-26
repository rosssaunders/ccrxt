use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::gateio::RestResult;

/// Request parameters for retrieving trades
#[derive(Debug, Clone, Serialize, Default)]
pub struct TradesRequest {
    /// Currency pair to query trades for
    pub currency_pair: String,

    /// Maximum number of trades to return (default: 100, max: 1000)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Page number for pagination (starts from 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// Start time for trade range (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End time for trade range (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
}

/// Trade information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    /// Trade ID
    pub id: String,

    /// Trading time (Unix timestamp in seconds)
    pub create_time: String,

    /// Trading time (Unix timestamp in milliseconds)
    pub create_time_ms: String,

    /// Currency pair
    pub currency_pair: String,

    /// Trade side (buy/sell)
    pub side: String,

    /// Trade role (taker/maker)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,

    /// Trade amount
    pub amount: String,

    /// Trade price
    pub price: String,

    /// Order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    /// Trade fee
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,

    /// Fee currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_currency: Option<String>,

    /// Point fee
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_fee: Option<String>,

    /// GT fee
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gt_fee: Option<String>,
}

impl RestClient {
    /// Retrieve recent trades for a currency pair
    ///
    /// This endpoint returns recent trades for the specified currency pair.
    /// You can filter by time range and limit the number of results.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#retrieve-market-trades)
    pub async fn get_trades(&self, params: TradesRequest) -> RestResult<Vec<Trade>> {
        self.get_with_query("/spot/trades", Some(&params)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trades_request_minimal_serialization() {
        let request = TradesRequest {
            currency_pair: "BTC_USDT".to_string(),
            limit: None,
            page: None,
            from: None,
            to: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "currency_pair=BTC_USDT");
    }

    #[test]
    fn test_trades_request_with_limit() {
        let request = TradesRequest {
            currency_pair: "ETH_USDT".to_string(),
            limit: Some(100),
            page: None,
            from: None,
            to: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("currency_pair=ETH_USDT"));
        assert!(serialized.contains("limit=100"));
        assert!(!serialized.contains("page="));
        assert!(!serialized.contains("from="));
        assert!(!serialized.contains("to="));
    }

    #[test]
    fn test_trades_request_with_pagination() {
        let request = TradesRequest {
            currency_pair: "BNB_USDT".to_string(),
            limit: Some(50),
            page: Some(2),
            from: None,
            to: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("currency_pair=BNB_USDT"));
        assert!(serialized.contains("limit=50"));
        assert!(serialized.contains("page=2"));
    }

    #[test]
    fn test_trades_request_with_time_range() {
        let request = TradesRequest {
            currency_pair: "SOL_USDT".to_string(),
            limit: None,
            page: None,
            from: Some(1640995200),
            to: Some(1641081600),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("currency_pair=SOL_USDT"));
        assert!(serialized.contains("from=1640995200"));
        assert!(serialized.contains("to=1641081600"));
    }

    #[test]
    fn test_trades_request_full_parameters() {
        let request = TradesRequest {
            currency_pair: "ADA_USDT".to_string(),
            limit: Some(100),
            page: Some(1),
            from: Some(1640995200),
            to: Some(1641081600),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("currency_pair=ADA_USDT"));
        assert!(serialized.contains("limit=100"));
        assert!(serialized.contains("page=1"));
        assert!(serialized.contains("from=1640995200"));
        assert!(serialized.contains("to=1641081600"));
    }

    #[test]
    fn test_trades_request_limit_edge_cases() {
        let limits = vec![1, 100, 500, 1000];

        for limit in limits {
            let request = TradesRequest {
                currency_pair: "BTC_USDT".to_string(),
                limit: Some(limit),
                page: None,
                from: None,
                to: None,
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("limit={}", limit)));
        }
    }

    #[test]
    fn test_trades_request_max_values() {
        let request = TradesRequest {
            currency_pair: "BTC_USDT".to_string(),
            limit: Some(u32::MAX),
            page: Some(u32::MAX),
            from: Some(i64::MIN),
            to: Some(i64::MAX),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains(&format!("limit={}", u32::MAX)));
        assert!(serialized.contains(&format!("page={}", u32::MAX)));
        assert!(serialized.contains(&format!("from={}", i64::MIN)));
        assert!(serialized.contains(&format!("to={}", i64::MAX)));
    }

    #[test]
    fn test_trades_request_negative_timestamps() {
        let request = TradesRequest {
            currency_pair: "BTC_USDT".to_string(),
            limit: None,
            page: None,
            from: Some(-1640995200),
            to: Some(-1640908800),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("from=-1640995200"));
        assert!(serialized.contains("to=-1640908800"));
    }

    #[test]
    fn test_trades_request_different_pairs() {
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
            let request = TradesRequest {
                currency_pair: pair.to_string(),
                limit: None,
                page: None,
                from: None,
                to: None,
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert_eq!(serialized, format!("currency_pair={}", pair));
        }
    }

    #[test]
    fn test_trades_request_default() {
        let request = TradesRequest::default();
        assert_eq!(request.currency_pair, "");
        assert_eq!(request.limit, None);
        assert_eq!(request.page, None);
        assert_eq!(request.from, None);
        assert_eq!(request.to, None);
    }

    #[test]
    fn test_trade_full_deserialization() {
        let json = r#"{
            "id": "123456789",
            "create_time": "1640995200",
            "create_time_ms": "1640995200123",
            "currency_pair": "BTC_USDT",
            "side": "buy",
            "role": "taker",
            "amount": "0.5",
            "price": "48500.50",
            "order_id": "987654321",
            "fee": "0.001",
            "fee_currency": "BTC",
            "point_fee": "0",
            "gt_fee": "0"
        }"#;

        let trade: Trade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, "123456789");
        assert_eq!(trade.create_time, "1640995200");
        assert_eq!(trade.create_time_ms, "1640995200123");
        assert_eq!(trade.currency_pair, "BTC_USDT");
        assert_eq!(trade.side, "buy");
        assert_eq!(trade.role, Some("taker".to_string()));
        assert_eq!(trade.amount, "0.5");
        assert_eq!(trade.price, "48500.50");
        assert_eq!(trade.order_id, Some("987654321".to_string()));
        assert_eq!(trade.fee, Some("0.001".to_string()));
        assert_eq!(trade.fee_currency, Some("BTC".to_string()));
        assert_eq!(trade.point_fee, Some("0".to_string()));
        assert_eq!(trade.gt_fee, Some("0".to_string()));
    }

    #[test]
    fn test_trade_minimal_deserialization() {
        let json = r#"{
            "id": "123456789",
            "create_time": "1640995200",
            "create_time_ms": "1640995200000",
            "currency_pair": "ETH_USDT",
            "side": "sell",
            "amount": "2.0",
            "price": "3000.00"
        }"#;

        let trade: Trade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, "123456789");
        assert_eq!(trade.create_time, "1640995200");
        assert_eq!(trade.create_time_ms, "1640995200000");
        assert_eq!(trade.currency_pair, "ETH_USDT");
        assert_eq!(trade.side, "sell");
        assert_eq!(trade.role, None);
        assert_eq!(trade.amount, "2.0");
        assert_eq!(trade.price, "3000.00");
        assert_eq!(trade.order_id, None);
        assert_eq!(trade.fee, None);
        assert_eq!(trade.fee_currency, None);
        assert_eq!(trade.point_fee, None);
        assert_eq!(trade.gt_fee, None);
    }

    #[test]
    fn test_trade_buy_side() {
        let json = r#"{
            "id": "111111111",
            "create_time": "1640995300",
            "create_time_ms": "1640995300000",
            "currency_pair": "BNB_USDT",
            "side": "buy",
            "role": "maker",
            "amount": "10.0",
            "price": "400.00",
            "fee": "0.0005",
            "fee_currency": "BNB"
        }"#;

        let trade: Trade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.side, "buy");
        assert_eq!(trade.role, Some("maker".to_string()));
        assert_eq!(trade.fee, Some("0.0005".to_string())); // Maker fee
    }

    #[test]
    fn test_trade_sell_side() {
        let json = r#"{
            "id": "222222222",
            "create_time": "1640995400",
            "create_time_ms": "1640995400000",
            "currency_pair": "SOL_USDT",
            "side": "sell",
            "role": "taker",
            "amount": "5.0",
            "price": "150.00",
            "fee": "0.001",
            "fee_currency": "USDT"
        }"#;

        let trade: Trade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.side, "sell");
        assert_eq!(trade.role, Some("taker".to_string()));
        assert_eq!(trade.fee, Some("0.001".to_string())); // Taker fee
        assert_eq!(trade.fee_currency, Some("USDT".to_string())); // Fee in quote currency for sell
    }

    #[test]
    fn test_trade_with_point_fee() {
        let json = r#"{
            "id": "333333333",
            "create_time": "1640995500",
            "create_time_ms": "1640995500000",
            "currency_pair": "ADA_USDT",
            "side": "buy",
            "amount": "100.0",
            "price": "1.25",
            "point_fee": "0.5",
            "gt_fee": "0"
        }"#;

        let trade: Trade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.point_fee, Some("0.5".to_string()));
        assert_eq!(trade.gt_fee, Some("0".to_string()));
    }

    #[test]
    fn test_trade_with_gt_fee() {
        let json = r#"{
            "id": "444444444",
            "create_time": "1640995600",
            "create_time_ms": "1640995600000",
            "currency_pair": "DOT_USDT",
            "side": "sell",
            "amount": "20.0",
            "price": "25.00",
            "point_fee": "0",
            "gt_fee": "0.2"
        }"#;

        let trade: Trade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.point_fee, Some("0".to_string()));
        assert_eq!(trade.gt_fee, Some("0.2".to_string()));
    }

    #[test]
    fn test_trade_large_amounts() {
        let json = r#"{
            "id": "555555555",
            "create_time": "1640995700",
            "create_time_ms": "1640995700000",
            "currency_pair": "BTC_USDT",
            "side": "buy",
            "amount": "999999.99999999",
            "price": "999999.99999999",
            "fee": "999.99999999",
            "fee_currency": "BTC"
        }"#;

        let trade: Trade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.amount, "999999.99999999");
        assert_eq!(trade.price, "999999.99999999");
        assert_eq!(trade.fee, Some("999.99999999".to_string()));
    }

    #[test]
    fn test_trade_small_amounts() {
        let json = r#"{
            "id": "666666666",
            "create_time": "1640995800",
            "create_time_ms": "1640995800000",
            "currency_pair": "SHIB_USDT",
            "side": "buy",
            "amount": "0.00000001",
            "price": "0.00000001",
            "fee": "0.00000000001",
            "fee_currency": "SHIB"
        }"#;

        let trade: Trade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.amount, "0.00000001");
        assert_eq!(trade.price, "0.00000001");
        assert_eq!(trade.fee, Some("0.00000000001".to_string()));
    }

    #[test]
    fn test_trade_zero_values() {
        let json = r#"{
            "id": "777777777",
            "create_time": "0",
            "create_time_ms": "0",
            "currency_pair": "TEST_USDT",
            "side": "buy",
            "amount": "0",
            "price": "0",
            "fee": "0",
            "fee_currency": "TEST",
            "point_fee": "0",
            "gt_fee": "0"
        }"#;

        let trade: Trade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.create_time, "0");
        assert_eq!(trade.create_time_ms, "0");
        assert_eq!(trade.amount, "0");
        assert_eq!(trade.price, "0");
        assert_eq!(trade.fee, Some("0".to_string()));
        assert_eq!(trade.point_fee, Some("0".to_string()));
        assert_eq!(trade.gt_fee, Some("0".to_string()));
    }

    #[test]
    fn test_trade_array_deserialization() {
        let json = r#"[
            {
                "id": "1",
                "create_time": "1640995200",
                "create_time_ms": "1640995200000",
                "currency_pair": "BTC_USDT",
                "side": "buy",
                "amount": "0.1",
                "price": "48000.00"
            },
            {
                "id": "2",
                "create_time": "1640995300",
                "create_time_ms": "1640995300000",
                "currency_pair": "BTC_USDT",
                "side": "sell",
                "amount": "0.2",
                "price": "48100.00"
            },
            {
                "id": "3",
                "create_time": "1640995400",
                "create_time_ms": "1640995400000",
                "currency_pair": "BTC_USDT",
                "side": "buy",
                "amount": "0.3",
                "price": "48200.00"
            }
        ]"#;

        let trades: Vec<Trade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 3);

        assert_eq!(trades[0].id, "1");
        assert_eq!(trades[0].side, "buy");
        assert_eq!(trades[0].price, "48000.00");

        assert_eq!(trades[1].id, "2");
        assert_eq!(trades[1].side, "sell");
        assert_eq!(trades[1].price, "48100.00");

        assert_eq!(trades[2].id, "3");
        assert_eq!(trades[2].side, "buy");
        assert_eq!(trades[2].price, "48200.00");
    }

    #[test]
    fn test_trade_empty_array_deserialization() {
        let json = r#"[]"#;
        let trades: Vec<Trade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 0);
    }

    #[test]
    fn test_trade_serialization() {
        let trade = Trade {
            id: "123456789".to_string(),
            create_time: "1640995200".to_string(),
            create_time_ms: "1640995200123".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            side: "buy".to_string(),
            role: Some("taker".to_string()),
            amount: "0.5".to_string(),
            price: "48500.50".to_string(),
            order_id: Some("987654321".to_string()),
            fee: Some("0.001".to_string()),
            fee_currency: Some("BTC".to_string()),
            point_fee: Some("0".to_string()),
            gt_fee: Some("0".to_string()),
        };

        let json = serde_json::to_value(&trade).unwrap();
        assert_eq!(json["id"], "123456789");
        assert_eq!(json["create_time"], "1640995200");
        assert_eq!(json["create_time_ms"], "1640995200123");
        assert_eq!(json["currency_pair"], "BTC_USDT");
        assert_eq!(json["side"], "buy");
        assert_eq!(json["role"], "taker");
        assert_eq!(json["amount"], "0.5");
        assert_eq!(json["price"], "48500.50");
        assert_eq!(json["order_id"], "987654321");
        assert_eq!(json["fee"], "0.001");
        assert_eq!(json["fee_currency"], "BTC");
        assert_eq!(json["point_fee"], "0");
        assert_eq!(json["gt_fee"], "0");
    }

    #[test]
    fn test_trade_serialization_with_none_fields() {
        let trade = Trade {
            id: "123456789".to_string(),
            create_time: "1640995200".to_string(),
            create_time_ms: "1640995200000".to_string(),
            currency_pair: "ETH_USDT".to_string(),
            side: "sell".to_string(),
            role: None,
            amount: "2.0".to_string(),
            price: "3000.00".to_string(),
            order_id: None,
            fee: None,
            fee_currency: None,
            point_fee: None,
            gt_fee: None,
        };

        let json = serde_json::to_string(&trade).unwrap();
        assert!(!json.contains("\"role\":"));
        assert!(!json.contains("\"order_id\":"));
        assert!(!json.contains("\"fee\":"));
        assert!(!json.contains("\"fee_currency\":"));
        assert!(!json.contains("\"point_fee\":"));
        assert!(!json.contains("\"gt_fee\":"));
    }

    #[test]
    fn test_trade_round_trip() {
        let original = Trade {
            id: "999999999".to_string(),
            create_time: "1640995200".to_string(),
            create_time_ms: "1640995200999".to_string(),
            currency_pair: "MATIC_USDT".to_string(),
            side: "buy".to_string(),
            role: Some("maker".to_string()),
            amount: "100.0".to_string(),
            price: "2.50".to_string(),
            order_id: Some("888888888".to_string()),
            fee: Some("0.05".to_string()),
            fee_currency: Some("MATIC".to_string()),
            point_fee: Some("0.1".to_string()),
            gt_fee: Some("0.05".to_string()),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Trade = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, original.id);
        assert_eq!(deserialized.create_time, original.create_time);
        assert_eq!(deserialized.create_time_ms, original.create_time_ms);
        assert_eq!(deserialized.currency_pair, original.currency_pair);
        assert_eq!(deserialized.side, original.side);
        assert_eq!(deserialized.role, original.role);
        assert_eq!(deserialized.amount, original.amount);
        assert_eq!(deserialized.price, original.price);
        assert_eq!(deserialized.order_id, original.order_id);
        assert_eq!(deserialized.fee, original.fee);
        assert_eq!(deserialized.fee_currency, original.fee_currency);
        assert_eq!(deserialized.point_fee, original.point_fee);
        assert_eq!(deserialized.gt_fee, original.gt_fee);
    }

    #[test]
    fn test_trade_realistic_btc_scenario() {
        let json = r#"{
            "id": "1234567890123",
            "create_time": "1640995200",
            "create_time_ms": "1640995200456",
            "currency_pair": "BTC_USDT",
            "side": "buy",
            "role": "taker",
            "amount": "0.12345678",
            "price": "47234.56",
            "order_id": "9876543210987",
            "fee": "0.00012346",
            "fee_currency": "BTC",
            "point_fee": "0",
            "gt_fee": "0"
        }"#;

        let trade: Trade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.currency_pair, "BTC_USDT");
        assert_eq!(trade.side, "buy");
        assert_eq!(trade.amount, "0.12345678"); // 8 decimal places for BTC
        assert_eq!(trade.price, "47234.56"); // 2 decimal places for USDT price

        // Calculate trade value
        let amount: f64 = trade.amount.parse().unwrap();
        let price: f64 = trade.price.parse().unwrap();
        let trade_value = amount * price;
        assert!(trade_value > 5000.0 && trade_value < 6000.0); // ~$5,832

        // Fee should be ~0.1% of amount for taker
        let fee: f64 = trade.fee.unwrap().parse().unwrap();
        let fee_percentage = fee / amount * 100.0;
        assert!(fee_percentage > 0.09 && fee_percentage < 0.11); // ~0.1%
    }

    #[test]
    fn test_trade_stablecoin_pair() {
        let json = r#"{
            "id": "2222222222222",
            "create_time": "1640995300",
            "create_time_ms": "1640995300789",
            "currency_pair": "USDC_USDT",
            "side": "sell",
            "role": "maker",
            "amount": "10000.000000",
            "price": "0.9999",
            "order_id": "8888888888888",
            "fee": "0.5",
            "fee_currency": "USDT",
            "point_fee": "0",
            "gt_fee": "0"
        }"#;

        let trade: Trade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.currency_pair, "USDC_USDT");
        assert_eq!(trade.price, "0.9999"); // Tight spread for stablecoins
        assert_eq!(trade.amount, "10000.000000"); // 6 decimal places
        assert_eq!(trade.fee, Some("0.5".to_string())); // 0.05% maker fee on 10000 USDT
    }

    #[test]
    fn test_trade_clone() {
        let original = Trade {
            id: "123456789".to_string(),
            create_time: "1640995200".to_string(),
            create_time_ms: "1640995200123".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            side: "buy".to_string(),
            role: Some("taker".to_string()),
            amount: "0.5".to_string(),
            price: "48500.50".to_string(),
            order_id: Some("987654321".to_string()),
            fee: Some("0.001".to_string()),
            fee_currency: Some("BTC".to_string()),
            point_fee: Some("0".to_string()),
            gt_fee: Some("0".to_string()),
        };

        let cloned = original.clone();
        assert_eq!(cloned.id, original.id);
        assert_eq!(cloned.create_time, original.create_time);
        assert_eq!(cloned.create_time_ms, original.create_time_ms);
        assert_eq!(cloned.currency_pair, original.currency_pair);
        assert_eq!(cloned.side, original.side);
        assert_eq!(cloned.role, original.role);
        assert_eq!(cloned.amount, original.amount);
        assert_eq!(cloned.price, original.price);
        assert_eq!(cloned.order_id, original.order_id);
        assert_eq!(cloned.fee, original.fee);
        assert_eq!(cloned.fee_currency, original.fee_currency);
        assert_eq!(cloned.point_fee, original.point_fee);
        assert_eq!(cloned.gt_fee, original.gt_fee);
    }

    #[test]
    fn test_trade_debug() {
        let trade = Trade {
            id: "123456789".to_string(),
            create_time: "1640995200".to_string(),
            create_time_ms: "1640995200123".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            side: "buy".to_string(),
            role: Some("taker".to_string()),
            amount: "0.5".to_string(),
            price: "48500.50".to_string(),
            order_id: Some("987654321".to_string()),
            fee: Some("0.001".to_string()),
            fee_currency: Some("BTC".to_string()),
            point_fee: Some("0".to_string()),
            gt_fee: Some("0".to_string()),
        };

        let debug_str = format!("{:?}", trade);
        assert!(debug_str.contains("Trade"));
        assert!(debug_str.contains("123456789"));
        assert!(debug_str.contains("BTC_USDT"));
        assert!(debug_str.contains("48500.50"));
    }
}
