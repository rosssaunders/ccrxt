use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::RestResult;

/// Endpoint path for auto-cancel all open orders.
const COUNTDOWN_CANCEL_ALL_ENDPOINT: &str = "/fapi/v1/countdownCancelAll";

/// Request parameters for the Auto-Cancel All Open Orders endpoint.
///
/// Cancels all open orders of the specified symbol at the end of the specified countdown.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CountdownCancelAllRequest {
    /// Trading symbol (e.g., "BTCUSDT").
    ///
    /// This is the symbol for which to set the countdown cancel timer.
    pub symbol: Cow<'static, str>,

    /// Countdown time in milliseconds. Set to 0 to disable the timer.
    ///
    /// Valid range: 0 or greater. Example: 120000 for 2 minutes.
    pub countdown_time: u64,

    /// Optional window of validity for the request in milliseconds.
    ///
    /// If not set, the default recvWindow is used by Binance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp (milliseconds since epoch).
    pub timestamp: u64,
}

/// Response for the Auto-Cancel All Open Orders endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CountdownCancelAllResponse {
    /// Trading symbol for which the countdown was set.
    pub symbol: String,

    /// Countdown time in milliseconds.
    pub countdown_time: u64,
}

impl UsdmClient {
    /// Auto-Cancel All Open Orders (POST /fapi/v1/countdownCancelAll)
    ///
    /// Cancel all open orders of the specified symbol at the end of the specified countdown.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Auto-Cancel-All-Open-Orders
    ///
    /// Rate limit: 10
    ///
    /// # Arguments
    /// * `params` - The countdown cancel all request parameters
    ///
    /// # Returns
    /// Response containing symbol and countdown time
    pub async fn countdown_cancel_all(
        &self,
        params: CountdownCancelAllRequest,
    ) -> RestResult<CountdownCancelAllResponse> {
        self.send_post_signed_request(COUNTDOWN_CANCEL_ALL_ENDPOINT, params, 10, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use serde_json;

    use super::*;

    #[test]
    fn test_countdown_cancel_all_response_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "countdownTime": 120000
        }"#;

        let response: CountdownCancelAllResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTCUSDT");
        assert_eq!(response.countdown_time, 120000);
    }

    #[test]
    fn test_countdown_cancel_all_request_serialization() {
        let req = CountdownCancelAllRequest {
            symbol: Cow::Borrowed("BTCUSDT"),
            countdown_time: 120000,
            recv_window: Some(5000),
            timestamp: 1234567890,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("BTCUSDT"));
        assert!(json.contains("120000"));
        assert!(json.contains("5000"));
        assert!(json.contains("1234567890"));
    }

    #[test]
    fn test_countdown_cancel_all_request_default() {
        let req = CountdownCancelAllRequest::default();
        assert_eq!(req.symbol, "");
        assert_eq!(req.countdown_time, 0);
        assert!(req.recv_window.is_none());
        assert_eq!(req.timestamp, 0);
    }
}
