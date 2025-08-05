use serde::{Deserialize, Serialize};

use super::RestClient;

const LIST_OPEN_ORDERS_ENDPOINT: &str = "/spot/open_orders";

/// Request parameters for listing all active open orders.
///
/// Used to retrieve currently active orders that are waiting for execution,
/// with optional filtering by currency pair, trading side, and account type.
/// Supports pagination for handling large numbers of open orders.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListOpenOrdersRequest {
    /// Trading pair filter for open orders query.
    /// 
    /// Optional filter to retrieve open orders for a specific currency pair.
    /// Format should be "BASE_QUOTE" (e.g., "BTC_USDT", "ETH_BTC"). If not specified,
    /// returns open orders for all trading pairs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,

    /// Page number for pagination (starting from 1).
    /// 
    /// Used for paginated results when there are many open orders. Default is 1
    /// if not specified. Page numbers start from 1, not 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,

    /// Maximum number of orders to return per page.
    /// 
    /// Controls the number of open orders returned in a single response. Valid range
    /// is 1-100, with default being 100 if not specified. Larger limits may improve
    /// efficiency but could increase response times.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Trading side filter for open orders.
    /// 
    /// Optional filter to retrieve only buy or sell orders. Valid values are "buy"
    /// or "sell". If not specified, returns open orders for both sides.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,

    /// Account type filter for the open orders query.
    /// 
    /// Specifies which account type to query open orders from. Common values include
    /// "spot", "margin", "cross_margin", or "unified". If not specified, uses the
    /// default account context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
}

/// Complete open order information returned from the API.
///
/// Represents an active order that is currently in the market waiting for execution
/// or partially filled. Contains all relevant order details including execution status,
/// fees, timing, and optional features like iceberg orders and auto-trading settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenOrder {
    /// Unique order identifier assigned by the exchange.
    /// 
    /// System-generated unique ID for this order, used for tracking and management.
    pub id: String,

    /// Trading pair for this order.
    /// 
    /// The currency pair being traded, formatted as "BASE_QUOTE" (e.g., "BTC_USDT").
    pub currency_pair: String,

    /// Current order status.
    /// 
    /// Indicates the current state of the order (e.g., "open", "partial").
    /// For this endpoint, typically "open" since only active orders are returned.
    pub status: String,

    /// Account type where the order was placed.
    /// 
    /// Identifies which account context this order belongs to (e.g., "spot", "margin").
    pub account: String,

    /// Order direction (buy or sell).
    /// 
    /// Indicates whether this is a buy order ("buy") or sell order ("sell").
    pub side: String,

    /// Total order quantity as a decimal string.
    /// 
    /// The original amount requested for this order, expressed in the base currency.
    /// Preserved as string to maintain precision for financial calculations.
    pub amount: String,

    /// Order price as a decimal string.
    /// 
    /// The price per unit for limit orders, or "0" for market orders.
    /// Expressed in the quote currency and preserved as string for precision.
    pub price: String,

    /// Order type specification.
    /// 
    /// Indicates the order type such as "limit", "market", "stop", etc.
    #[serde(rename = "type")]
    pub order_type: String,

    /// Time-in-force policy for this order.
    /// 
    /// Specifies how long the order remains active (e.g., "gtc" for Good Till Cancelled,
    /// "ioc" for Immediate Or Cancel, "fok" for Fill Or Kill).
    pub time_in_force: String,

    /// Amount already filled as a decimal string.
    /// 
    /// The portion of the order that has been executed, expressed in base currency.
    /// For open orders, this may be "0" for unfilled or partial amount for partially filled.
    pub filled_amount: String,

    /// Remaining unfilled amount as a decimal string.
    /// 
    /// The portion of the order still waiting to be executed, expressed in base currency.
    /// Calculated as original amount minus filled amount.
    pub left: String,

    /// Average execution price as a decimal string.
    /// 
    /// The weighted average price of all fills for this order. "0" if no fills yet,
    /// otherwise the average price of executed portions.
    pub avg_deal_price: String,

    /// Total fees paid for this order as a decimal string.
    /// 
    /// Cumulative fees charged for all executed portions of this order.
    /// Amount depends on fee structure and trading volume.
    pub fee: String,

    /// Currency in which fees are denominated.
    /// 
    /// The currency used for fee payment (e.g., "USDT", "BTC").
    /// May differ from trading pair currencies based on fee structure.
    pub fee_currency: String,

    /// Points-based fee component as a decimal string.
    /// 
    /// Fee amount paid using loyalty points or similar reward systems.
    /// "0" if no points were used for fee payment.
    pub points_fee: String,

    /// Gate Token (GT) discount fee as a decimal string.
    /// 
    /// Fee discount amount when using GT tokens for fee payment.
    /// "0" if GT discount is not applicable or not used.
    pub gt_fee: String,

    /// Order creation timestamp as a string.
    /// 
    /// Unix timestamp when the order was first created and submitted.
    /// Formatted as string representation of seconds since epoch.
    pub create_time: String,

    /// Last modification timestamp as a string.
    /// 
    /// Unix timestamp of the most recent update to this order (fills, amendments, etc.).
    /// Initially equals create_time, updates with each change.
    pub update_time: String,

    /// Client-specified order identifier.
    /// 
    /// Optional custom order ID provided by the client for order tracking.
    /// Useful for reconciliation and order management systems.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Iceberg order display amount as a decimal string.
    /// 
    /// For iceberg orders, the amount visible in the order book at any time.
    /// Allows large orders to be executed without revealing full size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg: Option<String>,

    /// Automatic borrowing enabled flag.
    /// 
    /// Indicates whether this order can automatically borrow funds if account
    /// balance is insufficient. Applies to margin trading contexts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_borrow: Option<bool>,

    /// Automatic repayment enabled flag.
    /// 
    /// Indicates whether proceeds from this order will automatically repay
    /// outstanding margin loans. Applies to margin trading contexts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_repay: Option<bool>,

    /// Self-trade prevention action.
    /// 
    /// Specifies the action taken when this order would trade against another
    /// order from the same account. Common values include "cancel_newest", "cancel_oldest".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp_act: Option<String>,

    /// Order completion status.
    /// 
    /// Indicates how the order finished when it's no longer active.
    /// Values include "filled", "cancelled", "expired", etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_as: Option<String>,
}

impl RestClient {
    /// List all open orders
    ///
    /// Retrieves all currently active orders that are waiting for execution or partially filled.
    /// This endpoint returns comprehensive information about open orders including execution status,
    /// fees, timing, and optional features. Supports filtering by currency pair, trading side,
    /// and account type, with pagination for efficient handling of large result sets.
    ///
    /// [docs]: https://www.gate.io/docs/developers/apiv4/en/#list-all-open-orders
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `params` - Request parameters for filtering and pagination of open orders
    ///
    /// # Returns
    /// List of active open orders matching the specified criteria
    pub async fn list_open_orders(
        &self,
        params: ListOpenOrdersRequest,
    ) -> crate::gateio::spot::RestResult<Vec<OpenOrder>> {
        self.get_with_query(LIST_OPEN_ORDERS_ENDPOINT, &params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_open_orders_request_minimal_serialization() {
        let request = ListOpenOrdersRequest::default();

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_list_open_orders_request_with_currency_pair() {
        let request = ListOpenOrdersRequest {
            currency_pair: Some("BTC_USDT".to_string()),
            ..Default::default()
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "currency_pair=BTC_USDT");
    }

    #[test]
    fn test_list_open_orders_request_with_pagination() {
        let request = ListOpenOrdersRequest {
            page: Some(2),
            limit: Some(50),
            ..Default::default()
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("page=2"));
        assert!(serialized.contains("limit=50"));
    }

    #[test]
    fn test_list_open_orders_request_with_side() {
        let request = ListOpenOrdersRequest {
            side: Some("buy".to_string()),
            ..Default::default()
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "side=buy");
    }

    #[test]
    fn test_list_open_orders_request_with_account() {
        let request = ListOpenOrdersRequest {
            account: Some("spot".to_string()),
            ..Default::default()
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "account=spot");
    }

    #[test]
    fn test_list_open_orders_request_full_parameters() {
        let request = ListOpenOrdersRequest {
            currency_pair: Some("ETH_USDT".to_string()),
            page: Some(1),
            limit: Some(100),
            side: Some("sell".to_string()),
            account: Some("margin".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("currency_pair=ETH_USDT"));
        assert!(serialized.contains("page=1"));
        assert!(serialized.contains("limit=100"));
        assert!(serialized.contains("side=sell"));
        assert!(serialized.contains("account=margin"));
    }

    #[test]
    fn test_list_open_orders_request_different_currency_pairs() {
        let pairs = vec!["BTC_USDT", "ETH_BTC", "BNB_USDT", "SOL_USDC", "ADA_USDT"];

        for pair in pairs {
            let request = ListOpenOrdersRequest {
                currency_pair: Some(pair.to_string()),
                ..Default::default()
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert_eq!(serialized, format!("currency_pair={}", pair));
        }
    }

    #[test]
    fn test_list_open_orders_request_different_sides() {
        let sides = vec!["buy", "sell"];

        for side in sides {
            let request = ListOpenOrdersRequest {
                side: Some(side.to_string()),
                ..Default::default()
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert_eq!(serialized, format!("side={}", side));
        }
    }

    #[test]
    fn test_list_open_orders_request_different_accounts() {
        let accounts = vec!["spot", "margin", "cross_margin"];

        for account in accounts {
            let request = ListOpenOrdersRequest {
                account: Some(account.to_string()),
                ..Default::default()
            };

            let serialized = serde_urlencoded::to_string(&request).unwrap();
            assert_eq!(serialized, format!("account={}", account));
        }
    }

    #[test]
    fn test_list_open_orders_request_pagination_ranges() {
        let pagination_tests = vec![(1, 1), (1, 100), (5, 50), (10, 25), (100, 10)];

        for (page, limit) in pagination_tests {
            let request = ListOpenOrdersRequest {
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
    fn test_list_open_orders_request_negative_pagination() {
        let request = ListOpenOrdersRequest {
            page: Some(-1),
            limit: Some(-50),
            ..Default::default()
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("page=-1"));
        assert!(serialized.contains("limit=-50"));
    }

    #[test]
    fn test_list_open_orders_request_zero_pagination() {
        let request = ListOpenOrdersRequest {
            page: Some(0),
            limit: Some(0),
            ..Default::default()
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("page=0"));
        assert!(serialized.contains("limit=0"));
    }

    #[test]
    fn test_list_open_orders_request_extreme_pagination() {
        let request = ListOpenOrdersRequest {
            page: Some(i32::MAX),
            limit: Some(i32::MAX),
            ..Default::default()
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains(&format!("page={}", i32::MAX)));
        assert!(serialized.contains(&format!("limit={}", i32::MAX)));
    }

    #[test]
    fn test_list_open_orders_request_json_serialization() {
        let request = ListOpenOrdersRequest {
            currency_pair: Some("BTC_USDT".to_string()),
            page: Some(1),
            limit: Some(50),
            side: Some("buy".to_string()),
            account: Some("spot".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency_pair"], "BTC_USDT");
        assert_eq!(json["page"], 1);
        assert_eq!(json["limit"], 50);
        assert_eq!(json["side"], "buy");
        assert_eq!(json["account"], "spot");
    }

    #[test]
    fn test_list_open_orders_request_json_serialization_defaults() {
        let request = ListOpenOrdersRequest::default();

        let json = serde_json::to_value(&request).unwrap();

        // All fields should be omitted when None
        let obj = json.as_object().unwrap();
        assert!(!obj.contains_key("currency_pair"));
        assert!(!obj.contains_key("page"));
        assert!(!obj.contains_key("limit"));
        assert!(!obj.contains_key("side"));
        assert!(!obj.contains_key("account"));
    }

    #[test]
    fn test_open_order_deserialization() {
        let json = r#"{
            "id": "12345678",
            "currency_pair": "BTC_USDT",
            "status": "open",
            "account": "spot",
            "side": "buy",
            "amount": "0.001",
            "price": "30000",
            "type": "limit",
            "time_in_force": "gtc",
            "filled_amount": "0",
            "left": "0.001",
            "avg_deal_price": "0",
            "fee": "0",
            "fee_currency": "USDT",
            "points_fee": "0",
            "gt_fee": "0",
            "create_time": "1640995200",
            "update_time": "1640995200"
        }"#;

        let order: OpenOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.id, "12345678");
        assert_eq!(order.currency_pair, "BTC_USDT");
        assert_eq!(order.status, "open");
        assert_eq!(order.account, "spot");
        assert_eq!(order.side, "buy");
        assert_eq!(order.amount, "0.001");
        assert_eq!(order.price, "30000");
        assert_eq!(order.order_type, "limit");
        assert_eq!(order.time_in_force, "gtc");
        assert_eq!(order.filled_amount, "0");
        assert_eq!(order.left, "0.001");
        assert_eq!(order.avg_deal_price, "0");
        assert_eq!(order.fee, "0");
        assert_eq!(order.fee_currency, "USDT");
        assert_eq!(order.points_fee, "0");
        assert_eq!(order.gt_fee, "0");
        assert_eq!(order.create_time, "1640995200");
        assert_eq!(order.update_time, "1640995200");
    }

    #[test]
    fn test_open_order_with_optional_fields() {
        let json = r#"{
            "id": "87654321",
            "currency_pair": "ETH_USDT",
            "status": "open",
            "account": "margin",
            "side": "sell",
            "amount": "1.5",
            "price": "2500",
            "type": "limit",
            "time_in_force": "gtc",
            "filled_amount": "0.5",
            "left": "1.0",
            "avg_deal_price": "2500",
            "fee": "1.25",
            "fee_currency": "USDT",
            "points_fee": "0",
            "gt_fee": "0.3125",
            "create_time": "1640995200",
            "update_time": "1640995300",
            "text": "client_order_123",
            "iceberg": "0.1",
            "auto_borrow": true,
            "auto_repay": false,
            "stp_act": "cancel_newest",
            "finish_as": "filled"
        }"#;

        let order: OpenOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.text, Some("client_order_123".to_string()));
        assert_eq!(order.iceberg, Some("0.1".to_string()));
        assert_eq!(order.auto_borrow, Some(true));
        assert_eq!(order.auto_repay, Some(false));
        assert_eq!(order.stp_act, Some("cancel_newest".to_string()));
        assert_eq!(order.finish_as, Some("filled".to_string()));
    }

    #[test]
    fn test_open_order_partially_filled() {
        let json = r#"{
            "id": "11111111",
            "currency_pair": "BTC_USDT",
            "status": "open",
            "account": "spot",
            "side": "buy",
            "amount": "1.0",
            "price": "30000",
            "type": "limit",
            "time_in_force": "gtc",
            "filled_amount": "0.3",
            "left": "0.7",
            "avg_deal_price": "29950",
            "fee": "8.985",
            "fee_currency": "USDT",
            "points_fee": "0",
            "gt_fee": "2.24625",
            "create_time": "1640995200",
            "update_time": "1640995300"
        }"#;

        let order: OpenOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.filled_amount, "0.3");
        assert_eq!(order.left, "0.7");
        assert_eq!(order.avg_deal_price, "29950");

        // Verify partial fill logic
        let amount: f64 = order.amount.parse().unwrap();
        let filled: f64 = order.filled_amount.parse().unwrap();
        let left: f64 = order.left.parse().unwrap();
        assert_eq!(amount, filled + left);
    }

    #[test]
    fn test_open_order_iceberg_order() {
        let json = r#"{
            "id": "22222222",
            "currency_pair": "ETH_USDT",
            "status": "open",
            "account": "spot",
            "side": "sell",
            "amount": "10.0",
            "price": "2500",
            "type": "limit",
            "time_in_force": "gtc",
            "filled_amount": "2.0",
            "left": "8.0",
            "avg_deal_price": "2500",
            "fee": "5.0",
            "fee_currency": "USDT",
            "points_fee": "0",
            "gt_fee": "1.25",
            "create_time": "1640995200",
            "update_time": "1640995300",
            "iceberg": "1.0"
        }"#;

        let order: OpenOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.iceberg, Some("1.0".to_string()));

        let iceberg_amount: f64 = order.iceberg.as_ref().unwrap().parse().unwrap();
        let total_amount: f64 = order.amount.parse().unwrap();
        assert!(iceberg_amount < total_amount); // Iceberg should be smaller than total
    }

    #[test]
    fn test_open_order_margin_with_auto_features() {
        let json = r#"{
            "id": "33333333",
            "currency_pair": "BTC_USDT",
            "status": "open",
            "account": "margin",
            "side": "buy",
            "amount": "0.5",
            "price": "30000",
            "type": "limit",
            "time_in_force": "gtc",
            "filled_amount": "0",
            "left": "0.5",
            "avg_deal_price": "0",
            "fee": "0",
            "fee_currency": "USDT",
            "points_fee": "0",
            "gt_fee": "0",
            "create_time": "1640995200",
            "update_time": "1640995200",
            "auto_borrow": true,
            "auto_repay": true
        }"#;

        let order: OpenOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.account, "margin");
        assert_eq!(order.auto_borrow, Some(true));
        assert_eq!(order.auto_repay, Some(true));
    }

    #[test]
    fn test_open_order_different_order_types() {
        let order_types = vec!["limit", "market", "ioc", "fok"];

        for order_type in order_types {
            let json = format!(
                r#"{{
                "id": "12345",
                "currency_pair": "BTC_USDT",
                "status": "open",
                "account": "spot",
                "side": "buy",
                "amount": "1.0",
                "price": "30000",
                "type": "{}",
                "time_in_force": "gtc",
                "filled_amount": "0",
                "left": "1.0",
                "avg_deal_price": "0",
                "fee": "0",
                "fee_currency": "USDT",
                "points_fee": "0",
                "gt_fee": "0",
                "create_time": "1640995200",
                "update_time": "1640995200"
            }}"#,
                order_type
            );

            let order: OpenOrder = serde_json::from_str(&json).unwrap();
            assert_eq!(order.order_type, order_type);
        }
    }

    #[test]
    fn test_open_order_different_time_in_force() {
        let tif_values = vec!["gtc", "ioc", "fok"];

        for tif in tif_values {
            let json = format!(
                r#"{{
                "id": "12345",
                "currency_pair": "BTC_USDT",
                "status": "open",
                "account": "spot",
                "side": "buy",
                "amount": "1.0",
                "price": "30000",
                "type": "limit",
                "time_in_force": "{}",
                "filled_amount": "0",
                "left": "1.0",
                "avg_deal_price": "0",
                "fee": "0",
                "fee_currency": "USDT",
                "points_fee": "0",
                "gt_fee": "0",
                "create_time": "1640995200",
                "update_time": "1640995200"
            }}"#,
                tif
            );

            let order: OpenOrder = serde_json::from_str(&json).unwrap();
            assert_eq!(order.time_in_force, tif);
        }
    }

    #[test]
    fn test_open_order_array_deserialization() {
        let json = r#"[
            {
                "id": "12345678",
                "currency_pair": "BTC_USDT",
                "status": "open",
                "account": "spot",
                "side": "buy",
                "amount": "0.001",
                "price": "30000",
                "type": "limit",
                "time_in_force": "gtc",
                "filled_amount": "0",
                "left": "0.001",
                "avg_deal_price": "0",
                "fee": "0",
                "fee_currency": "USDT",
                "points_fee": "0",
                "gt_fee": "0",
                "create_time": "1640995200",
                "update_time": "1640995200"
            },
            {
                "id": "87654321",
                "currency_pair": "ETH_USDT",
                "status": "open",
                "account": "margin",
                "side": "sell",
                "amount": "2.0",
                "price": "2500",
                "type": "limit",
                "time_in_force": "gtc",
                "filled_amount": "0.5",
                "left": "1.5",
                "avg_deal_price": "2500",
                "fee": "1.25",
                "fee_currency": "USDT",
                "points_fee": "0",
                "gt_fee": "0.3125",
                "create_time": "1640995200",
                "update_time": "1640995300"
            }
        ]"#;

        let orders: Vec<OpenOrder> = serde_json::from_str(json).unwrap();
        assert_eq!(orders.len(), 2);

        assert_eq!(orders[0].id, "12345678");
        assert_eq!(orders[0].currency_pair, "BTC_USDT");
        assert_eq!(orders[0].side, "buy");

        assert_eq!(orders[1].id, "87654321");
        assert_eq!(orders[1].currency_pair, "ETH_USDT");
        assert_eq!(orders[1].side, "sell");
    }

    #[test]
    fn test_open_order_empty_array() {
        let json = r#"[]"#;
        let orders: Vec<OpenOrder> = serde_json::from_str(json).unwrap();
        assert_eq!(orders.len(), 0);
    }

    #[test]
    fn test_open_order_serialization() {
        let order = OpenOrder {
            id: "12345678".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            status: "open".to_string(),
            account: "spot".to_string(),
            side: "buy".to_string(),
            amount: "0.001".to_string(),
            price: "30000".to_string(),
            order_type: "limit".to_string(),
            time_in_force: "gtc".to_string(),
            filled_amount: "0".to_string(),
            left: "0.001".to_string(),
            avg_deal_price: "0".to_string(),
            fee: "0".to_string(),
            fee_currency: "USDT".to_string(),
            points_fee: "0".to_string(),
            gt_fee: "0".to_string(),
            create_time: "1640995200".to_string(),
            update_time: "1640995200".to_string(),
            text: None,
            iceberg: None,
            auto_borrow: None,
            auto_repay: None,
            stp_act: None,
            finish_as: None,
        };

        let json = serde_json::to_value(&order).unwrap();
        assert_eq!(json["id"], "12345678");
        assert_eq!(json["currency_pair"], "BTC_USDT");
        assert_eq!(json["type"], "limit");
        assert_eq!(json["side"], "buy");
        assert_eq!(json["status"], "open");
    }

    #[test]
    fn test_list_open_orders_request_realistic_scenarios() {
        // Scenario 1: All BTC buy orders
        let btc_buys = ListOpenOrdersRequest {
            currency_pair: Some("BTC_USDT".to_string()),
            side: Some("buy".to_string()),
            account: Some("spot".to_string()),
            limit: Some(50),
            ..Default::default()
        };

        let btc_serialized = serde_urlencoded::to_string(&btc_buys).unwrap();
        assert!(btc_serialized.contains("currency_pair=BTC_USDT"));
        assert!(btc_serialized.contains("side=buy"));
        assert!(btc_serialized.contains("account=spot"));

        // Scenario 2: All open margin orders
        let margin_orders = ListOpenOrdersRequest {
            account: Some("margin".to_string()),
            limit: Some(100),
            ..Default::default()
        };

        let margin_serialized = serde_urlencoded::to_string(&margin_orders).unwrap();
        assert!(margin_serialized.contains("account=margin"));
        assert!(margin_serialized.contains("limit=100"));

        // Scenario 3: Paginated results
        let paginated = ListOpenOrdersRequest {
            page: Some(2),
            limit: Some(25),
            ..Default::default()
        };

        let paginated_serialized = serde_urlencoded::to_string(&paginated).unwrap();
        assert!(paginated_serialized.contains("page=2"));
        assert!(paginated_serialized.contains("limit=25"));
    }

    #[test]
    fn test_list_open_orders_request_default_values() {
        let request = ListOpenOrdersRequest::default();

        assert_eq!(request.currency_pair, None);
        assert_eq!(request.page, None);
        assert_eq!(request.limit, None);
        assert_eq!(request.side, None);
        assert_eq!(request.account, None);
    }

    #[test]
    fn test_list_open_orders_request_clone() {
        let original = ListOpenOrdersRequest {
            currency_pair: Some("BTC_USDT".to_string()),
            page: Some(1),
            limit: Some(50),
            side: Some("buy".to_string()),
            account: Some("spot".to_string()),
        };

        let cloned = original.clone();
        assert_eq!(cloned.currency_pair, original.currency_pair);
        assert_eq!(cloned.page, original.page);
        assert_eq!(cloned.limit, original.limit);
        assert_eq!(cloned.side, original.side);
        assert_eq!(cloned.account, original.account);
    }

    #[test]
    fn test_list_open_orders_request_debug() {
        let request = ListOpenOrdersRequest {
            currency_pair: Some("BTC_USDT".to_string()),
            side: Some("buy".to_string()),
            account: Some("spot".to_string()),
            ..Default::default()
        };

        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("ListOpenOrdersRequest"));
        assert!(debug_str.contains("BTC_USDT"));
        assert!(debug_str.contains("buy"));
        assert!(debug_str.contains("spot"));
    }

    #[test]
    fn test_open_order_clone() {
        let original = OpenOrder {
            id: "12345".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            status: "open".to_string(),
            account: "spot".to_string(),
            side: "buy".to_string(),
            amount: "1.0".to_string(),
            price: "30000".to_string(),
            order_type: "limit".to_string(),
            time_in_force: "gtc".to_string(),
            filled_amount: "0".to_string(),
            left: "1.0".to_string(),
            avg_deal_price: "0".to_string(),
            fee: "0".to_string(),
            fee_currency: "USDT".to_string(),
            points_fee: "0".to_string(),
            gt_fee: "0".to_string(),
            create_time: "1640995200".to_string(),
            update_time: "1640995200".to_string(),
            text: Some("test".to_string()),
            iceberg: None,
            auto_borrow: None,
            auto_repay: None,
            stp_act: None,
            finish_as: None,
        };

        let cloned = original.clone();
        assert_eq!(cloned.id, original.id);
        assert_eq!(cloned.currency_pair, original.currency_pair);
        assert_eq!(cloned.text, original.text);
    }

    #[test]
    fn test_open_order_debug() {
        let order = OpenOrder {
            id: "12345".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            status: "open".to_string(),
            account: "spot".to_string(),
            side: "buy".to_string(),
            amount: "1.0".to_string(),
            price: "30000".to_string(),
            order_type: "limit".to_string(),
            time_in_force: "gtc".to_string(),
            filled_amount: "0".to_string(),
            left: "1.0".to_string(),
            avg_deal_price: "0".to_string(),
            fee: "0".to_string(),
            fee_currency: "USDT".to_string(),
            points_fee: "0".to_string(),
            gt_fee: "0".to_string(),
            create_time: "1640995200".to_string(),
            update_time: "1640995200".to_string(),
            text: None,
            iceberg: None,
            auto_borrow: None,
            auto_repay: None,
            stp_act: None,
            finish_as: None,
        };

        let debug_str = format!("{:?}", order);
        assert!(debug_str.contains("OpenOrder"));
        assert!(debug_str.contains("12345"));
        assert!(debug_str.contains("BTC_USDT"));
    }
}
