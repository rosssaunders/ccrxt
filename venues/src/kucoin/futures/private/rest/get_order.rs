use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{
    OrderSide, OrderStatus, OrderType, ResponseHeaders, RestResponse, Result, TimeInForce,
};

/// Endpoint URL for get order (format string)
pub const GET_ORDER_ENDPOINT: &str = "/api/v1/orders/";

#[derive(Debug, Clone, Serialize)]
pub struct GetOrderRequest {
    pub order_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderDetails {
    pub id: String,
    pub symbol: String,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub side: OrderSide,
    pub size: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<String>,
    pub deal_size: String,
    pub deal_funds: String,
    pub fee: String,
    pub fee_currency: String,
    pub stop: Option<String>,
    pub time_in_force: Option<TimeInForce>,
    pub post_only: bool,
    pub hidden: bool,
    pub iceberg: bool,
    pub leverage: String,
    pub force_hold: bool,
    pub close_order: bool,
    pub visible_size: Option<String>,
    pub client_oid: Option<String>,
    pub remark: Option<String>,
    pub tags: Option<String>,
    pub is_active: bool,
    pub cancel_exist_stop: bool,
    pub created_at: i64,
    pub updated_at: i64,
    pub end_at: Option<i64>,
    pub order_time: i64,
    pub settle_currency: String,
    pub status: OrderStatus,
    pub filled_value: String,
    pub reduce_only: bool,
}

impl super::RestClient {
    /// Get order details
    ///
    /// Reference: <https://www.kucoin.com/docs-new/rest/futures-trading/orders/get-order-by-orderld>
    pub async fn get_order(
        &self,
        request: GetOrderRequest,
    ) -> Result<(RestResponse<OrderDetails>, ResponseHeaders)> {
        let endpoint = format!("{}{}", GET_ORDER_ENDPOINT, request.order_id);
        self.get::<OrderDetails, ()>(&endpoint, None).await
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
    fn test_get_order_endpoint() {
        assert_eq!(GET_ORDER_ENDPOINT, "/api/v1/orders/");
    }

    #[test]
    fn test_get_order_endpoint_formatting() {
        let order_id = "test123";
        let endpoint = format!("{}{}", GET_ORDER_ENDPOINT, order_id);
        assert_eq!(endpoint, "/api/v1/orders/test123");
    }
}
