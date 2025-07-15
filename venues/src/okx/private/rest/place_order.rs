use serde::{Deserialize, Serialize};

use super::{RestClient, common::OkxApiResponse};
use crate::okx::{EndpointType, OrderSide, OrderType, RestResult};

/// Endpoint URL for placing orders
const PLACE_ORDER_ENDPOINT: &str = "api/v5/trade/order";

/// Request to place a new order
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderRequest {
    /// Instrument ID, e.g. "BTC-USDT"
    pub inst_id: String,

    /// Trade mode
    /// Margin mode: "cross", "isolated"
    /// Non-Margin mode: "cash"
    /// "spot_isolated" (only applicable to SPOT lead trading)
    pub td_mode: String,

    /// Margin currency
    /// Applicable to all isolated MARGIN orders and cross MARGIN orders in Futures mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,

    /// Client Order ID as assigned by the client
    /// A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 32 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,

    /// Order tag
    /// A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 16 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    /// Order side: "buy" or "sell"
    pub side: OrderSide,

    /// Order type
    pub ord_type: OrderType,

    /// Quantity to buy or sell
    pub sz: String,

    /// Order price. Only applicable to limit, post_only, fok, ioc, mmp, mmp_and_post_only order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub px: Option<String>,

    /// Whether orders can only reduce in position size.
    /// Valid options: true or false. The default value is false.
    /// Only applicable to MARGIN orders, and FUTURES/SWAP orders in net mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,

    /// Whether the target currency uses the quote or base currency.
    /// "base_ccy": Base currency, "quote_ccy": Quote currency
    /// Only applicable to SPOT Market Orders
    /// Default is "quote_ccy" for buy, "base_ccy" for sell
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tgt_ccy: Option<String>,

    /// TP/SL information attached when placing order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_algo_ords: Option<Vec<AttachedAlgoOrder>>,
}

/// Attached TP/SL order information
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachedAlgoOrder {
    /// Client-supplied Algo ID when placing order attaching TP/SL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_algo_cl_ord_id: Option<String>,

    /// Take-profit trigger price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_px: Option<String>,

    /// Take-profit order price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_ord_px: Option<String>,

    /// TP order kind: "condition" or "limit"
    /// The default is "condition"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_ord_kind: Option<String>,

    /// Stop-loss trigger price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_px: Option<String>,

    /// Stop-loss order price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_ord_px: Option<String>,

    /// SL order kind: "condition" or "limit"
    /// The default is "condition"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_ord_kind: Option<String>,
}

/// Response from placing an order
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderResponse {
    /// Client Order ID as assigned by the client
    pub cl_ord_id: Option<String>,

    /// Order ID assigned by the system
    pub ord_id: String,

    /// Order tag
    pub tag: Option<String>,

    /// Response code for individual order: "0" means success
    pub s_code: String,

    /// Response message for individual order
    pub s_msg: String,
}

impl RestClient {
    /// Place a new order
    ///
    /// Place orders for spot, margin, futures, perpetual swap, and options.
    ///
    /// [API Documentation](https://www.okx.com/docs-v5/en/#order-book-trading-trade-post-place-order)
    ///
    /// Rate limit: 60 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The order placement request parameters
    ///
    /// # Returns
    /// A result containing the order placement response with order ID and status
    pub async fn place_order(
        &self,
        request: &PlaceOrderRequest,
    ) -> RestResult<OkxApiResponse<PlaceOrderResponse>> {
        self.send_request(
            PLACE_ORDER_ENDPOINT,
            reqwest::Method::POST,
            Some(request),
            EndpointType::PrivateTrading,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_place_order_request_serialization() {
        let request = PlaceOrderRequest {
            inst_id: "BTC-USDT".to_string(),
            td_mode: "cash".to_string(),
            ccy: None,
            cl_ord_id: Some("my_order_123".to_string()),
            tag: None,
            side: OrderSide::Buy,
            ord_type: OrderType::Limit,
            sz: "0.01".to_string(),
            px: Some("50000.0".to_string()),
            reduce_only: None,
            tgt_ccy: None,
            attach_algo_ords: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"instId\":\"BTC-USDT\""));
        assert!(json.contains("\"tdMode\":\"cash\""));
        assert!(json.contains("\"side\":\"buy\""));
        assert!(json.contains("\"ordType\":\"limit\""));
        assert!(json.contains("\"sz\":\"0.01\""));
        assert!(json.contains("\"px\":\"50000.0\""));
        assert!(json.contains("\"clOrdId\":\"my_order_123\""));
    }

    #[test]
    fn test_place_order_response_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "clOrdId": "my_order_123",
                    "ordId": "312269865356374016",
                    "tag": "",
                    "sCode": "0",
                    "sMsg": ""
                }
            ]
        }"#;

        let response: OkxApiResponse<PlaceOrderResponse> =
            serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        assert!(
            !response.data.is_empty(),
            "Expected at least one order in response"
        );
        assert_eq!(
            response.data.first().and_then(|d| d.cl_ord_id.as_ref()),
            Some(&"my_order_123".to_string())
        );
        assert_eq!(
            response.data.first().map(|d| &d.ord_id),
            Some(&"312269865356374016".to_string())
        );
        assert_eq!(
            response.data.first().map(|d| &d.s_code),
            Some(&"0".to_string())
        );
    }

    #[test]
    fn test_attached_algo_order_serialization() {
        let attached_order = AttachedAlgoOrder {
            attach_algo_cl_ord_id: Some("tp_sl_123".to_string()),
            tp_trigger_px: Some("55000.0".to_string()),
            tp_ord_px: Some("55000.0".to_string()),
            tp_ord_kind: Some("condition".to_string()),
            sl_trigger_px: Some("45000.0".to_string()),
            sl_ord_px: Some("45000.0".to_string()),
            sl_ord_kind: Some("condition".to_string()),
        };

        let json = serde_json::to_string(&attached_order).unwrap();
        assert!(json.contains("\"attachAlgoClOrdId\":\"tp_sl_123\""));
        assert!(json.contains("\"tpTriggerPx\":\"55000.0\""));
        assert!(json.contains("\"slTriggerPx\":\"45000.0\""));
    }
}
