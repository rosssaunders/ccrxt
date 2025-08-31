use serde::{Deserialize, Serialize};

use crate::kucoin::futures::{
    OrderSide, OrderType, ResponseHeaders, RestResponse, Result, StopType, TimeInForce,
    private_client::RestClient,
};

/// Endpoint URL for Add Take Profit and Stop Loss Order
const ADD_TP_SL_ORDER_ENDPOINT: &str = "/api/v1/st-orders";

/// Request parameters for adding take profit and stop loss orders.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddTpSlOrderRequest {
    /// Client order ID. Unique identifier for the order. Maximum length: 36 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_oid: Option<String>,

    /// Order side (buy or sell).
    pub side: OrderSide,

    /// Trading symbol (e.g., "XBTUSDTM").
    pub symbol: String,

    /// Order type (e.g., limit, market).
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Leverage ratio. Used for margin trading. Format: String representation of decimal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,

    /// Whether this order is reduce-only. Only valid for closing positions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,

    /// Whether this order closes the position. Used for position closure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_order: Option<bool>,

    /// Force hold mode. Whether to force hold the position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_hold: Option<bool>,

    /// Order size in lots. Required for most order types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,

    /// Order price. Required for limit orders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Time in force policy for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,

    /// Post only mode. Order will only be placed if it doesn't immediately match.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,

    /// Whether the order is hidden from the order book.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,

    /// Whether this is an iceberg order with partial visibility.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg: Option<bool>,

    /// Visible size for iceberg orders. Must be less than total size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible_size: Option<i64>,

    /// Remark or note for the order. Maximum length: 100 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,

    /// Stop order type (up, down).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<StopType>,

    /// Stop price type identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price_type: Option<String>,

    /// Stop price trigger level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<String>,

    /// Take profit trigger price. When reached, triggers take profit order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_stop_up_price: Option<String>,

    /// Stop loss trigger price. When reached, triggers stop loss order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_stop_down_price: Option<String>,
}

/// Response data from the add take profit and stop loss order endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddTpSlOrderResponse {
    /// Generated order ID for the TP/SL order.
    pub order_id: String,

    /// Client order ID if provided in the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_oid: Option<String>,
}

impl RestClient {
    /// Add Take Profit and Stop Loss Order
    ///
    /// Add a take profit and stop loss order for futures trading. This allows setting
    /// both profit-taking and loss-limiting triggers on a position.
    ///
    /// [docs](https://www.kucoin.com/docs-new/rest/futures-trading/orders/add-take-profit-and-stop-loss-order)
    ///
    /// Rate limit: 3
    ///
    /// # Arguments
    /// * `request` - The TP/SL order parameters
    ///
    /// # Returns
    /// TP/SL order response with the order ID
    pub async fn add_tp_sl_order(
        &self,
        request: AddTpSlOrderRequest,
    ) -> Result<(RestResponse<AddTpSlOrderResponse>, ResponseHeaders)> {
        self.post_with_request(ADD_TP_SL_ORDER_ENDPOINT, &request)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_tp_sl_order_request_serialization() {
        let request = AddTpSlOrderRequest {
            client_oid: Some("tp-sl-test123".to_string()),
            side: OrderSide::Buy,
            symbol: "XBTUSDTM".to_string(),
            order_type: OrderType::Limit,
            leverage: Some("10".to_string()),
            reduce_only: Some(false),
            close_order: Some(false),
            force_hold: Some(false),
            size: Some(1),
            price: Some("50000".to_string()),
            time_in_force: None,
            post_only: None,
            hidden: None,
            iceberg: None,
            visible_size: None,
            remark: Some("TP/SL order".to_string()),
            stop: None,
            stop_price_type: Some("TP".to_string()),
            stop_price: None,
            trigger_stop_up_price: Some("55000".to_string()),
            trigger_stop_down_price: Some("45000".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("XBTUSDTM"));
        assert!(json.contains("buy"));
        assert!(json.contains("limit"));
        assert!(json.contains("triggerStopUpPrice"));
        assert!(json.contains("triggerStopDownPrice"));
        assert!(json.contains("55000"));
        assert!(json.contains("45000"));
        assert!(json.contains("tp-sl-test123"));
    }

    #[test]
    fn test_add_tp_sl_order_request_serialization_minimal() {
        let request = AddTpSlOrderRequest {
            client_oid: None,
            side: OrderSide::Sell,
            symbol: "ETHUSDTM".to_string(),
            order_type: OrderType::Market,
            leverage: None,
            reduce_only: Some(true),
            close_order: None,
            force_hold: None,
            size: Some(2),
            price: None,
            time_in_force: None,
            post_only: None,
            hidden: None,
            iceberg: None,
            visible_size: None,
            remark: None,
            stop: None,
            stop_price_type: None,
            stop_price: None,
            trigger_stop_up_price: Some("3500".to_string()),
            trigger_stop_down_price: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("ETHUSDTM"));
        assert!(json.contains("sell"));
        assert!(json.contains("market"));
        assert!(json.contains("triggerStopUpPrice"));
        assert!(!json.contains("triggerStopDownPrice"));
        assert!(!json.contains("clientOid"));
        assert!(!json.contains("price"));
    }

    #[test]
    fn test_add_tp_sl_order_response_deserialization() {
        let json = r#"{
            "orderId": "5e8c8c2f1a3b4a001c5d8e31",
            "clientOid": "tp-sl-test123"
        }"#;

        let response: AddTpSlOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.order_id, "5e8c8c2f1a3b4a001c5d8e31");
        assert_eq!(response.client_oid, Some("tp-sl-test123".to_string()));
    }

    #[test]
    fn test_add_tp_sl_order_response_deserialization_without_client_oid() {
        let json = r#"{
            "orderId": "5e8c8c2f1a3b4a001c5d8e32"
        }"#;

        let response: AddTpSlOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.order_id, "5e8c8c2f1a3b4a001c5d8e32");
        assert_eq!(response.client_oid, None);
    }

    #[test]
    fn test_add_tp_sl_order_request_with_only_take_profit() {
        let request = AddTpSlOrderRequest {
            client_oid: Some("tp-only-test".to_string()),
            side: OrderSide::Sell,
            symbol: "ETHUSDTM".to_string(),
            order_type: OrderType::Market,
            leverage: Some("5".to_string()),
            reduce_only: Some(true),
            close_order: None,
            force_hold: None,
            size: Some(2),
            price: None,
            time_in_force: None,
            post_only: None,
            hidden: None,
            iceberg: None,
            visible_size: None,
            remark: None,
            stop: None,
            stop_price_type: None,
            stop_price: None,
            trigger_stop_up_price: Some("3500".to_string()),
            trigger_stop_down_price: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("ETHUSDTM"));
        assert!(json.contains("sell"));
        assert!(json.contains("market"));
        assert!(json.contains("triggerStopUpPrice"));
        assert!(!json.contains("triggerStopDownPrice"));
        assert!(json.contains("tp-only-test"));
    }

    #[test]
    fn test_add_tp_sl_order_request_with_only_stop_loss() {
        let request = AddTpSlOrderRequest {
            client_oid: Some("sl-only-test".to_string()),
            side: OrderSide::Buy,
            symbol: "BTCUSDTM".to_string(),
            order_type: OrderType::Limit,
            leverage: Some("15".to_string()),
            reduce_only: Some(false),
            close_order: Some(false),
            force_hold: Some(false),
            size: Some(1),
            price: Some("46000".to_string()),
            time_in_force: Some(TimeInForce::GoodTillCanceled),
            post_only: Some(false),
            hidden: Some(false),
            iceberg: Some(false),
            visible_size: None,
            remark: Some("stop loss only".to_string()),
            stop: None,
            stop_price_type: Some("SL".to_string()),
            stop_price: None,
            trigger_stop_up_price: None,
            trigger_stop_down_price: Some("44000".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("BTCUSDTM"));
        assert!(json.contains("buy"));
        assert!(json.contains("limit"));
        assert!(json.contains("triggerStopDownPrice"));
        assert!(!json.contains("triggerStopUpPrice"));
        assert!(json.contains("sl-only-test"));
        assert!(json.contains("44000"));
        assert!(json.contains("stop loss only"));
    }

    #[test]
    fn test_add_tp_sl_order_endpoint() {
        assert_eq!(ADD_TP_SL_ORDER_ENDPOINT, "/api/v1/st-orders");
    }

    #[test]
    fn test_add_tp_sl_order_request_field_types() {
        let request = AddTpSlOrderRequest {
            client_oid: Some("type-test".to_string()),
            side: OrderSide::Sell,
            symbol: "ETHUSDTM".to_string(),
            order_type: OrderType::Limit,
            leverage: Some("20".to_string()),
            reduce_only: Some(true),
            close_order: Some(false),
            force_hold: Some(false),
            size: Some(5),
            price: Some("3500.50".to_string()),
            time_in_force: Some(TimeInForce::ImmediateOrCancel),
            post_only: Some(false),
            hidden: Some(true),
            iceberg: Some(false),
            visible_size: None,
            remark: Some("field type validation".to_string()),
            stop: None,
            stop_price_type: Some("TP".to_string()),
            stop_price: None,
            trigger_stop_up_price: Some("3600.00".to_string()),
            trigger_stop_down_price: Some("3400.00".to_string()),
        };

        // Verify field types through serialization
        let json = serde_json::to_value(&request).unwrap();

        assert!(json["clientOid"].is_string());
        assert!(json["side"].is_string());
        assert!(json["symbol"].is_string());
        assert!(json["type"].is_string());
        assert!(json["leverage"].is_string());
        assert!(json["reduceOnly"].is_boolean());
        assert!(json["closeOrder"].is_boolean());
        assert!(json["forceHold"].is_boolean());
        assert!(json["size"].is_number());
        assert!(json["price"].is_string());
        assert!(json["timeInForce"].is_string());
        assert!(json["postOnly"].is_boolean());
        assert!(json["hidden"].is_boolean());
        assert!(json["iceberg"].is_boolean());
        assert!(json["remark"].is_string());
        assert!(json["stopPriceType"].is_string());
        assert!(json["triggerStopUpPrice"].is_string());
        assert!(json["triggerStopDownPrice"].is_string());
    }
}
