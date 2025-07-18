use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{OrderSide, OrderType, OrderStatus, ResponseHeaders, RestResponse, Result};

/// Get stop order list request
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetStopOrderListRequest {
    /// Symbol to filter by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Order side to filter by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<OrderSide>,
    /// Order type to filter by
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub order_type: Option<OrderType>,
    /// Start time in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i64>,
    /// End time in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_at: Option<i64>,
    /// Current page number (default: 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_page: Option<i32>,
    /// Page size (default: 50, max: 1000)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

/// Stop order item
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopOrderItem {
    /// Order ID
    pub id: String,
    /// Symbol
    pub symbol: String,
    /// User ID
    pub user_id: String,
    /// Order status
    pub status: OrderStatus,
    /// Order type
    #[serde(rename = "type")]
    pub order_type: OrderType,
    /// Order side
    pub side: OrderSide,
    /// Order size
    pub size: String,
    /// Order price
    pub price: String,
    /// Stop price
    pub stop_price: String,
    /// Stop price type
    pub stop_price_type: String,
    /// Order created time
    pub created_at: i64,
    /// Order updated time
    pub updated_at: i64,
    /// Client order ID
    pub client_oid: String,
    /// Stop type
    pub stop: String,
    /// Order tags
    pub tags: String,
    /// Order remark
    pub remark: String,
    /// Reduce only flag
    pub reduce_only: bool,
    /// Order leverage
    pub leverage: String,
    /// Force hold flag
    pub force_hold: bool,
    /// Close order flag
    pub close_order: bool,
    /// Time in force
    pub time_in_force: String,
    /// Post only flag
    pub post_only: bool,
    /// Hidden flag
    pub hidden: bool,
    /// Iceberg flag
    pub iceberg: bool,
    /// Visible size
    pub visible_size: String,
    /// Settlement currency
    pub settle_currency: String,
}

/// Paginated stop order list response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetStopOrderListResponse {
    /// Current page number
    pub current_page: i32,
    /// Page size
    pub page_size: i32,
    /// Total number of items
    pub total_num: i32,
    /// Total number of pages
    pub total_page: i32,
    /// List of stop orders
    pub items: Vec<StopOrderItem>,
}

impl super::RestClient {
    /// Get stop order list with optional filtering and pagination
    ///
    /// <https://www.kucoin.com/docs-new/rest/futures-trading/orders/get-stop-order-list>
    pub async fn get_stop_order_list(
        &self,
        request: GetStopOrderListRequest,
    ) -> Result<(RestResponse<GetStopOrderListResponse>, ResponseHeaders)> {
        const GET_STOP_ORDER_LIST_ENDPOINT: &str = "/api/v1/stopOrders";
        self.get(GET_STOP_ORDER_LIST_ENDPOINT, Some(&request)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_stop_order_list_request_creation() {
        let request = GetStopOrderListRequest {
            symbol: Some("XBTUSDTM".to_string()),
            side: Some(OrderSide::Buy),
            order_type: Some(OrderType::Limit),
            start_at: Some(1700000000000),
            end_at: Some(1700100000000),
            current_page: Some(1),
            page_size: Some(50),
        };
        
        assert_eq!(request.symbol, Some("XBTUSDTM".to_string()));
        assert_eq!(request.side, Some(OrderSide::Buy));
        assert_eq!(request.order_type, Some(OrderType::Limit));
        assert_eq!(request.current_page, Some(1));
        assert_eq!(request.page_size, Some(50));
    }

    #[test]
    fn test_stop_order_item_deserialization() {
        let json = r#"{
            "id": "5e8c8c2f1a3b4a001c5d8e31",
            "symbol": "XBTUSDTM",
            "userId": "60fe4956c43cbc0006562c2c",
            "status": "active",
            "type": "limit",
            "side": "buy",
            "size": "1",
            "price": "50000",
            "stopPrice": "49000",
            "stopPriceType": "TP",
            "createdAt": 1700000000000,
            "updatedAt": 1700000001000,
            "clientOid": "test123",
            "stop": "down",
            "tags": "",
            "remark": "test stop order",
            "reduceOnly": false,
            "leverage": "10",
            "forceHold": false,
            "closeOrder": false,
            "timeInForce": "GTC",
            "postOnly": false,
            "hidden": false,
            "iceberg": false,
            "visibleSize": "1",
            "settleCurrency": "USDT"
        }"#;

        let item: StopOrderItem = serde_json::from_str(json).unwrap();
        assert_eq!(item.id, "5e8c8c2f1a3b4a001c5d8e31");
        assert_eq!(item.symbol, "XBTUSDTM");
        assert_eq!(item.side, OrderSide::Buy);
        assert_eq!(item.order_type, OrderType::Limit);
        assert_eq!(item.status, OrderStatus::Active);
        assert_eq!(item.stop_price, "49000");
    }

    #[test]
    fn test_get_stop_order_list_response_deserialization() {
        let json = r#"{
            "currentPage": 1,
            "pageSize": 50,
            "totalNum": 100,
            "totalPage": 2,
            "items": []
        }"#;

        let response: GetStopOrderListResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.current_page, 1);
        assert_eq!(response.page_size, 50);
        assert_eq!(response.total_num, 100);
        assert_eq!(response.total_page, 2);
        assert!(response.items.is_empty());
    }
}