use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::UsdmClient;
use crate::binance::usdm::RestResult;

const CANCEL_ALL_OPEN_ORDERS_ENDPOINT: &str = "/fapi/v1/allOpenOrders";

/// Request to cancel all open orders for a symbol.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOpenOrdersRequest {
    /// Symbol to cancel all orders for
    pub symbol: Cow<'static, str>,
}

/// Response for cancel all open orders.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOpenOrdersResponse {
    /// Response code (200 for success)
    pub code: u16,

    /// Response message
    pub msg: Cow<'static, str>,
}

impl UsdmClient {
    /// Cancel all open orders for a symbol (DELETE /fapi/v1/allOpenOrders)
    ///
    /// Cancels all open orders for the specified symbol.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Cancel-All-Open-Orders
    ///
    /// Rate limit: 5
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
            5,
            false,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
