use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bybit::{EndpointType, RestResult, enums::*};

/// Endpoint URL for amending orders
const AMEND_ORDER_ENDPOINT: &str = "/v5/order/amend";

/// Request parameters for amending an existing order.
///
/// Allows modification of unfilled or partially filled orders. At least one
/// amendable field must be provided. Either order_id or order_link_id is required.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AmendOrderRequest {
    /// Product type (linear, spot, option, inverse)
    pub category: Category,

    /// Trading symbol (e.g., "BTCUSDT")
    pub symbol: String,

    /// Order ID. Either order_id or order_link_id is required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    /// User-defined order ID. Either order_id or order_link_id is required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,

    /// Implied volatility. Only valid for options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_iv: Option<String>,

    /// Trigger price for conditional orders
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,

    /// Order quantity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<String>,

    /// Order price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Take profit/stop loss mode (Full or Partial)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpsl_mode: Option<TpSlMode>,

    /// Take profit price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<String>,

    /// Stop loss price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<String>,

    /// Take profit trigger price type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_by: Option<TriggerBy>,

    /// Stop loss trigger price type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_by: Option<TriggerBy>,

    /// Trigger price type (LastPrice, IndexPrice, MarkPrice)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_by: Option<TriggerBy>,

    /// Take profit limit price. Only valid when tpOrderType is Limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_limit_price: Option<String>,

    /// Stop loss limit price. Only valid when slOrderType is Limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_limit_price: Option<String>,
}

/// Order amendment result data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmendOrderData {
    /// Unique order ID of the amended order
    pub order_id: String,

    /// User-defined order ID of the amended order
    pub order_link_id: String,
}

/// Response from the amend order API endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct AmendOrderResponse {
    /// Return code (0 indicates success)
    #[serde(rename = "retCode")]
    pub ret_code: i32,

    /// Return message
    #[serde(rename = "retMsg")]
    pub ret_msg: String,

    /// Order amendment result data
    pub result: AmendOrderData,

    /// Extended information (varies by endpoint)
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,

    /// Response timestamp in milliseconds
    pub time: u64,
}

impl RestClient {
    /// Amend an existing order
    ///
    /// Modify unfilled or partially filled orders. At least one amendable field must be provided.
    /// You can amend order price, quantity, trigger price, and take profit/stop loss settings.
    ///
    /// [API Documentation](https://bybit-exchange.github.io/docs/v5/order/amend-order)
    ///
    /// Rate limit: 10 requests per second per UID
    ///
    /// # Arguments
    /// * `request` - The order amendment request parameters including order identification and new values
    ///
    /// # Returns
    /// A result containing the amended order information with updated order ID and status
    pub async fn amend_order(&self, request: AmendOrderRequest) -> RestResult<AmendOrderResponse> {
        self.send_signed_request(
            AMEND_ORDER_ENDPOINT,
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
