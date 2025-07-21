use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::RestResult;

const COMMISSION_RATE_ENDPOINT: &str = "/fapi/v1/commissionRate";

/// Request parameters for the user commission rate endpoint.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCommissionRateRequest {
    /// Trading symbol (e.g., "BTCUSDT").
    pub symbol: String,
}

/// User commission rate response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommissionRateResponse {
    /// Trading symbol.
    pub symbol: String,

    /// Maker commission rate as a decimal string.
    pub maker_commission_rate: String,

    /// Taker commission rate as a decimal string.
    pub taker_commission_rate: String,
}

impl UsdmClient {
    /// User Commission Rate (USER_DATA)
    ///
    /// Get User Commission Rate
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
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=BTCUSDT");
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
