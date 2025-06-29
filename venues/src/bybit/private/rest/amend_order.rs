use serde::{Deserialize, Serialize};

use crate::bybit::{enums::*, EndpointType, RestResult};

use super::client::RestClient;

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
    pub async fn amend_order(
        &self,
        request: AmendOrderRequest,
    ) -> RestResult<AmendOrderResponse> {
        self.send_signed_request(
            "/v5/order/amend",
            reqwest::Method::POST,
            request,
            EndpointType::Trade,
        )
        .await
    }
}

impl AmendOrderRequest {
    /// Create a new amend order request by order ID
    pub fn by_order_id(category: Category, symbol: String, order_id: String) -> Self {
        Self {
            category,
            symbol,
            order_id: Some(order_id),
            order_link_id: None,
            order_iv: None,
            trigger_price: None,
            qty: None,
            price: None,
            tpsl_mode: None,
            take_profit: None,
            stop_loss: None,
            tp_trigger_by: None,
            sl_trigger_by: None,
            trigger_by: None,
            tp_limit_price: None,
            sl_limit_price: None,
        }
    }

    /// Create a new amend order request by order link ID
    pub fn by_order_link_id(category: Category, symbol: String, order_link_id: String) -> Self {
        Self {
            category,
            symbol,
            order_id: None,
            order_link_id: Some(order_link_id),
            order_iv: None,
            trigger_price: None,
            qty: None,
            price: None,
            tpsl_mode: None,
            take_profit: None,
            stop_loss: None,
            tp_trigger_by: None,
            sl_trigger_by: None,
            trigger_by: None,
            tp_limit_price: None,
            sl_limit_price: None,
        }
    }

    /// Set new order quantity
    pub fn qty(mut self, qty: String) -> Self {
        self.qty = Some(qty);
        self
    }

    /// Set new order price
    pub fn price(mut self, price: String) -> Self {
        self.price = Some(price);
        self
    }

    /// Set new take profit price (use "0" to cancel existing TP)
    pub fn take_profit(mut self, take_profit: String) -> Self {
        self.take_profit = Some(take_profit);
        self
    }

    /// Set new stop loss price (use "0" to cancel existing SL)
    pub fn stop_loss(mut self, stop_loss: String) -> Self {
        self.stop_loss = Some(stop_loss);
        self
    }

    /// Set trigger price for conditional orders
    pub fn trigger_price(mut self, trigger_price: String) -> Self {
        self.trigger_price = Some(trigger_price);
        self
    }

    /// Set take profit trigger by
    pub fn tp_trigger_by(mut self, tp_trigger_by: TriggerBy) -> Self {
        self.tp_trigger_by = Some(tp_trigger_by);
        self
    }

    /// Set stop loss trigger by
    pub fn sl_trigger_by(mut self, sl_trigger_by: TriggerBy) -> Self {
        self.sl_trigger_by = Some(sl_trigger_by);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amend_order_request_by_order_id() {
        let request = AmendOrderRequest::by_order_id(
            Category::Linear,
            "BTCUSDT".to_string(),
            "12345".to_string(),
        )
        .price("51000".to_string())
        .qty("0.002".to_string());

        assert_eq!(request.category, Category::Linear);
        assert_eq!(request.symbol, "BTCUSDT");
        assert_eq!(request.order_id, Some("12345".to_string()));
        assert!(request.order_link_id.is_none());
        assert_eq!(request.price, Some("51000".to_string()));
        assert_eq!(request.qty, Some("0.002".to_string()));
    }

    #[test]
    fn test_amend_order_request_by_order_link_id() {
        let request = AmendOrderRequest::by_order_link_id(
            Category::Spot,
            "ETHUSDT".to_string(),
            "custom-456".to_string(),
        )
        .take_profit("3500".to_string())
        .stop_loss("2800".to_string());

        assert_eq!(request.category, Category::Spot);
        assert_eq!(request.symbol, "ETHUSDT");
        assert!(request.order_id.is_none());
        assert_eq!(request.order_link_id, Some("custom-456".to_string()));
        assert_eq!(request.take_profit, Some("3500".to_string()));
        assert_eq!(request.stop_loss, Some("2800".to_string()));
    }

    #[test]
    fn test_amend_order_request_serialization() {
        let request = AmendOrderRequest::by_order_id(
            Category::Linear,
            "BTCUSDT".to_string(),
            "order123".to_string(),
        )
        .price("50000".to_string());

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"category\":\"linear\""));
        assert!(json.contains("\"symbol\":\"BTCUSDT\""));
        assert!(json.contains("\"orderId\":\"order123\""));
        assert!(json.contains("\"price\":\"50000\""));
        assert!(!json.contains("orderLinkId")); // Should be skipped when None
    }
}