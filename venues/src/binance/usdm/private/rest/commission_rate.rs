use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::RestResult;

/// Endpoint path for the user commission rate API.
const COMMISSION_RATE_ENDPOINT: &str = "/fapi/v1/commissionRate";

/// Request parameters for the user commission rate endpoint.
///
/// See the [Binance USDT-margined Futures API documentation][docs] for details.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCommissionRateRequest {
    /// Trading symbol (e.g., "BTCUSDT").
    pub symbol: String,

    /// The value cannot be greater than 60000. Optional. The value cannot be less than 10000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp (milliseconds since epoch).
    pub timestamp: u64,
}

/// Response for the user commission rate endpoint.
///
/// Contains commission rate information for a given symbol.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommissionRateResponse {
    /// Trading symbol.
    pub symbol: String,

    /// Maker commission rate as a decimal string (e.g., "0.0002").
    pub maker_commission_rate: String,

    /// Taker commission rate as a decimal string (e.g., "0.0004").
    pub taker_commission_rate: String,
}

impl UsdmClient {
    /// User Commission Rate (USER_DATA)
    ///
    /// Get User Commission Rate.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/User-Commission-Rate
    ///
    /// Rate limit: 20
    ///
    /// # Arguments
    /// * `request` - The commission rate request parameters
    ///
    /// # Returns
    /// Commission rate information including maker and taker rates
    pub async fn get_commission_rate(
        &self,
        request: GetCommissionRateRequest,
    ) -> RestResult<CommissionRateResponse> {
        self.send_signed_request(COMMISSION_RATE_ENDPOINT, Method::GET, request, 20, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_commission_rate_request_serialization() {
        let request = GetCommissionRateRequest {
            symbol: "BTCUSDT".to_string(),
            recv_window: Some(5000),
            timestamp: 1234567890,
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        // Order is not guaranteed, so check for all parts
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1234567890"));
    }

    #[test]
    fn test_get_commission_rate_request_serialization_optional_recv_window() {
        let request = GetCommissionRateRequest {
            symbol: "BTCUSDT".to_string(),
            recv_window: None,
            timestamp: 1234567890,
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(!serialized.contains("recvWindow"));
        assert!(serialized.contains("timestamp=1234567890"));
    }

    #[test]
    fn test_commission_rate_response_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "makerCommissionRate": "0.0002",
            "takerCommissionRate": "0.0004"
        }"#;

        let response: CommissionRateResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTCUSDT");
        assert_eq!(response.maker_commission_rate, "0.0002");
        assert_eq!(response.taker_commission_rate, "0.0004");
    }
}
