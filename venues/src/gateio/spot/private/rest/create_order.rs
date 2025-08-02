use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::gateio::spot::{OrderSide, OrderStatus, OrderType, StpMode, TimeInForce};

const CREATE_ORDER_ENDPOINT: &str = "/spot/orders";

/// Order creation request
#[derive(Debug, Clone, Serialize)]
pub struct CreateOrderRequest {
    /// Currency pair
    pub currency_pair: String,

    /// Order type
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Account type (spot, margin, cross_margin)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    /// Order side
    pub side: OrderSide,

    /// Order amount
    pub amount: String,

    /// Order price (required for limit orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Time in force
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,

    /// Iceberg amount (0 for normal orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg: Option<String>,

    /// Self-trade prevention mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp_mode: Option<StpMode>,

    /// Client order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// Order information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    /// Order ID
    pub id: String,

    /// User defined text
    pub text: String,

    /// Whether to cancel remaining orders, only used in batch orders
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amend_text: Option<String>,

    /// Order creation time
    pub create_time: String,

    /// Order update time
    pub update_time: String,

    /// Order status
    pub status: OrderStatus,

    /// Currency pair
    pub currency_pair: String,

    /// Order type
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Account type
    pub account: String,

    /// Order side
    pub side: OrderSide,

    /// Order amount
    pub amount: String,

    /// Order price
    pub price: String,

    /// Time in force
    pub time_in_force: TimeInForce,

    /// Iceberg amount
    pub iceberg: String,

    /// Amount to display
    pub left: String,

    /// Executed amount
    pub filled_amount: String,

    /// Executed value in quote currency
    pub fill_price: String,

    /// Fee paid
    pub fee: String,

    /// Fee currency
    pub fee_currency: String,

    /// Point fee
    pub point_fee: String,

    /// GT fee
    pub gt_fee: String,

    /// GT discount
    pub gt_discount: bool,

    /// Rebated fee
    pub rebated_fee: String,

    /// Rebated fee currency
    pub rebated_fee_currency: String,

    /// Self-trade prevention mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp_mode: Option<StpMode>,

    /// Self-trade prevention triggered
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp_act: Option<String>,
}

impl RestClient {
    /// Create a new order
    ///
    /// This endpoint creates a new spot order.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#create-an-order>
    pub async fn create_order(
        &self,
        order: CreateOrderRequest,
    ) -> crate::gateio::spot::Result<Order> {
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
