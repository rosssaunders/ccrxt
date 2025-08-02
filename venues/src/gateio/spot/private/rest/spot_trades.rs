//! Trading statistics and trade history functionality
use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for getting personal trading history
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetMyTradesRequest {
    /// Currency pair
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,
    /// Limit the number of records
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Page number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// Account type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Start timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    /// End timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
}

/// Personal trade information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyTrade {
    /// Trade ID
    pub id: String,
    /// Trading time
    pub create_time: String,
    /// Trading time in milliseconds
    pub create_time_ms: String,
    /// Currency pair
    pub currency_pair: String,
    /// Order ID
    pub order_id: String,
    /// Trade side
    pub side: String,
    /// Trade role (taker/maker)
    pub role: String,
    /// Trade amount
    pub amount: String,
    /// Trade price
    pub price: String,
    /// Trade fee
    pub fee: String,
    /// Fee currency
    pub fee_currency: String,
    /// Point fee
    pub point_fee: String,
    /// GT fee
    pub gt_fee: String,
    /// Whether GT fee is used
    pub gt_fee_deduction: bool,
    /// Rebated fee
    pub rebated_fee: String,
    /// Rebated fee currency
    pub rebated_fee_currency: String,
    /// Text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// Implementation for the client
impl RestClient {
    /// Get personal trading history
    ///
    /// This endpoint returns your personal trading history.
    /// You can filter by currency pair, time range, and other parameters.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#list-personal-trading-history>
    pub async fn get_my_trades(
        &self,
        request: GetMyTradesRequest,
    ) -> crate::gateio::spot::Result<Vec<MyTrade>> {
        self.get_with_query("/spot/my_trades", &request).await
    }

    /// Get all personal trades for a currency pair
    pub async fn get_my_trades_for_pair(
        &self,
        currency_pair: &str,
        limit: Option<u32>,
    ) -> crate::gateio::spot::Result<Vec<MyTrade>> {
        let request = GetMyTradesRequest {
            currency_pair: Some(currency_pair.to_string()),
            limit,
            ..Default::default()
        };
        self.get_my_trades(request).await
    }

    /// Get trades for a specific order
    pub async fn get_order_trades(
        &self,
        order_id: &str,
        currency_pair: &str,
    ) -> crate::gateio::spot::Result<Vec<MyTrade>> {
        let request = GetMyTradesRequest {
            currency_pair: Some(currency_pair.to_string()),
            order_id: Some(order_id.to_string()),
            ..Default::default()
        };
        self.get_my_trades(request).await
    }

    /// Get trades within a time range
    pub async fn get_my_trades_in_range(
        &self,
        currency_pair: Option<&str>,
        from: i64,
        to: i64,
        limit: Option<u32>,
    ) -> crate::gateio::spot::Result<Vec<MyTrade>> {
        let request = GetMyTradesRequest {
            currency_pair: currency_pair.map(|s| s.to_string()),
            from: Some(from),
            to: Some(to),
            limit,
            ..Default::default()
        };
        self.get_my_trades(request).await
    }

    /// Get recent trades (last 24 hours)
    pub async fn get_recent_my_trades(
        &self,
        currency_pair: Option<&str>,
        limit: Option<u32>,
    ) -> crate::gateio::spot::Result<Vec<MyTrade>> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        let yesterday = now - 86400; // 24 hours ago

        self.get_my_trades_in_range(currency_pair, yesterday, now, limit)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_my_trades_request_minimal_serialization() {
        let request = GetMyTradesRequest::default();

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_get_my_trades_request_with_currency_pair() {
        let request = GetMyTradesRequest {
            currency_pair: Some("BTC_USDT".to_string()),
            ..Default::default()
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "currency_pair=BTC_USDT");
    }

    #[test]
    fn test_get_my_trades_request_with_pagination() {
        let request = GetMyTradesRequest {
            limit: Some(50),
            page: Some(2),
            ..Default::default()
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("limit=50"));
        assert!(serialized.contains("page=2"));
    }

    #[test]
    fn test_get_my_trades_request_with_order_id() {
        let request = GetMyTradesRequest {
            order_id: Some("12345678".to_string()),
            ..Default::default()
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "order_id=12345678");
    }

    #[test]
    fn test_get_my_trades_request_with_account() {
        let request = GetMyTradesRequest {
            account: Some("spot".to_string()),
            ..Default::default()
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "account=spot");
    }

    #[test]
    fn test_get_my_trades_request_with_time_range() {
        let request = GetMyTradesRequest {
            from: Some(1640995200),
            to: Some(1641081600),
            ..Default::default()
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("from=1640995200"));
        assert!(serialized.contains("to=1641081600"));
    }

    #[test]
    fn test_get_my_trades_request_full_parameters() {
        let request = GetMyTradesRequest {
            currency_pair: Some("ETH_USDT".to_string()),
            limit: Some(100),
            page: Some(1),
            order_id: Some("87654321".to_string()),
            account: Some("margin".to_string()),
            from: Some(1640995200),
            to: Some(1641081600),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("currency_pair=ETH_USDT"));
        assert!(serialized.contains("limit=100"));
        assert!(serialized.contains("page=1"));
        assert!(serialized.contains("order_id=87654321"));
        assert!(serialized.contains("account=margin"));
        assert!(serialized.contains("from=1640995200"));
        assert!(serialized.contains("to=1641081600"));
    }

    #[test]
    fn test_get_my_trades_request_different_currency_pairs() {
        let pairs = vec!["BTC_USDT", "ETH_BTC", "BNB_USDT", "SOL_USDC", "ADA_USDT"];

        for pair in pairs {
            let request = GetMyTradesRequest {
                currency_pair: Some(pair.to_string()),
                ..Default::default()
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert_eq!(serialized, format!("currency_pair={}", pair));
        }
    }

    #[test]
    fn test_get_my_trades_request_different_accounts() {
        let accounts = vec!["spot", "margin", "cross_margin", "unified"];

        for account in accounts {
            let request = GetMyTradesRequest {
                account: Some(account.to_string()),
                ..Default::default()
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert_eq!(serialized, format!("account={}", account));
        }
    }

    #[test]
    fn test_get_my_trades_request_pagination_ranges() {
        let pagination_tests = vec![(1, 10), (1, 100), (5, 50), (10, 25), (100, 1000)];

        for (page, limit) in pagination_tests {
            let request = GetMyTradesRequest {
                page: Some(page),
                limit: Some(limit),
                ..Default::default()
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("page={}", page)));
            assert!(serialized.contains(&format!("limit={}", limit)));
        }
    }

    #[test]
    fn test_get_my_trades_request_extreme_values() {
        let request = GetMyTradesRequest {
            limit: Some(u32::MAX),
            page: Some(u32::MAX),
            from: Some(i64::MIN),
            to: Some(i64::MAX),
            ..Default::default()
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains(&format!("limit={}", u32::MAX)));
        assert!(serialized.contains(&format!("page={}", u32::MAX)));
        assert!(serialized.contains(&format!("from={}", i64::MIN)));
        assert!(serialized.contains(&format!("to={}", i64::MAX)));
    }

    #[test]
    fn test_get_my_trades_request_zero_values() {
        let request = GetMyTradesRequest {
            limit: Some(0),
            page: Some(0),
            from: Some(0),
            to: Some(0),
            ..Default::default()
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("limit=0"));
        assert!(serialized.contains("page=0"));
        assert!(serialized.contains("from=0"));
        assert!(serialized.contains("to=0"));
    }

    #[test]
    fn test_get_my_trades_request_json_serialization() {
        let request = GetMyTradesRequest {
            currency_pair: Some("BTC_USDT".to_string()),
            limit: Some(50),
            page: Some(1),
            order_id: Some("12345678".to_string()),
            account: Some("spot".to_string()),
            from: Some(1640995200),
            to: Some(1641081600),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency_pair"], "BTC_USDT");
        assert_eq!(json["limit"], 50);
        assert_eq!(json["page"], 1);
        assert_eq!(json["order_id"], "12345678");
        assert_eq!(json["account"], "spot");
        assert_eq!(json["from"], 1640995200);
        assert_eq!(json["to"], 1641081600);
    }

    #[test]
    fn test_my_trade_deserialization() {
        let json = r#"{
            "id": "123456789",
            "create_time": "1640995200",
            "create_time_ms": "1640995200000",
            "currency_pair": "BTC_USDT",
            "order_id": "12345678",
            "side": "buy",
            "role": "taker",
            "amount": "0.001",
            "price": "30000",
            "fee": "0.03",
            "fee_currency": "USDT",
            "point_fee": "0",
            "gt_fee": "0.0075",
            "gt_fee_deduction": true,
            "rebated_fee": "0",
            "rebated_fee_currency": "USDT"
        }"#;

        let trade: MyTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.id, "123456789");
        assert_eq!(trade.create_time, "1640995200");
        assert_eq!(trade.create_time_ms, "1640995200000");
        assert_eq!(trade.currency_pair, "BTC_USDT");
        assert_eq!(trade.order_id, "12345678");
        assert_eq!(trade.side, "buy");
        assert_eq!(trade.role, "taker");
        assert_eq!(trade.amount, "0.001");
        assert_eq!(trade.price, "30000");
        assert_eq!(trade.fee, "0.03");
        assert_eq!(trade.fee_currency, "USDT");
        assert_eq!(trade.point_fee, "0");
        assert_eq!(trade.gt_fee, "0.0075");
        assert_eq!(trade.gt_fee_deduction, true);
        assert_eq!(trade.rebated_fee, "0");
        assert_eq!(trade.rebated_fee_currency, "USDT");
    }

    #[test]
    fn test_my_trade_maker_order() {
        let json = r#"{
            "id": "987654321",
            "create_time": "1640995300",
            "create_time_ms": "1640995300000",
            "currency_pair": "ETH_USDT",
            "order_id": "87654321",
            "side": "sell",
            "role": "maker",
            "amount": "1.5",
            "price": "2500",
            "fee": "0",
            "fee_currency": "USDT",
            "point_fee": "0",
            "gt_fee": "0",
            "gt_fee_deduction": false,
            "rebated_fee": "0.75",
            "rebated_fee_currency": "USDT"
        }"#;

        let trade: MyTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.role, "maker");
        assert_eq!(trade.side, "sell");
        assert_eq!(trade.fee, "0");
        assert_eq!(trade.gt_fee_deduction, false);
        assert_eq!(trade.rebated_fee, "0.75");

        // Maker orders typically get rebates instead of paying fees
        let rebate: f64 = trade.rebated_fee.parse().unwrap();
        assert!(rebate > 0.0);
    }

    #[test]
    fn test_my_trade_with_gt_discount() {
        let json = r#"{
            "id": "111111111",
            "create_time": "1640995400",
            "create_time_ms": "1640995400000",
            "currency_pair": "BTC_USDT",
            "order_id": "11111111",
            "side": "buy",
            "role": "taker",
            "amount": "0.5",
            "price": "30000",
            "fee": "15",
            "fee_currency": "USDT",
            "point_fee": "0",
            "gt_fee": "3.75",
            "gt_fee_deduction": true,
            "rebated_fee": "0",
            "rebated_fee_currency": "USDT"
        }"#;

        let trade: MyTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.gt_fee_deduction, true);
        assert_eq!(trade.gt_fee, "3.75");

        // GT fee should be less than base fee (25% discount)
        let base_fee: f64 = trade.fee.parse().unwrap();
        let gt_fee: f64 = trade.gt_fee.parse().unwrap();
        assert!(gt_fee < base_fee);

        let discount = 1.0 - (gt_fee / base_fee);
        assert!(discount > 0.2); // Should have significant discount
    }

    #[test]
    fn test_my_trade_with_text() {
        let json = r#"{
            "id": "222222222",
            "create_time": "1640995500",
            "create_time_ms": "1640995500000",
            "currency_pair": "BNB_USDT",
            "order_id": "22222222",
            "side": "buy",
            "role": "taker",
            "amount": "10",
            "price": "300",
            "fee": "3",
            "fee_currency": "USDT",
            "point_fee": "0",
            "gt_fee": "0",
            "gt_fee_deduction": false,
            "rebated_fee": "0",
            "rebated_fee_currency": "USDT",
            "text": "client_order_123"
        }"#;

        let trade: MyTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.text, Some("client_order_123".to_string()));
    }

    #[test]
    fn test_my_trade_different_roles() {
        let roles = vec!["maker", "taker"];

        for role in roles {
            let json = format!(
                r#"{{
                "id": "12345",
                "create_time": "1640995200",
                "create_time_ms": "1640995200000",
                "currency_pair": "BTC_USDT",
                "order_id": "12345",
                "side": "buy",
                "role": "{}",
                "amount": "1.0",
                "price": "30000",
                "fee": "0",
                "fee_currency": "USDT",
                "point_fee": "0",
                "gt_fee": "0",
                "gt_fee_deduction": false,
                "rebated_fee": "0",
                "rebated_fee_currency": "USDT"
            }}"#,
                role
            );

            let trade: MyTrade = serde_json::from_str(&json).unwrap();
            assert_eq!(trade.role, role);
        }
    }

    #[test]
    fn test_my_trade_different_sides() {
        let sides = vec!["buy", "sell"];

        for side in sides {
            let json = format!(
                r#"{{
                "id": "12345",
                "create_time": "1640995200",
                "create_time_ms": "1640995200000",
                "currency_pair": "BTC_USDT",
                "order_id": "12345",
                "side": "{}",
                "role": "taker",
                "amount": "1.0",
                "price": "30000",
                "fee": "30",
                "fee_currency": "USDT",
                "point_fee": "0",
                "gt_fee": "0",
                "gt_fee_deduction": false,
                "rebated_fee": "0",
                "rebated_fee_currency": "USDT"
            }}"#,
                side
            );

            let trade: MyTrade = serde_json::from_str(&json).unwrap();
            assert_eq!(trade.side, side);
        }
    }

    #[test]
    fn test_my_trade_array_deserialization() {
        let json = r#"[
            {
                "id": "123456789",
                "create_time": "1640995200",
                "create_time_ms": "1640995200000",
                "currency_pair": "BTC_USDT",
                "order_id": "12345678",
                "side": "buy",
                "role": "taker",
                "amount": "0.001",
                "price": "30000",
                "fee": "0.03",
                "fee_currency": "USDT",
                "point_fee": "0",
                "gt_fee": "0",
                "gt_fee_deduction": false,
                "rebated_fee": "0",
                "rebated_fee_currency": "USDT"
            },
            {
                "id": "987654321",
                "create_time": "1640995300",
                "create_time_ms": "1640995300000",
                "currency_pair": "ETH_USDT",
                "order_id": "87654321",
                "side": "sell",
                "role": "maker",
                "amount": "1.0",
                "price": "2500",
                "fee": "0",
                "fee_currency": "USDT",
                "point_fee": "0",
                "gt_fee": "0",
                "gt_fee_deduction": false,
                "rebated_fee": "1.25",
                "rebated_fee_currency": "USDT"
            }
        ]"#;

        let trades: Vec<MyTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 2);

        assert_eq!(trades[0].id, "123456789");
        assert_eq!(trades[0].side, "buy");
        assert_eq!(trades[0].role, "taker");

        assert_eq!(trades[1].id, "987654321");
        assert_eq!(trades[1].side, "sell");
        assert_eq!(trades[1].role, "maker");
    }

    #[test]
    fn test_my_trade_empty_array() {
        let json = r#"[]"#;
        let trades: Vec<MyTrade> = serde_json::from_str(json).unwrap();
        assert_eq!(trades.len(), 0);
    }

    #[test]
    fn test_my_trade_serialization() {
        let trade = MyTrade {
            id: "123456789".to_string(),
            create_time: "1640995200".to_string(),
            create_time_ms: "1640995200000".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            order_id: "12345678".to_string(),
            side: "buy".to_string(),
            role: "taker".to_string(),
            amount: "0.001".to_string(),
            price: "30000".to_string(),
            fee: "0.03".to_string(),
            fee_currency: "USDT".to_string(),
            point_fee: "0".to_string(),
            gt_fee: "0".to_string(),
            gt_fee_deduction: false,
            rebated_fee: "0".to_string(),
            rebated_fee_currency: "USDT".to_string(),
            text: None,
        };

        let json = serde_json::to_value(&trade).unwrap();
        assert_eq!(json["id"], "123456789");
        assert_eq!(json["currency_pair"], "BTC_USDT");
        assert_eq!(json["side"], "buy");
        assert_eq!(json["role"], "taker");
        assert_eq!(json["gt_fee_deduction"], false);
    }

    #[test]
    fn test_get_my_trades_request_realistic_scenarios() {
        // Scenario 1: Recent BTC trades
        let btc_recent = GetMyTradesRequest {
            currency_pair: Some("BTC_USDT".to_string()),
            from: Some(1640995200),
            limit: Some(50),
            ..Default::default()
        };

        let btc_serialized = serde_urlencoded::to_string(&btc_recent).unwrap();
        assert!(btc_serialized.contains("currency_pair=BTC_USDT"));
        assert!(btc_serialized.contains("from=1640995200"));
        assert!(btc_serialized.contains("limit=50"));

        // Scenario 2: Trades for specific order
        let order_trades = GetMyTradesRequest {
            order_id: Some("12345678".to_string()),
            currency_pair: Some("ETH_USDT".to_string()),
            ..Default::default()
        };

        let order_serialized = serde_urlencoded::to_string(&order_trades).unwrap();
        assert!(order_serialized.contains("order_id=12345678"));
        assert!(order_serialized.contains("currency_pair=ETH_USDT"));

        // Scenario 3: Margin account trades in time range
        let margin_trades = GetMyTradesRequest {
            account: Some("margin".to_string()),
            from: Some(1638403200),
            to: Some(1640995200),
            limit: Some(100),
            page: Some(1),
            ..Default::default()
        };

        let margin_serialized = serde_urlencoded::to_string(&margin_trades).unwrap();
        assert!(margin_serialized.contains("account=margin"));
        assert!(margin_serialized.contains("from=1638403200"));
        assert!(margin_serialized.contains("to=1640995200"));
    }

    #[test]
    fn test_get_my_trades_for_pair_helper() {
        // Test the helper function parameters
        let currency_pair = "BTC_USDT";
        let limit = Some(50);

        let expected_request = GetMyTradesRequest {
            currency_pair: Some(currency_pair.to_string()),
            limit,
            ..Default::default()
        };

        let serialized = serde_urlencoded::to_string(&expected_request).unwrap();
        assert!(serialized.contains("currency_pair=BTC_USDT"));
        assert!(serialized.contains("limit=50"));
        assert!(!serialized.contains("page="));
        assert!(!serialized.contains("order_id="));
    }

    #[test]
    fn test_get_order_trades_helper() {
        // Test the helper function parameters
        let order_id = "12345678";
        let currency_pair = "ETH_USDT";

        let expected_request = GetMyTradesRequest {
            currency_pair: Some(currency_pair.to_string()),
            order_id: Some(order_id.to_string()),
            ..Default::default()
        };

        let serialized = serde_urlencoded::to_string(&expected_request).unwrap();
        assert!(serialized.contains("currency_pair=ETH_USDT"));
        assert!(serialized.contains("order_id=12345678"));
        assert!(!serialized.contains("limit="));
        assert!(!serialized.contains("from="));
    }

    #[test]
    fn test_get_my_trades_in_range_helper() {
        // Test the helper function parameters
        let currency_pair = Some("BTC_USDT");
        let from = 1640995200;
        let to = 1641081600;
        let limit = Some(100);

        let expected_request = GetMyTradesRequest {
            currency_pair: currency_pair.map(|s| s.to_string()),
            from: Some(from),
            to: Some(to),
            limit,
            ..Default::default()
        };

        let serialized = serde_urlencoded::to_string(&expected_request).unwrap();
        assert!(serialized.contains("currency_pair=BTC_USDT"));
        assert!(serialized.contains("from=1640995200"));
        assert!(serialized.contains("to=1641081600"));
        assert!(serialized.contains("limit=100"));
    }

    #[test]
    fn test_get_recent_my_trades_helper() {
        // Test time calculation logic
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        let yesterday = now - 86400;

        // Verify 24 hour calculation
        assert_eq!(now - yesterday, 86400);
        assert!(yesterday < now);

        // Test that recent trades logic is correct
        let currency_pair = Some("BTC_USDT");
        let limit = Some(50);

        let expected_request = GetMyTradesRequest {
            currency_pair: currency_pair.map(|s| s.to_string()),
            from: Some(yesterday),
            to: Some(now),
            limit,
            ..Default::default()
        };

        let serialized = serde_urlencoded::to_string(&expected_request).unwrap();
        assert!(serialized.contains("currency_pair=BTC_USDT"));
        assert!(serialized.contains(&format!("from={}", yesterday)));
        assert!(serialized.contains(&format!("to={}", now)));
        assert!(serialized.contains("limit=50"));
    }

    #[test]
    fn test_get_my_trades_request_default_values() {
        let request = GetMyTradesRequest::default();

        assert_eq!(request.currency_pair, None);
        assert_eq!(request.limit, None);
        assert_eq!(request.page, None);
        assert_eq!(request.order_id, None);
        assert_eq!(request.account, None);
        assert_eq!(request.from, None);
        assert_eq!(request.to, None);
    }

    #[test]
    fn test_get_my_trades_request_clone() {
        let original = GetMyTradesRequest {
            currency_pair: Some("BTC_USDT".to_string()),
            limit: Some(50),
            page: Some(1),
            order_id: Some("12345678".to_string()),
            account: Some("spot".to_string()),
            from: Some(1640995200),
            to: Some(1641081600),
        };

        let cloned = original.clone();
        assert_eq!(cloned.currency_pair, original.currency_pair);
        assert_eq!(cloned.limit, original.limit);
        assert_eq!(cloned.page, original.page);
        assert_eq!(cloned.order_id, original.order_id);
        assert_eq!(cloned.account, original.account);
        assert_eq!(cloned.from, original.from);
        assert_eq!(cloned.to, original.to);
    }

    #[test]
    fn test_get_my_trades_request_debug() {
        let request = GetMyTradesRequest {
            currency_pair: Some("BTC_USDT".to_string()),
            order_id: Some("12345678".to_string()),
            account: Some("spot".to_string()),
            ..Default::default()
        };

        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("GetMyTradesRequest"));
        assert!(debug_str.contains("BTC_USDT"));
        assert!(debug_str.contains("12345678"));
        assert!(debug_str.contains("spot"));
    }

    #[test]
    fn test_my_trade_clone() {
        let original = MyTrade {
            id: "123456789".to_string(),
            create_time: "1640995200".to_string(),
            create_time_ms: "1640995200000".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            order_id: "12345678".to_string(),
            side: "buy".to_string(),
            role: "taker".to_string(),
            amount: "0.001".to_string(),
            price: "30000".to_string(),
            fee: "0.03".to_string(),
            fee_currency: "USDT".to_string(),
            point_fee: "0".to_string(),
            gt_fee: "0".to_string(),
            gt_fee_deduction: false,
            rebated_fee: "0".to_string(),
            rebated_fee_currency: "USDT".to_string(),
            text: Some("test".to_string()),
        };

        let cloned = original.clone();
        assert_eq!(cloned.id, original.id);
        assert_eq!(cloned.currency_pair, original.currency_pair);
        assert_eq!(cloned.text, original.text);
        assert_eq!(cloned.gt_fee_deduction, original.gt_fee_deduction);
    }

    #[test]
    fn test_my_trade_debug() {
        let trade = MyTrade {
            id: "123456789".to_string(),
            create_time: "1640995200".to_string(),
            create_time_ms: "1640995200000".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            order_id: "12345678".to_string(),
            side: "buy".to_string(),
            role: "taker".to_string(),
            amount: "0.001".to_string(),
            price: "30000".to_string(),
            fee: "0.03".to_string(),
            fee_currency: "USDT".to_string(),
            point_fee: "0".to_string(),
            gt_fee: "0".to_string(),
            gt_fee_deduction: false,
            rebated_fee: "0".to_string(),
            rebated_fee_currency: "USDT".to_string(),
            text: None,
        };

        let debug_str = format!("{:?}", trade);
        assert!(debug_str.contains("MyTrade"));
        assert!(debug_str.contains("123456789"));
        assert!(debug_str.contains("BTC_USDT"));
    }

    #[test]
    fn test_my_trade_precision_handling() {
        let json = r#"{
            "id": "999999999",
            "create_time": "1640995200",
            "create_time_ms": "1640995200123",
            "currency_pair": "BTC_USDT",
            "order_id": "99999999",
            "side": "buy",
            "role": "taker",
            "amount": "0.12345678",
            "price": "30000.12345678",
            "fee": "3.70372222",
            "fee_currency": "USDT",
            "point_fee": "0",
            "gt_fee": "0.92593056",
            "gt_fee_deduction": true,
            "rebated_fee": "0",
            "rebated_fee_currency": "USDT"
        }"#;

        let trade: MyTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.amount, "0.12345678");
        assert_eq!(trade.price, "30000.12345678");
        assert_eq!(trade.fee, "3.70372222");
        assert_eq!(trade.gt_fee, "0.92593056");
        assert_eq!(trade.create_time_ms, "1640995200123");
    }

    #[test]
    fn test_my_trade_stablecoin_pairs() {
        let stablecoin_pairs = vec!["USDC_USDT", "BUSD_USDT", "DAI_USDT"];

        for pair in stablecoin_pairs {
            let json = format!(
                r#"{{
                "id": "12345",
                "create_time": "1640995200",
                "create_time_ms": "1640995200000",
                "currency_pair": "{}",
                "order_id": "12345",
                "side": "buy",
                "role": "taker",
                "amount": "1000.0",
                "price": "1.0001",
                "fee": "1.0001",
                "fee_currency": "USDT",
                "point_fee": "0",
                "gt_fee": "0",
                "gt_fee_deduction": false,
                "rebated_fee": "0",
                "rebated_fee_currency": "USDT"
            }}"#,
                pair
            );

            let trade: MyTrade = serde_json::from_str(&json).unwrap();
            assert_eq!(trade.currency_pair, pair);

            // Stablecoin trades should have tight spreads
            let price: f64 = trade.price.parse().unwrap();
            assert!(price > 0.99 && price < 1.01); // Should be close to 1.0
        }
    }
}
