use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

const GET_SPREAD_ORDER_ENDPOINT: &str = "/api/v5/sprd/order";

/// Request parameters for getting a spread order
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSpreadOrderRequest {
    /// Order ID
    /// Either `ord_id` or `cl_ord_id` is required. If both are passed, `ord_id` will be used.
    #[serde(rename = "ordId", skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,

    /// Client Order ID as assigned by the client
    /// Either `ord_id` or `cl_ord_id` is required. The latest order will be returned.
    #[serde(rename = "clOrdId", skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
}

/// Response data for getting a spread order
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpreadOrderData {
    /// Spread ID
    #[serde(rename = "sprdId")]
    pub sprd_id: String,

    /// Order ID
    #[serde(rename = "ordId")]
    pub ord_id: String,

    /// Client Order ID as assigned by the client
    #[serde(rename = "clOrdId")]
    pub cl_ord_id: String,

    /// Order tag
    #[serde(rename = "tag")]
    pub tag: String,

    /// Price
    #[serde(rename = "px")]
    pub px: String,

    /// Quantity to buy or sell
    #[serde(rename = "sz")]
    pub sz: String,

    /// Order type
    /// market: Market order
    /// limit: Limit order
    /// post_only: Post-only order
    /// ioc: Immediate-or-cancel order
    #[serde(rename = "ordType")]
    pub ord_type: String,

    /// Order side
    #[serde(rename = "side")]
    pub side: String,

    /// Last fill quantity
    #[serde(rename = "fillSz")]
    pub fill_sz: String,

    /// Last fill price
    #[serde(rename = "fillPx")]
    pub fill_px: String,

    /// Last trade ID
    #[serde(rename = "tradeId")]
    pub trade_id: String,

    /// Accumulated fill quantity
    #[serde(rename = "accFillSz")]
    pub acc_fill_sz: String,

    /// Live quantity
    #[serde(rename = "pendingFillSz")]
    pub pending_fill_sz: String,

    /// Quantity that's pending settlement
    #[serde(rename = "pendingSettleSz")]
    pub pending_settle_sz: String,

    /// Quantity canceled due order cancellations or trade rejections
    #[serde(rename = "canceledSz")]
    pub canceled_sz: String,

    /// Average filled price. If none is filled, it will return "0".
    #[serde(rename = "avgPx")]
    pub avg_px: String,

    /// State
    /// canceled, live, partially_filled, filled
    #[serde(rename = "state")]
    pub state: String,

    /// Source of the order cancellation
    #[serde(rename = "cancelSource")]
    pub cancel_source: String,

    /// Update time, Unix timestamp format in milliseconds
    #[serde(rename = "uTime")]
    pub u_time: String,

    /// Creation time, Unix timestamp format in milliseconds
    #[serde(rename = "cTime")]
    pub c_time: String,
}

impl RestClient {
    /// Get spread order
    ///
    /// Retrieve a spread order by order ID or client order ID
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#spread-trading-rest-api-get-order)
    pub async fn get_spread_order(
        &self,
        request: GetSpreadOrderRequest,
    ) -> RestResult<SpreadOrderData> {
        self.send_get_request(
            GET_SPREAD_ORDER_ENDPOINT,
            Some(&request),
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_get_spread_order_request_with_ord_id() {
        let request = GetSpreadOrderRequest {
            ord_id: Some("312269865356374016".to_string()),
            cl_ord_id: None,
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: GetSpreadOrderRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_get_spread_order_request_with_cl_ord_id() {
        let request = GetSpreadOrderRequest {
            ord_id: None,
            cl_ord_id: Some("client123".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("clOrdId"));
        assert!(!serialized.contains("ordId"));
    }

    #[test]
    fn test_get_spread_order_request_with_both_ids() {
        let request = GetSpreadOrderRequest {
            ord_id: Some("312269865356374016".to_string()),
            cl_ord_id: Some("client123".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("ordId"));
        assert!(serialized.contains("clOrdId"));
    }

    #[test]
    fn test_spread_order_data_deserialization() {
        let json_response = r#"{
            "sprdId": "BTC-USDT_BTC-USDT-SWAP",
            "ordId": "312269865356374016",
            "clOrdId": "client123",
            "tag": "",
            "px": "50000",
            "sz": "1",
            "ordType": "limit",
            "side": "buy",
            "fillSz": "0.5",
            "fillPx": "50000",
            "tradeId": "123456789",
            "accFillSz": "0.5",
            "pendingFillSz": "0.5",
            "pendingSettleSz": "0",
            "canceledSz": "0",
            "avgPx": "50000",
            "state": "partially_filled",
            "cancelSource": "",
            "uTime": "1597026383085",
            "cTime": "1597026383085"
        }"#;

        let order: SpreadOrderData = serde_json::from_str(json_response).unwrap();
        assert_eq!(order.sprd_id, "BTC-USDT_BTC-USDT-SWAP");
        assert_eq!(order.ord_id, "312269865356374016");
        assert_eq!(order.cl_ord_id, "client123");
        assert_eq!(order.px, "50000");
        assert_eq!(order.sz, "1");
        assert_eq!(order.ord_type, "limit");
        assert_eq!(order.side, "buy");
        assert_eq!(order.state, "partially_filled");
    }

    #[test]
    fn test_spread_order_data_serialization() {
        let order = SpreadOrderData {
            sprd_id: "BTC-USDT_BTC-USDT-SWAP".to_string(),
            ord_id: "312269865356374016".to_string(),
            cl_ord_id: "client123".to_string(),
            tag: "".to_string(),
            px: "50000".to_string(),
            sz: "1".to_string(),
            ord_type: "limit".to_string(),
            side: "buy".to_string(),
            fill_sz: "0".to_string(),
            fill_px: "0".to_string(),
            trade_id: "".to_string(),
            acc_fill_sz: "0".to_string(),
            pending_fill_sz: "1".to_string(),
            pending_settle_sz: "0".to_string(),
            canceled_sz: "0".to_string(),
            avg_px: "0".to_string(),
            state: "live".to_string(),
            cancel_source: "".to_string(),
            u_time: "1597026383085".to_string(),
            c_time: "1597026383085".to_string(),
        };

        let serialized = serde_json::to_string(&order).unwrap();
        let deserialized: SpreadOrderData = serde_json::from_str(&serialized).unwrap();
        assert_eq!(order, deserialized);
    }

    #[test]
    fn test_spread_order_states() {
        let states = vec!["live", "partially_filled", "filled", "canceled"];

        for state in states {
            let json = format!(
                r#"{{
                "sprdId": "BTC-USDT_BTC-USDT-SWAP",
                "ordId": "312269865356374016",
                "clOrdId": "client123",
                "tag": "",
                "px": "50000",
                "sz": "1",
                "ordType": "limit",
                "side": "buy",
                "fillSz": "0",
                "fillPx": "0",
                "tradeId": "",
                "accFillSz": "0",
                "pendingFillSz": "1",
                "pendingSettleSz": "0",
                "canceledSz": "0",
                "avgPx": "0",
                "state": "{}",
                "cancelSource": "",
                "uTime": "1597026383085",
                "cTime": "1597026383085"
            }}"#,
                state
            );

            let order: SpreadOrderData = serde_json::from_str(&json).unwrap();
            assert_eq!(order.state, state);
        }
    }

    #[test]
    fn test_spread_order_types() {
        let ord_types = vec!["market", "limit", "post_only", "ioc"];

        for ord_type in ord_types {
            let json = format!(
                r#"{{
                "sprdId": "BTC-USDT_BTC-USDT-SWAP",
                "ordId": "312269865356374016",
                "clOrdId": "client123",
                "tag": "",
                "px": "50000",
                "sz": "1",
                "ordType": "{}",
                "side": "buy",
                "fillSz": "0",
                "fillPx": "0",
                "tradeId": "",
                "accFillSz": "0",
                "pendingFillSz": "1",
                "pendingSettleSz": "0",
                "canceledSz": "0",
                "avgPx": "0",
                "state": "live",
                "cancelSource": "",
                "uTime": "1597026383085",
                "cTime": "1597026383085"
            }}"#,
                ord_type
            );

            let order: SpreadOrderData = serde_json::from_str(&json).unwrap();
            assert_eq!(order.ord_type, ord_type);
        }
    }
}
