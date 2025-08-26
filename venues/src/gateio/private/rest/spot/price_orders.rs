use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const SPOT_PRICE_ORDERS_ENDPOINT: &str = "/spot/price_orders";

/// Request parameters for listing price orders with advanced filtering capabilities.
///
/// Used to retrieve conditional price orders that trigger when market prices reach
/// specified thresholds. Supports comprehensive filtering by currency pair, order status,
/// and pagination for efficient handling of large order sets and trading strategies.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListPriceOrdersRequest {
    /// Currency pair filter for price order queries (e.g., "BTC_USDT", "ETH_USDT").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,

    /// Order status filter for conditional order queries (e.g., "open", "cancelled", "finished").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Page number for pagination navigation (starts from 1, default: 1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,

    /// Maximum number of records per page (range: 1-100, default: 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Request parameters for creating conditional price orders with sophisticated trigger configurations.
///
/// Used to establish price-triggered orders that automatically execute when market conditions
/// are met. Supports comprehensive order configurations including trigger rules, execution
/// timing, and account-specific trading strategies for automated market participation.
#[derive(Debug, Clone, Serialize)]
pub struct CreatePriceOrderRequest {
    /// Trading currency pair identifier (e.g., "BTC_USDT", "ETH_USDT").
    ///
    /// Specifies the market for the conditional order trigger and subsequent execution.
    pub currency_pair: String,

    /// Order execution type for triggered order ("limit", "market").
    ///
    /// Determines pricing behavior when the trigger condition is satisfied.
    /// Limit orders require price specification, market orders execute at current rates.
    #[serde(rename = "type")]
    pub order_type: String,

    /// Trading account context for order execution ("spot", "margin", "cross_margin").
    ///
    /// Defines the account environment and available balance for order fulfillment.
    pub account: String,

    /// Order direction for market participation ("buy", "sell").
    ///
    /// Specifies whether to acquire or dispose of the base currency when triggered.
    pub side: String,

    /// Quantity specification for order execution in base currency units.
    ///
    /// For limit orders: base currency amount. For market buy orders: quote currency amount.
    /// Precision requirements vary by trading pair specifications.
    pub amount: String,

    /// Execution price for limit orders when trigger condition is satisfied.
    ///
    /// Required for limit order types, omitted for market orders.
    /// Must comply with tick size requirements for the specified currency pair.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Order persistence policy after trigger activation ("gtc", "ioc", "poc", "fok").
    ///
    /// Controls order lifecycle behavior: GTC (Good Till Canceled), IOC (Immediate or Cancel),
    /// POC (Participate or Cancel), FOK (Fill or Kill). Affects execution guarantees.
    pub time_in_force: String,

    /// User-defined order identifier for tracking and reference purposes.
    ///
    /// Optional alphanumeric string (maximum 28 characters) for order correlation
    /// and identification in trading records and notifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Market price threshold for conditional order activation.
    ///
    /// Defines the price level that must be reached to trigger order execution.
    /// Compared against market price using the specified trigger rule.
    pub trigger_price: String,

    /// Price comparison operator for trigger condition evaluation ("<=", ">=").
    ///
    /// Determines trigger activation logic: "<=" for stop-loss scenarios (trigger when price
    /// falls to or below threshold), ">=" for breakout scenarios (trigger when price rises).
    pub rule: String,

    /// Conditional order expiration timestamp in Unix seconds format.
    ///
    /// Optional deadline after which the trigger becomes invalid and order is canceled.
    /// Provides time-bounded conditional execution for strategic trading scenarios.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<i64>,
}

/// Comprehensive information for conditional price orders with execution tracking.
///
/// Contains complete order state including trigger conditions, execution status,
/// timing information, and associated trade details. Used for monitoring conditional
/// order lifecycle from creation through trigger activation and final settlement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceOrder {
    /// Unique system identifier for the conditional price order.
    ///
    /// Persistent reference for order tracking, status queries, and modification operations.
    pub id: i64,

    /// Account owner identifier for order attribution and security validation.
    ///
    /// Links the conditional order to the specific user account for access control.
    pub user: i64,

    /// Trading market specification for the conditional order ("BTC_USDT", "ETH_USDT").
    ///
    /// Defines the currency pair context for price monitoring and order execution.
    pub currency_pair: String,

    /// Current lifecycle state of the conditional order ("open", "cancelled", "finished", "triggered").
    ///
    /// Indicates order progression: open (waiting), triggered (activated), finished (completed),
    /// cancelled (manually terminated). Affects available operations and state transitions.
    pub status: String,

    /// Trading account context for conditional order execution ("spot", "margin", "cross_margin").
    ///
    /// Specifies the account type used for balance validation and order fulfillment.
    pub account: String,

    /// Market direction for conditional order execution ("buy", "sell").
    ///
    /// Determines whether the triggered order will acquire or dispose of base currency.
    pub side: String,

    /// Quantity specification for conditional order execution in base currency units.
    ///
    /// Amount to be traded when trigger condition is satisfied, subject to precision requirements.
    pub amount: String,

    /// Execution price for the conditional order when triggered.
    ///
    /// For limit orders: specified price level. For market orders: "0" or average execution price.
    pub price: String,

    /// Order execution type applied when trigger condition is met ("limit", "market").
    ///
    /// Determines pricing behavior: limit orders use specified price, market orders use current rates.
    #[serde(rename = "type")]
    pub order_type: String,

    /// Order persistence policy for triggered execution ("gtc", "ioc", "poc", "fok").
    ///
    /// Controls order lifecycle after trigger: GTC (persistent), IOC (immediate), etc.
    pub time_in_force: String,

    /// Price threshold for conditional order activation monitoring.
    ///
    /// Market price level that must be reached to trigger order execution.
    pub trigger_price: String,

    /// Price comparison logic for trigger condition evaluation ("<=", ">=").
    ///
    /// Defines activation rule: "<=" for stop scenarios, ">=" for breakout scenarios.
    pub rule: String,

    /// Conditional order expiration deadline as Unix timestamp.
    ///
    /// Time limit after which the trigger becomes invalid and order is automatically cancelled.
    pub expiration: i64,

    /// Associated regular order identifier when trigger condition has been satisfied.
    ///
    /// Links to the actual market order created upon trigger activation for execution tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fired_order_id: Option<String>,

    /// Order creation timestamp in Unix seconds format.
    ///
    /// Records when the conditional order was initially established in the system.
    pub create_time: i64,

    /// Order placement timestamp in Unix seconds format.
    ///
    /// Records when the conditional order became active for trigger monitoring.
    pub put_time: i64,

    /// User-defined order identifier for correlation and tracking purposes.
    ///
    /// Optional custom reference string for order identification in trading workflows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Cancellation or failure explanation for terminated conditional orders.
    ///
    /// Provides context when orders are cancelled, expired, or encounter execution failures.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl RestClient {
    /// List price orders
    ///
    /// Retrieves conditional price orders for the authenticated user with advanced filtering capabilities.
    /// Price orders are triggered when market price reaches specified trigger conditions, enabling
    /// automated trading strategies including stop-loss, take-profit, and breakout scenarios.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#retrieve-running-auto-order-list)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `params` - Filtering parameters including currency pair, status, and pagination options
    ///
    /// # Returns
    /// Vector of price orders matching the specified criteria with complete order details
    pub async fn spot_list_price_orders(
        &self,
        params: ListPriceOrdersRequest,
    ) -> RestResult<Vec<PriceOrder>> {
        self.get_with_query(SPOT_PRICE_ORDERS_ENDPOINT, &params)
            .await
    }

    /// Get a specific price order
    ///
    /// Retrieves detailed information for a specific conditional price order by its unique identifier.
    /// Provides complete order state including trigger conditions, execution status, and timing information
    /// for comprehensive order lifecycle monitoring and management.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#get-a-single-order)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `order_id` - Unique identifier for the specific price order to retrieve
    ///
    /// # Returns
    /// Complete price order information including current status and execution details
    pub async fn spot_get_price_order(&self, order_id: &str) -> RestResult<PriceOrder> {
        let endpoint = format!("{}/{}", SPOT_PRICE_ORDERS_ENDPOINT, order_id);
        self.get(&endpoint).await
    }

    /// Create a price order
    ///
    /// Creates a new conditional price order that triggers when market price reaches specified thresholds.
    /// Enables sophisticated trading strategies including stop-loss protection, take-profit execution,
    /// and breakout trading with comprehensive trigger rule configuration and timing controls.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#create-a-price-triggered-order)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - Comprehensive price order configuration including trigger conditions and execution parameters
    ///
    /// # Returns
    /// Created price order with assigned identifier and initial status for monitoring
    pub async fn spot_create_price_order(
        &self,
        request: CreatePriceOrderRequest,
    ) -> RestResult<PriceOrder> {
        self.post(SPOT_PRICE_ORDERS_ENDPOINT, &request).await
    }

    /// Cancel all price orders
    ///
    /// Cancels all active conditional price orders for a specified currency pair and account type.
    /// Provides bulk order management for strategy adjustments, risk management, and position
    /// restructuring with immediate trigger deactivation and order termination.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#cancel-all-open-orders-under-specified-currency-pair)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `currency_pair` - Trading pair for which to cancel all price orders (e.g., "BTC_USDT")
    /// * `account` - Account type context for order cancellation ("spot", "margin", "cross_margin")
    ///
    /// # Returns
    /// Vector of cancelled price orders with updated status information
    pub async fn spot_cancel_all_price_orders(
        &self,
        currency_pair: &str,
        account: &str,
    ) -> RestResult<Vec<PriceOrder>> {
        let endpoint = format!(
            "{}?currency_pair={}&account={}",
            SPOT_PRICE_ORDERS_ENDPOINT, currency_pair, account
        );
        self.delete(&endpoint).await
    }

    /// Cancel a specific price order
    ///
    /// Cancels an individual conditional price order by its unique identifier, immediately
    /// deactivating trigger monitoring and removing the order from active status. Provides
    /// precise order management for strategy modifications and risk control adjustments.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#cancel-a-price-triggered-order)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `order_id` - Unique identifier for the specific price order to cancel
    ///
    /// # Returns
    /// Cancelled price order information with updated status and termination details
    pub async fn spot_cancel_price_order(&self, order_id: &str) -> RestResult<PriceOrder> {
        let endpoint = format!("{}/{}", SPOT_PRICE_ORDERS_ENDPOINT, order_id);
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
