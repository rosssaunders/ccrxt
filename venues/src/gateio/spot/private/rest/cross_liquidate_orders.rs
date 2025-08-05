use serde::{Deserialize, Serialize};

use super::RestClient;

const CROSS_LIQUIDATE_ORDERS_ENDPOINT: &str = "/spot/cross_liquidate_orders";

/// Request parameters for creating cross-margin liquidation orders.
///
/// Used to initiate liquidation actions in cross-margin trading mode when positions
/// need to be closed automatically due to margin requirements or risk management.
/// Supports different liquidation strategies including position closure and automatic borrowing/repayment.
#[derive(Debug, Clone, Serialize)]
pub struct CrossLiquidateOrdersRequest {
    /// Trading currency pair to liquidate (e.g., "BTC_USDT", "ETH_USDT").
    pub currency_pair: String,

    /// Type of liquidation action to perform ("close_position", "auto_borrow", "auto_repay").
    #[serde(rename = "type")]
    pub liquidation_type: String,

    /// Optional client identifier for tracking the liquidation order. Maximum 64 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// Response containing details of a cross-margin liquidation order.
///
/// Provides comprehensive information about liquidation orders executed in cross-margin mode,
/// including order status, execution details, and fee information. Used to track the progress
/// and results of automated liquidation actions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossLiquidateOrder {
    /// Unique order identifier assigned by the exchange.
    pub id: String,

    /// Trading currency pair for this liquidation order (e.g., "BTC_USDT").
    pub currency_pair: String,

    /// Current status of the liquidation order ("open", "closed", "cancelled").
    pub status: String,

    /// Account mode for this liquidation order (always "cross_margin" for cross liquidation).
    pub account: String,

    /// Order side indicating direction of liquidation ("buy" or "sell").
    pub side: String,

    /// Total amount of the liquidation order in base currency.
    pub amount: String,

    /// Execution price for the liquidation order. May be "0" for market orders before execution.
    pub price: String,

    /// Type of order used for liquidation ("market", "limit").
    #[serde(rename = "type")]
    pub order_type: String,

    /// Time in force policy applied to the liquidation order ("ioc", "gtc", etc.).
    pub time_in_force: String,

    /// Amount that has been executed/filled during liquidation.
    pub filled_amount: String,

    /// Remaining unfilled amount of the liquidation order.
    pub left: String,

    /// Average execution price for filled portions of the liquidation order.
    pub avg_deal_price: String,

    /// Total trading fee charged for this liquidation order.
    pub fee: String,

    /// Currency in which the liquidation trading fee was charged.
    pub fee_currency: String,

    /// Points fee used for the liquidation order (loyalty program feature).
    pub points_fee: String,

    /// GT (GateToken) fee charged when using GT for fee discount during liquidation.
    pub gt_fee: String,

    /// Unix timestamp when the liquidation order was created (seconds since epoch).
    pub create_time: String,

    /// Unix timestamp when the liquidation order was last updated (seconds since epoch).
    pub update_time: String,

    /// Optional client identifier for tracking this liquidation order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl RestClient {
    /// Create cross liquidate orders
    ///
    /// Creates liquidation orders for cross-margin trading to automatically handle margin calls,
    /// position closures, or risk management. Supports multiple liquidation strategies including
    /// position closure, automatic borrowing for margin maintenance, and debt repayment.
    ///
    /// [docs]: https://www.gate.io/docs/developers/apiv4/en/#create-cross-liquidate-orders
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - Liquidation request with currency pair, liquidation type, and optional client identifier
    ///
    /// # Returns
    /// Details of the created liquidation order including execution status and trade information
    pub async fn cross_liquidate_orders(
        &self,
        request: CrossLiquidateOrdersRequest,
    ) -> crate::gateio::spot::RestResult<CrossLiquidateOrder> {
        self.post(CROSS_LIQUIDATE_ORDERS_ENDPOINT, &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross_liquidate_orders_request_close_position() {
        let request = CrossLiquidateOrdersRequest {
            currency_pair: "BTC_USDT".to_string(),
            liquidation_type: "close_position".to_string(),
            text: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency_pair"], "BTC_USDT");
        assert_eq!(json["type"], "close_position");

        // text should be omitted when None
        let obj = json.as_object().unwrap();
        assert!(!obj.contains_key("text"));
    }

    #[test]
    fn test_cross_liquidate_orders_request_auto_borrow() {
        let request = CrossLiquidateOrdersRequest {
            currency_pair: "ETH_USDT".to_string(),
            liquidation_type: "auto_borrow".to_string(),
            text: Some("emergency_borrow".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency_pair"], "ETH_USDT");
        assert_eq!(json["type"], "auto_borrow");
        assert_eq!(json["text"], "emergency_borrow");
    }

    #[test]
    fn test_cross_liquidate_orders_request_auto_repay() {
        let request = CrossLiquidateOrdersRequest {
            currency_pair: "BNB_USDT".to_string(),
            liquidation_type: "auto_repay".to_string(),
            text: Some("liquidation_repay".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency_pair"], "BNB_USDT");
        assert_eq!(json["type"], "auto_repay");
        assert_eq!(json["text"], "liquidation_repay");
    }

    #[test]
    fn test_cross_liquidate_orders_request_different_currency_pairs() {
        let pairs = vec![
            "BTC_USDT",
            "ETH_USDT",
            "BNB_USDT",
            "SOL_USDC",
            "ETH_BTC",
            "USDC_USDT",
        ];

        for pair in pairs {
            let request = CrossLiquidateOrdersRequest {
                currency_pair: pair.to_string(),
                liquidation_type: "close_position".to_string(),
                text: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["currency_pair"], pair);
        }
    }

    #[test]
    fn test_cross_liquidate_orders_request_different_liquidation_types() {
        let types = vec!["close_position", "auto_borrow", "auto_repay"];

        for liquidation_type in types {
            let request = CrossLiquidateOrdersRequest {
                currency_pair: "BTC_USDT".to_string(),
                liquidation_type: liquidation_type.to_string(),
                text: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["type"], liquidation_type);
        }
    }

    #[test]
    fn test_cross_liquidate_order_response_deserialization() {
        let json = r#"{
            "id": "12345678",
            "currency_pair": "BTC_USDT",
            "status": "open",
            "account": "cross_margin",
            "side": "sell",
            "amount": "0.5",
            "price": "31000",
            "type": "market",
            "time_in_force": "ioc",
            "filled_amount": "0",
            "left": "0.5",
            "avg_deal_price": "0",
            "fee": "0",
            "fee_currency": "USDT",
            "points_fee": "0",
            "gt_fee": "0",
            "create_time": "1640995200",
            "update_time": "1640995200",
            "text": "liquidation_order"
        }"#;

        let response: CrossLiquidateOrder = serde_json::from_str(json).unwrap();
        assert_eq!(response.id, "12345678");
        assert_eq!(response.currency_pair, "BTC_USDT");
        assert_eq!(response.status, "open");
        assert_eq!(response.account, "cross_margin");
        assert_eq!(response.side, "sell");
        assert_eq!(response.amount, "0.5");
        assert_eq!(response.price, "31000");
        assert_eq!(response.order_type, "market");
        assert_eq!(response.time_in_force, "ioc");
        assert_eq!(response.text.as_ref().unwrap(), "liquidation_order");
    }

    #[test]
    fn test_cross_liquidate_order_response_without_text() {
        let json = r#"{
            "id": "87654321",
            "currency_pair": "ETH_USDT",
            "status": "closed",
            "account": "cross_margin",
            "side": "buy",
            "amount": "2.0",
            "price": "2500",
            "type": "limit",
            "time_in_force": "gtc",
            "filled_amount": "2.0",
            "left": "0",
            "avg_deal_price": "2500",
            "fee": "5.0",
            "fee_currency": "USDT",
            "points_fee": "0",
            "gt_fee": "0",
            "create_time": "1640995200",
            "update_time": "1640995300"
        }"#;

        let response: CrossLiquidateOrder = serde_json::from_str(json).unwrap();
        assert_eq!(response.id, "87654321");
        assert_eq!(response.status, "closed");
        assert_eq!(response.filled_amount, "2.0");
        assert_eq!(response.left, "0");
        assert_eq!(response.avg_deal_price, "2500");
        assert_eq!(response.fee, "5.0");
        assert!(response.text.is_none());
    }

    #[test]
    fn test_cross_liquidate_orders_request_realistic_margin_call_scenario() {
        // Scenario: Margin call liquidation for BTC position
        let request = CrossLiquidateOrdersRequest {
            currency_pair: "BTC_USDT".to_string(),
            liquidation_type: "close_position".to_string(),
            text: Some("margin_call_liquidation".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency_pair"], "BTC_USDT");
        assert_eq!(json["type"], "close_position");
        assert_eq!(json["text"], "margin_call_liquidation");
    }

    #[test]
    fn test_cross_liquidate_orders_request_realistic_risk_management_scenario() {
        // Scenario: Auto borrow to maintain margin ratio
        let request = CrossLiquidateOrdersRequest {
            currency_pair: "ETH_USDT".to_string(),
            liquidation_type: "auto_borrow".to_string(),
            text: Some("risk_mgmt_borrow".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency_pair"], "ETH_USDT");
        assert_eq!(json["type"], "auto_borrow");
        assert_eq!(json["text"], "risk_mgmt_borrow");
    }

    #[test]
    fn test_cross_liquidate_orders_request_realistic_debt_repayment_scenario() {
        // Scenario: Auto repay to reduce leverage
        let request = CrossLiquidateOrdersRequest {
            currency_pair: "BNB_USDT".to_string(),
            liquidation_type: "auto_repay".to_string(),
            text: Some("leverage_reduction".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency_pair"], "BNB_USDT");
        assert_eq!(json["type"], "auto_repay");
        assert_eq!(json["text"], "leverage_reduction");
    }

    #[test]
    fn test_cross_liquidate_orders_request_realistic_portfolio_rebalancing() {
        // Scenario: Portfolio rebalancing liquidation
        let request = CrossLiquidateOrdersRequest {
            currency_pair: "SOL_USDC".to_string(),
            liquidation_type: "close_position".to_string(),
            text: Some("portfolio_rebalance".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency_pair"], "SOL_USDC");
        assert_eq!(json["type"], "close_position");
        assert_eq!(json["text"], "portfolio_rebalance");
    }

    #[test]
    fn test_cross_liquidate_order_response_filled_order() {
        let json = r#"{
            "id": "11111111",
            "currency_pair": "BTC_USDT",
            "status": "closed",
            "account": "cross_margin",
            "side": "sell",
            "amount": "1.0",
            "price": "30000",
            "type": "market",
            "time_in_force": "ioc",
            "filled_amount": "1.0",
            "left": "0",
            "avg_deal_price": "30100",
            "fee": "30.1",
            "fee_currency": "USDT",
            "points_fee": "0",
            "gt_fee": "0",
            "create_time": "1640995200",
            "update_time": "1640995205",
            "text": "liquidation_executed"
        }"#;

        let response: CrossLiquidateOrder = serde_json::from_str(json).unwrap();
        assert_eq!(response.status, "closed");
        assert_eq!(response.filled_amount, "1.0");
        assert_eq!(response.left, "0");
        assert_eq!(response.avg_deal_price, "30100");
        assert_eq!(response.fee, "30.1");

        // Verify liquidation was completed
        let filled: f64 = response.filled_amount.parse().unwrap();
        let left: f64 = response.left.parse().unwrap();
        assert_eq!(left, 0.0);
        assert!(filled > 0.0);
    }

    #[test]
    fn test_cross_liquidate_order_response_partial_fill() {
        let json = r#"{
            "id": "22222222",
            "currency_pair": "ETH_USDT",
            "status": "open",
            "account": "cross_margin",
            "side": "buy",
            "amount": "5.0",
            "price": "2500",
            "type": "limit",
            "time_in_force": "gtc",
            "filled_amount": "2.0",
            "left": "3.0",
            "avg_deal_price": "2505",
            "fee": "5.01",
            "fee_currency": "USDT",
            "points_fee": "0",
            "gt_fee": "0",
            "create_time": "1640995200",
            "update_time": "1640995250"
        }"#;

        let response: CrossLiquidateOrder = serde_json::from_str(json).unwrap();
        assert_eq!(response.status, "open");
        assert_eq!(response.filled_amount, "2.0");
        assert_eq!(response.left, "3.0");

        // Verify partial fill math
        let total: f64 = response.amount.parse().unwrap();
        let filled: f64 = response.filled_amount.parse().unwrap();
        let left: f64 = response.left.parse().unwrap();
        assert_eq!(total, filled + left);
    }

    #[test]
    fn test_cross_liquidate_orders_request_edge_case_long_text() {
        let long_text = "a".repeat(200);
        let request = CrossLiquidateOrdersRequest {
            currency_pair: "BTC_USDT".to_string(),
            liquidation_type: "close_position".to_string(),
            text: Some(long_text.clone()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["text"], long_text);
    }

    #[test]
    fn test_cross_liquidate_orders_request_edge_case_empty_text() {
        let request = CrossLiquidateOrdersRequest {
            currency_pair: "BTC_USDT".to_string(),
            liquidation_type: "close_position".to_string(),
            text: Some("".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["text"], "");
    }

    #[test]
    fn test_cross_liquidate_orders_request_clone() {
        let original = CrossLiquidateOrdersRequest {
            currency_pair: "BTC_USDT".to_string(),
            liquidation_type: "close_position".to_string(),
            text: Some("test_order".to_string()),
        };

        let cloned = original.clone();
        assert_eq!(cloned.currency_pair, original.currency_pair);
        assert_eq!(cloned.liquidation_type, original.liquidation_type);
        assert_eq!(cloned.text, original.text);
    }

    #[test]
    fn test_cross_liquidate_order_clone() {
        let original = CrossLiquidateOrder {
            id: "12345678".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            status: "open".to_string(),
            account: "cross_margin".to_string(),
            side: "sell".to_string(),
            amount: "1.0".to_string(),
            price: "30000".to_string(),
            order_type: "market".to_string(),
            time_in_force: "ioc".to_string(),
            filled_amount: "0".to_string(),
            left: "1.0".to_string(),
            avg_deal_price: "0".to_string(),
            fee: "0".to_string(),
            fee_currency: "USDT".to_string(),
            points_fee: "0".to_string(),
            gt_fee: "0".to_string(),
            create_time: "1640995200".to_string(),
            update_time: "1640995200".to_string(),
            text: Some("liquidation".to_string()),
        };

        let cloned = original.clone();
        assert_eq!(cloned.id, original.id);
        assert_eq!(cloned.currency_pair, original.currency_pair);
        assert_eq!(cloned.status, original.status);
        assert_eq!(cloned.text, original.text);
    }

    #[test]
    fn test_cross_liquidate_orders_request_debug() {
        let request = CrossLiquidateOrdersRequest {
            currency_pair: "BTC_USDT".to_string(),
            liquidation_type: "close_position".to_string(),
            text: Some("test".to_string()),
        };

        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("CrossLiquidateOrdersRequest"));
        assert!(debug_str.contains("BTC_USDT"));
        assert!(debug_str.contains("close_position"));
    }

    #[test]
    fn test_cross_liquidate_order_debug() {
        let order = CrossLiquidateOrder {
            id: "12345678".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            status: "open".to_string(),
            account: "cross_margin".to_string(),
            side: "sell".to_string(),
            amount: "1.0".to_string(),
            price: "30000".to_string(),
            order_type: "market".to_string(),
            time_in_force: "ioc".to_string(),
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
        };

        let debug_str = format!("{:?}", order);
        assert!(debug_str.contains("CrossLiquidateOrder"));
        assert!(debug_str.contains("12345678"));
        assert!(debug_str.contains("cross_margin"));
    }

    #[test]
    fn test_cross_liquidate_orders_request_serialization() {
        let request = CrossLiquidateOrdersRequest {
            currency_pair: "BTC_USDT".to_string(),
            liquidation_type: "close_position".to_string(),
            text: Some("test".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        let obj = json.as_object().unwrap();
        assert!(obj.contains_key("currency_pair"));
        assert!(obj.contains_key("type"));
        assert!(obj.contains_key("text"));
        assert_eq!(obj.len(), 3);
    }

    #[test]
    fn test_cross_liquidate_order_serialization() {
        let order = CrossLiquidateOrder {
            id: "12345678".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            status: "open".to_string(),
            account: "cross_margin".to_string(),
            side: "sell".to_string(),
            amount: "1.0".to_string(),
            price: "30000".to_string(),
            order_type: "market".to_string(),
            time_in_force: "ioc".to_string(),
            filled_amount: "0".to_string(),
            left: "1.0".to_string(),
            avg_deal_price: "0".to_string(),
            fee: "0".to_string(),
            fee_currency: "USDT".to_string(),
            points_fee: "0".to_string(),
            gt_fee: "0".to_string(),
            create_time: "1640995200".to_string(),
            update_time: "1640995200".to_string(),
            text: Some("liquidation".to_string()),
        };

        let json = serde_json::to_value(&order).unwrap();
        assert_eq!(json["id"], "12345678");
        assert_eq!(json["currency_pair"], "BTC_USDT");
        assert_eq!(json["type"], "market");
        assert_eq!(json["account"], "cross_margin");
        assert_eq!(json["text"], "liquidation");
    }

    #[test]
    fn test_cross_liquidate_orders_request_endpoint_validation() {
        let request = CrossLiquidateOrdersRequest {
            currency_pair: "BTC_USDT".to_string(),
            liquidation_type: "close_position".to_string(),
            text: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert!(json.as_object().unwrap().contains_key("currency_pair"));
        assert!(json.as_object().unwrap().contains_key("type"));

        // Verify required fields are strings
        assert!(json["currency_pair"].is_string());
        assert!(json["type"].is_string());
    }

    #[test]
    fn test_cross_liquidate_order_round_trip() {
        let original = CrossLiquidateOrder {
            id: "12345678".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            status: "closed".to_string(),
            account: "cross_margin".to_string(),
            side: "sell".to_string(),
            amount: "1.0".to_string(),
            price: "30000".to_string(),
            order_type: "market".to_string(),
            time_in_force: "ioc".to_string(),
            filled_amount: "1.0".to_string(),
            left: "0".to_string(),
            avg_deal_price: "30100".to_string(),
            fee: "30.1".to_string(),
            fee_currency: "USDT".to_string(),
            points_fee: "0".to_string(),
            gt_fee: "0".to_string(),
            create_time: "1640995200".to_string(),
            update_time: "1640995205".to_string(),
            text: Some("liquidation".to_string()),
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: CrossLiquidateOrder = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, original.id);
        assert_eq!(deserialized.currency_pair, original.currency_pair);
        assert_eq!(deserialized.status, original.status);
        assert_eq!(deserialized.account, original.account);
        assert_eq!(deserialized.filled_amount, original.filled_amount);
        assert_eq!(deserialized.avg_deal_price, original.avg_deal_price);
        assert_eq!(deserialized.text, original.text);
    }

    #[test]
    fn test_cross_liquidate_orders_request_optional_text_behavior() {
        // Test with text
        let request_with_text = CrossLiquidateOrdersRequest {
            currency_pair: "BTC_USDT".to_string(),
            liquidation_type: "close_position".to_string(),
            text: Some("liquidation_order".to_string()),
        };

        // Test without text
        let request_without_text = CrossLiquidateOrdersRequest {
            currency_pair: "BTC_USDT".to_string(),
            liquidation_type: "close_position".to_string(),
            text: None,
        };

        let json_with = serde_json::to_value(&request_with_text).unwrap();
        let json_without = serde_json::to_value(&request_without_text).unwrap();

        // With text - should be included
        let obj_with = json_with.as_object().unwrap();
        assert!(obj_with.contains_key("text"));
        assert_eq!(obj_with.len(), 3);

        // Without text - should be omitted
        let obj_without = json_without.as_object().unwrap();
        assert!(!obj_without.contains_key("text"));
        assert_eq!(obj_without.len(), 2);
    }

    #[test]
    fn test_cross_liquidate_order_response_different_account_modes() {
        // Verify that the account field properly reflects cross_margin
        let json = r#"{
            "id": "12345678",
            "currency_pair": "BTC_USDT",
            "status": "open",
            "account": "cross_margin",
            "side": "sell",
            "amount": "1.0",
            "price": "30000",
            "type": "market",
            "time_in_force": "ioc",
            "filled_amount": "0",
            "left": "1.0",
            "avg_deal_price": "0",
            "fee": "0",
            "fee_currency": "USDT",
            "points_fee": "0",
            "gt_fee": "0",
            "create_time": "1640995200",
            "update_time": "1640995200"
        }"#;

        let response: CrossLiquidateOrder = serde_json::from_str(json).unwrap();
        assert_eq!(response.account, "cross_margin");
    }

    #[test]
    fn test_cross_liquidate_orders_request_liquidation_type_validation() {
        let valid_types = vec!["close_position", "auto_borrow", "auto_repay"];

        for liquidation_type in valid_types {
            let request = CrossLiquidateOrdersRequest {
                currency_pair: "BTC_USDT".to_string(),
                liquidation_type: liquidation_type.to_string(),
                text: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["type"], liquidation_type);
        }
    }

    #[test]
    fn test_cross_liquidate_order_response_order_sides() {
        let sides = vec!["buy", "sell"];

        for side in sides {
            let json = format!(
                r#"{{
                "id": "12345678",
                "currency_pair": "BTC_USDT",
                "status": "open",
                "account": "cross_margin",
                "side": "{}",
                "amount": "1.0",
                "price": "30000",
                "type": "market",
                "time_in_force": "ioc",
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
                side
            );

            let response: CrossLiquidateOrder = serde_json::from_str(&json).unwrap();
            assert_eq!(response.side, side);
        }
    }

    #[test]
    fn test_cross_liquidate_order_response_order_statuses() {
        let statuses = vec!["open", "closed", "cancelled"];

        for status in statuses {
            let json = format!(
                r#"{{
                "id": "12345678",
                "currency_pair": "BTC_USDT",
                "status": "{}",
                "account": "cross_margin",
                "side": "sell",
                "amount": "1.0",
                "price": "30000",
                "type": "market",
                "time_in_force": "ioc",
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
                status
            );

            let response: CrossLiquidateOrder = serde_json::from_str(&json).unwrap();
            assert_eq!(response.status, status);
        }
    }
}
