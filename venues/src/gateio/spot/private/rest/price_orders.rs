use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for listing price orders

const SPOT_PRICE_ORDERS_ENDPOINT: &str = "/spot/price_orders";

#[derive(Debug, Clone, Serialize, Default)]
pub struct ListPriceOrdersRequest {
    /// Currency pair
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,

    /// Status filter (open, cancelled, finished)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Page number (default: 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,

    /// Maximum number of records to return (1-100, default: 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Request to create a price order
#[derive(Debug, Clone, Serialize)]
pub struct CreatePriceOrderRequest {
    /// Currency pair
    pub currency_pair: String,

    /// Order type (limit, market)
    #[serde(rename = "type")]
    pub order_type: String,

    /// Account mode (spot, margin, cross_margin)
    pub account: String,

    /// Order side (buy or sell)
    pub side: String,

    /// Order amount
    pub amount: String,

    /// Order price (required for limit orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Time in force (gtc, ioc, poc, fok)
    pub time_in_force: String,

    /// Client order ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Trigger price for price orders
    pub trigger_price: String,

    /// Rule for trigger (<=, >=)
    pub rule: String,

    /// Trigger expiration time (seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<i64>,
}

/// Price order information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceOrder {
    /// Order ID
    pub id: i64,

    /// User ID
    pub user: i64,

    /// Currency pair
    pub currency_pair: String,

    /// Order status
    pub status: String,

    /// Account mode
    pub account: String,

    /// Order side (buy or sell)
    pub side: String,

    /// Order amount
    pub amount: String,

    /// Order price
    pub price: String,

    /// Order type
    #[serde(rename = "type")]
    pub order_type: String,

    /// Time in force
    pub time_in_force: String,

    /// Trigger price
    pub trigger_price: String,

    /// Trigger rule
    pub rule: String,

    /// Trigger expiration time
    pub expiration: i64,

    /// Order fired ID (when triggered)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fired_order_id: Option<String>,

    /// Create time timestamp
    pub create_time: i64,

    /// Put time timestamp
    pub put_time: i64,

    /// Client order id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Reason for cancellation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl RestClient {
    /// List price orders
    ///
    /// This endpoint returns price orders (conditional orders) for the authenticated user.
    /// Price orders are triggered when the market price reaches the specified trigger price.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#retrieve-running-auto-order-list>
    pub async fn list_price_orders(
        &self,
        params: ListPriceOrdersRequest,
    ) -> crate::gateio::spot::Result<Vec<PriceOrder>> {
        self.get_with_query(SPOT_PRICE_ORDERS_ENDPOINT, &params)
            .await
    }

    /// Get a specific price order
    ///
    /// This endpoint returns details for a specific price order by ID.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#get-a-single-order>
    pub async fn get_price_order(&self, order_id: &str) -> crate::gateio::spot::Result<PriceOrder> {
        let endpoint = format!("/spot/price_orders/{}", order_id);
        self.get(&endpoint).await
    }

    /// Create a price order
    ///
    /// This endpoint creates a new price order (conditional order) that will be
    /// triggered when the market price reaches the specified trigger price.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#create-a-price-triggered-order>
    pub async fn create_price_order(
        &self,
        request: CreatePriceOrderRequest,
    ) -> crate::gateio::spot::Result<PriceOrder> {
        self.post("/spot/price_orders", &request).await
    }

    /// Cancel all price orders
    ///
    /// This endpoint cancels all price orders for the specified currency pair.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#cancel-all-open-orders-under-specified-currency-pair>
    pub async fn cancel_all_price_orders(
        &self,
        currency_pair: &str,
        account: &str,
    ) -> crate::gateio::spot::Result<Vec<PriceOrder>> {
        let endpoint = format!(
            "/spot/price_orders?currency_pair={}&account={}",
            currency_pair, account
        );
        self.delete(&endpoint).await
    }

    /// Cancel a specific price order
    ///
    /// This endpoint cancels a specific price order by ID.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#cancel-a-price-triggered-order>
    pub async fn cancel_price_order(
        &self,
        order_id: &str,
    ) -> crate::gateio::spot::Result<PriceOrder> {
        let endpoint = format!("/spot/price_orders/{}", order_id);
        self.delete(&endpoint).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_price_orders_request_default() {
        let request = ListPriceOrdersRequest::default();

        let json = serde_json::to_value(&request).unwrap();
        let obj = json.as_object().unwrap();
        assert!(obj.is_empty()); // No fields should be serialized when None
    }

    #[test]
    fn test_list_price_orders_request_with_currency_pair() {
        let request = ListPriceOrdersRequest {
            currency_pair: Some("BTC_USDT".to_string()),
            status: None,
            page: None,
            limit: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency_pair"], "BTC_USDT");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 1);
        assert!(!obj.contains_key("status"));
        assert!(!obj.contains_key("page"));
        assert!(!obj.contains_key("limit"));
    }

    #[test]
    fn test_list_price_orders_request_with_status() {
        let request = ListPriceOrdersRequest {
            currency_pair: None,
            status: Some("open".to_string()),
            page: None,
            limit: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["status"], "open");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 1);
    }

    #[test]
    fn test_list_price_orders_request_full() {
        let request = ListPriceOrdersRequest {
            currency_pair: Some("BTC_USDT".to_string()),
            status: Some("open".to_string()),
            page: Some(1),
            limit: Some(50),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency_pair"], "BTC_USDT");
        assert_eq!(json["status"], "open");
        assert_eq!(json["page"], 1);
        assert_eq!(json["limit"], 50);
    }

    #[test]
    fn test_list_price_orders_request_different_statuses() {
        let statuses = vec!["open", "cancelled", "finished"];

        for status in statuses {
            let request = ListPriceOrdersRequest {
                currency_pair: None,
                status: Some(status.to_string()),
                page: None,
                limit: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["status"], status);
        }
    }

    #[test]
    fn test_list_price_orders_request_pagination() {
        let test_cases = vec![(1, 10), (2, 25), (5, 50), (10, 100)];

        for (page, limit) in test_cases {
            let request = ListPriceOrdersRequest {
                currency_pair: None,
                status: None,
                page: Some(page),
                limit: Some(limit),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["page"], page);
            assert_eq!(json["limit"], limit);
        }
    }

    #[test]
    fn test_create_price_order_request_limit_order() {
        let request = CreatePriceOrderRequest {
            currency_pair: "BTC_USDT".to_string(),
            order_type: "limit".to_string(),
            account: "spot".to_string(),
            side: "buy".to_string(),
            amount: "0.001".to_string(),
            price: Some("30000".to_string()),
            time_in_force: "gtc".to_string(),
            text: Some("trigger_buy_001".to_string()),
            trigger_price: "29500".to_string(),
            rule: "<=".to_string(),
            expiration: Some(1640995200),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency_pair"], "BTC_USDT");
        assert_eq!(json["type"], "limit");
        assert_eq!(json["account"], "spot");
        assert_eq!(json["side"], "buy");
        assert_eq!(json["amount"], "0.001");
        assert_eq!(json["price"], "30000");
        assert_eq!(json["time_in_force"], "gtc");
        assert_eq!(json["text"], "trigger_buy_001");
        assert_eq!(json["trigger_price"], "29500");
        assert_eq!(json["rule"], "<=");
        assert_eq!(json["expiration"], 1640995200);
    }

    #[test]
    fn test_create_price_order_request_market_order() {
        let request = CreatePriceOrderRequest {
            currency_pair: "ETH_USDT".to_string(),
            order_type: "market".to_string(),
            account: "margin".to_string(),
            side: "sell".to_string(),
            amount: "1.0".to_string(),
            price: None,
            time_in_force: "ioc".to_string(),
            text: None,
            trigger_price: "2600".to_string(),
            rule: ">=".to_string(),
            expiration: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency_pair"], "ETH_USDT");
        assert_eq!(json["type"], "market");
        assert_eq!(json["account"], "margin");
        assert_eq!(json["side"], "sell");
        assert_eq!(json["amount"], "1.0");
        assert_eq!(json["time_in_force"], "ioc");
        assert_eq!(json["trigger_price"], "2600");
        assert_eq!(json["rule"], ">=");

        let obj = json.as_object().unwrap();
        assert!(!obj.contains_key("price"));
        assert!(!obj.contains_key("text"));
        assert!(!obj.contains_key("expiration"));
    }

    #[test]
    fn test_create_price_order_request_different_trigger_rules() {
        let rules = vec!["<=", ">="];

        for rule in rules {
            let request = CreatePriceOrderRequest {
                currency_pair: "BTC_USDT".to_string(),
                order_type: "limit".to_string(),
                account: "spot".to_string(),
                side: "buy".to_string(),
                amount: "0.1".to_string(),
                price: Some("31000".to_string()),
                time_in_force: "gtc".to_string(),
                text: None,
                trigger_price: "30500".to_string(),
                rule: rule.to_string(),
                expiration: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["rule"], rule);
        }
    }

    #[test]
    fn test_create_price_order_request_different_accounts() {
        let accounts = vec!["spot", "margin", "cross_margin"];

        for account in accounts {
            let request = CreatePriceOrderRequest {
                currency_pair: "BTC_USDT".to_string(),
                order_type: "limit".to_string(),
                account: account.to_string(),
                side: "buy".to_string(),
                amount: "0.1".to_string(),
                price: Some("31000".to_string()),
                time_in_force: "gtc".to_string(),
                text: None,
                trigger_price: "30500".to_string(),
                rule: "<=".to_string(),
                expiration: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["account"], account);
        }
    }

    #[test]
    fn test_create_price_order_request_different_time_in_force() {
        let tif_values = vec!["gtc", "ioc", "poc", "fok"];

        for tif in tif_values {
            let request = CreatePriceOrderRequest {
                currency_pair: "BTC_USDT".to_string(),
                order_type: "limit".to_string(),
                account: "spot".to_string(),
                side: "buy".to_string(),
                amount: "0.1".to_string(),
                price: Some("31000".to_string()),
                time_in_force: tif.to_string(),
                text: None,
                trigger_price: "30500".to_string(),
                rule: "<=".to_string(),
                expiration: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["time_in_force"], tif);
        }
    }

    #[test]
    fn test_price_order_deserialization() {
        let json = r#"{
            "id": 12345678,
            "user": 987654321,
            "currency_pair": "BTC_USDT",
            "status": "open",
            "account": "spot",
            "side": "buy",
            "amount": "0.001",
            "price": "30000",
            "type": "limit",
            "time_in_force": "gtc",
            "trigger_price": "29500",
            "rule": "<=",
            "expiration": 1640995200,
            "fired_order_id": "789456123",
            "create_time": 1640995100,
            "put_time": 1640995110,
            "text": "trigger_buy_001",
            "reason": null
        }"#;

        let order: PriceOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.id, 12345678);
        assert_eq!(order.user, 987654321);
        assert_eq!(order.currency_pair, "BTC_USDT");
        assert_eq!(order.status, "open");
        assert_eq!(order.account, "spot");
        assert_eq!(order.side, "buy");
        assert_eq!(order.amount, "0.001");
        assert_eq!(order.price, "30000");
        assert_eq!(order.order_type, "limit");
        assert_eq!(order.time_in_force, "gtc");
        assert_eq!(order.trigger_price, "29500");
        assert_eq!(order.rule, "<=");
        assert_eq!(order.expiration, 1640995200);
        assert_eq!(order.fired_order_id.as_ref().unwrap(), "789456123");
        assert_eq!(order.create_time, 1640995100);
        assert_eq!(order.put_time, 1640995110);
        assert_eq!(order.text.as_ref().unwrap(), "trigger_buy_001");
        assert!(order.reason.is_none());
    }

    #[test]
    fn test_price_order_different_statuses() {
        let statuses = vec!["open", "cancelled", "finished", "triggered"];

        for status in statuses {
            let json = format!(
                r#"{{
                "id": 12345678,
                "user": 987654321,
                "currency_pair": "BTC_USDT",
                "status": "{}",
                "account": "spot",
                "side": "buy",
                "amount": "0.001",
                "price": "30000",
                "type": "limit",
                "time_in_force": "gtc",
                "trigger_price": "29500",
                "rule": "<=",
                "expiration": 1640995200,
                "create_time": 1640995100,
                "put_time": 1640995110
            }}"#,
                status
            );

            let order: PriceOrder = serde_json::from_str(&json).unwrap();
            assert_eq!(order.status, status);
        }
    }

    #[test]
    fn test_price_order_without_optional_fields() {
        let json = r#"{
            "id": 12345678,
            "user": 987654321,
            "currency_pair": "BTC_USDT",
            "status": "open",
            "account": "spot",
            "side": "buy",
            "amount": "0.001",
            "price": "30000",
            "type": "limit",
            "time_in_force": "gtc",
            "trigger_price": "29500",
            "rule": "<=",
            "expiration": 1640995200,
            "create_time": 1640995100,
            "put_time": 1640995110
        }"#;

        let order: PriceOrder = serde_json::from_str(json).unwrap();
        assert!(order.fired_order_id.is_none());
        assert!(order.text.is_none());
        assert!(order.reason.is_none());
    }

    #[test]
    fn test_price_order_realistic_buy_stop_loss_scenario() {
        let json = r#"{
            "id": 12345678,
            "user": 987654321,
            "currency_pair": "BTC_USDT",
            "status": "open",
            "account": "spot",
            "side": "sell",
            "amount": "0.1",
            "price": "28000",
            "type": "limit",
            "time_in_force": "gtc",
            "trigger_price": "29000",
            "rule": "<=",
            "expiration": 1641000000,
            "create_time": 1640995100,
            "put_time": 1640995110,
            "text": "stop_loss_btc"
        }"#;

        let order: PriceOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.side, "sell");
        assert_eq!(order.rule, "<=");
        assert_eq!(order.trigger_price, "29000");
        assert_eq!(order.text.as_ref().unwrap(), "stop_loss_btc");

        // Verify stop loss logic: sell when price falls to or below trigger
        let trigger_price: f64 = order.trigger_price.parse().unwrap();
        let order_price: f64 = order.price.parse().unwrap();
        assert!(trigger_price > order_price); // Trigger above order price for stop loss
    }

    #[test]
    fn test_price_order_realistic_buy_break_out_scenario() {
        let json = r#"{
            "id": 87654321,
            "user": 123456789,
            "currency_pair": "ETH_USDT",
            "status": "open",
            "account": "margin",
            "side": "buy",
            "amount": "2.0",
            "price": "2650",
            "type": "limit",
            "time_in_force": "gtc",
            "trigger_price": "2600",
            "rule": ">=",
            "expiration": 1641000000,
            "create_time": 1640995200,
            "put_time": 1640995210,
            "text": "breakout_eth"
        }"#;

        let order: PriceOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.side, "buy");
        assert_eq!(order.rule, ">=");
        assert_eq!(order.account, "margin");
        assert_eq!(order.text.as_ref().unwrap(), "breakout_eth");

        // Verify breakout logic: buy when price rises to or above trigger
        let trigger_price: f64 = order.trigger_price.parse().unwrap();
        let order_price: f64 = order.price.parse().unwrap();
        assert!(order_price > trigger_price); // Order price above trigger for breakout
    }

    #[test]
    fn test_price_order_realistic_triggered_scenario() {
        let json = r#"{
            "id": 11111111,
            "user": 444444444,
            "currency_pair": "BTC_USDT",
            "status": "finished",
            "account": "spot",
            "side": "buy",
            "amount": "0.05",
            "price": "31000",
            "type": "limit",
            "time_in_force": "gtc",
            "trigger_price": "30500",
            "rule": "<=",
            "expiration": 1641000000,
            "fired_order_id": "regular_order_123",
            "create_time": 1640995100,
            "put_time": 1640995700,
            "text": "dca_trigger"
        }"#;

        let order: PriceOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.status, "finished");
        assert!(order.fired_order_id.is_some());
        assert_eq!(order.fired_order_id.as_ref().unwrap(), "regular_order_123");
        assert_eq!(order.text.as_ref().unwrap(), "dca_trigger");
    }

    #[test]
    fn test_price_order_realistic_cancelled_scenario() {
        let json = r#"{
            "id": 22222222,
            "user": 555555555,
            "currency_pair": "BNB_USDT",
            "status": "cancelled",
            "account": "spot",
            "side": "sell",
            "amount": "50.0",
            "price": "320",
            "type": "limit",
            "time_in_force": "gtc",
            "trigger_price": "325",
            "rule": ">=",
            "expiration": 1640999999,
            "create_time": 1640995100,
            "put_time": 1640995100,
            "text": "profit_take",
            "reason": "user_cancelled"
        }"#;

        let order: PriceOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.status, "cancelled");
        assert_eq!(order.reason.as_ref().unwrap(), "user_cancelled");
        assert_eq!(order.text.as_ref().unwrap(), "profit_take");
        assert!(order.fired_order_id.is_none());
    }

    #[test]
    fn test_list_price_orders_request_realistic_open_orders_scenario() {
        // Scenario: Get all open price orders for BTC
        let request = ListPriceOrdersRequest {
            currency_pair: Some("BTC_USDT".to_string()),
            status: Some("open".to_string()),
            page: Some(1),
            limit: Some(100),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency_pair"], "BTC_USDT");
        assert_eq!(json["status"], "open");
        assert_eq!(json["page"], 1);
        assert_eq!(json["limit"], 100);
    }

    #[test]
    fn test_create_price_order_request_realistic_stop_loss_scenario() {
        // Scenario: Create stop loss order for BTC position
        let request = CreatePriceOrderRequest {
            currency_pair: "BTC_USDT".to_string(),
            order_type: "limit".to_string(),
            account: "spot".to_string(),
            side: "sell".to_string(),
            amount: "0.1".to_string(),
            price: Some("28000".to_string()),
            time_in_force: "gtc".to_string(),
            text: Some("stop_loss_protection".to_string()),
            trigger_price: "29000".to_string(),
            rule: "<=".to_string(),
            expiration: Some(1641000000),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["side"], "sell");
        assert_eq!(json["rule"], "<=");
        assert_eq!(json["text"], "stop_loss_protection");

        // Verify stop loss setup
        let trigger: f64 = json["trigger_price"].as_str().unwrap().parse().unwrap();
        let price: f64 = json["price"].as_str().unwrap().parse().unwrap();
        assert!(trigger > price); // Trigger above order price
    }

    #[test]
    fn test_create_price_order_request_realistic_take_profit_scenario() {
        // Scenario: Create take profit order for ETH position
        let request = CreatePriceOrderRequest {
            currency_pair: "ETH_USDT".to_string(),
            order_type: "limit".to_string(),
            account: "margin".to_string(),
            side: "sell".to_string(),
            amount: "1.5".to_string(),
            price: Some("2700".to_string()),
            time_in_force: "gtc".to_string(),
            text: Some("take_profit_eth".to_string()),
            trigger_price: "2650".to_string(),
            rule: ">=".to_string(),
            expiration: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["side"], "sell");
        assert_eq!(json["rule"], ">=");
        assert_eq!(json["account"], "margin");
        assert_eq!(json["text"], "take_profit_eth");

        // Verify take profit setup
        let trigger: f64 = json["trigger_price"].as_str().unwrap().parse().unwrap();
        let price: f64 = json["price"].as_str().unwrap().parse().unwrap();
        assert!(price > trigger); // Order price above trigger
    }

    #[test]
    fn test_create_price_order_request_realistic_dca_buy_scenario() {
        // Scenario: DCA buy order triggered on dip
        let request = CreatePriceOrderRequest {
            currency_pair: "BTC_USDT".to_string(),
            order_type: "market".to_string(),
            account: "spot".to_string(),
            side: "buy".to_string(),
            amount: "100".to_string(), // $100 worth
            price: None,
            time_in_force: "ioc".to_string(),
            text: Some("dca_weekly_buy".to_string()),
            trigger_price: "30000".to_string(),
            rule: "<=".to_string(),
            expiration: Some(1641604800), // 1 week expiry
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["type"], "market");
        assert_eq!(json["side"], "buy");
        assert_eq!(json["rule"], "<=");
        assert_eq!(json["text"], "dca_weekly_buy");
        assert_eq!(json["time_in_force"], "ioc");

        let obj = json.as_object().unwrap();
        assert!(!obj.contains_key("price")); // Market order has no price
    }

    #[test]
    fn test_price_order_high_precision_amounts() {
        let json = r#"{
            "id": 12345678,
            "user": 987654321,
            "currency_pair": "BTC_USDT",
            "status": "open",
            "account": "spot",
            "side": "buy",
            "amount": "0.12345678",
            "price": "30000.12345678",
            "type": "limit",
            "time_in_force": "gtc",
            "trigger_price": "29999.87654321",
            "rule": "<=",
            "expiration": 1640995200,
            "create_time": 1640995100,
            "put_time": 1640995110
        }"#;

        let order: PriceOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.amount, "0.12345678");
        assert_eq!(order.price, "30000.12345678");
        assert_eq!(order.trigger_price, "29999.87654321");
    }

    #[test]
    fn test_price_order_different_currency_pairs() {
        let pairs = vec![
            "BTC_USDT",
            "ETH_USDT",
            "BNB_USDT",
            "SOL_USDC",
            "ETH_BTC",
            "USDC_USDT",
            "ADA_USDT",
            "DOT_USDT",
        ];

        for pair in pairs {
            let json = format!(
                r#"{{
                "id": 12345678,
                "user": 987654321,
                "currency_pair": "{}",
                "status": "open",
                "account": "spot",
                "side": "buy",
                "amount": "1.0",
                "price": "100",
                "type": "limit",
                "time_in_force": "gtc",
                "trigger_price": "95",
                "rule": "<=",
                "expiration": 1640995200,
                "create_time": 1640995100,
                "put_time": 1640995110
            }}"#,
                pair
            );

            let order: PriceOrder = serde_json::from_str(&json).unwrap();
            assert_eq!(order.currency_pair, pair);
        }
    }

    #[test]
    fn test_list_price_orders_request_clone() {
        let original = ListPriceOrdersRequest {
            currency_pair: Some("BTC_USDT".to_string()),
            status: Some("open".to_string()),
            page: Some(1),
            limit: Some(50),
        };

        let cloned = original.clone();
        assert_eq!(cloned.currency_pair, original.currency_pair);
        assert_eq!(cloned.status, original.status);
        assert_eq!(cloned.page, original.page);
        assert_eq!(cloned.limit, original.limit);
    }

    #[test]
    fn test_create_price_order_request_clone() {
        let original = CreatePriceOrderRequest {
            currency_pair: "BTC_USDT".to_string(),
            order_type: "limit".to_string(),
            account: "spot".to_string(),
            side: "buy".to_string(),
            amount: "0.1".to_string(),
            price: Some("31000".to_string()),
            time_in_force: "gtc".to_string(),
            text: Some("test_order".to_string()),
            trigger_price: "30500".to_string(),
            rule: "<=".to_string(),
            expiration: Some(1640995200),
        };

        let cloned = original.clone();
        assert_eq!(cloned.currency_pair, original.currency_pair);
        assert_eq!(cloned.trigger_price, original.trigger_price);
        assert_eq!(cloned.rule, original.rule);
        assert_eq!(cloned.text, original.text);
    }

    #[test]
    fn test_price_order_clone() {
        let original = PriceOrder {
            id: 12345678,
            user: 987654321,
            currency_pair: "BTC_USDT".to_string(),
            status: "open".to_string(),
            account: "spot".to_string(),
            side: "buy".to_string(),
            amount: "0.1".to_string(),
            price: "31000".to_string(),
            order_type: "limit".to_string(),
            time_in_force: "gtc".to_string(),
            trigger_price: "30500".to_string(),
            rule: "<=".to_string(),
            expiration: 1640995200,
            fired_order_id: Some("789456123".to_string()),
            create_time: 1640995100,
            put_time: 1640995110,
            text: Some("test_order".to_string()),
            reason: None,
        };

        let cloned = original.clone();
        assert_eq!(cloned.id, original.id);
        assert_eq!(cloned.trigger_price, original.trigger_price);
        assert_eq!(cloned.rule, original.rule);
        assert_eq!(cloned.fired_order_id, original.fired_order_id);
    }

    #[test]
    fn test_list_price_orders_request_debug() {
        let request = ListPriceOrdersRequest {
            currency_pair: Some("BTC_USDT".to_string()),
            status: Some("open".to_string()),
            page: Some(1),
            limit: Some(50),
        };

        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("ListPriceOrdersRequest"));
        assert!(debug_str.contains("BTC_USDT"));
        assert!(debug_str.contains("open"));
    }

    #[test]
    fn test_create_price_order_request_debug() {
        let request = CreatePriceOrderRequest {
            currency_pair: "BTC_USDT".to_string(),
            order_type: "limit".to_string(),
            account: "spot".to_string(),
            side: "buy".to_string(),
            amount: "0.1".to_string(),
            price: Some("31000".to_string()),
            time_in_force: "gtc".to_string(),
            text: None,
            trigger_price: "30500".to_string(),
            rule: "<=".to_string(),
            expiration: None,
        };

        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("CreatePriceOrderRequest"));
        assert!(debug_str.contains("BTC_USDT"));
        assert!(debug_str.contains("30500"));
    }

    #[test]
    fn test_price_order_debug() {
        let order = PriceOrder {
            id: 12345678,
            user: 987654321,
            currency_pair: "BTC_USDT".to_string(),
            status: "open".to_string(),
            account: "spot".to_string(),
            side: "buy".to_string(),
            amount: "0.1".to_string(),
            price: "31000".to_string(),
            order_type: "limit".to_string(),
            time_in_force: "gtc".to_string(),
            trigger_price: "30500".to_string(),
            rule: "<=".to_string(),
            expiration: 1640995200,
            fired_order_id: None,
            create_time: 1640995100,
            put_time: 1640995110,
            text: None,
            reason: None,
        };

        let debug_str = format!("{:?}", order);
        assert!(debug_str.contains("PriceOrder"));
        assert!(debug_str.contains("12345678"));
        assert!(debug_str.contains("BTC_USDT"));
    }

    #[test]
    fn test_price_order_serialization() {
        let order = PriceOrder {
            id: 12345678,
            user: 987654321,
            currency_pair: "BTC_USDT".to_string(),
            status: "open".to_string(),
            account: "spot".to_string(),
            side: "buy".to_string(),
            amount: "0.1".to_string(),
            price: "31000".to_string(),
            order_type: "limit".to_string(),
            time_in_force: "gtc".to_string(),
            trigger_price: "30500".to_string(),
            rule: "<=".to_string(),
            expiration: 1640995200,
            fired_order_id: Some("789456123".to_string()),
            create_time: 1640995100,
            put_time: 1640995110,
            text: Some("test_order".to_string()),
            reason: None,
        };

        let json = serde_json::to_value(&order).unwrap();
        assert_eq!(json["id"], 12345678);
        assert_eq!(json["currency_pair"], "BTC_USDT");
        assert_eq!(json["type"], "limit");
        assert_eq!(json["trigger_price"], "30500");
        assert_eq!(json["rule"], "<=");
        assert_eq!(json["fired_order_id"], "789456123");
        assert_eq!(json["text"], "test_order");

        // reason should be omitted when None
        let obj = json.as_object().unwrap();
        assert!(!obj.contains_key("reason"));
    }

    #[test]
    fn test_list_price_orders_request_optional_fields_behavior() {
        // Test with all fields
        let full_request = ListPriceOrdersRequest {
            currency_pair: Some("BTC_USDT".to_string()),
            status: Some("open".to_string()),
            page: Some(1),
            limit: Some(50),
        };

        // Test with no fields
        let empty_request = ListPriceOrdersRequest {
            currency_pair: None,
            status: None,
            page: None,
            limit: None,
        };

        let json_full = serde_json::to_value(&full_request).unwrap();
        let json_empty = serde_json::to_value(&empty_request).unwrap();

        // Full request should have all fields
        let obj_full = json_full.as_object().unwrap();
        assert_eq!(obj_full.len(), 4);
        assert!(obj_full.contains_key("currency_pair"));
        assert!(obj_full.contains_key("status"));
        assert!(obj_full.contains_key("page"));
        assert!(obj_full.contains_key("limit"));

        // Empty request should have no fields
        let obj_empty = json_empty.as_object().unwrap();
        assert_eq!(obj_empty.len(), 0);
    }

    #[test]
    fn test_create_price_order_request_optional_fields_behavior() {
        // Test with optional fields
        let with_optional = CreatePriceOrderRequest {
            currency_pair: "BTC_USDT".to_string(),
            order_type: "limit".to_string(),
            account: "spot".to_string(),
            side: "buy".to_string(),
            amount: "0.1".to_string(),
            price: Some("31000".to_string()),
            time_in_force: "gtc".to_string(),
            text: Some("test_order".to_string()),
            trigger_price: "30500".to_string(),
            rule: "<=".to_string(),
            expiration: Some(1640995200),
        };

        // Test without optional fields
        let without_optional = CreatePriceOrderRequest {
            currency_pair: "BTC_USDT".to_string(),
            order_type: "market".to_string(),
            account: "spot".to_string(),
            side: "buy".to_string(),
            amount: "0.1".to_string(),
            price: None,
            time_in_force: "ioc".to_string(),
            text: None,
            trigger_price: "30500".to_string(),
            rule: "<=".to_string(),
            expiration: None,
        };

        let json_with = serde_json::to_value(&with_optional).unwrap();
        let json_without = serde_json::to_value(&without_optional).unwrap();

        // With optional fields
        let obj_with = json_with.as_object().unwrap();
        assert!(obj_with.contains_key("price"));
        assert!(obj_with.contains_key("text"));
        assert!(obj_with.contains_key("expiration"));

        // Without optional fields
        let obj_without = json_without.as_object().unwrap();
        assert!(!obj_without.contains_key("price"));
        assert!(!obj_without.contains_key("text"));
        assert!(!obj_without.contains_key("expiration"));
    }

    #[test]
    fn test_price_order_round_trip() {
        let original = PriceOrder {
            id: 12345678,
            user: 987654321,
            currency_pair: "BTC_USDT".to_string(),
            status: "open".to_string(),
            account: "spot".to_string(),
            side: "buy".to_string(),
            amount: "0.1".to_string(),
            price: "31000".to_string(),
            order_type: "limit".to_string(),
            time_in_force: "gtc".to_string(),
            trigger_price: "30500".to_string(),
            rule: "<=".to_string(),
            expiration: 1640995200,
            fired_order_id: Some("789456123".to_string()),
            create_time: 1640995100,
            put_time: 1640995110,
            text: Some("test_order".to_string()),
            reason: None,
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: PriceOrder = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, original.id);
        assert_eq!(deserialized.currency_pair, original.currency_pair);
        assert_eq!(deserialized.trigger_price, original.trigger_price);
        assert_eq!(deserialized.rule, original.rule);
        assert_eq!(deserialized.fired_order_id, original.fired_order_id);
        assert_eq!(deserialized.text, original.text);
        assert_eq!(deserialized.reason, original.reason);
    }

    #[test]
    fn test_create_price_order_request_endpoint_validation() {
        let request = CreatePriceOrderRequest {
            currency_pair: "BTC_USDT".to_string(),
            order_type: "limit".to_string(),
            account: "spot".to_string(),
            side: "buy".to_string(),
            amount: "0.1".to_string(),
            price: Some("31000".to_string()),
            time_in_force: "gtc".to_string(),
            text: None,
            trigger_price: "30500".to_string(),
            rule: "<=".to_string(),
            expiration: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        let obj = json.as_object().unwrap();

        // Verify all required fields are present
        assert!(obj.contains_key("currency_pair"));
        assert!(obj.contains_key("type"));
        assert!(obj.contains_key("account"));
        assert!(obj.contains_key("side"));
        assert!(obj.contains_key("amount"));
        assert!(obj.contains_key("time_in_force"));
        assert!(obj.contains_key("trigger_price"));
        assert!(obj.contains_key("rule"));

        // Verify field types
        assert!(json["currency_pair"].is_string());
        assert!(json["type"].is_string());
        assert!(json["account"].is_string());
        assert!(json["side"].is_string());
        assert!(json["amount"].is_string());
        assert!(json["time_in_force"].is_string());
        assert!(json["trigger_price"].is_string());
        assert!(json["rule"].is_string());
    }
}
