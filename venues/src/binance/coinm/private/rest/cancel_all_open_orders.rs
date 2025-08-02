use serde::{Deserialize, Serialize};

use crate::binance::{
    coinm::{RestResult, private::rest::client::RestClient},
};

const ALL_OPEN_ORDERS_ENDPOINT: &str = "/dapi/v1/allOpenOrders";

/// Request parameters for canceling all open orders (DELETE /dapi/v1/allOpenOrders).
#[derive(Debug, Clone, Serialize, Default)]
pub struct CancelAllOpenOrdersRequest {
    /// Trading symbol (e.g., "BTCUSD_PERP").
    pub symbol: String,

    /// Receive window in milliseconds.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp in milliseconds.
    pub timestamp: u64,
}

/// Response for canceling all open orders (DELETE /dapi/v1/allOpenOrders).
#[derive(Debug, Clone, Deserialize)]
pub struct CancelAllOpenOrdersResponse {
    /// Response code (200 for success).
    pub code: u32,

    /// Response message.
    pub msg: String,
}

impl RestClient {
    /// Cancels all open orders (TRADE) for a symbol on Binance Coin-M Futures.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Cancel-All-Open-Orders
    ///
    /// DELETE /dapi/v1/allOpenOrders
    /// Weight: 1
    /// Requires API key and signature.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`CancelAllOpenOrdersRequest`])
    ///
    /// # Returns
    /// A [`CancelAllOpenOrdersResponse`] with the operation result.
    pub async fn cancel_all_open_orders(
        &self,
        params: CancelAllOpenOrdersRequest,
    ) -> RestResult<CancelAllOpenOrdersResponse> {
        self.send_delete_signed_request(
            ALL_OPEN_ORDERS_ENDPOINT,
            params,
            1,
            true,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel_all_open_orders_request_serialization() {
        let request = CancelAllOpenOrdersRequest {
            symbol: "BTCUSD_PERP".to_string(),
            recv_window: None,
            timestamp: 1625097600000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSD_PERP"));
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(!serialized.contains("recvWindow"));
    }

    #[test]
    fn test_cancel_all_open_orders_request_serialization_with_recv_window() {
        let request = CancelAllOpenOrdersRequest {
            symbol: "ETHUSD_PERP".to_string(),
            recv_window: Some(5000),
            timestamp: 1625097600000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=ETHUSD_PERP"));
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1625097600000"));
    }

    #[test]
    fn test_cancel_all_open_orders_response_deserialization() {
        let json = r#"{
            "code": 200,
            "msg": "The operation of cancel all open order is done."
        }"#;

        let response: CancelAllOpenOrdersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.code, 200);
        assert_eq!(
            response.msg,
            "The operation of cancel all open order is done."
        );
    }

    #[test]
    fn test_cancel_all_open_orders_response_deserialization_with_different_message() {
        let json = r#"{
            "code": 200,
            "msg": "All orders cancelled successfully"
        }"#;

        let response: CancelAllOpenOrdersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.code, 200);
        assert_eq!(response.msg, "All orders cancelled successfully");
    }
}
