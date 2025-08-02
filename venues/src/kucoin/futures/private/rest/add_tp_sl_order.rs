use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{
    OrderSide, OrderType, ResponseHeaders, RestResponse, Result, StopType, TimeInForce,
};

/// Add take profit and stop loss order request
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddTpSlOrderRequest {
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
    /// Take profit trigger price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_stop_up_price: Option<String>,
    /// Stop loss trigger price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_stop_down_price: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddTpSlOrderResponse {
    pub order_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_oid: Option<String>,
}

impl super::RestClient {
    /// Add take profit and stop loss order
    ///
    /// <https://www.kucoin.com/docs-new/rest/futures-trading/orders/add-take-profit-and-stop-loss-order>
    pub async fn add_tp_sl_order(
        &self,
        request: AddTpSlOrderRequest,
    ) -> Result<(RestResponse<AddTpSlOrderResponse>, ResponseHeaders)> {
        const ADD_TP_SL_ORDER_ENDPOINT: &str = "/api/v1/st-orders";
        self.post(ADD_TP_SL_ORDER_ENDPOINT, &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kucoin::spot::{OrderSide, OrderType};

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
    }
}
