use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

const GET_SPREAD_ORDERS_HISTORY_ARCHIVE_ENDPOINT: &str = "/api/v5/sprd/orders-history-archive";

/// Request parameters for getting spread orders history archive
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSpreadOrdersHistoryArchiveRequest {
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
    /// canceled
    /// filled
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Instrument type
    /// SPOT
    /// FUTURES
    /// SWAP
    /// Any orders with spreads containing the specified instrument type in any legs will be returned
    #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,

    /// Instrument family, e.g. BTC-USDT
    /// Any orders with spreads containing the specified instrument family in any legs will be returned
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,

    /// Start order ID the request to begin with
    /// Pagination of data to return records newer than the requested order Id, not including beginId
    #[serde(rename = "beginId", skip_serializing_if = "Option::is_none")]
    pub begin_id: Option<String>,

    /// End order ID the request to end with
    /// Pagination of data to return records earlier than the requested order Id, not including endId
    #[serde(rename = "endId", skip_serializing_if = "Option::is_none")]
    pub end_id: Option<String>,

    /// Filter with a begin timestamp
    /// Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(rename = "begin", skip_serializing_if = "Option::is_none")]
    pub begin: Option<String>,

    /// Filter with an end timestamp
    /// Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,

    /// Number of results per request. The maximum is 100. The default is 100
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Response data for getting spread orders history archive
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArchiveSpreadOrderData {
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

    /// Quantity still remaining to be filled, including pendingSettleSz
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
    /// canceled
    /// filled
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
    /// Get spread orders history archive
    ///
    /// Retrieve the completed order data for the last 3 months
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#spread-trading-rest-api-get-orders-history-last-3-months)
    pub async fn get_spread_orders_history_archive(
        &self,
        request: Option<GetSpreadOrdersHistoryArchiveRequest>,
    ) -> RestResult<ArchiveSpreadOrderData> {
        self.send_get_request(
            GET_SPREAD_ORDERS_HISTORY_ARCHIVE_ENDPOINT,
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
    fn test_get_spread_orders_history_archive_request_full() {
        let request = GetSpreadOrdersHistoryArchiveRequest {
            sprd_id: Some("BTC-USDT_BTC-USDT-SWAP".to_string()),
            ord_type: Some("limit".to_string()),
            state: Some("filled".to_string()),
            inst_type: Some("SWAP".to_string()),
            inst_family: Some("BTC-USDT".to_string()),
            begin_id: Some("312269865356374016".to_string()),
            end_id: Some("312269865356374017".to_string()),
            begin: Some("1597026383085".to_string()),
            end: Some("1597112783085".to_string()),
            limit: Some("50".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: GetSpreadOrdersHistoryArchiveRequest =
            serde_json::from_str(&serialized).unwrap();
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_get_spread_orders_history_archive_request_minimal() {
        let request = GetSpreadOrdersHistoryArchiveRequest {
            sprd_id: None,
            ord_type: None,
            state: None,
            inst_type: None,
            inst_family: None,
            begin_id: None,
            end_id: None,
            begin: None,
            end: None,
            limit: None,
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert_eq!(serialized, "{}");
    }

    #[test]
    fn test_get_spread_orders_history_archive_request_none() {
        let request: Option<GetSpreadOrdersHistoryArchiveRequest> = None;
        assert!(request.is_none());
    }

    #[test]
    fn test_archive_spread_order_data_deserialization() {
        let json_response = r#"{
            "sprdId": "BTC-USDT_BTC-USDT-SWAP",
            "ordId": "312269865356374016",
            "clOrdId": "client123",
            "tag": "",
            "px": "50000",
            "sz": "1",
            "ordType": "limit",
            "side": "buy",
            "fillSz": "1",
            "fillPx": "50000",
            "tradeId": "123456789",
            "accFillSz": "1",
            "pendingFillSz": "0",
            "pendingSettleSz": "0",
            "canceledSz": "0",
            "avgPx": "50000",
            "state": "filled",
            "cancelSource": "",
            "uTime": "1597026383085",
            "cTime": "1597026383085"
        }"#;

        let order: ArchiveSpreadOrderData = serde_json::from_str(json_response).unwrap();
        assert_eq!(order.sprd_id, "BTC-USDT_BTC-USDT-SWAP");
        assert_eq!(order.ord_id, "312269865356374016");
        assert_eq!(order.state, "filled");
        assert_eq!(order.acc_fill_sz, "1");
        assert_eq!(order.pending_fill_sz, "0");
    }

    #[test]
    fn test_archive_spread_order_data_serialization() {
        let order = ArchiveSpreadOrderData {
            sprd_id: "BTC-USDT_BTC-USDT-SWAP".to_string(),
            ord_id: "312269865356374016".to_string(),
            cl_ord_id: "client123".to_string(),
            tag: "".to_string(),
            px: "50000".to_string(),
            sz: "1".to_string(),
            ord_type: "limit".to_string(),
            side: "buy".to_string(),
            fill_sz: "1".to_string(),
            fill_px: "50000".to_string(),
            trade_id: "123456789".to_string(),
            acc_fill_sz: "1".to_string(),
            pending_fill_sz: "0".to_string(),
            pending_settle_sz: "0".to_string(),
            canceled_sz: "0".to_string(),
            avg_px: "50000".to_string(),
            state: "filled".to_string(),
            cancel_source: "".to_string(),
            u_time: "1597026383085".to_string(),
            c_time: "1597026383085".to_string(),
        };

        let serialized = serde_json::to_string(&order).unwrap();
        let deserialized: ArchiveSpreadOrderData = serde_json::from_str(&serialized).unwrap();
        assert_eq!(order, deserialized);
    }

    #[test]
    fn test_archive_instrument_types() {
        let inst_types = vec!["SPOT", "FUTURES", "SWAP"];

        for inst_type in inst_types {
            let request = GetSpreadOrdersHistoryArchiveRequest {
                sprd_id: None,
                ord_type: None,
                state: None,
                inst_type: Some(inst_type.to_string()),
                inst_family: None,
                begin_id: None,
                end_id: None,
                begin: None,
                end: None,
                limit: None,
            };

            let serialized = serde_json::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("\"instType\":\"{}\"", inst_type)));
        }
    }

    #[test]
    fn test_archive_inst_family_filter() {
        let request = GetSpreadOrdersHistoryArchiveRequest {
            sprd_id: None,
            ord_type: None,
            state: None,
            inst_type: None,
            inst_family: Some("BTC-USDT".to_string()),
            begin_id: None,
            end_id: None,
            begin: None,
            end: None,
            limit: None,
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("\"instFamily\":\"BTC-USDT\""));
    }

    #[test]
    fn test_archive_order_states() {
        let states = vec!["filled", "canceled"];

        for state in states {
            let request = GetSpreadOrdersHistoryArchiveRequest {
                sprd_id: None,
                ord_type: None,
                state: Some(state.to_string()),
                inst_type: None,
                inst_family: None,
                begin_id: None,
                end_id: None,
                begin: None,
                end: None,
                limit: None,
            };

            let serialized = serde_json::to_string(&request).unwrap();
            assert!(serialized.contains(&format!("\"state\":\"{}\"", state)));
        }
    }
}
