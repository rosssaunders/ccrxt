use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, private::rest::client::RestClient};

const LEVERAGE_ENDPOINT: &str = "/dapi/v1/leverage";

/// Request parameters for changing initial leverage (POST /dapi/v1/leverage).
#[derive(Debug, Clone, Serialize, Default)]
pub struct ChangeInitialLeverageRequest {
    /// Trading symbol (e.g., "BTCUSD_PERP").
    pub symbol: String,

    /// Target initial leverage: int from 1 to 125.
    pub leverage: u32,

    /// Receive window in milliseconds.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp in milliseconds.
    pub timestamp: u64,
}

/// Response for changing initial leverage (POST /dapi/v1/leverage).
#[derive(Debug, Clone, Deserialize)]
pub struct ChangeInitialLeverageResponse {
    /// Applied leverage.
    pub leverage: u32,

    /// Maximum quantity of base asset.
    #[serde(rename = "maxQty")]
    pub max_qty: String,

    /// Trading symbol.
    pub symbol: String,
}

impl RestClient {
    /// Changes user's initial leverage (TRADE) for a specific symbol on Binance Coin-M Futures.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Change-Initial-Leverage)
    ///
    /// POST /dapi/v1/leverage
    /// Weight: 1
    /// Requires API key and signature.
    ///
    /// Changes user's initial leverage in the specific symbol market.
    /// For Hedge Mode, LONG and SHORT positions of one symbol use the same initial leverage and share a total notional value.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`ChangeInitialLeverageRequest`])
    ///
    /// # Returns
    /// A [`ChangeInitialLeverageResponse`] with the new leverage configuration.
    pub async fn change_initial_leverage(
        &self,
        params: ChangeInitialLeverageRequest,
    ) -> RestResult<ChangeInitialLeverageResponse> {
        self.send_post_signed_request(LEVERAGE_ENDPOINT, params, 1, true)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_initial_leverage_request_serialization() {
        let request = ChangeInitialLeverageRequest {
            symbol: "BTCUSD_PERP".to_string(),
            leverage: 20,
            recv_window: None,
            timestamp: 1625097600000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSD_PERP"));
        assert!(serialized.contains("leverage=20"));
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(!serialized.contains("recvWindow"));
    }

    #[test]
    fn test_change_initial_leverage_request_serialization_with_recv_window() {
        let request = ChangeInitialLeverageRequest {
            symbol: "ETHUSD_PERP".to_string(),
            leverage: 10,
            recv_window: Some(5000),
            timestamp: 1625097600000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=ETHUSD_PERP"));
        assert!(serialized.contains("leverage=10"));
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1625097600000"));
    }

    #[test]
    fn test_change_initial_leverage_request_serialization_max_leverage() {
        let request = ChangeInitialLeverageRequest {
            symbol: "BTCUSD_PERP".to_string(),
            leverage: 125,
            recv_window: None,
            timestamp: 1625097600000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("leverage=125"));
    }

    #[test]
    fn test_change_initial_leverage_response_deserialization() {
        let json = r#"{
            "leverage": 20,
            "maxQty": "100000.00000000",
            "symbol": "BTCUSD_PERP"
        }"#;

        let response: ChangeInitialLeverageResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.leverage, 20);
        assert_eq!(response.max_qty, "100000.00000000");
        assert_eq!(response.symbol, "BTCUSD_PERP");
    }

    #[test]
    fn test_change_initial_leverage_response_deserialization_low_leverage() {
        let json = r#"{
            "leverage": 1,
            "maxQty": "1000000.00000000",
            "symbol": "ETHUSD_240329"
        }"#;

        let response: ChangeInitialLeverageResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.leverage, 1);
        assert_eq!(response.max_qty, "1000000.00000000");
        assert_eq!(response.symbol, "ETHUSD_240329");
    }
}
