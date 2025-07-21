use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::RestResult;

const CHANGE_LEVERAGE_ENDPOINT: &str = "/fapi/v1/leverage";

/// Request parameters for changing initial leverage.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChangeLeverageRequest {
    /// Trading symbol (e.g., "BTCUSDT").
    pub symbol: String,

    /// Target initial leverage (1 to 125).
    pub leverage: u32,
}

/// Response from changing leverage.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeLeverageResponse {
    /// Current leverage value after change.
    pub leverage: u32,

    /// Maximum notional value for this leverage.
    pub max_notional_value: String,

    /// Trading symbol.
    pub symbol: String,
}

impl UsdmClient {
    /// Change Initial Leverage (USER_DATA)
    ///
    /// Changes user's initial leverage in the specific symbol market.
    /// For Hedge Mode, LONG and SHORT positions of one symbol use the same initial leverage and share a total notional value.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Change-Initial-Leverage
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
        self.send_signed_request(CHANGE_LEVERAGE_ENDPOINT, Method::POST, params, 1, false)
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
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("leverage=10"));
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
    }

    #[test]
    fn test_change_leverage_high_leverage() {
        let request = ChangeLeverageRequest {
            symbol: "ETHUSDT".to_string(),
            leverage: 125,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("leverage=125"));
        assert!(serialized.contains("symbol=ETHUSDT"));
    }
}
