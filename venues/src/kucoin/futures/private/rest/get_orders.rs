use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{
    OrderSide, OrderStatus, OrderType, ResponseHeaders, RestResponse, Result,
};

/// Endpoint URL for getting orders list
const GET_ORDERS_ENDPOINT: &str = "/api/v1/orders";

/// Request parameters for getting orders list.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrdersRequest {
    /// Filter by order status. Optional parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OrderStatus>,

    /// Filter by trading symbol. Optional parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Filter by order side. Optional parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<OrderSide>,

    /// Filter by order type. Optional parameter.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub order_type: Option<OrderType>,

    /// Start time for filtering (milliseconds since epoch). Optional parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i64>,

    /// End time for filtering (milliseconds since epoch). Optional parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_at: Option<i64>,

    /// Current page number (default: 1). Optional parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_page: Option<i32>,

    /// Number of items per page (default: 50, max: 1000). Optional parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

/// Paginated response containing orders list.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedOrdersResponse {
    /// Current page number.
    pub current_page: i32,

    /// Number of items per page.
    pub page_size: i32,

    /// Total number of items.
    pub total_num: i32,

    /// Total number of pages.
    pub total_page: i32,

    /// List of order details.
    pub items: Vec<super::OrderDetails>,
}

impl super::RestClient {
    /// Get Orders List
    ///
    /// Retrieve a paginated list of orders with optional filtering.
    ///
    /// [docs]: https://www.kucoin.com/docs-new/rest/futures-trading/orders/get-order-list
    ///
    /// Rate limit: 9
    ///
    /// # Arguments
    /// * `request` - The orders list request parameters
    ///
    /// # Returns
    /// Paginated list of orders matching the filter criteria
    pub async fn get_orders(
        &self,
        request: GetOrdersRequest,
    ) -> Result<(RestResponse<PaginatedOrdersResponse>, ResponseHeaders)> {
        self.get(GET_ORDERS_ENDPOINT, Some(&request)).await
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
    fn test_request_serialization() {
        let request = GetOrdersRequest {
            status: Some(OrderStatus::Active),
            symbol: Some("ETHUSDTM".to_string()),
            side: None,
            order_type: None,
            start_at: None,
            end_at: None,
            current_page: Some(2),
            page_size: Some(100),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["status"], "active");
        assert_eq!(json["symbol"], "ETHUSDTM");
        assert_eq!(json["currentPage"], 2);
        assert_eq!(json["pageSize"], 100);

        // Verify optional fields are not serialized when None
        assert!(json.get("side").is_none());
        assert!(json.get("type").is_none());
        assert!(json.get("startAt").is_none());
        assert!(json.get("endAt").is_none());
    }

    #[test]
    fn test_various_symbols() {
        let symbols = ["XBTUSDTM", "ETHUSDTM", "ADAUSDTM", "DOTUSDTM"];

        for symbol in symbols.iter() {
            let request = GetOrdersRequest {
                symbol: Some(symbol.to_string()),
                ..Default::default()
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["symbol"], *symbol);
        }
    }

    #[test]
    fn test_order_status_variations() {
        let statuses = [
            (Some(OrderStatus::Active), "active"),
            (Some(OrderStatus::Done), "done"),
        ];

        for (status, expected_str) in statuses.iter() {
            let request = GetOrdersRequest {
                status: *status,
                ..Default::default()
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["status"], *expected_str);
        }
    }

    #[test]
    fn test_order_side_variations() {
        let sides = [
            (Some(OrderSide::Buy), "buy"),
            (Some(OrderSide::Sell), "sell"),
        ];

        for (side, expected_str) in sides.iter() {
            let request = GetOrdersRequest {
                side: *side,
                ..Default::default()
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["side"], *expected_str);
        }
    }

    #[test]
    fn test_order_type_variations() {
        let types = [
            (Some(OrderType::Limit), "limit"),
            (Some(OrderType::Market), "market"),
        ];

        for (order_type, expected_str) in types.iter() {
            let request = GetOrdersRequest {
                order_type: *order_type,
                ..Default::default()
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["type"], *expected_str);
        }
    }

    #[test]
    fn test_pagination_parameters() {
        let request = GetOrdersRequest {
            current_page: Some(5),
            page_size: Some(200),
            ..Default::default()
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currentPage"], 5);
        assert_eq!(json["pageSize"], 200);
    }

    #[test]
    fn test_time_range_filtering() {
        let request = GetOrdersRequest {
            start_at: Some(1700000000000),
            end_at: Some(1700100000000),
            ..Default::default()
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["startAt"], 1700000000000i64);
        assert_eq!(json["endAt"], 1700100000000i64);
    }

    #[test]
    fn test_camel_case_conversion() {
        let request = GetOrdersRequest {
            start_at: Some(1700000000000),
            end_at: Some(1700100000000),
            current_page: Some(1),
            page_size: Some(50),
            ..Default::default()
        };

        let json = serde_json::to_value(&request).unwrap();

        // Verify camelCase fields exist
        assert!(json.get("startAt").is_some());
        assert!(json.get("endAt").is_some());
        assert!(json.get("currentPage").is_some());
        assert!(json.get("pageSize").is_some());
        // Verify snake_case fields do not exist
        assert!(json.get("start_at").is_none());
        assert!(json.get("end_at").is_none());
        assert!(json.get("current_page").is_none());
        assert!(json.get("page_size").is_none());
    }

    #[test]
    fn test_field_types() {
        let request = GetOrdersRequest {
            status: Some(OrderStatus::Active),
            symbol: Some("XBTUSDTM".to_string()),
            side: Some(OrderSide::Buy),
            order_type: Some(OrderType::Limit),
            start_at: Some(1700000000000),
            end_at: Some(1700100000000),
            current_page: Some(1),
            page_size: Some(50),
        };

        let json = serde_json::to_value(&request).unwrap();

        assert!(json["status"].is_string());
        assert!(json["symbol"].is_string());
        assert!(json["side"].is_string());
        assert!(json["type"].is_string());
        assert!(json["startAt"].is_number());
        assert!(json["endAt"].is_number());
        assert!(json["currentPage"].is_number());
        assert!(json["pageSize"].is_number());
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
    fn test_max_page_size() {
        let request = GetOrdersRequest {
            page_size: Some(1000), // Maximum allowed
            ..Default::default()
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["pageSize"], 1000);
    }

    #[test]
    fn test_endpoint_constant() {
        assert_eq!(GET_ORDERS_ENDPOINT, "/api/v1/orders");
    }

    #[test]
    fn test_response_structure() {
        let response = PaginatedOrdersResponse {
            current_page: 1,
            page_size: 50,
            total_num: 100,
            total_page: 2,
            items: vec![],
        };

        let json = serde_json::to_value(&response).unwrap();
        assert_eq!(json["currentPage"], 1);
        assert_eq!(json["pageSize"], 50);
        assert_eq!(json["totalNum"], 100);
        assert_eq!(json["totalPage"], 2);
        assert!(json["items"].is_array());
    }
}
