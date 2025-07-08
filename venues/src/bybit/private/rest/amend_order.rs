use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bybit::{EndpointType, RestResult, enums::*};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AmendOrderRequest {
    pub category: Category,
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_iv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpsl_mode: Option<TpSlMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_by: Option<TriggerBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_by: Option<TriggerBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_by: Option<TriggerBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_limit_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_limit_price: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmendOrderData {
    pub order_id: String,
    pub order_link_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AmendOrderResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: AmendOrderData,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

impl RestClient {
    /// Amend an existing order
    ///
    /// Modify unfilled or partially filled orders. At least one amendable field must be provided.
    ///
    /// # Arguments
    /// * `request` - The order amendment request parameters
    ///
    /// # Returns
    /// A result containing the order amendment response or an error
    pub async fn amend_order(&self, request: AmendOrderRequest) -> RestResult<AmendOrderResponse> {
        self.send_signed_request(
            "/v5/order/amend",
            reqwest::Method::POST,
            request,
            EndpointType::Trade,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amend_order_request_by_order_id() {
        let request = AmendOrderRequest {
            category: Category::Linear,
            symbol: "BTCUSDT".to_string(),
            order_id: Some("12345".to_string()),
            order_link_id: None,
            order_iv: None,
            trigger_price: None,
            qty: Some("0.002".to_string()),
            price: Some("51000".to_string()),
            tpsl_mode: None,
            take_profit: None,
            stop_loss: None,
            tp_trigger_by: None,
            sl_trigger_by: None,
            trigger_by: None,
            tp_limit_price: None,
            sl_limit_price: None,
        };

        assert_eq!(request.category, Category::Linear);
        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.order_id, Some("12345".to_string()));
        assert!(request.order_link_id.is_none());
        assert_eq!(request.price, Some("51000".to_string()));
        assert_eq!(request.qty, Some("0.002".to_string()));
    }

    #[test]
    fn test_amend_order_request_by_order_link_id() {
        let request = AmendOrderRequest {
            category: Category::Spot,
            symbol: "ETHUSDT".to_string(),
            order_id: None,
            order_link_id: Some("custom-456".to_string()),
            order_iv: None,
            trigger_price: None,
            qty: None,
            price: None,
            tpsl_mode: None,
            take_profit: Some("3500".to_string()),
            stop_loss: Some("2800".to_string()),
            tp_trigger_by: None,
            sl_trigger_by: None,
            trigger_by: None,
            tp_limit_price: None,
            sl_limit_price: None,
        };

        assert_eq!(request.category, Category::Spot);
        assert_eq!(request.symbol, "ETHUSDT");
        assert!(request.order_id.is_none());
        assert_eq!(request.order_link_id, Some("custom-456".to_string()));
        assert_eq!(request.take_profit, Some("3500".to_string()));
        assert_eq!(request.stop_loss, Some("2800".to_string()));
    }

    #[test]
    fn test_amend_order_request_serialization() {
        let request = AmendOrderRequest {
            category: Category::Linear,
            symbol: "BTCUSDT".to_string(),
            order_id: Some("order123".to_string()),
            order_link_id: None,
            order_iv: None,
            trigger_price: None,
            qty: None,
            price: Some("50000".to_string()),
            tpsl_mode: None,
            take_profit: None,
            stop_loss: None,
            tp_trigger_by: None,
            sl_trigger_by: None,
            trigger_by: None,
            tp_limit_price: None,
            sl_limit_price: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"category\":\"linear\""));
        assert!(json.contains("\"symbol\":\"BTCUSDT\""));
        assert!(json.contains("\"orderId\":\"order123\""));
        assert!(json.contains("\"price\":\"50000\""));
        assert!(!json.contains("orderLinkId")); // Should be skipped when None
    }
}
