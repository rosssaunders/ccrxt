use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{
    OrderSide, OrderType, ResponseHeaders, RestResponse, Result, StopType, TimeInForce,
};

/// Batch add orders request for futures
#[derive(Debug, Clone, Serialize)]
pub struct BatchAddOrdersRequest {
    /// List of orders to place (maximum 20)
    pub orders: Vec<BatchOrderItem>,
}

/// Individual order item in batch request
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchOrderItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_oid: Option<String>,
    pub side: OrderSide,
    pub symbol: String,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_order: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_hold: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<StopType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_mode: Option<MarginMode>,
}

/// Margin mode for orders
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum MarginMode {
    Isolated,
    Cross,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchAddOrdersResponse {
    pub data: Vec<BatchOrderResult>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchOrderResult {
    pub order_id: Option<String>,
    pub client_oid: Option<String>,
    pub symbol: String,
    pub code: String,
    pub msg: String,
}

impl super::RestClient {
    /// Place multiple orders in a single request (up to 20 orders)
    ///
    /// <https://www.kucoin.com/docs-new/rest/futures-trading/orders/batch-add-orders>
    pub async fn batch_add_orders(
        &self,
        request: BatchAddOrdersRequest,
    ) -> Result<(RestResponse<BatchAddOrdersResponse>, ResponseHeaders)> {
        const BATCH_ADD_ORDERS_ENDPOINT: &str = "/api/v1/orders/multi";
        self.post(BATCH_ADD_ORDERS_ENDPOINT, &request).await
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
        assert_eq!(response.data[0].order_id, Some("5e8c8c2f1a3b4a001c5d8e31".to_string()));
        assert_eq!(response.data[0].code, "200000");
        assert_eq!(response.data[1].order_id, None);
        assert_eq!(response.data[1].code, "400001");
    }

    #[test]
    fn test_margin_mode_serialization() {
        let isolated = MarginMode::Isolated;
        let cross = MarginMode::Cross;
        
        assert_eq!(serde_json::to_string(&isolated).unwrap(), "\"ISOLATED\"");
        assert_eq!(serde_json::to_string(&cross).unwrap(), "\"CROSS\"");
    }
}