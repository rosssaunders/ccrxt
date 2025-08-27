use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

const GET_SPREAD_ORDERS_PENDING_ENDPOINT: &str = "/api/v5/sprd/orders-pending";

/// Request parameters for getting pending spread orders
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSpreadOrdersPendingRequest {
    /// Spread ID
    #[serde(rename = "sprdId", skip_serializing_if = "Option::is_none")]
    pub sprd_id: Option<String>,

    /// Order type
    /// market: Market order
    /// limit: Limit order
    /// post_only: Post-only order
    /// ioc: Immediate-or-cancel order
    #[serde(rename = "ordType", skip_serializing_if = "Option::is_none")]
    pub ord_type: Option<String>,

    /// State
    /// live
    /// partially_filled
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Start order ID the request to begin with
    /// Pagination of data to return records newer than the requested order Id, not including beginId
    #[serde(rename = "beginId", skip_serializing_if = "Option::is_none")]
    pub begin_id: Option<String>,

    /// End order ID the request to end with
    /// Pagination of data to return records earlier than the requested order Id, not including endId
    #[serde(rename = "endId", skip_serializing_if = "Option::is_none")]
    pub end_id: Option<String>,

    /// Number of results per request. The maximum is 100. The default is 100
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Response data for getting pending spread orders
/// The fields are the same as SpreadOrderData but separated for clarity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PendingSpreadOrderData {
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

    /// Quantity still remaining to be filled
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
    /// live
    /// partially_filled
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
    /// Get pending spread orders
    ///
    /// Retrieve all incomplete orders under the current account
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#spread-trading-rest-api-get-active-orders)
    pub async fn get_spread_orders_pending(
        &self,
        request: Option<GetSpreadOrdersPendingRequest>,
    ) -> RestResult<PendingSpreadOrderData> {
        self.send_get_request(
            GET_SPREAD_ORDERS_PENDING_ENDPOINT,
            request.as_ref(),
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
    fn test_get_spread_orders_pending_request_full() {
        let request = GetSpreadOrdersPendingRequest {
            sprd_id: Some("BTC-USDT_BTC-USDT-SWAP".to_string()),
            ord_type: Some("limit".to_string()),
            state: Some("live".to_string()),
            begin_id: Some("312269865356374016".to_string()),
            end_id: Some("312269865356374017".to_string()),
            limit: Some("50".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: GetSpreadOrdersPendingRequest =
            serde_json::from_str(&serialized).unwrap();
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_get_spread_orders_pending_request_minimal() {
        let request = GetSpreadOrdersPendingRequest {
            sprd_id: None,
            ord_type: None,
            state: None,
            begin_id: None,
            end_id: None,
            limit: None,
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert_eq!(serialized, "{}");
    }

    #[test]
    fn test_get_spread_orders_pending_request_none() {
        let request: Option<GetSpreadOrdersPendingRequest> = None;
        assert!(request.is_none());
    }

    #[test]
    fn test_pending_spread_order_data_deserialization() {
        let json_response = r#"{
            "sprdId": "BTC-USDT_BTC-USDT-SWAP",
            "ordId": "312269865356374016",
            "clOrdId": "client123",
            "tag": "",
            "px": "50000",
            "sz": "1",
            "ordType": "limit",
            "side": "buy",
            "fillSz": "0.3",
            "fillPx": "50000",
            "tradeId": "123456789",
            "accFillSz": "0.3",
            "pendingFillSz": "0.7",
            "pendingSettleSz": "0",
            "canceledSz": "0",
            "avgPx": "50000",
            "state": "partially_filled",
            "cancelSource": "",
            "uTime": "1597026383085",
            "cTime": "1597026383085"
        }"#;

        let order: PendingSpreadOrderData = serde_json::from_str(json_response).unwrap();
        assert_eq!(order.sprd_id, "BTC-USDT_BTC-USDT-SWAP");
        assert_eq!(order.ord_id, "312269865356374016");
        assert_eq!(order.state, "partially_filled");
        assert_eq!(order.pending_fill_sz, "0.7");
    }

    #[test]
    fn test_pending_spread_order_data_serialization() {
        let order = PendingSpreadOrderData {
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
        let deserialized: PendingSpreadOrderData = serde_json::from_str(&serialized).unwrap();
        assert_eq!(order, deserialized);
    }

    #[test]
    fn test_pending_order_states() {
        let states = vec!["live", "partially_filled"];

        for state in states {
            let request = GetSpreadOrdersPendingRequest {
                sprd_id: None,
                ord_type: None,
                state: Some(state.to_string()),
                begin_id: None,
                end_id: None,
                limit: None,
            };

            let serialized = serde_json::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("\"state\":\"{}\"", state)));
        }
    }

    #[test]
    fn test_pending_order_types() {
        let ord_types = vec!["market", "limit", "post_only", "ioc"];

        for ord_type in ord_types {
            let request = GetSpreadOrdersPendingRequest {
                sprd_id: None,
                ord_type: Some(ord_type.to_string()),
                state: None,
                begin_id: None,
                end_id: None,
                limit: None,
            };

            let serialized = serde_json::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("\"ordType\":\"{}\"", ord_type)));
        }
    }
}
