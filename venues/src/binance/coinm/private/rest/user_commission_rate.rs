use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, private::rest::client::RestClient};

const COMMISSION_RATE_ENDPOINT: &str = "/dapi/v1/commissionRate";

/// Request parameters for getting user commission rate (GET /dapi/v1/commissionRate).
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetUserCommissionRateRequest {
    /// Trading symbol (e.g., "BTCUSD_PERP").
    pub symbol: String,

    /// Receive window in milliseconds.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp in milliseconds.
    pub timestamp: u64,
}

/// Response for getting user commission rate (GET /dapi/v1/commissionRate).
#[derive(Debug, Clone, Deserialize)]
pub struct GetUserCommissionRateResponse {
    /// Trading symbol.
    pub symbol: String,

    /// Maker commission rate (e.g., "0.00015" for 0.015%).
    #[serde(rename = "makerCommissionRate")]
    pub maker_commission_rate: String,

    /// Taker commission rate (e.g., "0.00040" for 0.040%).
    #[serde(rename = "takerCommissionRate")]
    pub taker_commission_rate: String,
}

impl RestClient {
    /// Gets user commission rate (USER_DATA) on Binance Coin-M Futures.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/coin-margined-futures/account/rest-api/User-Commission-Rate
    ///
    /// GET /dapi/v1/commissionRate
    /// Weight: 20
    /// Requires API key and signature.
    ///
    /// Query user commission rate.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`GetUserCommissionRateRequest`])
    ///
    /// # Returns
    /// A [`GetUserCommissionRateResponse`] with the commission rates.
    pub async fn get_user_commission_rate(
        &self,
        params: GetUserCommissionRateRequest,
    ) -> RestResult<GetUserCommissionRateResponse> {
        let weight = 20;
        self.send_get_signed_request(COMMISSION_RATE_ENDPOINT, params, weight, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_commission_rate_request_serialization() {
        let request = GetUserCommissionRateRequest {
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
    fn test_user_commission_rate_request_with_recv_window() {
        let request = GetUserCommissionRateRequest {
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
    fn test_user_commission_rate_response_deserialization() {
        let json = r#"{
            "symbol": "BTCUSD_PERP",
            "makerCommissionRate": "0.00015",
            "takerCommissionRate": "0.00040"
        }"#;

        let response: GetUserCommissionRateResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTCUSD_PERP");
        assert_eq!(response.maker_commission_rate, "0.00015");
        assert_eq!(response.taker_commission_rate, "0.00040");
    }

    #[test]
    fn test_user_commission_rate_response_different_rates() {
        let json = r#"{
            "symbol": "ETHUSD_PERP",
            "makerCommissionRate": "0.00020",
            "takerCommissionRate": "0.00050"
        }"#;

        let response: GetUserCommissionRateResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "ETHUSD_PERP");
        assert_eq!(response.maker_commission_rate, "0.00020");
        assert_eq!(response.taker_commission_rate, "0.00050");
    }

    #[test]
    fn test_user_commission_rate_response_zero_rates() {
        let json = r#"{
            "symbol": "BTCUSD_PERP",
            "makerCommissionRate": "0.00000",
            "takerCommissionRate": "0.00000"
        }"#;

        let response: GetUserCommissionRateResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTCUSD_PERP");
        assert_eq!(response.maker_commission_rate, "0.00000");
        assert_eq!(response.taker_commission_rate, "0.00000");
    }
}
