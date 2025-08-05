use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::gateio::spot::{OrderSide, OrderStatus, OrderType, StpMode, TimeInForce};

const CREATE_ORDER_ENDPOINT: &str = "/spot/orders";

/// Request parameters for creating a new spot order.
///
/// This request creates a new trading order with specified parameters including
/// trading pair, order type, side, and amount. Supports various order configurations
/// including limit orders, market orders, and iceberg orders.
#[derive(Debug, Clone, Serialize)]
pub struct CreateOrderRequest {
    /// Trading currency pair (e.g., "BTC_USDT", "ETH_BTC").
    pub currency_pair: String,

    /// Type of order to create (limit, market).
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Account type for the order ("spot", "margin", "cross_margin"). If not specified, defaults to spot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    /// Side of the order (buy or sell).
    pub side: OrderSide,

    /// Amount to buy or sell in base currency. For buy market orders, amount is in quote currency.
    pub amount: String,

    /// Price per unit for limit orders. Required for limit orders, omitted for market orders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Time in force policy for the order (GTC, IOC, POC, FOK). Defaults to GTC if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,

    /// Iceberg order amount to display on order book. Set to "0" for normal orders, or specify amount for iceberg.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg: Option<String>,

    /// Self-trade prevention mode to handle orders that would trade with user's own orders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp_mode: Option<StpMode>,

    /// User-defined text identifier for the order. Maximum 28 characters, alphanumeric and underscore only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// Detailed order information returned from the API.
///
/// Contains comprehensive order details including execution status, timing,
/// fees, and trade information. Used for both newly created orders and
/// queried order status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    /// Unique order identifier assigned by the exchange.
    pub id: String,

    /// User-defined text identifier for the order. Limited to 28 characters.
    pub text: String,

    /// Amendment text for modified orders. Used only in batch order operations when canceling remaining orders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amend_text: Option<String>,

    /// Unix timestamp when the order was created (seconds since epoch).
    pub create_time: String,

    /// Unix timestamp when the order was last updated (seconds since epoch).
    pub update_time: String,

    /// Current status of the order (open, closed, cancelled, etc.).
    pub status: OrderStatus,

    /// Trading currency pair for this order (e.g., "BTC_USDT").
    pub currency_pair: String,

    /// Type of order (limit, market).
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Account type used for this order (spot, margin, cross_margin).
    pub account: String,

    /// Side of the order (buy or sell).
    pub side: OrderSide,

    /// Original order amount in base currency.
    pub amount: String,

    /// Order price per unit. For market orders, this may be "0" or average fill price.
    pub price: String,

    /// Time in force policy applied to this order (GTC, IOC, POC, FOK).
    pub time_in_force: TimeInForce,

    /// Iceberg order display amount. "0" for normal orders, specific amount for iceberg orders.
    pub iceberg: String,

    /// Remaining unfilled amount in base currency.
    pub left: String,

    /// Total amount that has been executed/filled in base currency.
    pub filled_amount: String,

    /// Average execution price for filled portions. Weighted average if multiple fills.
    pub fill_price: String,

    /// Total trading fee paid for this order in fee currency.
    pub fee: String,

    /// Currency in which the trading fee was charged (usually quote currency).
    pub fee_currency: String,

    /// Points fee charged for this order (loyalty program feature).
    pub point_fee: String,

    /// GT (GateToken) fee charged when using GT for fee discount.
    pub gt_fee: String,

    /// Whether GT fee discount was applied to this order.
    pub gt_discount: bool,

    /// Amount of fee rebated due to maker/loyalty programs.
    pub rebated_fee: String,

    /// Currency in which the rebated fee was credited.
    pub rebated_fee_currency: String,

    /// Self-trade prevention mode that was applied to this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp_mode: Option<StpMode>,

    /// Action taken if self-trade prevention was triggered (cancel_newest, cancel_oldest, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp_act: Option<String>,
}

impl RestClient {
    /// Create an order
    ///
    /// Creates a new spot trading order with the specified parameters. Supports limit orders,
    /// market orders, and various time-in-force options. Orders can be configured with iceberg
    /// display amounts and self-trade prevention settings.
    ///
    /// [docs]: https://www.gate.io/docs/developers/apiv4/en/#create-an-order
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `order` - Order creation request with trading pair, amount, price, and other parameters
    ///
    /// # Returns
    /// Created order details including order ID, status, and execution information
    pub async fn create_order(
        &self,
        order: CreateOrderRequest,
    ) -> crate::gateio::spot::RestResult<Order> {
        self.post(CREATE_ORDER_ENDPOINT, &order).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gateio::spot::enums::{OrderSide, OrderStatus, OrderType, TimeInForce};

    #[test]
    fn test_request_validation() {
        // Test valid order request
        let valid_order = CreateOrderRequest {
            currency_pair: "BTC_USDT".to_string(),
            order_type: OrderType::Limit,
            account: Some("spot".to_string()),
            side: OrderSide::Buy,
            amount: "0.001".to_string(),
            price: Some("30000".to_string()),
            time_in_force: Some(TimeInForce::GoodTillCanceled),
            text: Some("test_order".to_string()),
            iceberg: None,
            stp_mode: None,
        };

        // Verify required fields are present
        assert!(!valid_order.currency_pair.is_empty());
        assert!(matches!(valid_order.side, OrderSide::Buy));
        assert!(!valid_order.amount.is_empty());

        // Test amount parsing
        assert!(valid_order.amount.parse::<f64>().is_ok());
        if let Some(ref price) = valid_order.price {
            assert!(price.parse::<f64>().is_ok());
        }
    }

    #[test]
    fn test_create_order_request_serialization() {
        let request = CreateOrderRequest {
            currency_pair: "BTC_USDT".to_string(),
            order_type: OrderType::Limit,
            account: Some("spot".to_string()),
            side: OrderSide::Buy,
            amount: "0.001".to_string(),
            price: Some("30000".to_string()),
            time_in_force: Some(TimeInForce::GoodTillCanceled),
            iceberg: None,
            stp_mode: None,
            text: Some("client_order_123".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency_pair"], "BTC_USDT");
        assert_eq!(json["type"], "limit");
        assert_eq!(json["account"], "spot");
        assert_eq!(json["side"], "buy");
        assert_eq!(json["amount"], "0.001");
        assert_eq!(json["price"], "30000");
        assert_eq!(json["time_in_force"], "gtc");
        assert_eq!(json["text"], "client_order_123");
    }

    #[test]
    fn test_create_order_request_market_order() {
        let request = CreateOrderRequest {
            currency_pair: "ETH_USDT".to_string(),
            order_type: OrderType::Market,
            account: None,
            side: OrderSide::Sell,
            amount: "1.5".to_string(),
            price: None,
            time_in_force: None,
            iceberg: None,
            stp_mode: None,
            text: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency_pair"], "ETH_USDT");
        assert_eq!(json["type"], "market");
        assert_eq!(json["side"], "sell");
        assert_eq!(json["amount"], "1.5");
        assert!(!json.as_object().unwrap().contains_key("price"));
        assert!(!json.as_object().unwrap().contains_key("account"));
    }

    #[test]
    fn test_create_order_request_with_iceberg() {
        let request = CreateOrderRequest {
            currency_pair: "BTC_USDT".to_string(),
            order_type: OrderType::Limit,
            account: Some("spot".to_string()),
            side: OrderSide::Buy,
            amount: "1.0".to_string(),
            price: Some("40000".to_string()),
            time_in_force: Some(TimeInForce::GoodTillCanceled),
            iceberg: Some("0.1".to_string()),
            stp_mode: None,
            text: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["iceberg"], "0.1");
    }

    #[test]
    fn test_create_order_request_different_pairs() {
        let pairs = vec!["BTC_USDT", "ETH_BTC", "BNB_USDT", "SOL_USDC"];

        for pair in pairs {
            let request = CreateOrderRequest {
                currency_pair: pair.to_string(),
                order_type: OrderType::Limit,
                account: None,
                side: OrderSide::Buy,
                amount: "1.0".to_string(),
                price: Some("1000".to_string()),
                time_in_force: None,
                iceberg: None,
                stp_mode: None,
                text: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["currency_pair"], pair);
        }
    }

    #[test]
    fn test_create_order_request_different_amounts() {
        let amounts = vec!["0.001", "1.5", "100.0", "0.00000001", "999999.99999999"];

        for amount in amounts {
            let request = CreateOrderRequest {
                currency_pair: "BTC_USDT".to_string(),
                order_type: OrderType::Limit,
                account: None,
                side: OrderSide::Buy,
                amount: amount.to_string(),
                price: Some("30000".to_string()),
                time_in_force: None,
                iceberg: None,
                stp_mode: None,
                text: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["amount"], amount);
        }
    }

    #[test]
    fn test_create_order_request_different_prices() {
        let prices = vec!["0.001", "30000", "100000.5", "0.00000001"];

        for price in prices {
            let request = CreateOrderRequest {
                currency_pair: "BTC_USDT".to_string(),
                order_type: OrderType::Limit,
                account: None,
                side: OrderSide::Buy,
                amount: "1.0".to_string(),
                price: Some(price.to_string()),
                time_in_force: None,
                iceberg: None,
                stp_mode: None,
                text: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["price"], price);
        }
    }

    #[test]
    fn test_order_deserialization() {
        let json = r#"{
            "id": "12345678",
            "text": "client_order_123",
            "amend_text": null,
            "create_time": "1640995200",
            "update_time": "1640995300",
            "status": "open",
            "currency_pair": "BTC_USDT",
            "type": "limit",
            "account": "spot",
            "side": "buy",
            "amount": "0.001",
            "price": "30000",
            "time_in_force": "gtc",
            "iceberg": "0",
            "left": "0.001",
            "filled_amount": "0",
            "fill_price": "0",
            "fee": "0",
            "fee_currency": "USDT",
            "point_fee": "0",
            "gt_fee": "0",
            "gt_discount": false,
            "rebated_fee": "0",
            "rebated_fee_currency": "USDT"
        }"#;

        let order: Order = serde_json::from_str(json).unwrap();
        assert_eq!(order.id, "12345678");
        assert_eq!(order.text, "client_order_123");
        assert_eq!(order.currency_pair, "BTC_USDT");
        assert_eq!(order.amount, "0.001");
        assert_eq!(order.price, "30000");
        assert_eq!(order.left, "0.001");
        assert_eq!(order.filled_amount, "0");
        assert_eq!(order.gt_discount, false);
    }

    #[test]
    fn test_order_filled_status() {
        let json = r#"{
            "id": "87654321",
            "text": "filled_order",
            "create_time": "1640995200",
            "update_time": "1640995400",
            "status": "closed",
            "currency_pair": "ETH_USDT",
            "type": "market",
            "account": "spot",
            "side": "sell",
            "amount": "1.0",
            "price": "2500",
            "time_in_force": "ioc",
            "iceberg": "0",
            "left": "0",
            "filled_amount": "1.0",
            "fill_price": "2500",
            "fee": "2.5",
            "fee_currency": "USDT",
            "point_fee": "0",
            "gt_fee": "0",
            "gt_discount": false,
            "rebated_fee": "0",
            "rebated_fee_currency": "USDT"
        }"#;

        let order: Order = serde_json::from_str(json).unwrap();
        assert_eq!(order.id, "87654321");
        assert_eq!(order.filled_amount, "1.0");
        assert_eq!(order.left, "0");
        assert_eq!(order.fill_price, "2500");
        assert_eq!(order.fee, "2.5");
        assert_eq!(order.fee_currency, "USDT");
    }

    #[test]
    fn test_order_with_gt_discount() {
        let json = r#"{
            "id": "11111111",
            "text": "gt_discounted_order",
            "create_time": "1640995200",
            "update_time": "1640995300",
            "status": "closed",
            "currency_pair": "BTC_USDT",
            "type": "limit",
            "account": "spot",
            "side": "buy",
            "amount": "0.5",
            "price": "30000",
            "time_in_force": "gtc",
            "iceberg": "0",
            "left": "0",
            "filled_amount": "0.5",
            "fill_price": "30000",
            "fee": "15",
            "fee_currency": "USDT",
            "point_fee": "0",
            "gt_fee": "3.75",
            "gt_discount": true,
            "rebated_fee": "0",
            "rebated_fee_currency": "USDT"
        }"#;

        let order: Order = serde_json::from_str(json).unwrap();
        assert_eq!(order.gt_discount, true);
        assert_eq!(order.gt_fee, "3.75");
        assert_eq!(order.fee, "15");

        // Verify GT discount calculation
        let total_fee: f64 = order.fee.parse().unwrap();
        let gt_fee: f64 = order.gt_fee.parse().unwrap();
        assert!(gt_fee < total_fee); // GT fee should be less than base fee
    }

    #[test]
    fn test_order_with_rebate() {
        let json = r#"{
            "id": "22222222",
            "text": "rebated_order",
            "create_time": "1640995200",
            "update_time": "1640995300",
            "status": "closed",
            "currency_pair": "ETH_USDT",
            "type": "limit",
            "account": "spot",
            "side": "sell",
            "amount": "2.0",
            "price": "2500",
            "time_in_force": "gtc",
            "iceberg": "0",
            "left": "0",
            "filled_amount": "2.0",
            "fill_price": "2500",
            "fee": "5.0",
            "fee_currency": "USDT",
            "point_fee": "0",
            "gt_fee": "0",
            "gt_discount": false,
            "rebated_fee": "1.25",
            "rebated_fee_currency": "USDT"
        }"#;

        let order: Order = serde_json::from_str(json).unwrap();
        assert_eq!(order.rebated_fee, "1.25");
        assert_eq!(order.rebated_fee_currency, "USDT");

        // Verify rebate is positive
        let rebate: f64 = order.rebated_fee.parse().unwrap();
        assert!(rebate > 0.0);
    }

    #[test]
    fn test_order_iceberg_order() {
        let json = r#"{
            "id": "33333333",
            "text": "iceberg_order",
            "create_time": "1640995200",
            "update_time": "1640995300",
            "status": "open",
            "currency_pair": "BTC_USDT",
            "type": "limit",
            "account": "spot",
            "side": "buy",
            "amount": "1.0",
            "price": "30000",
            "time_in_force": "gtc",
            "iceberg": "0.1",
            "left": "0.8",
            "filled_amount": "0.2",
            "fill_price": "30000",
            "fee": "6",
            "fee_currency": "USDT",
            "point_fee": "0",
            "gt_fee": "0",
            "gt_discount": false,
            "rebated_fee": "0",
            "rebated_fee_currency": "USDT"
        }"#;

        let order: Order = serde_json::from_str(json).unwrap();
        assert_eq!(order.iceberg, "0.1");
        assert_eq!(order.amount, "1.0");
        assert_eq!(order.left, "0.8");
        assert_eq!(order.filled_amount, "0.2");

        // Verify iceberg logic
        let total: f64 = order.amount.parse().unwrap();
        let filled: f64 = order.filled_amount.parse().unwrap();
        let left: f64 = order.left.parse().unwrap();
        assert_eq!(total, filled + left);
    }

    #[test]
    fn test_order_cancelled_status() {
        let json = r#"{
            "id": "44444444",
            "text": "cancelled_order",
            "create_time": "1640995200",
            "update_time": "1640995250",
            "status": "cancelled",
            "currency_pair": "BNB_USDT",
            "type": "limit",
            "account": "spot",
            "side": "buy",
            "amount": "10.0",
            "price": "300",
            "time_in_force": "gtc",
            "iceberg": "0",
            "left": "10.0",
            "filled_amount": "0",
            "fill_price": "0",
            "fee": "0",
            "fee_currency": "USDT",
            "point_fee": "0",
            "gt_fee": "0",
            "gt_discount": false,
            "rebated_fee": "0",
            "rebated_fee_currency": "USDT"
        }"#;

        let order: Order = serde_json::from_str(json).unwrap();
        assert_eq!(order.filled_amount, "0");
        assert_eq!(order.left, "10.0");
        assert_eq!(order.fill_price, "0");
        assert_eq!(order.fee, "0");
    }

    #[test]
    fn test_order_different_currencies() {
        let currencies = vec![
            ("BTC_USDT", "BTC", "USDT"),
            ("ETH_BTC", "ETH", "BTC"),
            ("BNB_USDT", "BNB", "USDT"),
            ("SOL_USDC", "SOL", "USDC"),
        ];

        for (pair, _base, quote) in currencies {
            let json = format!(
                r#"{{
                "id": "12345",
                "text": "test",
                "create_time": "1640995200",
                "update_time": "1640995300",
                "status": "closed",
                "currency_pair": "{}",
                "type": "market",
                "account": "spot",
                "side": "buy",
                "amount": "1.0",
                "price": "100",
                "time_in_force": "ioc",
                "iceberg": "0",
                "left": "0",
                "filled_amount": "1.0",
                "fill_price": "100",
                "fee": "0.1",
                "fee_currency": "{}",
                "point_fee": "0",
                "gt_fee": "0",
                "gt_discount": false,
                "rebated_fee": "0",
                "rebated_fee_currency": "{}"
            }}"#,
                pair, quote, quote
            );

            let order: Order = serde_json::from_str(&json).unwrap();
            assert_eq!(order.currency_pair, pair);
            assert_eq!(order.fee_currency, quote);
            assert_eq!(order.rebated_fee_currency, quote);
        }
    }

    #[test]
    fn test_order_serialization() {
        let order = Order {
            id: "12345678".to_string(),
            text: "test_order".to_string(),
            amend_text: None,
            create_time: "1640995200".to_string(),
            update_time: "1640995300".to_string(),
            status: OrderStatus::Open,
            currency_pair: "BTC_USDT".to_string(),
            order_type: OrderType::Limit,
            account: "spot".to_string(),
            side: OrderSide::Buy,
            amount: "0.001".to_string(),
            price: "30000".to_string(),
            time_in_force: TimeInForce::GoodTillCanceled,
            iceberg: "0".to_string(),
            left: "0.001".to_string(),
            filled_amount: "0".to_string(),
            fill_price: "0".to_string(),
            fee: "0".to_string(),
            fee_currency: "USDT".to_string(),
            point_fee: "0".to_string(),
            gt_fee: "0".to_string(),
            gt_discount: false,
            rebated_fee: "0".to_string(),
            rebated_fee_currency: "USDT".to_string(),
            stp_mode: None,
            stp_act: None,
        };

        let json = serde_json::to_value(&order).unwrap();
        assert_eq!(json["id"], "12345678");
        assert_eq!(json["currency_pair"], "BTC_USDT");
        assert_eq!(json["type"], "limit");
        assert_eq!(json["side"], "buy");
        assert_eq!(json["status"], "open");
    }

    #[test]
    fn test_order_round_trip() {
        let original = Order {
            id: "87654321".to_string(),
            text: "round_trip_test".to_string(),
            amend_text: Some("amended".to_string()),
            create_time: "1640995200".to_string(),
            update_time: "1640995400".to_string(),
            status: OrderStatus::Closed,
            currency_pair: "ETH_USDT".to_string(),
            order_type: OrderType::Market,
            account: "spot".to_string(),
            side: OrderSide::Sell,
            amount: "1.5".to_string(),
            price: "2500".to_string(),
            time_in_force: TimeInForce::ImmediateOrCancel,
            iceberg: "0".to_string(),
            left: "0".to_string(),
            filled_amount: "1.5".to_string(),
            fill_price: "2500".to_string(),
            fee: "3.75".to_string(),
            fee_currency: "USDT".to_string(),
            point_fee: "0".to_string(),
            gt_fee: "0".to_string(),
            gt_discount: false,
            rebated_fee: "0.5".to_string(),
            rebated_fee_currency: "USDT".to_string(),
            stp_mode: None,
            stp_act: None,
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Order = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, original.id);
        assert_eq!(deserialized.text, original.text);
        assert_eq!(deserialized.amend_text, original.amend_text);
        assert_eq!(deserialized.currency_pair, original.currency_pair);
        assert_eq!(deserialized.amount, original.amount);
        assert_eq!(deserialized.filled_amount, original.filled_amount);
        assert_eq!(deserialized.rebated_fee, original.rebated_fee);
    }
}
