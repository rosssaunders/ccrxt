use serde::{Deserialize, Serialize};

use super::{RestClient, common::OkxApiResponse};
use crate::okx::{EndpointType, OrderSide, OrderType, RestResult};

/// Request to get order details
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderRequest {
    /// Instrument ID, e.g. "BTC-USDT"
    pub inst_id: String,

    /// Order ID
    /// Either ordId or clOrdId is required. If both are passed, ordId will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,

    /// Client Order ID as assigned by the client
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
}

/// Order details response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderDetails {
    /// Instrument type
    pub inst_type: String,

    /// Instrument ID
    pub inst_id: String,

    /// Margin currency
    pub ccy: Option<String>,

    /// Order ID
    pub ord_id: String,

    /// Client Order ID as assigned by the client
    pub cl_ord_id: Option<String>,

    /// Order tag
    pub tag: Option<String>,

    /// Order price
    pub px: String,

    /// Quantity to buy or sell
    pub sz: String,

    /// Order type
    pub ord_type: OrderType,

    /// Order side
    pub side: OrderSide,

    /// Position side
    pub pos_side: Option<String>,

    /// Trade mode
    pub td_mode: String,

    /// Accumulated fill quantity
    pub acc_fill_sz: String,

    /// Last filled price
    pub fill_px: Option<String>,

    /// Last trade ID
    pub trade_id: Option<String>,

    /// Last filled quantity
    pub fill_sz: Option<String>,

    /// Last filled time
    pub fill_time: Option<String>,

    /// Average filled price
    pub avg_px: Option<String>,

    /// Order state
    /// "canceled", "live", "partially_filled", "filled"
    pub state: String,

    /// Leverage
    pub lever: Option<String>,

    /// Take-profit trigger price
    pub tp_trigger_px: Option<String>,

    /// Take-profit order price
    pub tp_ord_px: Option<String>,

    /// Stop-loss trigger price
    pub sl_trigger_px: Option<String>,

    /// Stop-loss order price
    pub sl_ord_px: Option<String>,

    /// Fee currency
    pub fee_ccy: Option<String>,

    /// Fee
    pub fee: Option<String>,

    /// Rebate currency
    pub rebate_ccy: Option<String>,

    /// Rebate amount
    pub rebate: Option<String>,

    /// Whether the order can only reduce in position size
    pub reduce_only: Option<bool>,

    /// Category
    pub category: Option<String>,

    /// Creation time
    pub c_time: String,

    /// Update time
    pub u_time: String,

    /// Request ID
    pub req_id: Option<String>,

    /// Amend result
    pub amend_result: Option<String>,

    /// Code
    pub code: Option<String>,

    /// Message
    pub msg: Option<String>,
}

impl RestClient {
    /// Get order details
    ///
    /// # Arguments
    /// * `request` - The get order request
    ///
    /// # Returns
    /// A result containing the order details or an error
    pub async fn get_order(&self, request: &GetOrderRequest) -> RestResult<OkxApiResponse<OrderDetails>> {
        self.send_request(
            "api/v5/trade/order",
            reqwest::Method::GET,
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
    fn test_get_order_request_serialization() {
        let request = GetOrderRequest {
            inst_id: "BTC-USDT".to_string(),
            ord_id: Some("312269865356374016".to_string()),
            cl_ord_id: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("instId=BTC-USDT"));
        assert!(serialized.contains("ordId=312269865356374016"));
        assert!(!serialized.contains("clOrdId"));
    }

    #[test]
    fn test_get_order_request_with_client_id() {
        let request = GetOrderRequest {
            inst_id: "BTC-USDT".to_string(),
            ord_id: None,
            cl_ord_id: Some("my_order_123".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("instId=BTC-USDT"));
        assert!(serialized.contains("clOrdId=my_order_123"));
        assert!(!serialized.contains("ordId"));
    }

    #[test]
    fn test_order_details_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instType": "SPOT",
                    "instId": "BTC-USDT",
                    "ccy": "",
                    "ordId": "312269865356374016",
                    "clOrdId": "my_order_123",
                    "tag": "",
                    "px": "50000.0",
                    "sz": "0.01",
                    "ordType": "limit",
                    "side": "buy",
                    "posSide": "",
                    "tdMode": "cash",
                    "accFillSz": "0",
                    "fillPx": "",
                    "tradeId": "",
                    "fillSz": "",
                    "fillTime": "",
                    "avgPx": "",
                    "state": "live",
                    "lever": "",
                    "tpTriggerPx": "",
                    "tpOrdPx": "",
                    "slTriggerPx": "",
                    "slOrdPx": "",
                    "feeCcy": "",
                    "fee": "",
                    "rebateCcy": "",
                    "rebate": "",
                    "reduceOnly": false,
                    "category": "normal",
                    "cTime": "1597026383085",
                    "uTime": "1597026383085",
                    "reqId": "",
                    "amendResult": "",
                    "code": "",
                    "msg": ""
                }
            ]
        }"#;

        let response: OkxApiResponse<OrderDetails> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let order = &response.data[0];
        assert_eq!(order.inst_id, "BTC-USDT");
        assert_eq!(order.ord_id, "312269865356374016");
        assert_eq!(order.cl_ord_id, Some("my_order_123".to_string()));
        assert_eq!(order.px, "50000.0");
        assert_eq!(order.sz, "0.01");
        assert_eq!(order.side, OrderSide::Buy);
        assert_eq!(order.ord_type, OrderType::Limit);
        assert_eq!(order.state, "live");
    }
}
