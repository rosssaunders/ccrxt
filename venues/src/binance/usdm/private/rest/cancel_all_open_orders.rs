use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::UsdmClient;
use crate::binance::usdm::RestResult;

const CANCEL_ALL_OPEN_ORDERS_ENDPOINT: &str = "/fapi/v1/allOpenOrders";

/// Request parameters for cancelling all open orders for a symbol on USDM futures.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOpenOrdersRequest {
    /// Trading symbol to cancel all orders for (e.g., "BTCUSDT").
    pub symbol: Cow<'static, str>,

    /// Optional: The number of milliseconds the request is valid for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp (milliseconds since epoch).
    pub timestamp: u64,
}

/// Response for cancelling all open orders for a symbol.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOpenOrdersResponse {
    /// Response code (200 for success)
    pub code: u16,

    /// Response message from the API.
    pub msg: Cow<'static, str>,
}

impl UsdmClient {
    /// Cancel All Open Orders
    ///
    /// Cancels all open orders for the specified symbol.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Cancel-All-Open-Orders
    ///
    /// Rate limit: 1
    ///
    /// # Arguments
    /// * `params` - The cancel all open orders request parameters
    ///
    /// # Returns
    /// Response containing code and message indicating success
    pub async fn cancel_all_open_orders(
        &self,
        params: CancelAllOpenOrdersRequest,
    ) -> RestResult<CancelAllOpenOrdersResponse> {
        self.send_signed_request(
            CANCEL_ALL_OPEN_ORDERS_ENDPOINT,
            Method::DELETE,
            params,
            1,
            false,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel_all_open_orders_request_serialization() {
        let req = CancelAllOpenOrdersRequest {
            symbol: Cow::Borrowed("BTCUSDT"),
            recv_window: Some(5000),
            timestamp: 1625184000000,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains(r#"symbol":"BTCUSDT"#));
        assert!(json.contains(r#"recvWindow":5000"#));
        assert!(json.contains(r#"timestamp":1625184000000"#));
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
}
