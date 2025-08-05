
use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{
    OrderSide, OrderStatus, OrderType, ResponseHeaders, RestResponse, Result,
};

/// Endpoint URL for get orders
pub const GET_ORDERS_ENDPOINT: &str = "/api/v1/orders";

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetOrdersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OrderStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<OrderSide>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub order_type: Option<OrderType>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "startAt")]
    pub start_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "endAt")]
    pub end_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "currentPage")]
    pub current_page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "pageSize")]
    pub page_size: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedOrdersResponse {
    pub current_page: i32,
    pub page_size: i32,
    pub total_num: i32,
    pub total_page: i32,
    pub items: Vec<super::OrderDetails>,
}

impl super::RestClient {
    /// Get orders with pagination
    ///
    /// Reference: <https://www.kucoin.com/docs-new/rest/futures-trading/orders/get-order-list>
    pub async fn get_orders(
        &self,
        request: GetOrdersRequest,
    ) -> Result<(PaginatedOrdersResponse, ResponseHeaders)> {
        let (response, headers): (RestResponse<PaginatedOrdersResponse>, ResponseHeaders) =
            self.get_with_request(GET_ORDERS_ENDPOINT, &request).await?;

        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_orders_request_default() {
        let request = GetOrdersRequest::default();
        assert!(request.status.is_none());
        assert!(request.symbol.is_none());
        assert!(request.side.is_none());
        assert!(request.order_type.is_none());
        assert!(request.start_at.is_none());
        assert!(request.end_at.is_none());
        assert!(request.current_page.is_none());
        assert!(request.page_size.is_none());
    }

    #[test]
    fn test_get_orders_request_with_all_fields() {
        let request = GetOrdersRequest {
            status: Some(OrderStatus::Active),
            symbol: Some("XBTUSDTM".to_string()),
            side: Some(OrderSide::Buy),
            order_type: Some(OrderType::Limit),
            start_at: Some(1234567890000),
            end_at: Some(1234567900000),
            current_page: Some(1),
            page_size: Some(50),
        };

        assert_eq!(request.status, Some(OrderStatus::Active));
        assert_eq!(request.symbol, Some("XBTUSDTM".to_string()));
        assert_eq!(request.side, Some(OrderSide::Buy));
        assert_eq!(request.order_type, Some(OrderType::Limit));
        assert_eq!(request.start_at, Some(1234567890000));
        assert_eq!(request.end_at, Some(1234567900000));
        assert_eq!(request.current_page, Some(1));
        assert_eq!(request.page_size, Some(50));
    }

    #[test]
    fn test_paginated_orders_response_deserialization() {
        let json = r#"{
            "currentPage": 1,
            "pageSize": 2,
            "totalNum": 5,
            "totalPage": 3,
            "items": [
                {
                    "id": "order1",
                    "symbol": "XBTUSDTM",
                    "type": "limit",
                    "side": "buy",
                    "size": "1000",
                    "price": "50000",
                    "dealSize": "0",
                    "dealFunds": "0",
                    "fee": "0",
                    "feeCurrency": "USDT",
                    "timeInForce": "GTC",
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
                },
                {
                    "id": "order2",
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
                    "createdAt": 1234567880000,
                    "updatedAt": 1234567885000,
                    "orderTime": 1234567880000,
                    "settleCurrency": "USDT",
                    "status": "done",
                    "filledValue": "15000",
                    "reduceOnly": true
                }
            ]
        }"#;

        let response: PaginatedOrdersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.current_page, 1);
        assert_eq!(response.page_size, 2);
        assert_eq!(response.total_num, 5);
        assert_eq!(response.total_page, 3);
        assert_eq!(response.items.len(), 2);

        let order1 = &response.items[0];
        assert_eq!(order1.id, "order1");
        assert_eq!(order1.symbol, "XBTUSDTM");
        assert_eq!(order1.status, OrderStatus::Active);

        let order2 = &response.items[1];
        assert_eq!(order2.id, "order2");
        assert_eq!(order2.symbol, "ETHUSDTM");
        assert_eq!(order2.status, OrderStatus::Done);
    }

    #[test]
    fn test_paginated_orders_response_deserialization_empty() {
        let json = r#"{
            "currentPage": 1,
            "pageSize": 50,
            "totalNum": 0,
            "totalPage": 0,
            "items": []
        }"#;

        let response: PaginatedOrdersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.total_num, 0);
        assert!(response.items.is_empty());
    }

    #[test]
    fn test_get_orders_endpoint() {
        assert_eq!(GET_ORDERS_ENDPOINT, "/api/v1/orders");
    }
}
