use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{
    OrderSide, OrderType, ResponseHeaders, RestResponse, Result, StopType, TimeInForce,
};

/// Endpoint URL for Batch Add Orders
const BATCH_ADD_ORDERS_ENDPOINT: &str = "/api/v1/orders/multi";

/// Request parameters for placing multiple futures orders in a single batch.
#[derive(Debug, Clone, Serialize)]
pub struct BatchAddOrdersRequest {
    /// List of orders to place (maximum 20 orders per batch).
    pub orders: Vec<BatchOrderItem>,
}

/// Individual order item in a batch request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchOrderItem {
    /// Client order identifier to ensure unique operations. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_oid: Option<String>,

    /// Order side (buy or sell).
    pub side: OrderSide,

    /// Trading symbol (e.g., "XBTUSDTM").
    pub symbol: String,

    /// Order type (limit, market, etc.).
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Leverage multiplier for the position. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,

    /// Whether the order is reduce-only. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,

    /// Whether the order is a close order. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_order: Option<bool>,

    /// Whether to force hold the position. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_hold: Option<bool>,

    /// Order size/quantity. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,

    /// Order price (required for limit orders). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Time in force specification. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,

    /// Whether the order is post-only. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,

    /// Whether the order is hidden. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,

    /// Whether the order is an iceberg order. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg: Option<bool>,

    /// Visible size for iceberg orders. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible_size: Option<i64>,

    /// Order remark/comment. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,

    /// Stop order type. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<StopType>,

    /// Stop price type specification. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price_type: Option<String>,

    /// Stop trigger price. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<String>,

    /// Margin mode for the order (isolated or cross). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_mode: Option<MarginMode>,
}

/// Margin mode for orders.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum MarginMode {
    /// Isolated margin mode.
    Isolated,
    /// Cross margin mode.
    Cross,
}

/// Response for batch add orders containing success and error information.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BatchAddOrdersResponse {
    /// List of order results, one for each order in the batch.
    pub data: Vec<BatchOrderResult>,
}

/// Individual result for each order in the batch.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchOrderResult {
    /// Generated order ID (null if order failed).
    pub order_id: Option<String>,

    /// Client order ID if provided.
    pub client_oid: Option<String>,

    /// Trading symbol.
    pub symbol: String,

    /// Response code indicating success or failure.
    pub code: String,

    /// Response message.
    pub msg: String,
}

impl super::RestClient {
    /// Place Multiple Orders (Batch)
    ///
    /// Place multiple orders in a single request with up to 20 orders per batch.
    /// Each order in the batch is processed independently.
    ///
    /// [docs](https://www.kucoin.com/docs-new/rest/futures-trading/orders/batch-add-orders)
    ///
    /// Rate limit: 20
    ///
    /// # Arguments
    /// * `request` - The batch add orders request parameters
    ///
    /// # Returns
    /// List of results for each order in the batch, including success/failure status
    pub async fn batch_add_orders(
        &self,
        request: BatchAddOrdersRequest,
    ) -> Result<(RestResponse<BatchAddOrdersResponse>, ResponseHeaders)> {
        self.post_with_request(BATCH_ADD_ORDERS_ENDPOINT, &request)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kucoin::spot::{OrderSide, OrderType};

    #[test]
    fn test_batch_add_orders_request_serialization() {
        let orders = vec![
            BatchOrderItem {
                client_oid: Some("order1".to_string()),
                side: OrderSide::Buy,
                symbol: "XBTUSDTM".to_string(),
                order_type: OrderType::Limit,
                leverage: Some("10".to_string()),
                reduce_only: None,
                close_order: None,
                force_hold: None,
                size: Some(1),
                price: Some("50000".to_string()),
                time_in_force: None,
                post_only: None,
                hidden: None,
                iceberg: None,
                visible_size: None,
                remark: None,
                stop: None,
                stop_price_type: None,
                stop_price: None,
                margin_mode: Some(MarginMode::Isolated),
            },
            BatchOrderItem {
                client_oid: Some("order2".to_string()),
                side: OrderSide::Sell,
                symbol: "ETHUSDTM".to_string(),
                order_type: OrderType::Market,
                leverage: Some("5".to_string()),
                reduce_only: None,
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
                margin_mode: Some(MarginMode::Cross),
            },
        ];

        let request = BatchAddOrdersRequest { orders };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("XBTUSDTM"));
        assert!(json.contains("ETHUSDTM"));
        assert!(json.contains("buy"));
        assert!(json.contains("sell"));
        assert!(json.contains("ISOLATED"));
        assert!(json.contains("CROSS"));
        assert!(json.contains("order1"));
        assert!(json.contains("order2"));
    }

    #[test]
    fn test_batch_add_orders_request_serialization_minimal() {
        let orders = vec![BatchOrderItem {
            client_oid: None,
            side: OrderSide::Buy,
            symbol: "ADAUSDTM".to_string(),
            order_type: OrderType::Market,
            leverage: None,
            reduce_only: Some(true),
            close_order: None,
            force_hold: None,
            size: Some(100),
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
            margin_mode: None,
        }];

        let request = BatchAddOrdersRequest { orders };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("ADAUSDTM"));
        assert!(json.contains("buy"));
        assert!(json.contains("market"));
        assert!(!json.contains("clientOid"));
        assert!(!json.contains("price"));
        assert!(!json.contains("marginMode"));
    }

    #[test]
    fn test_batch_add_orders_response_deserialization() {
        let json = r#"{
            "data": [
                {
                    "orderId": "5e8c8c2f1a3b4a001c5d8e31",
                    "clientOid": "order1",
                    "symbol": "XBTUSDTM",
                    "code": "200000",
                    "msg": "success"
                },
                {
                    "orderId": null,
                    "clientOid": "order2",
                    "symbol": "ETHUSDTM",
                    "code": "400001",
                    "msg": "Insufficient balance"
                }
            ]
        }"#;

        let response: BatchAddOrdersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.data.len(), 2);
        assert_eq!(
            response.data[0].order_id,
            Some("5e8c8c2f1a3b4a001c5d8e31".to_string())
        );
        assert_eq!(response.data[0].code, "200000");
        assert_eq!(response.data[0].msg, "success");
        assert_eq!(response.data[1].order_id, None);
        assert_eq!(response.data[1].code, "400001");
        assert_eq!(response.data[1].msg, "Insufficient balance");
    }

    #[test]
    fn test_batch_add_orders_response_deserialization_empty() {
        let json = r#"{
            "data": []
        }"#;

        let response: BatchAddOrdersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.data.len(), 0);
    }

    #[test]
    fn test_margin_mode_serialization() {
        let isolated = MarginMode::Isolated;
        let cross = MarginMode::Cross;

        assert_eq!(serde_json::to_string(&isolated).unwrap(), "\"ISOLATED\"");
        assert_eq!(serde_json::to_string(&cross).unwrap(), "\"CROSS\"");
    }

    #[test]
    fn test_batch_add_orders_endpoint() {
        assert_eq!(BATCH_ADD_ORDERS_ENDPOINT, "/api/v1/orders/multi");
    }

    #[test]
    fn test_batch_order_item_field_types() {
        let order = BatchOrderItem {
            client_oid: Some("test-order-123".to_string()),
            side: OrderSide::Buy,
            symbol: "SOLUSDTM".to_string(),
            order_type: OrderType::Limit,
            leverage: Some("15".to_string()),
            reduce_only: Some(false),
            close_order: Some(true),
            force_hold: Some(false),
            size: Some(10),
            price: Some("100.50".to_string()),
            time_in_force: Some(TimeInForce::GoodTillCanceled),
            post_only: Some(true),
            hidden: Some(false),
            iceberg: Some(false),
            visible_size: Some(5),
            remark: Some("test order".to_string()),
            stop: None,
            stop_price_type: Some("TP".to_string()),
            stop_price: Some("105.00".to_string()),
            margin_mode: Some(MarginMode::Isolated),
        };

        // Verify field types through serialization
        let json = serde_json::to_value(&order).unwrap();

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
        assert!(json["visibleSize"].is_number());
        assert!(json["remark"].is_string());
        assert!(json["stopPriceType"].is_string());
        assert!(json["stopPrice"].is_string());
        assert!(json["marginMode"].is_string());
    }

    #[test]
    fn test_batch_order_result_field_types() {
        let json = r#"{
            "orderId": "5e8c8c2f1a3b4a001c5d8e31",
            "clientOid": "test-order-123",
            "symbol": "XBTUSDTM",
            "code": "200000",
            "msg": "success"
        }"#;

        let result: BatchOrderResult = serde_json::from_str(json).unwrap();

        // Verify deserialization and field types
        assert_eq!(
            result.order_id,
            Some("5e8c8c2f1a3b4a001c5d8e31".to_string())
        );
        assert_eq!(result.client_oid, Some("test-order-123".to_string()));
        assert_eq!(result.symbol, "XBTUSDTM");
        assert_eq!(result.code, "200000");
        assert_eq!(result.msg, "success");

        let json_value = serde_json::to_value(&result).unwrap();
        assert!(json_value["orderId"].is_string());
        assert!(json_value["clientOid"].is_string());
        assert!(json_value["symbol"].is_string());
        assert!(json_value["code"].is_string());
        assert!(json_value["msg"].is_string());
    }

    #[test]
    fn test_batch_add_orders_max_limit() {
        // Test creating a batch with maximum allowed orders (20)
        let mut orders = Vec::new();
        for i in 1..=20 {
            orders.push(BatchOrderItem {
                client_oid: Some(format!("order{}", i)),
                side: OrderSide::Buy,
                symbol: "XBTUSDTM".to_string(),
                order_type: OrderType::Market,
                leverage: None,
                reduce_only: None,
                close_order: None,
                force_hold: None,
                size: Some(1),
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
                margin_mode: None,
            });
        }

        let request = BatchAddOrdersRequest { orders };
        assert_eq!(request.orders.len(), 20);

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("order1"));
        assert!(json.contains("order20"));
    }
}
