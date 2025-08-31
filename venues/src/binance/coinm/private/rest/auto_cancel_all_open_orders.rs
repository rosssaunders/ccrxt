use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, private_client::RestClient};

const COUNTDOWN_CANCEL_ALL_ENDPOINT: &str = "/dapi/v1/countdownCancelAll";

/// Request parameters for auto-canceling all open orders (POST /dapi/v1/countdownCancelAll).
#[derive(Debug, Clone, Serialize, Default)]
pub struct AutoCancelAllOpenOrdersRequest {
    /// Trading symbol (e.g., "BTCUSD_PERP").
    pub symbol: String,

    /// Countdown time in milliseconds (1000 for 1 second).
    /// Set to 0 to cancel the timer.
    #[serde(rename = "countdownTime")]
    pub countdown_time: u64,

    /// Receive window in milliseconds.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp in milliseconds.
    pub timestamp: u64,
}

/// Response for auto-canceling all open orders (POST /dapi/v1/countdownCancelAll).
#[derive(Debug, Clone, Deserialize)]
pub struct AutoCancelAllOpenOrdersResponse {
    /// Trading symbol.
    pub symbol: String,

    /// Countdown time in milliseconds.
    #[serde(rename = "countdownTime")]
    pub countdown_time: String,
}

impl RestClient {
    /// Sets up auto-cancel for all open orders (TRADE) on Binance Coin-M Futures.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Auto-Cancel-All-Open-Orders)
    ///
    /// POST /dapi/v1/countdownCancelAll
    /// Weight: 10
    /// Requires API key and signature.
    ///
    /// This endpoint cancels all open orders of the specified symbol at the end of the specified countdown.
    /// This rest endpoint is designed to ensure your open orders are canceled in case of an outage.
    /// The endpoint should be called repeatedly as heartbeats so that the existing countdown time can be
    /// canceled and replaced by a new one.
    ///
    /// The system will check all countdowns approximately every 10 milliseconds, so please note that
    /// sufficient redundancy should be considered when using this function.
    /// We do not recommend setting the countdown time to be too precise or too small.
    ///
    /// Example usage:
    /// - Call this endpoint at 30s intervals with a countdownTime of 120000 (120s).
    /// - If this endpoint is not called within 120 seconds, all your orders of the specified symbol will be automatically canceled.
    /// - If this endpoint is called with a countdownTime of 0, the countdown timer will be stopped.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`AutoCancelAllOpenOrdersRequest`])
    ///
    /// # Returns
    /// An [`AutoCancelAllOpenOrdersResponse`] with the countdown configuration.
    pub async fn auto_cancel_all_open_orders(
        &self,
        params: AutoCancelAllOpenOrdersRequest,
    ) -> RestResult<AutoCancelAllOpenOrdersResponse> {
        self.send_post_signed_request(COUNTDOWN_CANCEL_ALL_ENDPOINT, params, 10, true)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auto_cancel_all_open_orders_request_serialization() {
        let request = AutoCancelAllOpenOrdersRequest {
            symbol: "BTCUSD_PERP".to_string(),
            countdown_time: 120000,
            recv_window: None,
            timestamp: 1625097600000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSD_PERP"));
        assert!(serialized.contains("countdownTime=120000"));
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(!serialized.contains("recvWindow"));
    }

    #[test]
    fn test_auto_cancel_all_open_orders_request_serialization_with_recv_window() {
        let request = AutoCancelAllOpenOrdersRequest {
            symbol: "ETHUSD_PERP".to_string(),
            countdown_time: 0, // Cancel the timer
            recv_window: Some(5000),
            timestamp: 1625097600000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=ETHUSD_PERP"));
        assert!(serialized.contains("countdownTime=0"));
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1625097600000"));
    }

    #[test]
    fn test_auto_cancel_all_open_orders_response_deserialization() {
        let json = r#"{
            "symbol": "BTCUSD_PERP",
            "countdownTime": "120000"
        }"#;

        let response: AutoCancelAllOpenOrdersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTCUSD_PERP");
        assert_eq!(response.countdown_time, "120000");
    }

    #[test]
    fn test_auto_cancel_all_open_orders_response_deserialization_cancelled() {
        let json = r#"{
            "symbol": "ETHUSD_PERP",
            "countdownTime": "0"
        }"#;

        let response: AutoCancelAllOpenOrdersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "ETHUSD_PERP");
        assert_eq!(response.countdown_time, "0");
    }
}
