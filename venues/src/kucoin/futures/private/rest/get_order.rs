use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{
    OrderSide, OrderStatus, OrderType, ResponseHeaders, RestResponse, Result, TimeInForce,
};

/// Endpoint URL for get order (format string)
const GET_ORDER_ENDPOINT: &str = "/api/v1/orders/{orderId}";

/// Request parameters for getting order details.
#[derive(Debug, Clone, Serialize)]
pub struct GetOrderRequest {
    /// Order ID to retrieve details for. Required parameter.
    pub order_id: String,
}

/// Detailed information about a futures order.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderDetails {
    /// Order ID.
    pub id: String,

    /// Trading symbol (e.g., "XBTUSDTM").
    pub symbol: String,

    /// Order type (limit, market, etc.).
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Order side (buy or sell).
    pub side: OrderSide,

    /// Order size.
    pub size: String,

    /// Order price (for limit orders).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Stop price (for stop orders).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<String>,

    /// Amount of the order that has been filled.
    pub deal_size: String,

    /// Value of the order that has been filled.
    pub deal_funds: String,

    /// Total fee paid for the order.
    pub fee: String,

    /// Currency in which the fee is paid.
    pub fee_currency: String,

    /// Stop order trigger direction.
    pub stop: Option<String>,

    /// Time in force setting for the order.
    pub time_in_force: Option<TimeInForce>,

    /// Whether the order is post-only.
    pub post_only: bool,

    /// Whether the order is hidden.
    pub hidden: bool,

    /// Whether the order is an iceberg order.
    pub iceberg: bool,

    /// Leverage setting for the order.
    pub leverage: String,

    /// Whether the order is force hold.
    pub force_hold: bool,

    /// Whether the order is a close order.
    pub close_order: bool,

    /// Visible size for iceberg orders.
    pub visible_size: Option<String>,

    /// Client-provided order ID.
    pub client_oid: Option<String>,

    /// Order remark/note.
    pub remark: Option<String>,

    /// Order tags.
    pub tags: Option<String>,

    /// Whether the order is currently active.
    pub is_active: bool,

    /// Whether to cancel existing stop orders.
    pub cancel_exist_stop: bool,

    /// Order creation timestamp (milliseconds since epoch).
    pub created_at: i64,

    /// Order last update timestamp (milliseconds since epoch).
    pub updated_at: i64,

    /// Order end timestamp (milliseconds since epoch).
    pub end_at: Option<i64>,

    /// Order time (milliseconds since epoch).
    pub order_time: i64,

    /// Settlement currency.
    pub settle_currency: String,

    /// Current order status.
    pub status: OrderStatus,

    /// Total value filled for the order.
    pub filled_value: String,

    /// Whether the order is reduce-only.
    pub reduce_only: bool,
}

impl super::RestClient {
    /// Get Order Details
    ///
    /// Retrieve detailed information about a specific order by its ID.
    ///
    /// [docs]: https://www.kucoin.com/docs-new/rest/futures-trading/orders/get-order-by-orderld
    ///
    /// Rate limit: 9
    ///
    /// # Arguments
    /// * `request` - The order request containing the order ID
    ///
    /// # Returns
    /// Detailed order information including status, fills, and timestamps
    pub async fn get_order(
        &self,
        request: GetOrderRequest,
    ) -> Result<(RestResponse<OrderDetails>, ResponseHeaders)> {
        let endpoint = GET_ORDER_ENDPOINT.replace("{orderId}", &request.order_id);
        self.get(&endpoint, None::<&()>).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_order_request_creation() {
        let request = GetOrderRequest {
            order_id: "5e8c8c2f1a3b4a001c5d8e31".to_string(),
        };
        assert_eq!(request.order_id, "5e8c8c2f1a3b4a001c5d8e31");
    }

    #[test]
    fn test_order_details_deserialization_complete() {
        let json = r#"{
            "id": "5e8c8c2f1a3b4a001c5d8e31",
            "symbol": "XBTUSDTM",
            "type": "limit",
            "side": "buy",
            "size": "1000",
            "price": "50000",
            "stopPrice": "49500",
            "dealSize": "500",
            "dealFunds": "25000000",
            "fee": "12.5",
            "feeCurrency": "USDT",
            "stop": "up",
            "timeInForce": "GTC",
            "postOnly": false,
            "hidden": false,
            "iceberg": false,
            "leverage": "10",
            "forceHold": false,
            "closeOrder": false,
            "visibleSize": "100",
            "clientOid": "my-order-123",
            "remark": "Test order",
            "tags": "strategy1",
            "isActive": true,
            "cancelExistStop": false,
            "createdAt": 1234567890000,
            "updatedAt": 1234567900000,
            "endAt": 1234568000000,
            "orderTime": 1234567890000,
            "settleCurrency": "USDT",
            "status": "active",
            "filledValue": "25000",
            "reduceOnly": false
        }"#;

        let order: OrderDetails = serde_json::from_str(json).unwrap();
        assert_eq!(order.id, "5e8c8c2f1a3b4a001c5d8e31");
        assert_eq!(order.symbol, "XBTUSDTM");
        assert_eq!(order.order_type, OrderType::Limit);
        assert_eq!(order.side, OrderSide::Buy);
        assert_eq!(order.size, "1000");
        assert_eq!(order.price, Some("50000".to_string()));
        assert_eq!(order.stop_price, Some("49500".to_string()));
        assert_eq!(order.time_in_force, Some(TimeInForce::GoodTillCanceled));
        assert_eq!(order.status, OrderStatus::Active);
        assert_eq!(order.client_oid, Some("my-order-123".to_string()));
        assert_eq!(order.reduce_only, false);
    }

    #[test]
    fn test_order_details_deserialization_minimal() {
        let json = r#"{
            "id": "order123",
            "symbol": "ETHUSDTM",
            "type": "market",
            "side": "sell",
            "size": "500",
            "dealSize": "500",
            "dealFunds": "15000000",
            "fee": "7.5",
            "feeCurrency": "USDT",
            "postOnly": false,
            "hidden": false,
            "iceberg": false,
            "leverage": "5",
            "forceHold": false,
            "closeOrder": false,
            "isActive": false,
            "cancelExistStop": false,
            "createdAt": 1234567890000,
            "updatedAt": 1234567900000,
            "orderTime": 1234567890000,
            "settleCurrency": "USDT",
            "status": "done",
            "filledValue": "15000",
            "reduceOnly": true
        }"#;

        let order: OrderDetails = serde_json::from_str(json).unwrap();
        assert_eq!(order.id, "order123");
        assert_eq!(order.order_type, OrderType::Market);
        assert_eq!(order.side, OrderSide::Sell);
        assert!(order.price.is_none());
        assert!(order.stop_price.is_none());
        assert!(order.time_in_force.is_none());
        assert!(order.client_oid.is_none());
        assert_eq!(order.status, OrderStatus::Done);
        assert_eq!(order.reduce_only, true);
    }

    #[test]
    fn test_request_serialization() {
        let request = GetOrderRequest {
            order_id: "test-order-123".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["order_id"], "test-order-123");
    }

    #[test]
    fn test_various_order_ids() {
        let order_ids = [
            "5e8c8c2f1a3b4a001c5d8e31",
            "order123",
            "TEST-ORDER-456",
            "12345678901234567890",
        ];

        for order_id in order_ids.iter() {
            let request = GetOrderRequest {
                order_id: order_id.to_string(),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["order_id"], *order_id);
        }
    }

    #[test]
    fn test_order_status_variations() {
        let statuses = [("active", OrderStatus::Active), ("done", OrderStatus::Done)];

        for (status_str, expected_status) in statuses.iter() {
            let json = format!(
                r#"{{
                "id": "test123",
                "symbol": "XBTUSDTM",
                "type": "limit",
                "side": "buy",
                "size": "1000",
                "dealSize": "0",
                "dealFunds": "0",
                "fee": "0",
                "feeCurrency": "USDT",
                "postOnly": false,
                "hidden": false,
                "iceberg": false,
                "leverage": "10",
                "forceHold": false,
                "closeOrder": false,
                "isActive": true,
                "cancelExistStop": false,
                "createdAt": 1234567890000,
                "updatedAt": 1234567890000,
                "orderTime": 1234567890000,
                "settleCurrency": "USDT",
                "status": "{}",
                "filledValue": "0",
                "reduceOnly": false
            }}"#,
                status_str
            );

            let order: OrderDetails = serde_json::from_str(&json).unwrap();
            assert_eq!(order.status, *expected_status);
        }
    }

    #[test]
    fn test_order_side_variations() {
        let sides = [("buy", OrderSide::Buy), ("sell", OrderSide::Sell)];

        for (side_str, expected_side) in sides.iter() {
            let json = format!(
                r#"{{
                "id": "test123",
                "symbol": "XBTUSDTM",
                "type": "limit",
                "side": "{}",
                "size": "1000",
                "dealSize": "0",
                "dealFunds": "0",
                "fee": "0",
                "feeCurrency": "USDT",
                "postOnly": false,
                "hidden": false,
                "iceberg": false,
                "leverage": "10",
                "forceHold": false,
                "closeOrder": false,
                "isActive": true,
                "cancelExistStop": false,
                "createdAt": 1234567890000,
                "updatedAt": 1234567890000,
                "orderTime": 1234567890000,
                "settleCurrency": "USDT",
                "status": "active",
                "filledValue": "0",
                "reduceOnly": false
            }}"#,
                side_str
            );

            let order: OrderDetails = serde_json::from_str(&json).unwrap();
            assert_eq!(order.side, *expected_side);
        }
    }

    #[test]
    fn test_order_type_variations() {
        let types = [("limit", OrderType::Limit), ("market", OrderType::Market)];

        for (type_str, expected_type) in types.iter() {
            let json = format!(
                r#"{{
                "id": "test123",
                "symbol": "XBTUSDTM",
                "type": "{}",
                "side": "buy",
                "size": "1000",
                "dealSize": "0",
                "dealFunds": "0",
                "fee": "0",
                "feeCurrency": "USDT",
                "postOnly": false,
                "hidden": false,
                "iceberg": false,
                "leverage": "10",
                "forceHold": false,
                "closeOrder": false,
                "isActive": true,
                "cancelExistStop": false,
                "createdAt": 1234567890000,
                "updatedAt": 1234567890000,
                "orderTime": 1234567890000,
                "settleCurrency": "USDT",
                "status": "active",
                "filledValue": "0",
                "reduceOnly": false
            }}"#,
                type_str
            );

            let order: OrderDetails = serde_json::from_str(&json).unwrap();
            assert_eq!(order.order_type, *expected_type);
        }
    }

    #[test]
    fn test_field_types() {
        let json = r#"{
            "id": "test123",
            "symbol": "XBTUSDTM",
            "type": "limit",
            "side": "buy",
            "size": "1000",
            "price": "50000",
            "dealSize": "500",
            "dealFunds": "25000000",
            "fee": "12.5",
            "feeCurrency": "USDT",
            "postOnly": false,
            "hidden": false,
            "iceberg": false,
            "leverage": "10",
            "forceHold": false,
            "closeOrder": false,
            "isActive": true,
            "cancelExistStop": false,
            "createdAt": 1234567890000,
            "updatedAt": 1234567890000,
            "orderTime": 1234567890000,
            "settleCurrency": "USDT",
            "status": "active",
            "filledValue": "25000",
            "reduceOnly": false
        }"#;

        let order: OrderDetails = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_value(&order).unwrap();

        assert!(serialized["id"].is_string());
        assert!(serialized["symbol"].is_string());
        assert!(serialized["type"].is_string());
        assert!(serialized["side"].is_string());
        assert!(serialized["size"].is_string());
        assert!(serialized["leverage"].is_string());
        assert!(serialized["postOnly"].is_boolean());
        assert!(serialized["hidden"].is_boolean());
        assert!(serialized["iceberg"].is_boolean());
        assert!(serialized["isActive"].is_boolean());
        assert!(serialized["createdAt"].is_number());
    }

    #[test]
    fn test_camel_case_conversion() {
        let json = r#"{
            "id": "test123",
            "symbol": "XBTUSDTM",
            "type": "limit",
            "side": "buy",
            "size": "1000",
            "dealSize": "500",
            "dealFunds": "25000000",
            "fee": "12.5",
            "feeCurrency": "USDT",
            "postOnly": false,
            "hidden": false,
            "iceberg": false,
            "leverage": "10",
            "forceHold": false,
            "closeOrder": false,
            "isActive": true,
            "cancelExistStop": false,
            "createdAt": 1234567890000,
            "updatedAt": 1234567890000,
            "orderTime": 1234567890000,
            "settleCurrency": "USDT",
            "status": "active",
            "filledValue": "25000",
            "reduceOnly": false
        }"#;

        let order: OrderDetails = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_value(&order).unwrap();

        // Verify camelCase fields exist
        assert!(serialized.get("dealSize").is_some());
        assert!(serialized.get("dealFunds").is_some());
        assert!(serialized.get("feeCurrency").is_some());
        assert!(serialized.get("postOnly").is_some());
        assert!(serialized.get("forceHold").is_some());
        assert!(serialized.get("closeOrder").is_some());
        assert!(serialized.get("isActive").is_some());
        assert!(serialized.get("cancelExistStop").is_some());
        assert!(serialized.get("createdAt").is_some());
        assert!(serialized.get("updatedAt").is_some());
        assert!(serialized.get("orderTime").is_some());
        assert!(serialized.get("settleCurrency").is_some());
        assert!(serialized.get("filledValue").is_some());
        assert!(serialized.get("reduceOnly").is_some());
    }

    #[test]
    fn test_optional_fields() {
        let json = r#"{
            "id": "test123",
            "symbol": "XBTUSDTM",
            "type": "market",
            "side": "buy",
            "size": "1000",
            "dealSize": "1000",
            "dealFunds": "50000000",
            "fee": "25",
            "feeCurrency": "USDT",
            "postOnly": false,
            "hidden": false,
            "iceberg": false,
            "leverage": "10",
            "forceHold": false,
            "closeOrder": false,
            "isActive": false,
            "cancelExistStop": false,
            "createdAt": 1234567890000,
            "updatedAt": 1234567890000,
            "orderTime": 1234567890000,
            "settleCurrency": "USDT",
            "status": "done",
            "filledValue": "50000",
            "reduceOnly": false
        }"#;

        let order: OrderDetails = serde_json::from_str(json).unwrap();

        // For market orders, these should be None
        assert!(order.price.is_none());
        assert!(order.stop_price.is_none());
        assert!(order.time_in_force.is_none());
        assert!(order.client_oid.is_none());
        assert!(order.remark.is_none());
        assert!(order.tags.is_none());
        assert!(order.visible_size.is_none());
        assert!(order.end_at.is_none());
    }

    #[test]
    fn test_leverage_string_values() {
        let leverages = ["1", "3", "5", "10", "20", "50", "100"];

        for leverage in leverages.iter() {
            let json = format!(
                r#"{{
                "id": "test123",
                "symbol": "XBTUSDTM",
                "type": "limit",
                "side": "buy",
                "size": "1000",
                "dealSize": "0",
                "dealFunds": "0",
                "fee": "0",
                "feeCurrency": "USDT",
                "postOnly": false,
                "hidden": false,
                "iceberg": false,
                "leverage": "{}",
                "forceHold": false,
                "closeOrder": false,
                "isActive": true,
                "cancelExistStop": false,
                "createdAt": 1234567890000,
                "updatedAt": 1234567890000,
                "orderTime": 1234567890000,
                "settleCurrency": "USDT",
                "status": "active",
                "filledValue": "0",
                "reduceOnly": false
            }}"#,
                leverage
            );

            let order: OrderDetails = serde_json::from_str(&json).unwrap();
            assert_eq!(order.leverage, *leverage);
        }
    }

    #[test]
    fn test_endpoint_constant() {
        assert_eq!(GET_ORDER_ENDPOINT, "/api/v1/orders/{orderId}");
    }

    #[test]
    fn test_endpoint_formatting() {
        let order_id = "test123";
        let endpoint = GET_ORDER_ENDPOINT.replace("{orderId}", order_id);
        assert_eq!(endpoint, "/api/v1/orders/test123");
    }

    #[test]
    fn test_symbol_variations() {
        let symbols = ["XBTUSDTM", "ETHUSDTM", "ADAUSDTM", "DOTUSDTM"];

        for symbol in symbols.iter() {
            let json = format!(
                r#"{{
                "id": "test123",
                "symbol": "{}",
                "type": "limit",
                "side": "buy",
                "size": "1000",
                "dealSize": "0",
                "dealFunds": "0",
                "fee": "0",
                "feeCurrency": "USDT",
                "postOnly": false,
                "hidden": false,
                "iceberg": false,
                "leverage": "10",
                "forceHold": false,
                "closeOrder": false,
                "isActive": true,
                "cancelExistStop": false,
                "createdAt": 1234567890000,
                "updatedAt": 1234567890000,
                "orderTime": 1234567890000,
                "settleCurrency": "USDT",
                "status": "active",
                "filledValue": "0",
                "reduceOnly": false
            }}"#,
                symbol
            );

            let order: OrderDetails = serde_json::from_str(&json).unwrap();
            assert_eq!(order.symbol, *symbol);
        }
    }
}
