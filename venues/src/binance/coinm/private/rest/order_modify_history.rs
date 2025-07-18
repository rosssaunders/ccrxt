// Get Order Modify History (USER_DATA) endpoint implementation for GET /dapi/v1/orderAmendment
// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Get-Order-Modify-History>

use serde::{Deserialize, Serialize};

use crate::binance::{
    coinm::{RestResult, private::rest::client::RestClient},
    shared,
};

const ORDER_AMENDMENT_ENDPOINT: &str = "/dapi/v1/orderAmendment";

/// Request parameters for getting order modification history (GET /dapi/v1/orderAmendment).
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetOrderModifyHistoryRequest {
    /// Trading symbol (e.g., "BTCUSD_PERP").
    pub symbol: String,

    /// Order ID. Either orderId or origClientOrderId must be sent.
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,

    /// Original client order ID. Either orderId or origClientOrderId must be sent.
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id: Option<String>,

    /// Timestamp in ms to get modification history from (INCLUSIVE).
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// Timestamp in ms to get modification history until (INCLUSIVE).
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Default 50; max 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Receive window in milliseconds.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp in milliseconds.
    pub timestamp: u64,
}

/// Represents a field change in order modification.
#[derive(Debug, Clone, Deserialize)]
pub struct OrderAmendmentField {
    /// Value before the modification.
    pub before: String,

    /// Value after the modification.
    pub after: String,
}

/// Represents the changes made in an order modification.
#[derive(Debug, Clone, Deserialize)]
pub struct OrderAmendment {
    /// Price change information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<OrderAmendmentField>,

    /// Original quantity change information.
    #[serde(rename = "origQty", skip_serializing_if = "Option::is_none")]
    pub orig_qty: Option<OrderAmendmentField>,

    /// Order modification count, representing the number of times the order has been modified.
    pub count: u32,
}

/// Individual order modification history entry.
#[derive(Debug, Clone, Deserialize)]
pub struct OrderModifyHistoryEntry {
    /// Order modification ID.
    #[serde(rename = "amendmentId")]
    pub amendment_id: u64,

    /// Trading symbol.
    pub symbol: String,

    /// Trading pair.
    pub pair: String,

    /// Order ID.
    #[serde(rename = "orderId")]
    pub order_id: u64,

    /// Client order ID.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,

    /// Order modification time.
    pub time: u64,

    /// Order modification details.
    pub amendment: OrderAmendment,
}

/// Response for getting order modification history (GET /dapi/v1/orderAmendment).
pub type GetOrderModifyHistoryResponse = Vec<OrderModifyHistoryEntry>;

impl RestClient {
    /// Gets order modification history (USER_DATA) for Binance Coin-M Futures.
    ///
    /// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Get-Order-Modify-History>
    /// GET /dapi/v1/orderAmendment
    /// Weight: 1
    /// Requires API key and signature.
    ///
    /// Either orderId or origClientOrderId must be sent, and the orderId will prevail if both are sent.
    /// Order modify history longer than 3 months is not available.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`GetOrderModifyHistoryRequest`])
    ///
    /// # Returns
    /// A [`GetOrderModifyHistoryResponse`] - array of order modification history entries.
    pub async fn get_order_modify_history(
        &self,
        params: GetOrderModifyHistoryRequest,
    ) -> RestResult<GetOrderModifyHistoryResponse> {
        let weight = 1;
        shared::send_signed_request(
            self,
            ORDER_AMENDMENT_ENDPOINT,
            reqwest::Method::GET,
            params,
            weight,
            false,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_modify_history_request_serialization() {
        let request = GetOrderModifyHistoryRequest {
            symbol: "BTCUSD_PERP".to_string(),
            order_id: Some(12345),
            orig_client_order_id: None,
            start_time: None,
            end_time: None,
            limit: None,
            recv_window: None,
            timestamp: 1625097600000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSD_PERP"));
        assert!(serialized.contains("orderId=12345"));
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(!serialized.contains("origClientOrderId"));
        assert!(!serialized.contains("startTime"));
        assert!(!serialized.contains("endTime"));
        assert!(!serialized.contains("limit"));
    }

    #[test]
    fn test_order_modify_history_request_with_client_order_id() {
        let request = GetOrderModifyHistoryRequest {
            symbol: "ETHUSD_PERP".to_string(),
            order_id: None,
            orig_client_order_id: Some("my_order_123".to_string()),
            start_time: Some(1625097500000),
            end_time: Some(1625097700000),
            limit: Some(50),
            recv_window: Some(5000),
            timestamp: 1625097600000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=ETHUSD_PERP"));
        assert!(serialized.contains("origClientOrderId=my_order_123"));
        assert!(serialized.contains("startTime=1625097500000"));
        assert!(serialized.contains("endTime=1625097700000"));
        assert!(serialized.contains("limit=50"));
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(!serialized.contains("orderId"));
    }

    #[test]
    fn test_order_amendment_field_deserialization() {
        let json = r#"{
            "before": "45000.0",
            "after": "45500.0"
        }"#;

        let field: OrderAmendmentField = serde_json::from_str(json).unwrap();
        assert_eq!(field.before, "45000.0");
        assert_eq!(field.after, "45500.0");
    }

    #[test]
    fn test_order_amendment_deserialization() {
        let json = r#"{
            "price": {
                "before": "45000.0",
                "after": "45500.0"
            },
            "origQty": {
                "before": "10.0",
                "after": "15.0"
            },
            "count": 2
        }"#;

        let amendment: OrderAmendment = serde_json::from_str(json).unwrap();
        assert_eq!(amendment.count, 2);

        let price = amendment.price.unwrap();
        assert_eq!(price.before, "45000.0");
        assert_eq!(price.after, "45500.0");

        let qty = amendment.orig_qty.unwrap();
        assert_eq!(qty.before, "10.0");
        assert_eq!(qty.after, "15.0");
    }

    #[test]
    fn test_order_modify_history_response_deserialization() {
        let json = r#"[
            {
                "amendmentId": 100001,
                "symbol": "BTCUSD_PERP",
                "pair": "BTCUSD",
                "orderId": 12345,
                "clientOrderId": "my_order_123",
                "time": 1625097600000,
                "amendment": {
                    "price": {
                        "before": "45000.0",
                        "after": "45500.0"
                    },
                    "count": 1
                }
            },
            {
                "amendmentId": 100002,
                "symbol": "BTCUSD_PERP",
                "pair": "BTCUSD",
                "orderId": 12345,
                "clientOrderId": "my_order_123",
                "time": 1625097700000,
                "amendment": {
                    "origQty": {
                        "before": "10.0",
                        "after": "20.0"
                    },
                    "count": 2
                }
            }
        ]"#;

        let response: GetOrderModifyHistoryResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 2);

        let entry1 = &response[0];
        assert_eq!(entry1.amendment_id, 100001);
        assert_eq!(entry1.symbol, "BTCUSD_PERP");
        assert_eq!(entry1.pair, "BTCUSD");
        assert_eq!(entry1.order_id, 12345);
        assert_eq!(entry1.client_order_id, "my_order_123");
        assert_eq!(entry1.time, 1625097600000);
        assert_eq!(entry1.amendment.count, 1);
        assert!(entry1.amendment.price.is_some());
        assert!(entry1.amendment.orig_qty.is_none());

        let entry2 = &response[1];
        assert_eq!(entry2.amendment_id, 100002);
        assert_eq!(entry2.amendment.count, 2);
        assert!(entry2.amendment.price.is_none());
        assert!(entry2.amendment.orig_qty.is_some());
    }

    #[test]
    fn test_empty_order_modify_history_response() {
        let json = r#"[]"#;
        let response: GetOrderModifyHistoryResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 0);
    }
}
