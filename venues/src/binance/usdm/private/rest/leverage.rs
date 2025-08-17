use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::RestResult;

const CHANGE_LEVERAGE_ENDPOINT: &str = "/fapi/v1/leverage";

/// Request parameters for the Change Initial Leverage endpoint.
///
/// Changes the user's initial leverage for a specific symbol market.
/// All fields must match Binance API requirements.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChangeLeverageRequest {
    /// Trading symbol (e.g., "BTCUSDT").
    /// Must be a valid symbol listed on Binance USDM Futures.
    pub symbol: String,

    /// Target initial leverage (1 to 125).
    /// Must be an integer between 1 and 125, inclusive.
    pub leverage: u32,

    /// The number of milliseconds after timestamp the request is valid for. Optional.
    /// If omitted, Binance default is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp (milliseconds since epoch).
    /// Required by Binance for all signed requests.
    pub timestamp: u64,
}

/// Response from the Change Initial Leverage endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeLeverageResponse {
    /// Current leverage value after change.
    pub leverage: u32,

    /// Maximum notional value for this leverage, as returned by Binance.
    pub max_notional_value: String,

    /// Trading symbol for which leverage was changed.
    pub symbol: String,
}

impl UsdmClient {
    /// Change Initial Leverage (USER_DATA)
    ///
    /// Changes user's initial leverage in the specific symbol market.
    /// For Hedge Mode, LONG and SHORT positions of one symbol use the same initial leverage and share a total notional value.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Change-Initial-Leverage)
    ///
    /// Rate limit: 1
    ///
    /// # Arguments
    /// * `params` - The request parameters
    ///
    /// # Returns
    /// Response containing the new leverage configuration.
    pub async fn change_leverage(
        &self,
        params: ChangeLeverageRequest,
    ) -> RestResult<ChangeLeverageResponse> {
        self.send_post_signed_request(CHANGE_LEVERAGE_ENDPOINT, params, 1, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_leverage_request_serialization() {
        let request = ChangeLeverageRequest {
            symbol: "BTCUSDT".to_string(),
            leverage: 10,
            recv_window: Some(5000),
            timestamp: 1234567890,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("leverage=10"));
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1234567890"));
    }

    #[test]
    fn test_change_leverage_response_deserialization() {
        let json = r#"{"leverage":10,"maxNotionalValue":"1000000","symbol":"BTCUSDT"}"#;
        let response: ChangeLeverageResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.leverage, 10);
        assert_eq!(response.max_notional_value, "1000000");
        assert_eq!(response.symbol, "BTCUSDT");
    }

    #[test]
    fn test_change_leverage_request_default() {
        let request = ChangeLeverageRequest::default();
        assert_eq!(request.symbol, "");
        assert_eq!(request.leverage, 0);
        assert_eq!(request.recv_window, None);
        assert_eq!(request.timestamp, 0);
    }

    #[test]
    fn test_change_leverage_high_leverage() {
        let request = ChangeLeverageRequest {
            symbol: "ETHUSDT".to_string(),
            leverage: 125,
            recv_window: None,
            timestamp: 9876543210,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("leverage=125"));
        assert!(serialized.contains("symbol=ETHUSDT"));
        assert!(serialized.contains("timestamp=9876543210"));
    }
}
