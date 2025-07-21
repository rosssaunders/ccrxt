use reqwest::Method;
use secrecy::SecretString;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::UsdmClient;
use crate::binance::usdm::RestResult;

const COUNTDOWN_CANCEL_ALL_ENDPOINT: &str = "/fapi/v1/countdownCancelAll";

/// Request to set up countdown cancel all orders.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CountdownCancelAllRequest {
    /// API key (securely stored)
    #[serde(skip)]
    pub api_key: SecretString,

    /// API secret (securely stored)
    #[serde(skip)]
    pub api_secret: SecretString,

    /// Symbol to set countdown cancel for
    pub symbol: Cow<'static, str>,

    /// Countdown time in milliseconds (0 to disable)
    pub countdown_time: u64,
}

/// Response for countdown cancel all orders.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CountdownCancelAllResponse {
    /// Symbol
    pub symbol: Cow<'static, str>,

    /// Countdown time in milliseconds
    pub countdown_time: u64,
}

impl UsdmClient {
    /// Auto-Cancel All Open Orders (POST /fapi/v1/countdownCancelAll)
    ///
    /// Cancel all open orders of the specified symbol at the end of the specified countdown.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Auto-Cancel-All-Open-Orders
    ///
    /// Rate limit: 5
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
        self.send_signed_request(
            COUNTDOWN_CANCEL_ALL_ENDPOINT,
            Method::POST,
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
    fn test_countdown_cancel_all_response_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "countdownTime": 120000
        }"#;

        let response: CountdownCancelAllResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTCUSDT");
        assert_eq!(response.countdown_time, 120000);
    }
}
